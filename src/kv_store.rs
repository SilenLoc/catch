use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, http::header::HeaderValue, post, web,
};

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

    let Some(inner) = store.get(context) else {
        // no context found, means key not found
        return HttpResponse::NotFound().body("Key not found");
    };

    match inner.get(&key.clone()) {
        // todo return the value as a string
        Some(value) => HttpResponse::Ok().json(value),
        None => HttpResponse::NotFound().body("Key not found"),
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
