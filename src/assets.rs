use actix_web::{HttpRequest, HttpResponse, Responder, get};
const TCSS: &str = include_str!("../assets/t.css");
const HTMX: &str = include_str!("../assets/h.js");
const PRISM_CSS: &str = include_str!("../assets/prism-tomorrow.min.css");
const PRISM_JS: &str = include_str!("../assets/prism.min.js");
const PRISM_JAVASCRIPT_JS: &str = include_str!("../assets/prism-javascript.min.js");
const COPY_BUTTON_JS: &str = include_str!("../assets/copy-button.js");
const FAVICON_ICO: &[u8] = include_bytes!("../assets/favicon.ico");

#[get("/assets/{filename:.*}")]
pub async fn assets(req: HttpRequest) -> impl Responder {
    let path = req.match_info().query("filename");

    match path {
        "t.css" => HttpResponse::Ok()
            .content_type("text/css; charset=utf-8")
            .body(TCSS),
        "h.js" => HttpResponse::Ok()
            .content_type("application/javascript; charset=utf-8")
            .body(HTMX),
        "prism-tomorrow.min.css" => HttpResponse::Ok()
            .content_type("text/css; charset=utf-8")
            .body(PRISM_CSS),
        "prism.min.js" => HttpResponse::Ok()
            .content_type("application/javascript; charset=utf-8")
            .body(PRISM_JS),
        "prism-javascript.min.js" => HttpResponse::Ok()
            .content_type("application/javascript; charset=utf-8")
            .body(PRISM_JAVASCRIPT_JS),
        "copy-button.js" => HttpResponse::Ok()
            .content_type("application/javascript; charset=utf-8")
            .body(COPY_BUTTON_JS),
        "favicon.ico" => HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(FAVICON_ICO),
        _ => HttpResponse::NotFound().body("Not found"),
    }
}
