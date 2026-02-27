use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, web};
use log::{error, info};
use std::fs;
use std::path::PathBuf;

pub const STORAGE_DIR: &str = "_catch/files";

pub fn get_storage_dir() -> PathBuf {
    PathBuf::from(STORAGE_DIR)
}

pub fn ensure_storage_dir() -> std::io::Result<()> {
    fs::create_dir_all(STORAGE_DIR)
}

pub fn get_file_path(filename: &str, custom_path: Option<&str>) -> PathBuf {
    let mut storage_dir = get_storage_dir();

    if let Some(custom) = custom_path {
        // Use custom path if provided
        for part in custom.trim_start_matches('/').split('/') {
            if !part.is_empty() {
                storage_dir = storage_dir.join(part);
            }
        }
    }

    storage_dir.join(filename)
}

#[post("/file/{filename:.*}")]
pub async fn upload_file(
    req: HttpRequest,
    filename: web::Path<String>,
    body: web::Bytes,
) -> impl Responder {
    // Check for custom file path in X-File-Path header
    let custom_path = req
        .headers()
        .get("X-File-Path")
        .and_then(|h| h.to_str().ok());

    // Ensure storage directory exists
    if let Err(e) = ensure_storage_dir() {
        error!("Failed to create storage directory: {e}");
        return HttpResponse::InternalServerError().body("Failed to create storage directory");
    }

    let file_path = get_file_path(&filename, custom_path);

    // Create subdirectories if they don't exist
    if let Some(parent) = file_path.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        error!("Failed to create directory: {e}");
        return HttpResponse::InternalServerError().body("Failed to create directory");
    }

    // Write file to disk
    match fs::write(&file_path, &body) {
        Ok(()) => {
            info!("File uploaded: {}", file_path.display());
            HttpResponse::Ok().body(format!("File uploaded: {filename}"))
        }
        Err(e) => {
            error!("Failed to write file: {e}");
            HttpResponse::InternalServerError().body("Failed to write file")
        }
    }
}

#[get("/file/{filename:.*}")]
pub async fn download_file(req: HttpRequest, filename: web::Path<String>) -> impl Responder {
    info!("Download request for filename: {filename}");

    // Check for custom file path in X-File-Path header
    let custom_path = req
        .headers()
        .get("X-File-Path")
        .and_then(|h| h.to_str().ok());

    let file_path = get_file_path(&filename, custom_path);
    info!("Resolved file path: {}", file_path.display());

    // Check if file exists
    if !file_path.exists() {
        error!("File not found at path: {}", file_path.display());
        return HttpResponse::NotFound().body("File not found");
    }

    // Read file from disk
    match fs::read(&file_path) {
        Ok(content) => {
            info!("File downloaded: {}", file_path.display());

            // Try to determine content type from file extension
            let content_type = mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string();

            // Extract just the filename for the download
            let download_filename = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("download");

            HttpResponse::Ok()
                .content_type(content_type)
                .insert_header((
                    "Content-Disposition",
                    format!("attachment; filename=\"{download_filename}\""),
                ))
                .body(content)
        }
        Err(e) => {
            error!("Failed to read file: {e}");
            HttpResponse::InternalServerError().body("Failed to read file")
        }
    }
}

#[delete("/file/{filename:.*}")]
pub async fn delete_file(req: HttpRequest, filename: web::Path<String>) -> impl Responder {
    // Check for custom file path in X-File-Path header
    let custom_path = req
        .headers()
        .get("X-File-Path")
        .and_then(|h| h.to_str().ok());

    let file_path = get_file_path(&filename, custom_path);

    // Check if file exists
    if !file_path.exists() {
        return HttpResponse::NotFound().body("File not found");
    }

    // Delete file
    match fs::remove_file(&file_path) {
        Ok(()) => {
            info!("File deleted: {}", file_path.display());
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            error!("Failed to delete file: {e}");
            HttpResponse::InternalServerError().body("Failed to delete file")
        }
    }
}
