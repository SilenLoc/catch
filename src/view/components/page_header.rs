use maud::html;

/// Renders a page header with title and subtitle
pub fn render(title: &str, subtitle: &str) -> maud::Markup {
    html! {
        header class="mb4 pb3 bb b--black-40 flex items-center justify-between" {
            h2 class="f3 f2-ns fw6 ma0 white" { (title) }
            span class="f6 fw4 white" { (subtitle) }
        }
    }
}
