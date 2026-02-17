pub mod javascript;

pub enum ScriptType {
    JavaScript(String),
}

impl ScriptType {
    pub fn available() -> Vec<String> {
        vec!["javascript".to_string()]
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
