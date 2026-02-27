use serde::{Deserialize, Serialize};

pub mod javascript;
pub mod shell;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScriptLanguage {
    JavaScript,
    Shell,
}

impl ScriptLanguage {
    pub fn as_str(&self) -> &str {
        match self {
            ScriptLanguage::JavaScript => "javascript",
            ScriptLanguage::Shell => "shell",
        }
    }
}

pub enum ScriptType {
    JavaScript(String),
    Shell(String),
}

impl ScriptType {
    pub fn available() -> Vec<String> {
        vec!["javascript".to_string(), "shell".to_string()]
    }

    pub fn language(&self) -> ScriptLanguage {
        match self {
            ScriptType::JavaScript(_) => ScriptLanguage::JavaScript,
            ScriptType::Shell(_) => ScriptLanguage::Shell,
        }
    }
}

pub enum RuntimeError {
    UserError(String),
    InternalError(String),
}

impl ScriptType {
    pub fn run(&self) -> Result<String, RuntimeError> {
        match self {
            ScriptType::JavaScript(script) => javascript::run(script.to_owned()),
            ScriptType::Shell(script) => shell::run(script),
        }
    }
}
