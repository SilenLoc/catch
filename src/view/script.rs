use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::{PreEscaped, html};

use crate::kv_store::KeyValueStore;
use crate::runtime::ScriptLanguage;
use crate::script::Script;

pub struct ScriptView;

impl ScriptView {
    pub fn render(scripts: Vec<Script>) -> maud::Markup {
        html! {
            header class="mb4 pb3 bb b--black-40 flex items-center justify-between" {
                h2 class="f3 f2-ns fw6 ma0 near-white" { "Scripts" }
                span class="f6 fw4 moon-gray" { "Embedded script engines" }
            }

            @if scripts.is_empty() {
                section class="bg-dark-gray br3 pa3 pa4-ns mv3 shadow-1" {
                    p class="f6 moon-gray i mv0" { "No scripts have been executed yet." }
                }
            } @else {
                div class="overflow-y-scroll" style="max-height: 80vh;" {
                    @for script in scripts {
                        section class="bg-dark-gray br3 pa3 pa4-ns mv3 shadow-1 mr4" {
                            div class="flex items-center justify-between mb3 pb2 bb b--black-40" {
                                h3 class="f5 fw6 light-silver ma0" { (script.name()) }
                                span class="f7 fw6 ttu tracked ph2 pv1 br2 bg-gold near-black" { (script.language().as_str()) }
                            }

                            div class="flex flex-wrap flex-nowrap-ns gap3" {
                                // Script content (left side)
                                div class="w-100 w-50-ns pr3-ns" {
                                    div class="flex items-center justify-between mb2" {
                                        h4 class="f6 ttu tracked moon-gray ma0" { "Script" }
                                        button class="copy-btn bn br2 ph2 pv1 f7 fw6 bg-moon-gray near-black pointer transition"
                                               onclick={ "copyToClipboard(this, " (PreEscaped(&format!("'{}'", script.content().replace('\'', "\\'").replace('\n', "\\n")))) ")" } {
                                            "Copy"
                                        }
                                    }
                                    @if script.language() == &ScriptLanguage::JavaScript {
                                        pre class="br2 ma0" {
                                            code class="language-javascript f7" { (script.content()) }
                                        }
                                    } @else {
                                        pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                            code class="f7 light-gray" { (script.content()) }
                                        }
                                    }
                                }

                                // Result (right side)
                                div class="w-100 w-50-ns pl3-ns mt3 mt0-ns" {
                                    div class="flex items-center justify-between mb2" {
                                        h4 class="f6 ttu tracked moon-gray ma0" { "Result" }
                                        button class="copy-btn bn br2 ph2 pv1 f7 fw6 bg-moon-gray near-black pointer transition"
                                               onclick={ "copyToClipboard(this, " (PreEscaped(&format!("'{}'", script.result().replace('\'', "\\'").replace('\n', "\\n")))) ")" } {
                                            "Copy"
                                        }
                                    }
                                    pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                        code class="f7 near-white" { (script.result()) }
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
}

#[get("/ui/scripts")]
pub async fn script_page(store: web::Data<KeyValueStore>) -> AwResult<maud::Markup> {
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

    Ok(ScriptView::render(scripts))
}
