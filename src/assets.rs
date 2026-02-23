use actix_web::{HttpRequest, HttpResponse, Responder, get};
const TCSS: &str = include_str!("../assets/t.css");
const HTMX: &str = include_str!("../assets/h.js");
const PRISM_CSS: &str = include_str!("../assets/prism-tomorrow.min.css");
const PRISM_JS: &str = include_str!("../assets/prism.min.js");
const PRISM_JAVASCRIPT_JS: &str = include_str!("../assets/prism-javascript.min.js");

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
        _ => HttpResponse::NotFound().body("Not found"),
    }
}
