use maud::html;

use crate::view::components;

/// Renders the hurl file section with copy button
pub fn render(hurl_content: &str) -> maud::Markup {
    html! {
        div class="mt3 pt3 bt b--black-40" {
            div class="flex items-center justify-between mb2" {
                h4 class="f6 ttu tracked white ma0" { ".hurl File" }
                (components::copy_button::render(hurl_content))
            }
            pre class="bg-black-90 br2 pa3 ma0 overflow-x-auto" {
                code class="f7 white" { (hurl_content) }
            }
        }
    }
}
