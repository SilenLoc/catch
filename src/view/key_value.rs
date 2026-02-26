use std::collections::HashMap;

use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::html;

use crate::kv_store::KeyValueStore;

use super::components;

pub struct KeyValueView;

impl KeyValueView {
    pub fn render(kv: &HashMap<String, HashMap<String, String>>) -> maud::Markup {
        let kv = kv.clone();

        html! {
            (components::page_header::render("Key-Value Store", "In-memory namespaces and values"))

            div class="overflow-y-auto" style="max-height: calc(100vh - 8rem);" {
                @for (key, value) in kv {
                    section class="bg-catch-dark br3 pa3 pa4-ns mv3 shadow-1 mr4" {
                        h3 class="f6 ttu tracked white ma0 mb3" { (key) }
                        (render_hash_map(value))
                    }
                }
            }
        }
    }
}

#[get("/ui/kv")]
pub async fn kv_page(store: web::Data<KeyValueStore>) -> AwResult<maud::Markup> {
    let kv_snapshot = store.inner().lock().unwrap().clone();

    Ok(KeyValueView::render(&kv_snapshot))
}

/// Attempt to pretty-print a value if it's valid JSON, otherwise return as-is
fn pretty_print_value(value: &str) -> String {
    // Try to parse as JSON and pretty-print
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(value)
        && let Ok(pretty) = serde_json::to_string_pretty(&json_value)
    {
        return pretty;
    }
    // If not valid JSON or pretty-print fails, return original value
    value.to_string()
}

fn render_hash_map(hash_map: HashMap<String, String>) -> maud::Markup {
    html! {
        @if hash_map.is_empty() {
            p class="f6 white i mv0" { "No entries in this namespace yet." }
        } @else {
            ul class="list pl0 ma0" {
                @for (key, value) in hash_map {
                    @let pretty_value = pretty_print_value(&value);
                    li class="pv2 bt b--black-40" {
                        // Key header with copy button
                        div class="flex items-center justify-between mb2" {
                            span class="f6 fw6 white" { (key) }
                            (components::copy_button::render(&key))
                        }
                        // Value section with copy button
                        div class="flex items-start" {
                            div class="flex-auto" {
                                pre class="bg-black-90 br2 pa2 ma0 overflow-x-auto" {
                                    code class="f7 light-gray" { (pretty_value) }
                                }
                            }
                            div class="ml2 flex-shrink-0" {
                                (components::copy_button::render(&value))
                            }
                        }
                    }
                }
            }

            (components::copy_button::script())
        }
    }
}
