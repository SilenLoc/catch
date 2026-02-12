use actix_web::{delete, http::header::ContentType};
use std::{collections::HashMap, sync::Mutex};

use actix_web::{HttpRequest, HttpResponse, Responder, get, http::header::HeaderValue, post, web};
use serde_json::Value;

pub type KeyValueStore = Mutex<HashMap<String, HashMap<String, String>>>;

pub fn new() -> KeyValueStore {
    Mutex::new(HashMap::new())
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

    let store = store.lock().unwrap();

    // check if context exists
    let Some(inner) = store.get(context) else {
        return HttpResponse::NotFound().body("Key not found");
    };

    // check if key exists
    let Some(value) = inner.get(&key.clone()) else {
        return HttpResponse::NotFound().body("Key not found");
    };

    // check if value is json, else return as plaintext
    let Ok(json) = serde_json::from_str::<Value>(value) else {
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

    let mut store = store.lock().unwrap();

    let Some(inner) = store.get_mut(context) else {
        // no context found, create context
        let mut k = HashMap::new();
        k.insert(key.clone(), value);
        store.insert(context.to_string(), k);
        return HttpResponse::Ok().body("Key set");
    };

    inner.insert(key.clone(), value);

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

    let mut store = store.lock().unwrap();

    let Some(inner) = store.get_mut(context) else {
        return HttpResponse::NoContent();
    };

    inner.remove(&key.to_string());
    HttpResponse::NoContent()
}
