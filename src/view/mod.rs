use actix_web::Result as AwResult;
use actix_web::{get, web};
use std::collections::HashMap;

use crate::kv_store::KeyValueStore;

#[get("/")]
pub async fn index(store: web::Data<KeyValueStore>) -> AwResult<maud::Markup> {
    let s = store.into_inner().lock().unwrap().clone();
    Ok(render_kv(&s))
}

pub fn render_kv(kv: &HashMap<String, HashMap<String, String>>) -> maud::Markup {
    let kv = kv.clone();
    maud::html! {
       div{
           h2 { "Key-Value Store" }
           ul {
               @for (key, value) in kv {
                   li {
                       label { (key) }
                       span{ (render_hash_map(value))}

                   }
               }
           }
       }
    }
}

fn render_hash_map(hash_map: HashMap<String, String>) -> maud::Markup {
    maud::html! {
        div {
            h2 { "Hash Map" }
            ul {
                @for (key, value) in hash_map {
                    li {
                        label { (key) }
                        span { (value) }
                    }
                }
            }
        }
    }
}
