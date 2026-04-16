// src-tauri/src/commands.rs
use tauri::State;
use crate::state::AppState;
use crate::zotero::{ZoteroLibrary, ZoteroItem};
use crate::pdf::PdfReader;
use anyhow::Result;

#[tauri::command]
pub async fn get_zotero_items(state: State<'_, AppState>) -> Result<Vec<ZoteroItem>, String> {
    let mut zotero_lock = state.zotero.lock().await;
    if zotero_lock.is_none() {
        // Find default Zotero path
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        let db_path = home.join("Zotero").join("zotero.sqlite");
        *zotero_lock = Some(ZoteroLibrary::new(db_path));
    }
    
    let zotero = zotero_lock.as_ref().unwrap();
    zotero.search_parent_items_with_pdfs().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn extract_pdf_text(path: String) -> Result<String, String> {
    PdfReader::extract_text(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn chat_about_zotero(query: String, state: State<'_, AppState>) -> Result<String, String> {
    // Placeholder for actual RAG chat logic
    Ok(format!("Chatting about: {}. (RAG logic coming soon)", query))
}

#[tauri::command]
pub async fn index_library(state: State<'_, AppState>) -> Result<String, String> {
    let items = get_z_items(&state).await?;
    
    // This would normally be a long-running task with status updates
    // For now, we'll just mock the start
    let mut store_lock = state.vector_store.lock().await;
    if store_lock.is_none() {
        *store_lock = Some(crate::vector_db::VectorStore::new());
    }

    Ok(format!("Indexed {} items (simulated)", items.len()))
}

// Helper for commands
async fn get_z_items(state: &State<'_, AppState>) -> Result<Vec<ZoteroItem>, String> {
    let mut zotero_lock = state.zotero.lock().await;
    if zotero_lock.is_none() {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        let db_path = home.join("Zotero").join("zotero.sqlite");
        *zotero_lock = Some(ZoteroLibrary::new(db_path));
    }
    let zotero = zotero_lock.as_ref().unwrap();
    zotero.search_parent_items_with_pdfs().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn index_status(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    // Placeholder for status logic
    Ok(serde_json::json!({
        "status": "idle",
        "progress": {"total": 0, "current": 0}
    }))
}
