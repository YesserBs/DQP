# DQP · Diverging Questions Protocol

**A questionnaire that adapts to your answers, with an explainable path and an analyzable report.**

Write or generate a small JSON file, run one command, and get your answers and reasoning in a JSON report.

```bash
dqp run python.dqp.json --output result.json
```

**Status: experimental version 0.1.1 · DQP format 1.0 · MIT license.** Version names refer to this project and do not indicate external certification.

## I want to study

1. Open the repository's **Releases** section and download the package for your computer. If no release has been published yet, use the source installation below.
2. Extract the package and follow its `INSTALLATION.md`. Rust and Cargo are not required for a prebuilt package.
3. Open the `starter` directory in VS Code, then run the command above.

Answer with `a`, `b`, or `c`. For each question, you can explain your reasoning or simply press Enter. The `result.json` file contains your answers, corrections, and the reasons for the path that was followed.

To start again without choosing an output name, run `dqp run python.dqp.json`: a new report name will be generated automatically. DQP refuses to overwrite an existing report.

[Installation](docs/INSTALLATION.en.md) · [First questionnaire](docs/QUICKSTART.en.md) · [AI guide](docs/v1/AI_GUIDE.md)

## Ask an AI for a questionnaire

Open the [standalone DQP 1.0 guide](docs/v1/AI_GUIDE.md), copy its public URL, and ask:

> Lis cette documentation : [URL du guide]. Génère uniquement un scénario DQP 1.0 pour réviser les listes Python, niveau débutant, avec un rattrapage sur les indices. Respecte exactement le format décrit. Si tu ne peux pas accéder au guide, indique-le avant de générer le fichier.

Save the result as `python.dqp.json`, then validate it:

```bash
dqp check python.dqp.json
```

Published documentation can be read by an AI with web access. It is not automatically known to every model. The compiler checks technical conformance; review corrections for pedagogical accuracy.

## Why DQP?

- **Explicit adaptation**: an error can trigger a remedial exercise; repeated failures can redirect to the basics.
- **Deterministic execution**: the same inputs, program, and engine version produce the same results.
- **Optional reasoning**: justifications are retained but are not scored automatically.
- **Self-contained report**: questions, choices, corrections, counters, and transition rules are recorded.
- **Local operation**: no account, API key, or AI connection is required.

## One command for each task

| Commande | Utilité |
|---|---|
| `dqp check python.dqp.json` | Validate a scenario without running it |
| `dqp run python.dqp.json` | Answer questions and create a report |
| `dqp compile python.dqp.json --out python.dqpc` | Produce a compiled program |
| `dqp replay python.dqp.json result.json` | Verify the report's decisions |

The user directory does not need `src/` or `Cargo.toml`. Those files belong to the contributor repository.

## I want to develop DQP

Install Rust and Cargo from [rust-lang.org](https://rust-lang.org/tools/install/), then run these commands from this repository:

```bash
cargo test --locked
cargo install --path . --locked
```

Make sure `~/.cargo/bin` is in your PATH. You can then use `dqp` from any directory, including `starter/`. On Ubuntu, compilation also requires the usual linker tools, available in `build-essential`.

[Contribuer](CONTRIBUTING.md) · [Architecture](docs/ARCHITECTURE.md) · [Spécification](docs/SPECIFICATION.md) · [Validation](docs/VALIDATION.md) · [Publier une release](docs/PUBLISHING.md)

## Current scope

Single-choice questions, acyclic paths, integer counters, immediate or final corrections, and free one-line justifications. Traces remain accessible without AI. The engine does not verify the truth of corrections or measure certified pedagogical mastery. It does not provide anti-cheating features or report signatures.

Prebuilt versions are produced by the release workflow. Check the workflow status and the assets that were actually published; the presence of a workflow file alone does not guarantee platform compatibility.
