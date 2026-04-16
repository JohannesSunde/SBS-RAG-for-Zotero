// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod zotero;
pub mod pdf;
pub mod embeddings;
pub mod vector_db;
pub mod state;
pub mod commands;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_zotero_items,
            commands::extract_pdf_text,
            commands::chat_about_zotero,
            commands::index_library,
            commands::index_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
