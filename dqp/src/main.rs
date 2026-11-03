use clap::{Parser, Subcommand, ValueEnum};
use dqp::{
    compiler,
    diagnostic::Diagnostic,
    engine::{self, Input, Report, Status},
    model::Feedback,
    source, Engine,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, BufRead, Read, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::NamedTempFile;

#[derive(Parser)]
#[command(version, about = "DQP — questionnaires déterministes compilés")]
struct Cli {
    #[arg(long, global = true, value_enum, default_value = "human")]
    diagnostics: Format,
    #[command(subcommand)]
    command: Command,
}
#[derive(Clone, ValueEnum)]
enum Format {
    Human,
    Json,
}
#[derive(Subcommand)]
enum Command {
    /// Vérifier un scénario ou un artefact, sans exécution.
    Check { input: PathBuf },
    /// Compiler un scénario JSON, sans écraser de fichier existant.
    Compile {
        input: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Exécuter un scénario ou un artefact. :quit interrompt la session.
    Run {
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Rejouer les réponses et vérifier tous les résultats déterministes.
    Replay { input: PathBuf, report: PathBuf },
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SessionFile {
    result: Report,
    metadata: Metadata,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    started_unix_ms: u128,
    updated_unix_ms: u128,
    reason: String,
    feedback_written: Vec<String>,
}
struct Failure {
    diagnostics: Vec<Diagnostic>,
    file: String,
    source: String,
    exit: i32,
}
type AppResult<T> = Result<T, Failure>;
fn fail(message: impl Into<String>) -> Failure {
    Failure {
        diagnostics: vec![Diagnostic::plain("DQP-E400", message)],
        file: String::new(),
        source: String::new(),
        exit: 4,
    }
}
fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
fn read(path: &Path) -> AppResult<String> {
    let f = fs::File::open(path)
        .map_err(|e| fail(format!("Lecture de {} impossible : {e}", path.display())))?;
    let mut text = String::new();
    f.take((source::MAX_BYTES + 1) as u64)
        .read_to_string(&mut text)
        .map_err(|e| fail(format!("UTF-8 ou lecture invalide : {e}")))?;
    if text.len() > source::MAX_BYTES {
        return Err(fail("Fichier trop volumineux."));
    }
    Ok(text)
}
fn load(path: &Path) -> AppResult<compiler::Artifact> {
    let text = read(path)?;
    compiler::load(&text).map_err(|diagnostics| Failure {
        diagnostics,
        file: path.display().to_string(),
        source: text,
        exit: 2,
    })
}
fn save<T: Serialize>(path: &Path, value: &T, new: bool) -> AppResult<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temp = NamedTempFile::new_in(parent)
        .map_err(|e| fail(format!("Création du fichier temporaire impossible : {e}")))?;
    serde_json::to_writer_pretty(&mut temp, value)
        .map_err(|e| fail(format!("Écriture impossible : {e}")))?;
    temp.write_all(b"\n").map_err(|e| fail(e.to_string()))?;
    temp.as_file().sync_all().map_err(|e| fail(e.to_string()))?;
    if new {
        temp.persist_noclobber(path).map_err(|e| {
            fail(format!(
                "Impossible de créer {} (aucun écrasement autorisé) : {e}",
                path.display()
            ))
        })?;
    } else {
        temp.persist(path)
            .map_err(|e| fail(format!("Sauvegarde impossible : {e}")))?;
    }
    Ok(())
}
fn safe(text: &str) -> String {
    text.chars()
        .flat_map(|c| {
            if c.is_control() && c != '\n' && c != '\t' {
                c.escape_default().collect::<Vec<_>>()
            } else {
                vec![c]
            }
        })
        .collect()
}
fn print_line(text: &str) -> AppResult<()> {
    writeln!(io::stdout().lock(), "{}", safe(text))
        .map_err(|e| fail(format!("Sortie terminal indisponible : {e}")))
}
fn prompt(
    label: &str,
    rx: &mpsc::Receiver<io::Result<String>>,
    stopped: &AtomicBool,
) -> AppResult<Option<String>> {
    print_line(label)?;
    loop {
        if stopped.load(Ordering::SeqCst) {
            return Ok(None);
        }
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(line)) => return Ok(Some(line)),
            Ok(Err(e)) => return Err(fail(format!("Lecture du terminal : {e}"))),
            Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(None),
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }
}
fn run(input: &Path, output: Option<PathBuf>) -> AppResult<()> {
    let artifact = load(input)?;
    let mut engine = Engine::new(artifact).map_err(|diagnostics| Failure {
        diagnostics,
        file: input.display().to_string(),
        source: String::new(),
        exit: 2,
    })?;
    let output = output
        .unwrap_or_else(|| PathBuf::from(format!("session-{}-{}.json", now(), std::process::id())));
    let started = now();
    let mut session = SessionFile {
        result: engine.report(),
        metadata: Metadata {
            started_unix_ms: started,
            updated_unix_ms: started,
            reason: "in_progress".into(),
            feedback_written: vec![],
        },
    };
    // Reserve output and verify write access BEFORE asking the first question.
    save(&output, &session, true)?;
    let stopped = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stopped);
    ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst))
        .map_err(|e| fail(e.to_string()))?;
    let (tx, rx) = mpsc::sync_channel(8);
    std::thread::spawn(move || {
        for line in io::stdin().lock().lines() {
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    let interaction: AppResult<String> = (|| {
        print_line("DQP — sélectionnez l’identifiant du choix. Justification facultative ; :quit pour arrêter.")?;
        while let Some(q) = engine.current().cloned() {
            print_line(&format!("\n[{}] {}", q.id, q.prompt))?;
            for c in &q.choices {
                print_line(&format!("  {} — {}", c.id, c.text))?;
            }
            let choice = loop {
                let Some(raw) = prompt("Votre choix :", &rx, &stopped)? else {
                    return Ok("input_closed_or_signal".into());
                };
                let choice = raw.trim().to_owned();
                if choice == ":quit" {
                    return Ok("user_quit".into());
                }
                if q.choices.iter().any(|c| c.id == choice) {
                    break choice;
                }
                print_line("DQP-E301 — Choix inconnu. Saisissez un identifiant affiché ; les compteurs sont inchangés.")?;
            };
            let justification = loop {
                let Some(text) = prompt("Justification (Entrée pour passer) :", &rx, &stopped)?
                else {
                    return Ok("input_closed_or_signal".into());
                };
                if text == ":quit" {
                    return Ok("user_quit".into());
                }
                if text.len() > 8192 {
                    print_line("DQP-E302 — Justification trop longue (8192 octets maximum).")?;
                    continue;
                }
                break if text.is_empty() { None } else { Some(text) };
            };
            let step = engine
                .submit(Input {
                    choice,
                    justification,
                })
                .map_err(|d| Failure {
                    diagnostics: vec![d],
                    file: String::new(),
                    source: String::new(),
                    exit: 3,
                })?
                .clone();
            session.result = engine.report();
            session.metadata.updated_unix_ms = now();
            save(&output, &session, false)?;
            if step.feedback_after_answer {
                print_line(&format!(
                    "{} Bonne réponse : {}. {}",
                    if step.correct {
                        "Correct."
                    } else {
                        "Incorrect."
                    },
                    q.answer.choice,
                    q.answer.explanation
                ))?;
                session.metadata.feedback_written.push(q.id.clone());
            }
        }
        if engine.feedback() == &Feedback::End {
            print_line("\nCorrections :")?;
            for s in &engine.report().steps {
                print_line(&format!(
                    "{} — {} : {}",
                    s.question.id, s.question.answer.choice, s.question.answer.explanation
                ))?;
                session
                    .metadata
                    .feedback_written
                    .push(s.question.id.clone());
            }
        }
        Ok("completed".into())
    })();
    session.result = engine.report();
    session.metadata.updated_unix_ms = now();
    session.metadata.reason = interaction
        .as_ref()
        .cloned()
        .unwrap_or_else(|_| "io_error".into());
    // Checkpoints from previous accepted answers remain if this final write fails.
    save(&output, &session, false)?;
    interaction?;
    print_line(&format!(
        "\nRésultat : {}/{}. Rapport : {}",
        session.result.counters.correct_count,
        session.result.counters.answered_count,
        output.display()
    ))?;
    if session.result.status == Status::Interrupted {
        return Err(Failure {
            diagnostics: vec![Diagnostic::plain(
                "DQP-I001",
                "Session interrompue ; rapport partiel sauvegardé.",
            )],
            file: String::new(),
            source: String::new(),
            exit: 130,
        });
    }
    Ok(())
}
fn execute(command: Command) -> AppResult<()> {
    match command {
        Command::Check { input } => {
            let a = load(&input)?;
            print_line(&format!(
                "Valide : {} ({} questions).",
                a.program.scenario.title,
                a.program.scenario.questions.len()
            ))
        }
        Command::Compile { input, out } => {
            let text = read(&input)?;
            let a = compiler::compile(&text).map_err(|diagnostics| Failure {
                diagnostics,
                file: input.display().to_string(),
                source: text,
                exit: 2,
            })?;
            save(&out, &a, true)?;
            print_line(&format!("Compilation réussie : {}", out.display()))
        }
        Command::Run { input, output } => run(&input, output),
        Command::Replay { input, report } => {
            let a = load(&input)?;
            let text = read(&report)?;
            let parsed = source::parse(&text).map_err(|d| Failure {
                diagnostics: vec![d],
                file: report.display().to_string(),
                source: text.clone(),
                exit: 2,
            })?;
            let session: SessionFile = compiler::decode(&parsed, &text).map_err(|d| Failure {
                diagnostics: vec![d],
                file: report.display().to_string(),
                source: text,
                exit: 2,
            })?;
            engine::replay(a, &session.result).map_err(|d| Failure {
                diagnostics: vec![d],
                file: report.display().to_string(),
                source: String::new(),
                exit: 3,
            })?;
            print_line("Rejeu conforme : réponses, compteurs, règles et parcours identiques.")
        }
    }
}
fn main() {
    let cli = Cli::parse();
    if let Err(failure) = execute(cli.command) {
        let mut stderr = io::stderr().lock();
        match cli.diagnostics {
            Format::Json => {
                let _ = serde_json::to_writer_pretty(&mut stderr, &failure.diagnostics);
                let _ = writeln!(stderr);
            }
            Format::Human => {
                for d in failure.diagnostics {
                    let _ = writeln!(
                        stderr,
                        "{}",
                        safe(&d.render(&failure.file, &failure.source))
                    );
                }
            }
        }
        std::process::exit(failure.exit);
    }
}
