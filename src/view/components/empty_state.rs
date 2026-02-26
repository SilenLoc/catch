use maud::html;

/// Renders an empty state section with a message
pub fn render(message: &str) -> maud::Markup {
    html! {
        section class="bg-catch-dark br3 pa3 pa4-ns mv3 shadow-1" {
            p class="f6 white i mv0" { (message) }
        }
    }
}
