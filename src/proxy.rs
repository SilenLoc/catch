use actix_web::{HttpRequest, HttpResponse, web};
use awc::Client;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyCall {
    id: String,
    timestamp: String,
    method: String,
    path: String,
    query: String,
    target_url: String,
    request_headers: HashMap<String, String>,
    request_body: String,
    response_status: u16,
    response_headers: HashMap<String, String>,
    response_body: String,
}

fn should_skip_hurl_header(name: &str) -> bool {
    matches!(name, "host" | "connection" | "content-length") || name.starts_with("x-forwarded")
}

fn should_forward_header(name: &str) -> bool {
    name != "host"
        && name != "connection"
        && name != "x-proxy-target"
        && !name.starts_with("x-forwarded")
}

fn should_forward_response_header(name: &str) -> bool {
    name != "connection" && name != "transfer-encoding"
}

impl ProxyCall {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        method: String,
        path: String,
        query: String,
        target_url: String,
        request_headers: HashMap<String, String>,
        request_body: String,
        response_status: u16,
        response_headers: HashMap<String, String>,
        response_body: String,
    ) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let id = format!("{}-{}", timestamp, uuid::Uuid::new_v4());

        ProxyCall {
            id,
            timestamp,
            method,
            path,
            query,
            target_url,
            request_headers,
            request_body,
            response_status,
            response_headers,
            response_body,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn target_url(&self) -> &str {
        &self.target_url
    }

    pub fn request_headers(&self) -> &HashMap<String, String> {
        &self.request_headers
    }

    pub fn request_body(&self) -> &str {
        &self.request_body
    }

    pub fn response_status(&self) -> u16 {
        self.response_status
    }

    pub fn response_headers(&self) -> &HashMap<String, String> {
        &self.response_headers
    }

    pub fn response_body(&self) -> &str {
        &self.response_body
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Generate a .hurl file representation of this proxy call
    pub fn to_hurl(&self) -> String {
        let mut hurl = String::new();

        // Add comment with metadata
        let _ = writeln!(hurl, "# Proxy call recorded at {}", self.timestamp);
        let _ = writeln!(hurl, "# ID: {}\n", self.id);

        // Request line: METHOD URL (target_url is already the complete URL used for the proxy request)
        let _ = writeln!(hurl, "{} {}", self.method, self.target_url);

        // Request headers
        for (key, value) in &self.request_headers {
            // Skip certain headers that are connection-specific or auto-generated
            if !should_skip_hurl_header(&key.to_lowercase()) {
                let _ = writeln!(hurl, "{key}: {value}");
            }
        }

        // Request body
        if !self.request_body.is_empty() {
            // Check if body is JSON
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&self.request_body) {
                if json_value.is_object() || json_value.is_array() {
                    let _ = writeln!(
                        hurl,
                        "{}",
                        serde_json::to_string_pretty(&json_value)
                            .unwrap_or(self.request_body.clone())
                    );
                } else {
                    // JSON primitive - use backtick syntax
                    let _ = writeln!(hurl, "`{}`", self.request_body.trim());
                }
            } else {
                // Plain text or other - use backtick syntax
                let _ = writeln!(hurl, "`{}`", self.request_body.trim());
            }
        }

        // Expected response status
        let _ = writeln!(hurl, "HTTP {}", self.response_status);

        // Add assertions for response headers (optional, commented out by default)
        if !self.response_headers.is_empty() {
            hurl.push_str("\n# [Asserts]\n");
            for (key, value) in &self.response_headers {
                let key_lower = key.to_lowercase();
                if key_lower == "content-type" {
                    let _ = writeln!(hurl, "# header \"{key}\" == \"{value}\"");
                }
            }
        }

        // Add response body assertion (commented out by default)
        if !self.response_body.is_empty() {
            // Try to format as JSON if possible
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&self.response_body)
                && (json_value.is_object() || json_value.is_array())
            {
                hurl.push_str("# body == ```\n");
                let _ = writeln!(
                    hurl,
                    "# {}",
                    serde_json::to_string_pretty(&json_value)
                        .unwrap_or(self.response_body.clone())
                        .replace('\n', "\n# ")
                );
                hurl.push_str("# ```\n");
            }
        }

        hurl
    }
}

