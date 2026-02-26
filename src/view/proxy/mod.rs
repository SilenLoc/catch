use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::html;

use crate::kv_store::KeyValueStore;
use crate::proxy::ProxyCall;

use super::components;

mod badge;
mod body;
mod call_item;
mod headers;
mod hurl_section;

pub struct ProxyView;

impl ProxyView {
    pub fn render(proxy_calls: Vec<ProxyCall>) -> maud::Markup {
        html! {
            (components::page_header::render("Proxy", "HTTP proxy and recording"))

            @if proxy_calls.is_empty() {
                (components::empty_state::render("No proxy calls have been recorded yet."))
            } @else {
                div class="overflow-y-scroll" style="max-height: 80vh;" {
                    @for call in proxy_calls {
                        (call_item::render(&call))
                    }
                }

                // Copy functionality script
                (components::copy_button::script())
            }
        }
    }
}

#[get("/ui/proxy")]
pub async fn proxy_page(store: web::Data<KeyValueStore>) -> AwResult<maud::Markup> {
    // Get all proxy calls from the _catch_proxy context
    let store_snapshot = store.inner().lock().unwrap();
    let proxy_calls: Vec<ProxyCall> = store_snapshot
        .get("_catch_proxy")
        .map(|proxy_map| {
            let mut calls: Vec<ProxyCall> = proxy_map
                .values()
                .filter_map(|json| ProxyCall::from_json(json).ok())
                .collect();
            // Sort by timestamp descending (newest first)
            calls.sort_by(|a, b| b.timestamp().cmp(a.timestamp()));
            calls
        })
        .unwrap_or_default();

    drop(store_snapshot);

    Ok(ProxyView::render(proxy_calls))
}
