use actix_web::HttpRequest;

pub mod javascript;

pub trait Run {
    fn run(&self, req: HttpRequest, value: String) -> Result<String, String>;
}