/// Proxy handler for unmatched routes.
/// Forwards requests to the target specified via X-Proxy-Target header or `PROXY_TARGET` env var.
pub async fn default_proxy(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<crate::config::Server>,
    store: web::Data<crate::kv_store::KeyValueStore>,
) -> HttpResponse {
    // Check for X-Proxy-Target header first, then fall back to env var
    let proxy_target = req
        .headers()
        .get("X-Proxy-Target")
        .and_then(|v| v.to_str().ok())
        .or_else(|| config.proxy_target());

    let Some(target) = proxy_target else {
        return HttpResponse::NotFound().body("No route found and no proxy target configured");
    };

    // Build the target URL
    let path = req.uri().path();
    let query = req.uri().query().map_or(String::new(), |q| format!("?{q}"));

    // If target contains a path, use it as-is, otherwise append the request path
    let target_url =
        if target.contains("://") && target.split("://").nth(1).is_some_and(|s| s.contains('/')) {
            // Target has a path component, use it directly
            if query.is_empty() {
                target.to_string()
            } else {
                format!("{target}{query}")
            }
        } else {
            // Target is just a base URL, append the path
            format!("{target}{path}{query}")
        };

    info!("Proxying {} {} to {}", req.method(), req.uri(), target_url);

    // Capture request details
    let method = req.method().to_string();
    let path_str = path.to_string();
    let query_str = query.clone();
    let mut request_headers = HashMap::new();
    for (header_name, header_value) in req.headers() {
        if let Ok(value) = header_value.to_str() {
            request_headers.insert(header_name.to_string(), value.to_string());
        }
    }
    let request_body = String::from_utf8_lossy(&body).to_string();

    // Create HTTP client and forward the request
    let client = Client::default();
    let mut forwarded_req = client.request(req.method().clone(), &target_url);

    // Forward headers (except host and connection-related headers)
    for (header_name, header_value) in req.headers() {
        if should_forward_header(header_name.as_str()) {
            forwarded_req =
                forwarded_req.insert_header((header_name.clone(), header_value.clone()));
        }
    }

    // Add X-Forwarded-For header
    if let Some(peer_addr) = req.peer_addr() {
        forwarded_req =
            forwarded_req.insert_header(("X-Forwarded-For", peer_addr.ip().to_string()));
    }

    // Send the request with body
    let mut res = match forwarded_req.send_body(body).await {
        Ok(res) => res,
        Err(e) => {
            error!("Proxy request failed: {e}");
            return HttpResponse::BadGateway().body(format!("Proxy error: {e}"));
        }
    };

    // Capture response details
    let response_status = res.status().as_u16();
    let mut response_headers = HashMap::new();
    for (header_name, header_value) in res.headers() {
        if let Ok(value) = header_value.to_str() {
            response_headers.insert(header_name.to_string(), value.to_string());
        }
    }

    // Read response body
    let response_body = match res.body().await {
        Ok(body_bytes) => String::from_utf8_lossy(&body_bytes).to_string(),
        Err(e) => {
            error!("Failed to read proxy response body: {e}");
            return HttpResponse::BadGateway().body(format!("Failed to read response: {e}"));
        }
    };

    // Save proxy call to store
    let proxy_call = ProxyCall::new(
        method,
        path_str,
        query_str,
        target_url,
        request_headers,
        request_body,
        response_status,
        response_headers.clone(),
        response_body.clone(),
    );
    let _ = store.insert_proxy_call(&proxy_call);

    // Build response with forwarded status and headers
    let mut client_resp = HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response_status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    );

    // Forward response headers (except connection-related)
    for (header_name, header_value) in response_headers {
        if should_forward_response_header(header_name.as_str())
            && let Ok(value) = actix_web::http::header::HeaderValue::from_str(&header_value)
        {
            client_resp.insert_header((
                actix_web::http::header::HeaderName::from_bytes(header_name.as_bytes()).unwrap(),
                value,
            ));
        }
    }

    client_resp.body(response_body)
}
