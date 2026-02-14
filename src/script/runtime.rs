use actix_web::HttpRequest;

pub mod javascript;

pub enum RuntimeError {
    UserError(String),
    InternalError(String),
}

pub trait Run {
    fn run(&self, req: HttpRequest, script: String) -> Result<String, RuntimeError>;
}
