use crate::runtime::{self, ScriptType};

use super::runtime::RuntimeError;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, body::BoxBody, post};

struct RequestAndScript<'a> {
    pub request: &'a HttpRequest,
    pub script: &'a String,
}

impl TryFrom<RequestAndScript<'_>> for ScriptType {
    type Error = RuntimeError;

    fn try_from(value: RequestAndScript) -> Result<Self, Self::Error> {
        let req = value.request;
        let content_type = if req.content_type().is_empty() {
            "application/javascript"
        } else {
            req.content_type()
        };
        match content_type {
            "application/javascript" => Ok(ScriptType::JavaScript(value.script.to_owned())),
            _ => Err(RuntimeError::UserError(ScriptType::available().join(", "))),
        }
    }
}

impl Responder for RuntimeError {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            RuntimeError::UserError(error_msg) => HttpResponse::BadRequest().body(error_msg),
            RuntimeError::InternalError(error_msg) => {
                HttpResponse::InternalServerError().body(error_msg)
            }
        }
    }
}

#[post("/script")]
pub async fn run(req: HttpRequest, script: String) -> impl Responder {
    let input = RequestAndScript {
        request: &req,
        script: &script,
    };

    let script_type = runtime::ScriptType::try_from(input);

    match script_type {
        Ok(script_type) => {
            let result = script_type.run();
            match result {
                Ok(result) => HttpResponse::Ok().body(result),
                Err(error) => error.respond_to(&req),
            }
        }
        Err(error) => error.respond_to(&req),
    }
}
