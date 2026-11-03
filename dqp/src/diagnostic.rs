use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub length: usize,
}
impl Diagnostic {
    pub fn at(
        code: &str,
        message: impl Into<String>,
        path: &str,
        source: &str,
        offset: usize,
        length: usize,
    ) -> Self {
        let offset = offset.min(source.len());
        // Offsets supplied by the parser always fall on UTF-8 boundaries.
        let prefix = &source[..offset];
        Self {
            code: code.into(),
            message: message.into(),
            path: path.into(),
            line: prefix.bytes().filter(|b| *b == b'\n').count() + 1,
            column: prefix.rsplit('\n').next().unwrap_or("").chars().count() + 1,
            offset,
            length,
        }
    }
    pub fn plain(code: &str, message: impl Into<String>) -> Self {
        Self::at(code, message, "", "", 0, 0)
    }
    pub fn render(&self, filename: &str, source: &str) -> String {
        let line = source
            .lines()
            .nth(self.line.saturating_sub(1))
            .unwrap_or("");
        // Escape controls in excerpts so an untrusted scenario cannot inject terminal commands.
        let escaped: String = line
            .chars()
            .flat_map(|c| {
                if c.is_control() {
                    c.escape_default().collect::<Vec<_>>()
                } else {
                    vec![c]
                }
            })
            .collect();
        format!(
            "{} — {}\n  {}:{}:{}\n  chemin: {}\n  {}\n  {}^\n",
            self.code,
            self.message,
            filename,
            self.line,
            self.column,
            self.path,
            escaped,
            " ".repeat(self.column.saturating_sub(1).min(200))
        )
    }
}
impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.code, self.message, self.path)
    }
}
impl std::error::Error for Diagnostic {}
