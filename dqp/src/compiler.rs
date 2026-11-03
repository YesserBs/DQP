use crate::{
    diagnostic::Diagnostic,
    model::*,
    source::{self, Source},
};
use serde::{Deserialize, Serialize};
use serde_path_to_error::Segment;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub artifact_version: String,
    pub source_sha256: String,
    pub source: String,
    pub program: Program,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub scenario: Scenario,
    pub entry: usize,
    pub transitions: Vec<Transitions>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Transitions {
    pub routes: Vec<Option<usize>>,
    pub fallback: Option<usize>,
}

pub fn hash(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}
fn diagnostic(
    s: &Source,
    text: &str,
    code: &str,
    path: &str,
    message: impl Into<String>,
) -> Diagnostic {
    let mut location = path.to_owned();
    while !s.spans.contains_key(&location) && !location.is_empty() {
        location = location
            .rsplit_once('/')
            .map(|(p, _)| p.to_owned())
            .unwrap_or_default();
    }
    let (offset, length) = s.spans.get(&location).copied().unwrap_or((0, 0));
    Diagnostic::at(code, message, path, text, offset, length)
}
pub fn decode<T: serde::de::DeserializeOwned>(s: &Source, text: &str) -> Result<T, Diagnostic> {
    serde_path_to_error::deserialize(s.value.clone()).map_err(|error| {
        let mut path = String::new();
        for seg in error.path().iter() {
            match seg {
                Segment::Seq { index } => path.push_str(&format!("/{index}")),
                Segment::Map { key } => {
                    path.push('/');
                    path.push_str(&source::escape(key));
                }
                Segment::Enum { variant } => {
                    path.push('/');
                    path.push_str(&source::escape(variant));
                }
                Segment::Unknown => {}
            }
        }
        diagnostic(
            s,
            text,
            "DQP-E110",
            &path,
            format!("Structure invalide : {}", error.inner()),
        )
    })
}
fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 80
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
}
pub fn compile(text: &str) -> Result<Artifact, Vec<Diagnostic>> {
    if text.len() > 4 * 1024 * 1024 {
        return Err(vec![Diagnostic::plain(
            "DQP-E001",
            "Source supérieur à 4 Mio.",
        )]);
    }
    let source = source::parse(text).map_err(|e| vec![e])?;
    let scenario: Scenario = decode(&source, text).map_err(|e| vec![e])?;
    let mut errors = Vec::new();
    let mut err = |code: &str, path: &str, message: String| {
        errors.push(diagnostic(&source, text, code, path, message))
    };
    if scenario.dqp_version != "1.0" {
        err(
            "DQP-E200",
            "/dqp_version",
            "Version DQP attendue : 1.0.".into(),
        );
    }
    if !valid_id(&scenario.id) {
        err(
            "DQP-E201",
            "/id",
            "Identifiant attendu : 1 à 80 caractères ASCII alphanumériques, _ ou -.".into(),
        );
    }
    for (name, value) in [
        ("title", &scenario.title),
        ("description", &scenario.description),
    ] {
        if value.trim().is_empty() {
            err(
                "DQP-E202",
                &format!("/{name}"),
                "Texte non vide requis.".into(),
            );
        }
    }
    if scenario.questions.is_empty() || scenario.questions.len() > 256 {
        err(
            "DQP-E202",
            "/questions",
            "Il faut entre 1 et 256 questions.".into(),
        );
    }
    let mut ids = BTreeMap::new();
    for (i, q) in scenario.questions.iter().enumerate() {
        let p = format!("/questions/{i}");
        if !valid_id(&q.id) || q.id == "end" {
            err(
                "DQP-E201",
                &format!("{p}/id"),
                "Identifiant invalide ou réservé : end.".into(),
            );
        }
        if ids.insert(q.id.clone(), i).is_some() {
            err(
                "DQP-E203",
                &format!("{p}/id"),
                format!("Question dupliquée : {}.", q.id),
            );
        }
        for (field, content) in [
            ("prompt", &q.prompt),
            ("answer/explanation", &q.answer.explanation),
        ] {
            if content.trim().is_empty() {
                err(
                    "DQP-E202",
                    &format!("{p}/{field}"),
                    "Texte non vide requis.".into(),
                );
            }
        }
        let mut concepts = BTreeSet::new();
        for (j, c) in q.concepts.iter().enumerate() {
            if c.trim().is_empty() || !concepts.insert(c) {
                err(
                    "DQP-E203",
                    &format!("{p}/concepts/{j}"),
                    "Notion vide ou dupliquée.".into(),
                );
            }
        }
        if !(2..=26).contains(&q.choices.len()) {
            err(
                "DQP-E202",
                &format!("{p}/choices"),
                "Il faut entre 2 et 26 propositions.".into(),
            );
        }
        let mut choices = BTreeSet::new();
        for (j, c) in q.choices.iter().enumerate() {
            if !valid_id(&c.id) || !choices.insert(c.id.as_str()) {
                err(
                    "DQP-E203",
                    &format!("{p}/choices/{j}/id"),
                    "Identifiant de choix invalide ou dupliqué.".into(),
                );
            }
            if c.text.trim().is_empty() {
                err(
                    "DQP-E202",
                    &format!("{p}/choices/{j}/text"),
                    "Texte non vide requis.".into(),
                );
            }
        }
        if !choices.contains(q.answer.choice.as_str()) {
            err(
                "DQP-E204",
                &format!("{p}/answer/choice"),
                format!(
                    "Choix correct inconnu : {}. Choix disponibles : {:?}.",
                    q.answer.choice, choices
                ),
            );
        }
        let mut routes = BTreeSet::new();
        for (j, r) in q.routes.iter().enumerate() {
            let rp = format!("{p}/routes/{j}");
            if !valid_id(&r.id) || !routes.insert(&r.id) {
                err(
                    "DQP-E203",
                    &format!("{rp}/id"),
                    "Identifiant de règle invalide ou dupliqué dans la question.".into(),
                );
            }
            validate_condition(&r.when, &format!("{rp}/when"), &choices, &mut err);
        }
    }
    if !ids.contains_key(&scenario.start) {
        err(
            "DQP-E205",
            "/start",
            format!("Question de départ inconnue : {}.", scenario.start),
        );
    }
    let mut transitions = vec![];
    for (i, q) in scenario.questions.iter().enumerate() {
        let mut resolve = |target: &str, path: &str| {
            if target == "end" {
                None
            } else {
                let result = ids.get(target).copied();
                if result.is_none() {
                    err("DQP-E205", path, format!("Destination inconnue : {target}. Destinations disponibles : {} ou end.", ids.keys().cloned().collect::<Vec<_>>().join(", ")));
                }
                result
            }
        };
        let routes = q
            .routes
            .iter()
            .enumerate()
            .map(|(j, r)| resolve(&r.goto, &format!("/questions/{i}/routes/{j}/goto")))
            .collect();
        let fallback = resolve(&q.next, &format!("/questions/{i}/next"));
        transitions.push(Transitions { routes, fallback });
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let entry = ids[&scenario.start];
    // Kahn's algorithm avoids recursion and rejects all cycles, including unreachable ones.
    let n = transitions.len();
    let edges: Vec<Vec<usize>> = transitions
        .iter()
        .map(|t| {
            t.routes
                .iter()
                .chain(std::iter::once(&t.fallback))
                .filter_map(|x| *x)
                .collect()
        })
        .collect();
    let mut incoming = vec![0usize; n];
    for es in &edges {
        for &j in es {
            incoming[j] += 1;
        }
    }
    let mut queue: VecDeque<usize> = (0..n).filter(|&i| incoming[i] == 0).collect();
    let mut count = 0;
    while let Some(i) = queue.pop_front() {
        count += 1;
        for &j in &edges[i] {
            incoming[j] -= 1;
            if incoming[j] == 0 {
                queue.push_back(j);
            }
        }
    }
    if count != n {
        let i = incoming.iter().position(|x| *x > 0).unwrap_or(0);
        errors.push(diagnostic(
            &source,
            text,
            "DQP-E206",
            &format!("/questions/{i}"),
            "Cycle détecté : les parcours DQP 1.0 doivent être acycliques.",
        ));
    }
    let mut reachable = vec![false; n];
    let mut pending = vec![entry];
    while let Some(i) = pending.pop() {
        if !reachable[i] {
            reachable[i] = true;
            pending.extend(&edges[i]);
        }
    }
    for (i, reached) in reachable.iter().enumerate() {
        if !reached {
            errors.push(diagnostic(
                &source,
                text,
                "DQP-E207",
                &format!("/questions/{i}/id"),
                format!(
                    "Question structurellement inaccessible : {}.",
                    scenario.questions[i].id
                ),
            ));
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(Artifact {
        artifact_version: "1.0".into(),
        source_sha256: hash(text),
        source: text.into(),
        program: Program {
            scenario,
            entry,
            transitions,
        },
    })
}
fn validate_condition(
    c: &Condition,
    path: &str,
    choices: &BTreeSet<&str>,
    err: &mut impl FnMut(&str, &str, String),
) {
    match c {
        Condition::Compare {
            metric,
            operator,
            value,
        } => {
            let compatible = matches!(
                (metric, value),
                (Metric::LastCorrect, Scalar::Bool(_))
                    | (Metric::LastChoice, Scalar::Text(_))
                    | (
                        Metric::Answered | Metric::Correct | Metric::Errors,
                        Scalar::Int(_)
                    )
            );
            if !compatible {
                err(
                    "DQP-E210",
                    &format!("{path}/value"),
                    "Type incompatible avec la mesure (booléen, chaîne ou entier attendu).".into(),
                );
            }
            if matches!(metric, Metric::LastCorrect | Metric::LastChoice)
                && !matches!(operator, Operator::Eq | Operator::Ne)
            {
                err(
                    "DQP-E211",
                    &format!("{path}/operator"),
                    "Seuls eq et ne sont autorisés pour les booléens et identifiants.".into(),
                );
            }
            if let Scalar::Int(n) = value {
                if *n < 0 {
                    err(
                        "DQP-E210",
                        &format!("{path}/value"),
                        "Un compteur se compare à un entier positif ou nul.".into(),
                    );
                }
            }
            if let (Metric::LastChoice, Scalar::Text(v)) = (metric, value) {
                if !choices.contains(v.as_str()) {
                    err(
                        "DQP-E204",
                        &format!("{path}/value"),
                        format!("Choix inconnu dans cette question : {v}."),
                    );
                }
            }
        }
        Condition::All { conditions } | Condition::Any { conditions } => {
            if conditions.is_empty() {
                err(
                    "DQP-E212",
                    &format!("{path}/conditions"),
                    "all et any exigent au moins une condition.".into(),
                );
            }
            for (i, child) in conditions.iter().enumerate() {
                validate_condition(child, &format!("{path}/conditions/{i}"), choices, err);
            }
        }
        Condition::Not { condition } => {
            validate_condition(condition, &format!("{path}/condition"), choices, err)
        }
    }
}
/// Load either source JSON or a compiled artifact. Compiled plans are revalidated
/// against embedded source before use. This is consistency checking, not authentication.
pub fn load(text: &str) -> Result<Artifact, Vec<Diagnostic>> {
    let s = source::parse(text).map_err(|e| vec![e])?;
    if s.value.get("artifact_version").is_none() {
        return compile(text);
    }
    let artifact: Artifact = decode(&s, text).map_err(|e| vec![e])?;
    validate_artifact(&artifact)?;
    Ok(artifact)
}
pub fn validate_artifact(artifact: &Artifact) -> Result<(), Vec<Diagnostic>> {
    if artifact.artifact_version != "1.0" {
        return Err(vec![Diagnostic::plain(
            "DQP-E220",
            "Version d’artefact non prise en charge.",
        )]);
    }
    let rebuilt = compile(&artifact.source)?;
    if &rebuilt != artifact {
        return Err(vec![Diagnostic::plain(
            "DQP-E221",
            "Artefact incohérent avec son source ou son empreinte. Recompilez le scénario.",
        )]);
    }
    Ok(())
}
