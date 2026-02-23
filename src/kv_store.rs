use actix_web::{delete, http::header::ContentType};
use std::{collections::HashMap, sync::Mutex};

use actix_web::{HttpRequest, HttpResponse, Responder, get, http::header::HeaderValue, post, web};
use serde_json::Value;

use crate::script::Script;

pub type KeyValueStoreInner = Mutex<HashMap<String, HashMap<String, String>>>;
pub type KeyValueStore = KeyValueStoreWrapper;

pub struct KeyValueStoreWrapper {
    store: KeyValueStoreInner,
}

const SCRIPT_CONTEXT: &str = "_catch_script";

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, context: impl Into<String>, key: impl Into<String>) -> Option<String> {
        let store = self.store.lock().unwrap();
        store
            .get(&context.into())
            .and_then(|inner| inner.get(&key.into()).cloned())
    }

    pub fn insert(
        &self,
        context: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> String {
        let mut store = self.store.lock().unwrap();

        let context: String = context.into().clone();
        let Some(inner) = store.get_mut(&context) else {
            // no context found, create context
            let mut k = HashMap::new();
            k.insert(key.into(), value.into());
            store.insert(context, k);
            return "Key Set".into();
        };

        inner.insert(key.into(), value.into());

        "Key Set".into()
    }

    pub fn remove(&self, context: impl Into<String>, key: impl Into<String>) -> String {
        let mut store = self.store.lock().unwrap();

        let Some(inner) = store.get_mut(&context.into()) else {
            // no context found, create context
            return "Key did not exist".into();
        };

        inner.remove(&key.into());

        "Key Removed".into()
    }

    #[allow(dead_code)]
    pub fn get_script(&self, name: impl Into<String>) -> Result<Script, String> {
        let Some(json) = self.get(SCRIPT_CONTEXT, name.into()) else {
            return Err("Script not found".into());
        };
        let s = Script::from_json(&json).map_err(|e| e.to_string())?;
        Ok(s)
    }

    pub fn insert_script(&self, script_with_result: &Script) -> String {
        self.insert(
            SCRIPT_CONTEXT,
            script_with_result.name(),
            script_with_result.as_json(),
        )
    }

    pub fn inner(&self) -> &KeyValueStoreInner {
        &self.store
    }
}

#[get("/kv/{key}")]
pub async fn get_kv(
    req: HttpRequest,
    key: web::Path<String>,
    store: web::Data<KeyValueStore>,
) -> impl Responder {
    let static_def = &HeaderValue::from_str("default").unwrap();
    let context = req
        .headers()
        .get("X-Context")
        .unwrap_or(static_def)
        .to_str()
        .unwrap();

    // check if key exists
    let Some(value) = store.get(context, key.clone()) else {
        return HttpResponse::NotFound().body("Key not found");
    };

    // check if value is json, else return as plaintext
    let Ok(json) = serde_json::from_str::<Value>(&value) else {
        return HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(value.clone());
    };

    // return json objects and arrays as json, else plaintext (number, string, boolean)
    match json {
        Value::Object(obj) => HttpResponse::Ok().json(obj),
        Value::Array(arr) => HttpResponse::Ok().json(arr),
        _ => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(value.clone()),
    }
}

#[post("/kv/{key}")]
pub async fn set_kv(
    req: HttpRequest,
    key: web::Path<String>,
    value: String,
    store: web::Data<KeyValueStore>,
) -> impl Responder {
    let static_def = &HeaderValue::from_str("default").unwrap();
    let context = req
        .headers()
        .get("X-Context")
        .unwrap_or(static_def)
        .to_str()
        .unwrap();

    let _ = store.insert(context, key.clone(), value);
    HttpResponse::Ok().body("Key set")
}

#[delete("/kv/{key}")]
pub async fn delete_kv(
    req: HttpRequest,
    key: web::Path<String>,
    store: web::Data<KeyValueStore>,
) -> impl Responder {
    let static_def = &HeaderValue::from_str("default").unwrap();
    let context = req
        .headers()
        .get("X-Context")
        .unwrap_or(static_def)
        .to_str()
        .unwrap();

    let key: String = key.to_string();
    let _ = store.remove(context, key);
    HttpResponse::NoContent()
}
