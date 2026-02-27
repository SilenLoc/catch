use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get};
use maud::html;
use std::fs;
use std::path::{Path, PathBuf};

use crate::file_store::STORAGE_DIR;

use super::components;

pub struct FileStoreView;

/// Recursively list all files in the storage directory
pub fn list_files(dir: &Path, base_path: &Path) -> Vec<FileEntry> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let relative_path = path
                .strip_prefix(base_path)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            if path.is_file() {
                let size = entry.metadata().ok().map_or(0, |m| m.len());
                files.push(FileEntry {
                    path: relative_path,
                    size,
                });
            } else if path.is_dir() {
                // Recursively list subdirectories
                files.extend(list_files(&path, base_path));
            }
        }
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));
    files
}

pub struct FileEntry {
    pub path: String,
    pub size: u64,
}

impl FileStoreView {
    pub fn render(files: &[FileEntry]) -> maud::Markup {
        html! {
            (components::page_header::render("File Store", "Upload, download, and manage files"))

            div class="overflow-y-auto" style="max-height: calc(100vh - 8rem);" {
                // Upload section
                section class="bg-catch-dark br3 pa3 pa4-ns mv3 shadow-1 mr4" {
                    h3 class="f6 ttu tracked white ma0 mb3" { "Upload File" }

                    form
                        hx-post="/ui/files/upload"
                        hx-target="#file-list"
                        hx-encoding="multipart/form-data"
                        class="flex flex-column flex-row-ns items-end-ns" {
                        div class="flex-auto mr3-ns mb2 mb0-ns" {
                            label class="db f7 white mb2" { "Select file to upload" }
                            input
                                type="file"
                                name="file"
                                required
                                class="db w-100 pa2 br2 ba b--black-40 bg-black-90 white f6";
                        }
                        div class="flex-auto mr3-ns mb2 mb0-ns" {
                            label class="db f7 white mb2" { "Custom path (optional)" }
                            input
                                type="text"
                                name="custom_path"
                                placeholder="e.g., documents/2024"
                                class="db w-100 pa2 br2 ba b--black-40 bg-black-90 white f6";
                        }
                        button
                            type="submit"
                            class="pa2 ph3-ns br2 bg-catch-green white bn pointer hover-bg-catch-dark-green f6 fw6" {
                            "Upload"
                        }
                    }

                    // Upload feedback area
                    div id="upload-feedback" class="mt3" {}
                }

                // File list section
                section id="file-list" class="bg-catch-dark br3 pa3 pa4-ns mv3 shadow-1 mr4" {
                    (Self::render_file_list(&files))
                }
            }
        }
    }

    pub fn render_file_list(files: &[FileEntry]) -> maud::Markup {
        html! {
            h3 class="f6 ttu tracked white ma0 mb3" { "Files (" (files.len()) ")" }

            @if files.is_empty() {
                p class="f6 white i mv0" { "No files uploaded yet." }
            } @else {
                div class="overflow-x-auto" {
                    table class="w-100 collapse ba br2 b--black-40" {
                        thead {
                            tr class="bg-black-80" {
                                th class="tl pa2 f7 fw6 ttu tracked white bb b--black-40" { "Path" }
                                th class="tr pa2 f7 fw6 ttu tracked white bb b--black-40" { "Size" }
                                th class="tc pa2 f7 fw6 ttu tracked white bb b--black-40" { "Actions" }
                            }
                        }
                        tbody {
                            @for (idx, file) in files.iter().enumerate() {
                                tr class="bg-black-90 hover-bg-black-80" id=(format!("file-row-{}", idx)) {
                                    td class="pa2 f6 white bb b--black-40" {
                                        code class="f7 catch-green" { (&file.path) }
                                    }
                                    td class="tr pa2 f6 silver bb b--black-40" {
                                        (Self::format_file_size(file.size))
                                    }
                                    td class="tc pa2 bb b--black-40" {
                                        div class="flex items-center justify-center" {
                                            a
                                                href=(format!("/file/{}", &file.path))
                                                class="dib pa1 ph2 mr2 br2 bg-blue white no-underline f7 hover-bg-dark-blue" {
                                                "Download"
                                            }
                                            button
                                                hx-delete=(format!("/ui/files/delete/{}", &file.path))
                                                hx-target="closest tr"
                                                hx-swap="outerHTML swap:0s"
                                                hx-confirm=(format!("Are you sure you want to delete '{}'?", &file.path))
                                                class="pa1 ph2 br2 bg-red white bn pointer f7 hover-bg-dark-red" {
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn format_file_size(size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if size >= GB {
            format!("{:.2} GB", size as f64 / GB as f64)
        } else if size >= MB {
            format!("{:.2} MB", size as f64 / MB as f64)
        } else if size >= KB {
            format!("{:.2} KB", size as f64 / KB as f64)
        } else {
            format!("{size} B")
        }
    }
}

#[get("/ui/files")]
pub async fn files_page(req: HttpRequest) -> AwResult<maud::Markup> {
    let storage_path = PathBuf::from(STORAGE_DIR);
    let files = list_files(&storage_path, &storage_path);

    let content = FileStoreView::render(&files);

    // Check if this is an htmx request
    if req.headers().get("HX-Request").is_some() {
        Ok(content)
    } else {
        Ok(super::render_layout(&content))
    }
}
