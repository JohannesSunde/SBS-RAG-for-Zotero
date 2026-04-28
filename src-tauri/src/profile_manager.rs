use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderCredentials {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderConfig {
    pub enabled: bool,
    pub credentials: Option<ProviderCredentials>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppProfile {
    pub id: String,
    pub name: String,
    pub active_provider_id: String,
    pub active_model: Option<String>,
    pub embedding_model: String,
    pub zotero_path: String,
    pub chroma_path: Option<String>, // We will migrate this to lancedb_path
    pub providers: HashMap<String, ProviderConfig>,
}

pub struct ProfileManager {
    base_dir: PathBuf,
}

impl ProfileManager {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let base_dir = app_data_dir.join("RAG_Assistant");
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir).unwrap_or_else(|e| {
                log::error!("Failed to create base directory: {}", e);
            });
        }
        
        ProfileManager { base_dir }
    }

    pub fn get_active_profile(&self) -> Option<AppProfile> {
        let active_path = self.base_dir.join("active_profile.json");
        if !active_path.exists() {
            return None;
        }

        let contents = fs::read_to_string(active_path).ok()?;
        serde_json::from_str(&contents).ok()
    }

    pub fn save_active_profile(&self, profile: &AppProfile) -> Result<(), String> {
        let active_path = self.base_dir.join("active_profile.json");
        let contents = serde_json::to_string_pretty(profile)
            .map_err(|e| format!("Failed to serialize profile: {}", e))?;
            
        fs::write(active_path, contents)
            .map_err(|e| format!("Failed to write profile: {}", e))
    }
}
