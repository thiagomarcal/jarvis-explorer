// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io};
use std::fs::DirEntry;
use std::sync::Mutex;
use std::time::SystemTime;
use serde::Serialize;

struct AppState {
    counter: Mutex<u32>,
}

#[derive(Serialize)]
struct AppFile {
    filename: String,
    size: u64,
    created: u64,
    modified: u64,
    is_dir: bool,
}

#[derive(Debug, Serialize)]
pub enum AppError {
    RetrievingFile
}

#[tauri::command]
fn list_files(path: &str) -> Result<Vec<Result<AppFile, AppError>>, AppError> {
    let result = match fs::read_dir(path) {
        Ok(paths) => {
            let mut files: Vec<Result<AppFile, AppError>> = Vec::new();
            for path in paths {
                let result = extract_app_file(path);
                files.push(result);
            }
            Ok(files)
        }
        Err(_e) => {
            Err(AppError::RetrievingFile)
        }
    };

    result
}

fn extract_app_file(path: io::Result<DirEntry>) -> Result<AppFile, AppError> {
    let result = match path {
        Ok(entry) => {
            let entry_metadata = entry.path().metadata().unwrap();

            let app_file = AppFile {
                filename: String::from(entry.path().to_str().unwrap_or_default()),
                size: entry_metadata.len(),
                created: entry_metadata.created().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                modified: entry_metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                is_dir: entry.path().is_dir(),
            };
            Ok(app_file)
        }
        Err(_e) => {
            Err(AppError::RetrievingFile)
        }
    };
    result
}


fn main() {
    tauri::Builder::default()
        .manage(AppState { counter: Mutex::new(0) })
        .invoke_handler(tauri::generate_handler![list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
