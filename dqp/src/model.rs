use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub dqp_version: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub start: String,
    #[serde(default)]
    pub settings: Settings,
    pub questions: Vec<Question>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    #[serde(default)]
    pub feedback: Feedback,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Feedback {
    Immediate,
    #[default]
    End,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Question {
    pub id: String,
    #[serde(default)]
    pub concepts: Vec<String>,
    pub prompt: String,
    pub choices: Vec<Choice>,
    pub answer: Answer,
    #[serde(default)]
    pub routes: Vec<Route>,
    pub next: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Choice {
    pub id: String,
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Answer {
    pub choice: String,
    pub explanation: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Route {
    pub id: String,
    pub when: Condition,
    pub goto: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Condition {
    Compare {
        metric: Metric,
        operator: Operator,
        value: Scalar,
    },
    All {
        conditions: Vec<Condition>,
    },
    Any {
        conditions: Vec<Condition>,
    },
    Not {
        condition: Box<Condition>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Metric {
    #[serde(rename = "last_answer.correct")]
    LastCorrect,
    #[serde(rename = "last_answer.choice")]
    LastChoice,
    #[serde(rename = "answered_count")]
    Answered,
    #[serde(rename = "correct_count")]
    Correct,
    #[serde(rename = "consecutive_errors")]
    Errors,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Scalar {
    Bool(bool),
    Int(i64),
    Text(String),
}
