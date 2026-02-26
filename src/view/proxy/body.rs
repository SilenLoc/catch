use maud::html;

use crate::view::components;

/// Renders a body section (request or response) with copy button
pub fn render(body: &str, title: &str, code_class: &str) -> maud::Markup {
    if body.is_empty() {
        return html! {
            p class="f7 white i" { "No " (title.to_lowercase()) " body" }
        };
    }

    html! {
        div class="mb2 flex items-center justify-between" {
            h5 class="f7 ttu tracked white ma0" { (title) }
            (components::copy_button::render(body))
        }
        pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
            code class={"f7 " (code_class)} { (body) }
        }
    }
}
