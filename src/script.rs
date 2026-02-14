use actix_web::{HttpRequest, HttpResponse, Responder, post};
use runtime::{Run, javascript::JavaScriptRuntime};
pub mod runtime;

#[post("/script")]
pub async fn run(req: HttpRequest, script: String) -> impl Responder {
    let language = req
        .headers()
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/javascript")
        .to_string();

    let result: Result<String, String> = match language.as_str() {
        "application/javascript" => JavaScriptRuntime.run(req, script),
        _ => Err("Supported Content-Types: [application/javascript]".to_owned()),
    };

    match result {
        Ok(value) => HttpResponse::Ok().body(value),
        Err(error_msg) => HttpResponse::BadRequest().body(error_msg),
    }
}
