use crate::{
    kv_store::KeyValueStore,
    runtime::{self, ScriptLanguage, ScriptType},
};

use super::runtime::RuntimeError;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, body::BoxBody, post, web};
use serde::{Deserialize, Serialize};

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
pub async fn run(
    req: HttpRequest,
    script: String,
    store: web::Data<KeyValueStore>,
) -> impl Responder {
    let input = RequestAndScript {
        request: &req,
        script: &script,
    };

    let script_type = runtime::ScriptType::try_from(input);

    match script_type {
        Ok(script_type) => {
            let result = script_type.run();
            match result {
                Ok(result) => {
                    let language = script_type.language();
                    let s = Script::new(script, result.clone(), language);
                    let _ = store.insert_script(s);
                    HttpResponse::Ok().body(result)
                }
                Err(error) => error.respond_to(&req),
            }
        }
        Err(error) => error.respond_to(&req),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    name: String,
    content: String,
    result: String,
    script_type: ScriptLanguage,
}

impl Script {
    pub fn new(content: String, result: String, script_type: ScriptLanguage) -> Self {
        let name = script_name(&content);
        Script {
            name,
            content,
            result,
            script_type,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn result(&self) -> &str {
        &self.result
    }

    pub fn script_type(&self) -> &ScriptLanguage {
        &self.script_type
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

fn script_name(script_file: &str) -> String {
    script_file
        .lines()
        .take(1)
        .collect::<String>()
        .trim()
        .replace("//", "")
}
