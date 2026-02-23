use std::collections::HashMap;

use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::{PreEscaped, html};

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
    let kv_snapshot = store.inner().lock().unwrap().clone();

    Ok(KeyValueView::render(&kv_snapshot))
}

fn render_hash_map(hash_map: HashMap<String, String>) -> maud::Markup {
    html! {
        @if hash_map.is_empty() {
            p class="f6 moon-gray i mv0" { "No entries in this namespace yet." }
        } @else {
            ul class="list pl0 ma0" {
                @for (key, value) in hash_map {
                    li class="flex items-baseline pv2 bt b--black-40 relative" {
                        div class="w-40 pr3 flex items-center" {
                            span class="f6 fw6 light-gray truncate flex-auto" { (key) }
                            button class="copy-btn bn br2 ph2 pv1 f7 fw6 bg-moon-gray near-black pointer transition ml2 flex-shrink-0"
                                   onclick={ "copyToClipboard(this, '" (&key.replace('\'', "\\'")) "')" } {
                                "Copy"
                            }
                        }
                        div class="w-60 flex items-center" {
                            span class="f6 lh-copy near-white flex-auto" { (value) }
                            button class="copy-btn bn br2 ph2 pv1 f7 fw6 bg-moon-gray near-black pointer transition ml2 flex-shrink-0"
                                   onclick={ "copyToClipboard(this, '" (&value.replace('\'', "\\'")) "')" } {
                                "Copy"
                            }
                        }
                    }
                }
            }

            script {
                (PreEscaped("
                    function copyToClipboard(btn, text) {
                        navigator.clipboard.writeText(text).then(function() {
                            var originalText = btn.textContent;
                            btn.textContent = 'Copied!';
                            btn.classList.add('bg-green');
                            btn.classList.remove('bg-moon-gray');
                            setTimeout(function() {
                                btn.textContent = originalText;
                                btn.classList.remove('bg-green');
                                btn.classList.add('bg-moon-gray');
                            }, 2000);
                        }).catch(function(err) {
                            btn.textContent = 'Failed';
                            setTimeout(function() {
                                btn.textContent = 'Copy';
                            }, 2000);
                        });
                    }
                "))
            }
        }
    }
}
