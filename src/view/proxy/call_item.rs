use maud::html;

use crate::proxy::ProxyCall;

use super::{badge, body, headers, hurl_section};

/// Renders a single proxy call item
pub fn render(call: &ProxyCall) -> maud::Markup {
    html! {
        section class="bg-catch-dark br3 pa3 pa4-ns mv3 shadow-1 mr4" {
            // Header with method, path, status, timestamp
            div class="flex items-center justify-between mb3 pb2 bb b--black-40" {
                div {
                    (badge::method_badge(call.method()))
                    span class="f6 fw5 white" { (call.path()) }
                    @if !call.query().is_empty() {
                        span class="f7 white" { "?" (call.query()) }
                    }
                }
                div class="tr" {
                    (badge::status_badge(call.response_status()))
                    span class="f7 white" { (call.timestamp()) }
                }
            }

            // Target URL
            div class="mb3" {
                h4 class="f6 ttu tracked white mb2" { "Target URL" }
                pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                    code class="f7 light-blue" { (call.target_url()) }
                }
            }

            // Request and Response columns
            div class="flex flex-wrap flex-nowrap-ns gap3" {
                // Request (left side)
                div class="w-100 w-50-ns pr3-ns" {
                    h4 class="f6 ttu tracked white mb2" { "Request" }

                    // Request Headers
                    (headers::render(call.request_headers(), "Headers"))

                    // Request Body
                    (body::render(call.request_body(), "Body", "light-gray"))
                }

                // Response (right side)
                div class="w-100 w-50-ns pl3-ns mt3 mt0-ns" {
                    h4 class="f6 ttu tracked white mb2" { "Response" }

                    // Response Headers
                    (headers::render(call.response_headers(), "Headers"))

                    // Response Body
                    (body::render(call.response_body(), "Body", "near-white"))
                }
            }

            // Hurl file section (full width at bottom)
            (hurl_section::render(&call.to_hurl()))
        }
    }
}
