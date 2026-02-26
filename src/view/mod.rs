use actix_web::Result as AwResult;
use actix_web::get;

pub mod components;
pub mod key_value;
pub mod proxy;
pub mod script;

#[get("/")]
pub async fn index() -> AwResult<maud::Markup> {
    Ok(render_layout(&maud::html! {}))
}

pub fn render_layout(main_content: &maud::Markup) -> maud::Markup {
    maud::html! {
        html {
            head {
                meta charset="utf-8";
                title { "catch" }
                link rel="icon" type="image/x-icon" href="/assets/favicon.ico" {}
                link rel="stylesheet" href="/assets/t.css" {}
                script src="/assets/h.js" {}
                // Prism.js for syntax highlighting (vendored)
                link rel="stylesheet" href="/assets/prism-tomorrow.min.css" {}
                script src="/assets/prism.min.js" {}
                script src="/assets/prism-javascript.min.js" {}
            }

            body class="min-vh-100 bg-near-black white sans-serif" {
                div class="flex min-vh-100" {
                    aside class="w-100 mw5-ns bg-black-90 white ph3 pv4" {
                        h1 class="f4 fw6 tracked ttu mb4 white" { "catch" }

                        nav class="f6" {
                            a class="db pv2 ph2 br2 catch-green bg-black-60 mb2 no-underline bg-animate hover-bg-catch-dark-green hover-white"
                              href="/ui/kv" hx-get="/ui/kv" hx-target="#feature" hx-swap="innerHTML" hx-trigger="load, click" {
                                span class="dib" { "Key-Value Store" }
                            }

                            a class="db pv2 ph2 br2 catch-green bg-black-60 mb2 no-underline bg-animate hover-bg-catch-dark-green hover-white flex items-center justify-between"
                              href="/ui/scripts" hx-get="/ui/scripts" hx-target="#feature" hx-swap="innerHTML" {
                                span class="dib" { "Scripts" }
                                span class="ml2 f7 ttu tracked-mega gold" { "beta" }
                            }

                            a class="db pv2 ph2 br2 catch-green bg-black-60 no-underline bg-animate hover-bg-catch-dark-green hover-white flex items-center justify-between"
                              href="/ui/proxy" hx-get="/ui/proxy" hx-target="#feature" hx-swap="innerHTML" {
                                span class="dib" { "Proxy" }
                                span class="ml2 f7 ttu tracked-mega gold" { "beta" }
                            }
                        }
                    }

                    // Main content area (htmx target for feature content)
                    main id="feature" class="flex-auto ph3 ph4-ns pv4 bg-near-black" { (main_content) }
                }
            }
        }
    }
}
