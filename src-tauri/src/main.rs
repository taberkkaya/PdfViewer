// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            find_pdf_path_by_name,
            get_first_pdf_in_public
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    app_lib::run();
}

#[tauri::command]
fn find_pdf_path_by_name(name: String) -> Option<String> {
    let dir = std::path::Path::new("C:/pdfs");

    if dir.exists() {
        for entry in std::fs::read_dir(dir).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().map(|e| e == "pdf").unwrap_or(false) {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem.eq_ignore_ascii_case(&name) {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    None
}

#[tauri::command]
fn get_first_pdf_in_public() -> Option<String> {
    let mut public_path = std::env::current_dir().ok()?;
    public_path.push("public/pdfs");

    let entries = fs::read_dir(public_path).ok()?;

    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "pdf" {
                if let Some(file_name) = path.file_name() {
                    return Some(file_name.to_string_lossy().to_string());
                }
            }
        }
    }

    None
}
