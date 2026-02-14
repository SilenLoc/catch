use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, post};
use runtime::{Run, RuntimeError, javascript::JavaScriptRuntime};
pub mod runtime;

#[post("/script")]
pub async fn run(req: HttpRequest, script: String) -> impl Responder {
    let content_type = if req.content_type().is_empty() {
        "application/javascript"
    } else {
        req.content_type()
    };

    let result: Result<String, RuntimeError> = match content_type {
        "application/javascript" => JavaScriptRuntime.run(req, script),
        _ => Err(RuntimeError::UserError(
            "Supported Content-Types: [application/javascript]".to_owned(),
        )),
    };

    match result {
        Ok(value) => HttpResponse::Ok().body(value),
        Err(e) => match e {
            RuntimeError::UserError(error_msg) => HttpResponse::BadRequest().body(error_msg),
            RuntimeError::InternalError(error_msg) => {
                HttpResponse::InternalServerError().body(error_msg)
            }
        },
    }
}
