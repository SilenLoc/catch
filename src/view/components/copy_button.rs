use maud::html;

/// Renders a copy button that copies text to clipboard
pub fn render(text: &str) -> maud::Markup {
    // Use base64 encoding to safely store the text in a data attribute
    let encoded =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, text.as_bytes());

    html! {
        button class="copy-btn bn br2 ph2 pv1 f7 fw6 bg-black-80 catch-green pointer transition hover-bg-catch-green hover-white"
               data-copy-text=(encoded)
               onclick="copyToClipboard(this)" {
            "Copy"
        }
    }
}

/// Renders the script tag to load the copy button JavaScript
/// This should be included once per page that uses copy buttons
pub fn script() -> maud::Markup {
    html! {
        script src="/assets/copy-button.js" {}
    }
}
