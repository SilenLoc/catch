use std::collections::HashMap;

use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::html;

use crate::kv_store::KeyValueStore;

pub struct KeyValueView;

impl KeyValueView {
    pub fn render(kv: &HashMap<String, HashMap<String, String>>) -> maud::Markup {
        let kv = kv.clone();

        html! {
            header class="mb4 pb3 bb b--black-40 flex items-center justify-between" {
                h2 class="f3 f2-ns fw6 ma0 near-white" { "Key-Value Store" }
                span class="f6 fw4 moon-gray" { "In-memory namespaces and values" }
            }

            @for (key, value) in kv {
                section class="bg-dark-gray br3 pa3 pa4-ns mv3 shadow-1" {
                    h3 class="f6 ttu tracked light-silver ma0 mb3" { (key) }
                    (render_hash_map(value))
                }
            }
        }
    }
}

#[get("/ui/kv")]
pub async fn kv_page(store: web::Data<KeyValueStore>) -> AwResult<maud::Markup> {
    let kv_snapshot = store.get_ref().lock().unwrap().clone();

    Ok(KeyValueView::render(&kv_snapshot))
}

fn render_hash_map(hash_map: HashMap<String, String>) -> maud::Markup {
    html! {
        @if hash_map.is_empty() {
            p class="f6 moon-gray i mv0" { "No entries in this namespace yet." }
        } @else {
            ul class="list pl0 ma0" {
                @for (key, value) in hash_map {
                    li class="flex items-baseline pv2 bt b--black-40" {
                        span class="w-40 pr3 f6 fw6 light-gray truncate" { (key) }
                        span class="w-60 f6 lh-copy near-white" { (value) }
                    }
                }
            }
        }
    }
}
