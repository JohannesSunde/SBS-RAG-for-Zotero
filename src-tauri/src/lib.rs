use tokio::sync::Mutex;
use tauri::{State, Manager, Emitter};

mod zotero_db;
mod profile_manager;
mod pdf_extractor;
mod embed_engine;
mod vector_db;
mod rrf;
mod llm_provider;
mod model_downloader;

use embed_engine::EmbeddingEngine;
use vector_db::VectorStore;
use profile_manager::ProfileManager;

struct AppState {
    embed_engine: Mutex<Option<EmbeddingEngine>>,
    vector_store: Mutex<Option<VectorStore>>,
    profile_manager: Mutex<Option<ProfileManager>>,
}

#[tauri::command]
async fn health_check(state: State<'_, AppState>) -> Result<String, String> {
    let engine_ready = state.embed_engine.lock().await.is_some();
    let store_ready = state.vector_store.lock().await.is_some();
    
    Ok(format!(
        "Backend: Native Rust | Embedder: {} | VectorStore: {}", 
        if engine_ready { "READY" } else { "NOT_READY" },
        if store_ready { "READY" } else { "NOT_READY" }
    ))
}

#[tauri::command]
async fn search_items(query: String, state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut engine_lock = state.embed_engine.lock().await;
    let engine = engine_lock.as_mut().ok_or("Embedding engine not initialized")?;
    
    let vector_lock = state.vector_store.lock().await;
    let store = vector_lock.as_ref().ok_or("Vector store not initialized")?;

    let embedding = engine.generate_embedding(&query)?;
    
    // Perform hybrid search combining vector and full-text keyword retrieval
    let results = store.hybrid_search("items", &query, embedding, 5).await?;
    
    Ok(results)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(AppState {
        embed_engine: Mutex::new(None),
        vector_store: Mutex::new(None),
        profile_manager: Mutex::new(None),
    })
    .invoke_handler(tauri::generate_handler![
        health_check,
        search_items
    ])
    .setup(|app| {
        let handle = app.handle().clone();
        let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
        
        // Run initialization in a background task so we don't block the UI thread
        tauri::async_runtime::spawn(async move {
            let _ = handle.emit("init-status", "Initializing native engines...");
            
            // 1. Ensure models are downloaded
            match model_downloader::download_models(app_data_dir.clone(), Some(handle.clone())).await {
                Ok((model_path, tokenizer_path)) => {
                    // 2. Initialize engines
                    match EmbeddingEngine::new(model_path, tokenizer_path) {
                        Ok(engine) => {
                            let state = handle.state::<AppState>();
                            *state.embed_engine.lock().await = Some(engine);
                            let _ = handle.emit("init-status", "Embedding engine ready.");
                        },
                        Err(e) => { let _ = handle.emit("init-status", format!("Embedder failed: {}", e)); }
                    }
                },
                Err(e) => { let _ = handle.emit("init-status", format!("Download failed: {}", e)); }
            }

            // 3. Initialize vector store
            let db_path = app_data_dir.join("vector_db");
            match VectorStore::new(db_path).await {
                Ok(store) => {
                    let state = handle.state::<AppState>();
                    *state.vector_store.lock().await = Some(store);
                    let _ = handle.emit("init-status", "Vector store ready.");
                },
                Err(e) => { let _ = handle.emit("init-status", format!("VectorDB failed: {}", e)); }
            }

            // 4. Initialize profile manager
            let profile_mgr = ProfileManager::new(app_data_dir);
            let state = handle.state::<AppState>();
            *state.profile_manager.lock().await = Some(profile_mgr);
            
            let _ = handle.emit("init-status", "Ready.");
        });

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
