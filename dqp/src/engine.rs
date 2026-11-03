use crate::{
    compiler::{validate_artifact, Artifact},
    diagnostic::Diagnostic,
    model::*,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Counters {
    pub answered_count: u32,
    pub correct_count: u32,
    pub consecutive_errors: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub choice: String,
    #[serde(default)]
    pub justification: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evaluation {
    pub rule_id: String,
    pub matched: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub number: usize,
    pub question: Question,
    pub input: Input,
    pub correct: bool,
    pub counters: Counters,
    pub evaluated_rules: Vec<Evaluation>,
    pub selected_rule: Option<String>,
    pub destination: String,
    /// Corrections scheduled before the next question. Terminal I/O may still fail.
    pub feedback_after_answer: bool,
    pub prior_feedback: Vec<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConceptResult {
    pub answered: u32,
    pub correct: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub report_version: String,
    pub engine_version: String,
    pub source_sha256: String,
    pub scenario_id: String,
    pub title: String,
    pub description: String,
    pub feedback: Feedback,
    pub status: Status,
    pub next_question: Option<String>,
    pub counters: Counters,
    pub concepts: BTreeMap<String, ConceptResult>,
    pub steps: Vec<Step>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Completed,
    Interrupted,
}

pub struct Engine {
    artifact: Artifact,
    current: Option<usize>,
    counters: Counters,
    steps: Vec<Step>,
}
impl Engine {
    pub fn new(artifact: Artifact) -> Result<Self, Vec<Diagnostic>> {
        validate_artifact(&artifact)?;
        let current = Some(artifact.program.entry);
        Ok(Self {
            artifact,
            current,
            counters: Counters::default(),
            steps: vec![],
        })
    }
    pub fn current(&self) -> Option<&Question> {
        self.current
            .map(|i| &self.artifact.program.scenario.questions[i])
    }
    pub fn feedback(&self) -> &Feedback {
        &self.artifact.program.scenario.settings.feedback
    }
    pub fn submit(&mut self, input: Input) -> Result<&Step, Diagnostic> {
        let index = self
            .current
            .ok_or_else(|| Diagnostic::plain("DQP-E300", "La session est déjà terminée."))?;
        let q = &self.artifact.program.scenario.questions[index];
        if !q.choices.iter().any(|c| c.id == input.choice) {
            return Err(Diagnostic::plain(
                "DQP-E301",
                format!("Choix inconnu pour {} : {}.", q.id, input.choice),
            ));
        }
        if input.justification.as_ref().is_some_and(|s| s.len() > 8192) {
            return Err(Diagnostic::plain(
                "DQP-E302",
                "Justification supérieure à 8192 octets UTF-8.",
            ));
        }
        let correct = input.choice == q.answer.choice;
        let counters = Counters {
            answered_count: self.counters.answered_count + 1,
            correct_count: self.counters.correct_count + u32::from(correct),
            consecutive_errors: if correct {
                0
            } else {
                self.counters.consecutive_errors + 1
            },
        };
        let links = &self.artifact.program.transitions[index];
        let mut destination = links.fallback;
        let mut selected_rule = None;
        let mut evaluated_rules = vec![];
        for (i, r) in q.routes.iter().enumerate() {
            let matched = evaluate(&r.when, &counters, correct, &input.choice);
            evaluated_rules.push(Evaluation {
                rule_id: r.id.clone(),
                matched,
            });
            if matched {
                selected_rule = Some(r.id.clone());
                destination = links.routes[i];
                break;
            }
        }
        let prior_feedback = self
            .steps
            .iter()
            .filter(|s| s.feedback_after_answer)
            .map(|s| s.question.id.clone())
            .collect();
        let step = Step {
            number: self.steps.len() + 1,
            question: q.clone(),
            input,
            correct,
            counters: counters.clone(),
            evaluated_rules,
            selected_rule,
            destination: destination
                .map(|i| self.artifact.program.scenario.questions[i].id.clone())
                .unwrap_or_else(|| "end".into()),
            feedback_after_answer: self.feedback() == &Feedback::Immediate,
            prior_feedback,
        };
        self.counters = counters;
        self.current = destination;
        self.steps.push(step);
        Ok(self.steps.last().expect("step inserted"))
    }
    pub fn report(&self) -> Report {
        let mut concepts: BTreeMap<String, ConceptResult> = BTreeMap::new();
        for step in &self.steps {
            for concept in &step.question.concepts {
                let c = concepts.entry(concept.clone()).or_default();
                c.answered += 1;
                c.correct += u32::from(step.correct);
            }
        }
        let scenario = &self.artifact.program.scenario;
        Report {
            report_version: "1.0".into(),
            engine_version: env!("CARGO_PKG_VERSION").into(),
            source_sha256: self.artifact.source_sha256.clone(),
            scenario_id: scenario.id.clone(),
            title: scenario.title.clone(),
            description: scenario.description.clone(),
            feedback: scenario.settings.feedback.clone(),
            status: if self.current.is_none() {
                Status::Completed
            } else {
                Status::Interrupted
            },
            next_question: self.current().map(|q| q.id.clone()),
            counters: self.counters.clone(),
            concepts,
            steps: self.steps.clone(),
        }
    }
}
fn evaluate(c: &Condition, counters: &Counters, correct: bool, choice: &str) -> bool {
    match c {
        Condition::All { conditions } => conditions
            .iter()
            .all(|c| evaluate(c, counters, correct, choice)),
        Condition::Any { conditions } => conditions
            .iter()
            .any(|c| evaluate(c, counters, correct, choice)),
        Condition::Not { condition } => !evaluate(condition, counters, correct, choice),
        Condition::Compare {
            metric,
            operator,
            value,
        } => {
            let actual = match metric {
                Metric::LastCorrect => Scalar::Bool(correct),
                Metric::LastChoice => Scalar::Text(choice.into()),
                Metric::Answered => Scalar::Int(counters.answered_count.into()),
                Metric::Correct => Scalar::Int(counters.correct_count.into()),
                Metric::Errors => Scalar::Int(counters.consecutive_errors.into()),
            };
            match operator {
                Operator::Eq => &actual == value,
                Operator::Ne => &actual != value,
                _ => {
                    if let (Scalar::Int(a), Scalar::Int(b)) = (&actual, value) {
                        match operator {
                            Operator::Lt => a < b,
                            Operator::Lte => a <= b,
                            Operator::Gt => a > b,
                            Operator::Gte => a >= b,
                            _ => false,
                        }
                    } else {
                        false
                    }
                } // Unreachable for compiler-validated programs.
            }
        }
    }
}
/// Recompute and compare every deterministic report field; timestamps belong outside Report.
pub fn replay(artifact: Artifact, report: &Report) -> Result<(), Diagnostic> {
    let mut engine = Engine::new(artifact).map_err(|e| e.into_iter().next().unwrap())?;
    for step in &report.steps {
        engine.submit(step.input.clone())?;
    }
    if engine.report() != *report {
        return Err(Diagnostic::plain("DQP-E310", "Le rapport ne correspond pas au rejeu (version, source, parcours ou résultats différents)."));
    }
    Ok(())
}
