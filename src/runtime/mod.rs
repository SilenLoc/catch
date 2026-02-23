use serde::{Deserialize, Serialize};

pub mod javascript;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScriptLanguage {
    JavaScript,
}

impl ScriptLanguage {
    pub fn as_str(&self) -> &str {
        match self {
            ScriptLanguage::JavaScript => "javascript",
        }
    }
}

pub enum ScriptType {
    JavaScript(String),
}

impl ScriptType {
    pub fn available() -> Vec<String> {
        vec!["javascript".to_string()]
    }

    pub fn language(&self) -> ScriptLanguage {
        match self {
            ScriptType::JavaScript(_) => ScriptLanguage::JavaScript,
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
        }
    }
}
