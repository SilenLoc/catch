use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::{PreEscaped, html};

use crate::kv_store::KeyValueStore;
use crate::proxy::ProxyCall;

pub struct ProxyView;

impl ProxyView {
    pub fn render(proxy_calls: Vec<ProxyCall>) -> maud::Markup {
        html! {
            header class="mb4 pb3 bb b--black-40 flex items-center justify-between" {
                h2 class="f3 f2-ns fw6 ma0 near-white" { "Proxy" }
                span class="f6 fw4 moon-gray" { "HTTP proxy and recording" }
            }

            @if proxy_calls.is_empty() {
                section class="bg-dark-gray br3 pa3 pa4-ns mv3 shadow-1" {
                    p class="f6 moon-gray i mv0" { "No proxy calls have been recorded yet." }
                }
            } @else {
                div class="overflow-y-scroll" style="max-height: 80vh;" {
                    @for call in proxy_calls {
                        section class="bg-dark-gray br3 pa3 pa4-ns mv3 shadow-1 mr4" {
                            div class="flex items-center justify-between mb3 pb2 bb b--black-40" {
                                div {
                                    span class="f7 fw6 ttu tracked ph2 pv1 br2 mr2 " class={
                                        @if call.method() == "GET" { "bg-blue near-black" }
                                        @else if call.method() == "POST" { "bg-green near-black" }
                                        @else if call.method() == "PUT" { "bg-gold near-black" }
                                        @else if call.method() == "DELETE" { "bg-red white" }
                                        @else { "bg-moon-gray near-black" }
                                    } { (call.method()) }
                                    span class="f6 fw5 light-silver" { (call.path()) }
                                    @if !call.query().is_empty() {
                                        span class="f7 moon-gray" { "?" (call.query()) }
                                    }
                                }
                                div class="tr" {
                                    span class="f7 fw6 ttu tracked ph2 pv1 br2 mr2 " class={
                                        @if call.response_status() >= 200 && call.response_status() < 300 { "bg-green near-black" }
                                        @else if call.response_status() >= 300 && call.response_status() < 400 { "bg-blue near-black" }
                                        @else if call.response_status() >= 400 && call.response_status() < 500 { "bg-gold near-black" }
                                        @else { "bg-red white" }
                                    } { (call.response_status()) }
                                    span class="f7 moon-gray" { (call.timestamp()) }
                                }
                            }

                            div class="mb3" {
                                h4 class="f6 ttu tracked moon-gray mb2" { "Target URL" }
                                pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                    code class="f7 light-blue" { (call.target_url()) }
                                }
                            }

                            div class="flex flex-wrap flex-nowrap-ns gap3" {
                                // Request (left side)
                                div class="w-100 w-50-ns pr3-ns" {
                                    h4 class="f6 ttu tracked moon-gray mb2" { "Request" }

                                    // Request Headers
                                    @if !call.request_headers().is_empty() {
                                        details class="mb3" {
                                            summary class="f7 fw6 moon-gray pointer mb2" { "Headers" }
                                            pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                                code class="f7 moon-gray" {
                                                    @for (key, value) in call.request_headers() {
                                                        (key) ": " (value) "\n"
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Request Body
                                    @if !call.request_body().is_empty() {
                                        div class="mb2 flex items-center justify-between" {
                                            h5 class="f7 ttu tracked moon-gray ma0" { "Body" }
                                            button class="copy-btn bn br2 ph2 pv1 f7 fw6 bg-moon-gray near-black pointer transition"
                                                   onclick={ "copyToClipboard(this, " (PreEscaped(&format!("'{}'", call.request_body().replace('\'', "\\'").replace('\n', "\\n")))) ")" } {
                                                "Copy"
                                            }
                                        }
                                        pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                            code class="f7 light-gray" { (call.request_body()) }
                                        }
                                    } @else {
                                        p class="f7 moon-gray i" { "No request body" }
                                    }
                                }

                                // Response (right side)
                                div class="w-100 w-50-ns pl3-ns mt3 mt0-ns" {
                                    h4 class="f6 ttu tracked moon-gray mb2" { "Response" }

                                    // Response Headers
                                    @if !call.response_headers().is_empty() {
                                        details class="mb3" {
                                            summary class="f7 fw6 moon-gray pointer mb2" { "Headers" }
                                            pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                                code class="f7 moon-gray" {
                                                    @for (key, value) in call.response_headers() {
                                                        (key) ": " (value) "\n"
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Response Body
                                    @if !call.response_body().is_empty() {
                                        div class="mb2 flex items-center justify-between" {
                                            h5 class="f7 ttu tracked moon-gray ma0" { "Body" }
                                            button class="copy-btn bn br2 ph2 pv1 f7 fw6 bg-moon-gray near-black pointer transition"
                                                   onclick={ "copyToClipboard(this, " (PreEscaped(&format!("'{}'", call.response_body().replace('\'', "\\'").replace('\n', "\\n")))) ")" } {
                                                "Copy"
                                            }
                                        }
                                        pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                            code class="f7 near-white" { (call.response_body()) }
                                        }
                                    } @else {
                                        p class="f7 moon-gray i" { "No response body" }
                                    }
                                }
                            }

                            // Hurl file section (full width at bottom)
                            div class="mt3 pt3 bt b--black-40" {
                                div class="flex items-center justify-between mb2" {
                                    h4 class="f6 ttu tracked moon-gray ma0" { ".hurl File" }
                                    button class="copy-btn bn br2 ph2 pv1 f7 fw6 bg-moon-gray near-black pointer transition"
                                           onclick={ "copyToClipboard(this, " (PreEscaped(&format!("'{}'", call.to_hurl().replace('\'', "\\'").replace('\n', "\\n")))) ")" } {
                                        "Copy"
                                    }
                                }
                                pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                                    code class="f7 light-gray" { (call.to_hurl()) }
                                }
                            }
                        }
                    }
                }

                // Copy functionality script
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
