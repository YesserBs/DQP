//! JSON reader retaining JSON Pointer -> byte span mappings and rejecting duplicate keys.
use crate::diagnostic::Diagnostic;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

pub const MAX_BYTES: usize = 32 * 1024 * 1024;
pub const MAX_DEPTH: usize = 64;
pub struct Source {
    pub value: Value,
    pub spans: BTreeMap<String, (usize, usize)>,
}
pub fn escape(s: &str) -> String {
    s.replace('~', "~0").replace('/', "~1")
}
pub fn parse(text: &str) -> Result<Source, Diagnostic> {
    if text.len() > MAX_BYTES {
        return Err(Diagnostic::plain("DQP-E001", "Fichier supérieur à 32 Mio."));
    }
    let mut p = Parser {
        text,
        pos: 0,
        spans: BTreeMap::new(),
    };
    let value = p.value("", 0)?;
    p.ws();
    if p.pos != text.len() {
        return Err(p.error("DQP-E100", "Contenu après la fin du JSON.", ""));
    }
    Ok(Source {
        value,
        spans: p.spans,
    })
}
struct Parser<'a> {
    text: &'a str,
    pos: usize,
    spans: BTreeMap<String, (usize, usize)>,
}
impl Parser<'_> {
    fn byte(&self) -> Option<u8> {
        self.text.as_bytes().get(self.pos).copied()
    }
    fn ws(&mut self) {
        while matches!(self.byte(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.pos += 1;
        }
    }
    fn error(&self, code: &str, msg: &str, path: &str) -> Diagnostic {
        Diagnostic::at(code, msg, path, self.text, self.pos, 1)
    }
    fn string(&mut self, path: &str) -> Result<String, Diagnostic> {
        let start = self.pos;
        if self.byte() != Some(b'"') {
            return Err(self.error("DQP-E100", "Chaîne JSON attendue.", path));
        }
        self.pos += 1;
        let mut escaped = false;
        while let Some(b) = self.byte() {
            self.pos += 1;
            if !escaped && b == b'"' {
                return serde_json::from_str(&self.text[start..self.pos]).map_err(|e| {
                    Diagnostic::at(
                        "DQP-E100",
                        e.to_string(),
                        path,
                        self.text,
                        start,
                        self.pos - start,
                    )
                });
            }
            if !escaped && b == b'\\' {
                escaped = true;
            } else {
                escaped = false;
            }
        }
        Err(Diagnostic::at(
            "DQP-E100",
            "Chaîne non terminée.",
            path,
            self.text,
            start,
            1,
        ))
    }
    fn value(&mut self, path: &str, depth: usize) -> Result<Value, Diagnostic> {
        self.ws();
        if depth > MAX_DEPTH {
            return Err(self.error("DQP-E102", "Imbrication JSON supérieure à 64.", path));
        }
        let start = self.pos;
        let value = match self.byte() {
            Some(b'{') => {
                self.pos += 1;
                self.ws();
                let mut map = Map::new();
                if self.byte() == Some(b'}') {
                    self.pos += 1;
                } else {
                    loop {
                        self.ws();
                        let key_start = self.pos;
                        let key = self.string(path)?;
                        let child = format!("{}/{}", path, escape(&key));
                        if map.contains_key(&key) {
                            return Err(Diagnostic::at(
                                "DQP-E101",
                                format!("Clé dupliquée : {key}"),
                                &child,
                                self.text,
                                key_start,
                                self.pos - key_start,
                            ));
                        }
                        self.ws();
                        if self.byte() != Some(b':') {
                            return Err(self.error("DQP-E100", "Deux-points attendu.", &child));
                        }
                        self.pos += 1;
                        map.insert(key, self.value(&child, depth + 1)?);
                        self.ws();
                        match self.byte() {
                            Some(b',') => {
                                self.pos += 1;
                            }
                            Some(b'}') => {
                                self.pos += 1;
                                break;
                            }
                            _ => {
                                return Err(self.error(
                                    "DQP-E100",
                                    "Virgule ou accolade fermante attendue.",
                                    path,
                                ))
                            }
                        }
                    }
                }
                Value::Object(map)
            }
            Some(b'[') => {
                self.pos += 1;
                self.ws();
                let mut a = vec![];
                if self.byte() == Some(b']') {
                    self.pos += 1;
                } else {
                    loop {
                        a.push(self.value(&format!("{}/{}", path, a.len()), depth + 1)?);
                        self.ws();
                        match self.byte() {
                            Some(b',') => {
                                self.pos += 1;
                            }
                            Some(b']') => {
                                self.pos += 1;
                                break;
                            }
                            _ => {
                                return Err(self.error(
                                    "DQP-E100",
                                    "Virgule ou crochet fermant attendu.",
                                    path,
                                ))
                            }
                        }
                    }
                }
                Value::Array(a)
            }
            Some(b'"') => Value::String(self.string(path)?),
            Some(_) => {
                // Advance by Unicode scalar boundaries even for malformed input.
                while let Some(c) = self.text[self.pos..].chars().next() {
                    if c.is_ascii_whitespace() || matches!(c, ',' | ']' | '}' | ':') {
                        break;
                    }
                    self.pos += c.len_utf8();
                }
                serde_json::from_str::<Value>(&self.text[start..self.pos]).map_err(|e| {
                    Diagnostic::at(
                        "DQP-E100",
                        e.to_string(),
                        path,
                        self.text,
                        start,
                        (self.pos - start).max(1),
                    )
                })?
            }
            None => return Err(self.error("DQP-E100", "Valeur JSON attendue.", path)),
        };
        self.spans.insert(path.into(), (start, self.pos - start));
        Ok(value)
    }
}
