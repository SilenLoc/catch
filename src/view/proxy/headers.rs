use std::collections::HashMap;

use maud::html;

/// Renders headers as a collapsible details section
pub fn render(headers: &HashMap<String, String>, title: &str) -> maud::Markup {
    if headers.is_empty() {
        return html! {};
    }

    html! {
        details class="mb3" {
            summary class="f7 fw6 catch-green pointer mb2 hover-catch-green" { (title) }
            pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                code class="f7 white" {
                    @for (key, value) in headers {
                        (key) ": " (value) "\n"
                    }
                }
            }
        }
    }
}
