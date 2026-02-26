use actix_web::{HttpRequest, HttpResponse, get};
use serde_json::json;

/// Test endpoint that returns request details
/// Used for testing proxy functionality
#[get("_catch/test-target")]
pub async fn test_target(req: HttpRequest) -> HttpResponse {
    let query = req.query_string();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let response = json!({
        "method": method,
        "path": path,
        "query": query,
        "message": "Test endpoint response"
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}
