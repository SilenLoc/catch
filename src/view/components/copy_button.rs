use maud::{PreEscaped, html};

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

/// Renders the JavaScript copy functionality script
/// This should be included once per page that uses copy buttons
pub fn script() -> maud::Markup {
    html! {
        script {
            (PreEscaped("
                function copyToClipboard(btn) {
                    // Decode the base64 text from data attribute
                    var encodedText = btn.getAttribute('data-copy-text');
                    var text = atob(encodedText);
                    
                    // Try modern clipboard API first, fallback to textarea method
                    if (navigator.clipboard && window.isSecureContext) {
                        navigator.clipboard.writeText(text).then(function() {
                            showCopySuccess(btn);
                        }).catch(function(err) {
                            fallbackCopy(btn, text);
                        });
                    } else {
                        fallbackCopy(btn, text);
                    }
                }
                
                function fallbackCopy(btn, text) {
                    // Create a temporary textarea element
                    var textarea = document.createElement('textarea');
                    textarea.value = text;
                    textarea.style.position = 'fixed';
                    textarea.style.opacity = '0';
                    document.body.appendChild(textarea);
                    textarea.select();
                    
                    try {
                        var successful = document.execCommand('copy');
                        if (successful) {
                            showCopySuccess(btn);
                        } else {
                            showCopyFailure(btn);
                        }
                    } catch (err) {
                        showCopyFailure(btn);
                    }
                    
                    document.body.removeChild(textarea);
                }
                
                function showCopySuccess(btn) {
                    var originalText = btn.textContent;
                    btn.textContent = 'Copied!';
                    btn.classList.add('bg-catch-green', 'white');
                    btn.classList.remove('bg-black-80', 'catch-green');
                    setTimeout(function() {
                        btn.textContent = originalText;
                        btn.classList.remove('bg-catch-green', 'white');
                        btn.classList.add('bg-black-80', 'catch-green');
                    }, 2000);
                }
                
                function showCopyFailure(btn) {
                    var originalText = btn.textContent;
                    btn.textContent = 'Failed';
                    setTimeout(function() {
                        btn.textContent = originalText;
                    }, 2000);
                }
            "))
        }
    }
}
