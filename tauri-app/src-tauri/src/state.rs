// src-tauri/src/state.rs
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::zotero::ZoteroLibrary;
use crate::vector_db::VectorStore;
use crate::embeddings::EmbeddingModel;

pub struct AppState {
    pub zotero: Mutex<Option<ZoteroLibrary>>,
    pub vector_store: Mutex<Option<VectorStore>>,
    pub embedding_model: Mutex<Option<EmbeddingModel>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            zotero: Mutex::new(None),
            vector_store: Mutex::new(None),
            embedding_model: Mutex::new(None),
        }
    }
}
