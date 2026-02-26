use maud::html;

/// Renders a method badge (GET, POST, PUT, DELETE, etc.)
pub fn method_badge(method: &str) -> maud::Markup {
    html! {
        span class="f7 fw6 ttu tracked ph2 pv1 br2 mr2 " class={
            @if method == "GET" { "bg-blue near-black" }
            @else if method == "POST" { "bg-catch-green near-black" }
            @else if method == "PUT" { "bg-gold near-black" }
            @else if method == "DELETE" { "bg-red white" }
            @else { "bg-moon-gray near-black" }
        } { (method) }
    }
}

/// Renders a status code badge (200, 404, 500, etc.)
pub fn status_badge(status: u16) -> maud::Markup {
    html! {
        span class="f7 fw6 ttu tracked ph2 pv1 br2 mr2 " class={
            @if (200..300).contains(&status) { "bg-catch-green near-black" }
            @else if (300..400).contains(&status) { "bg-blue near-black" }
            @else if (400..500).contains(&status) { "bg-gold near-black" }
            @else { "bg-red white" }
        } { (status) }
    }
}
