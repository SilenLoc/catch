use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get, web};
use maud::{PreEscaped, html};

use crate::kv_store::KeyValueStore;
use crate::runtime::ScriptLanguage;
use crate::script::Script;

use super::components;

pub struct ScriptView;

impl ScriptView {
    pub fn render(scripts: Vec<Script>) -> maud::Markup {
        html! {
            (components::page_header::render("Scripts", "Embedded script engines"))

            @if scripts.is_empty() {
                (components::empty_state::render("No scripts have been executed yet."))
            } @else {
                div class="overflow-y-scroll" style="max-height: 80vh;" {
                    @for script in scripts {
                        section class="bg-catch-dark br3 pa3 pa4-ns mv3 shadow-1 mr4" {
                            div class="flex items-center justify-between mb3 pb2 bb b--black-40" {
                                h3 class="f5 fw6 white ma0" { (script.name()) }
                                span class="f7 fw6 ttu tracked ph2 pv1 br2 bg-gold near-black" { (script.language().as_str()) }
                            }

                            div class="flex flex-wrap flex-nowrap-ns gap3" {
                                // Script content (left side)
                                div class="w-100 w-50-ns pr3-ns" {
                                    div class="flex items-center justify-between mb2" {
                                        h4 class="f6 ttu tracked white ma0" { "Script" }
                                        (components::copy_button::render(script.content()))
                                    }
                                    @if script.language() == &ScriptLanguage::JavaScript {
                                        pre class="br2 ma0" {
                                            code class="language-javascript f7" { (script.content()) }
                                        }
                                    } @else {
                                        pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                            code class="f7 white" { (script.content()) }
                                        }
                                    }
                                }

                                // Result (right side)
                                div class="w-100 w-50-ns pl3-ns mt3 mt0-ns" {
                                    div class="flex items-center justify-between mb2" {
                                        h4 class="f6 ttu tracked white ma0" { "Result" }
                                        (components::copy_button::render(script.result()))
                                    }
                                    pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                        code class="f7 white" { (script.result()) }
                                    }
                                }
                            }
                        }
                    }
                }

                // Trigger Prism syntax highlighting after HTMX loads content
                script {
                    (PreEscaped("
                        if (typeof Prism !== 'undefined') {
                            Prism.highlightAll();
                        }
                    "))
                }

                // Copy functionality script
                (components::copy_button::script())
            }
        }
    }
}

#[get("/ui/scripts")]
pub async fn script_page(
    req: HttpRequest,
    store: web::Data<KeyValueStore>,
) -> AwResult<maud::Markup> {
    // Get all scripts from the _catch_script context
    let store_snapshot = store.inner().lock().unwrap();
    let scripts: Vec<Script> = store_snapshot
        .get("_catch_script")
        .map(|script_map| {
            script_map
                .values()
                .filter_map(|json| Script::from_json(json).ok())
                .collect()
        })
        .unwrap_or_default();

    drop(store_snapshot);

    let content = ScriptView::render(scripts);

    // Check if this is an htmx request
    if req.headers().get("HX-Request").is_some() {
        Ok(content)
    } else {
        Ok(super::render_layout(&content))
    }
}
