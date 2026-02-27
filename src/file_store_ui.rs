use actix_multipart::Multipart;
use actix_web::Result as AwResult;
use actix_web::{HttpResponse, delete, post, web};
use futures_util::StreamExt;
use log::{error, info};
use maud::html;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::file_store::STORAGE_DIR;
use crate::view::file_store::FileStoreView;

#[post("/ui/files/upload")]
pub async fn upload_file_ui(mut payload: Multipart) -> AwResult<maud::Markup> {
    let mut filename = String::new();
    let mut custom_path: Option<String> = None;
    let mut file_data: Vec<u8> = Vec::new();

    // Process multipart fields
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();
        let field_name = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name())
            .unwrap_or("");

        match field_name {
            "file" => {
                filename = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("upload")
                    .to_string();

                // Read file data
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    file_data.extend_from_slice(&data);
                }
            }
            "custom_path" => {
                // Read custom path
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    let path_str = String::from_utf8_lossy(&data).to_string();
                    if !path_str.trim().is_empty() {
                        custom_path = Some(path_str.trim().to_string());
                    }
                }
            }
            _ => {}
        }
    }

    if filename.is_empty() || file_data.is_empty() {
        return Ok(html! {
            h3 class="f6 ttu tracked white ma0 mb3" { "Files (0)" }
            p class="f6 red i mv0" { "No file selected" }
        });
    }

    // Ensure storage directory exists
    if let Err(e) = fs::create_dir_all(STORAGE_DIR) {
        error!("Failed to create storage directory: {e}");
        return Ok(html! {
            h3 class="f6 ttu tracked white ma0 mb3" { "Files" }
            p class="f6 red i mv0" { "Failed to create storage directory" }
        });
    }

    // Build file path
    let file_path = if let Some(custom) = custom_path {
        let path_parts: Vec<&str> = custom.trim_start_matches('/').split('/').collect();
        let mut full_path = PathBuf::from(STORAGE_DIR);
        for part in path_parts {
            if !part.is_empty() {
                full_path = full_path.join(part);
            }
        }
        full_path.join(&filename)
    } else {
        PathBuf::from(STORAGE_DIR).join(&filename)
    };

    // Create subdirectories if needed
    if let Some(parent) = file_path.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        error!("Failed to create directory: {e}");
        return Ok(html! {
            h3 class="f6 ttu tracked white ma0 mb3" { "Files" }
            p class="f6 red i mv0" { "Failed to create directory" }
        });
    }

    // Write file
    match fs::File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(&file_data) {
                error!("Failed to write file: {e}");
                return Ok(html! {
                    h3 class="f6 ttu tracked white ma0 mb3" { "Files" }
                    p class="f6 red i mv0" { "Failed to write file" }
                });
            }
            info!("File uploaded via UI: {}", file_path.display());
        }
        Err(e) => {
            error!("Failed to create file: {e}");
            return Ok(html! {
                h3 class="f6 ttu tracked white ma0 mb3" { "Files" }
                p class="f6 red i mv0" { "Failed to create file" }
            });
        }
    }

    // Return updated file list
    let storage_path = PathBuf::from(STORAGE_DIR);
    let files = crate::view::file_store::list_files(&storage_path, &storage_path);
    Ok(FileStoreView::render_file_list(&files))
}

#[delete("/ui/files/delete/{filename:.*}")]
pub async fn delete_file_ui(filename: web::Path<String>) -> HttpResponse {
    let file_path = PathBuf::from(STORAGE_DIR).join(filename.as_str());

    // Check if file exists
    if !file_path.exists() {
        error!("File not found: {}", file_path.display());
        return HttpResponse::NotFound().body("File not found");
    }

    // Delete file
    match fs::remove_file(&file_path) {
        Ok(()) => {
            info!("File deleted via UI: {}", file_path.display());
            // Return empty response to swap out the row
            HttpResponse::Ok().body("")
        }
        Err(e) => {
            error!("Failed to delete file: {e}");
            HttpResponse::InternalServerError().body("Failed to delete file")
        }
    }
}
