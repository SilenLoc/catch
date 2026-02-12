use actix_web::Result as AwResult;
use actix_web::get;
use maud::html;

pub struct ProxyView;

impl ProxyView {
    pub fn render() -> maud::Markup {
        html! {
            header class="mb4 pb3 bb b--black-40 flex items-center justify-between" {
                h2 class="f3 f2-ns fw6 ma0 near-white" { "Proxy" }
                span class="f6 fw4 moon-gray" { "HTTP proxy and recording" }
            }

            section class="bg-dark-gray br3 pa3 pa4-ns mv3 shadow-1" {
                p class="f6 moon-gray i mv0" { "Proxy feature is not implemented yet." }
            }
        }
    }
}

#[get("/ui/proxy")]
pub async fn proxy_page() -> AwResult<maud::Markup> {
    Ok(ProxyView::render())
}
