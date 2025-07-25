#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::PathBuf};
use dirs::home_dir;
use base64::{engine::general_purpose, Engine as _};

#[tauri::command]
fn ensure_pdf_folder_exists() -> Result<(), String> {
    let base = if cfg!(target_os = "windows") {
        PathBuf::from("C:/pdfs")
    } else {
        PathBuf::from("/opt/pdfviewer/pdfs") // ya da ev dizini altı: dirs::home_dir()?.join("pdfs")
    };

    if !base.exists() {
        fs::create_dir_all(&base).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn read_pdf_as_base64(path: String) -> Result<String, String> {
    let buffer = fs::read(path).map_err(|e| e.to_string())?;
    Ok(general_purpose::STANDARD.encode(&buffer))
}

#[tauri::command]
fn get_first_pdf_in_public() -> Option<String> {
    let dir = get_platform_pdf_dir();

    let entries = fs::read_dir(dir).ok()?;
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "pdf" {
                    return path.file_name().and_then(|n| n.to_str()).map(String::from);
                }
            }
        }
    }
    None
}

#[tauri::command]
fn find_pdf_path_by_name(name: String) -> Option<String> {
    let mut path = get_platform_pdf_dir();
    path.push(name);
    if path.exists() {
        path.to_str().map(String::from)
    } else {
        None
    }
}

#[tauri::command]
fn copy_pdf_to_public(path: String) -> Result<(), String> {
    let source = PathBuf::from(&path);

    if source.extension().map(|e| e != "pdf").unwrap_or(true) {
        return Err("Sadece PDF dosyaları yüklenebilir.".into());
    }

    // Platforma göre hedef dizini belirle
    let mut dest = if cfg!(target_os = "windows") {
        PathBuf::from("C:/pdfs")
    } else {
        // Linux veya macOS için
        dirs::home_dir()
            .map(|mut p| {
                p.push("pdfs");
                p
            })
            .ok_or("Ev dizini alınamadı.")?
    };

    // Hedef klasör yoksa oluştur
    if !dest.exists() {
        fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
    }

    dest.push(
        source.file_name().ok_or("Dosya adı alınamadı.")?
    );

    fs::copy(&source, &dest).map_err(|e| e.to_string())?;

    Ok(())
}


#[tauri::command]
fn get_base_pdf_dir() -> Result<String, String> {
    let dir = get_platform_pdf_dir();
    Ok(dir.to_string_lossy().into())
}

fn get_platform_pdf_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        PathBuf::from("C:/pdfs")
    } else {
        let mut base = home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        base.push("pdfviewer/pdfs");
        base
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_base_pdf_dir,
            ensure_pdf_folder_exists,
            copy_pdf_to_public,
            read_pdf_as_base64,
            get_first_pdf_in_public,
            find_pdf_path_by_name
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

