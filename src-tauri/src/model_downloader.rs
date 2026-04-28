use reqwest::Client;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use tauri::{Window, Emitter};

const MODEL_URL: &str = "https://huggingface.co/Xenova/bge-base-en-v1.5/resolve/main/onnx/model_quantized.onnx";
const TOKENIZER_URL: &str = "https://huggingface.co/Xenova/bge-base-en-v1.5/resolve/main/tokenizer.json";

pub async fn download_models(app_data_dir: PathBuf, handle: Option<tauri::AppHandle>) -> Result<(PathBuf, PathBuf), String> {
    let models_dir = app_data_dir.join("models");
    if !models_dir.exists() {
        fs::create_dir_all(&models_dir).map_err(|e| format!("Failed to create models dir: {}", e))?;
    }

    let model_path = models_dir.join("bge_base_quantized.onnx");
    let tokenizer_path = models_dir.join("tokenizer.json");

    if !model_path.exists() || !tokenizer_path.exists() {
        let client = Client::new();
        
        // Let frontend know we are downloading
        if let Some(h) = &handle {
            let _ = h.emit("download-status", "Downloading embedding model...");
        }

        download_file(&client, MODEL_URL, &model_path).await?;
        
        if let Some(h) = &handle {
            let _ = h.emit("download-status", "Downloading tokenizer...");
        }
        
        download_file(&client, TOKENIZER_URL, &tokenizer_path).await?;

        if let Some(h) = &handle {
            let _ = h.emit("download-status", "Download complete.");
        }
    }

    Ok((model_path, tokenizer_path))
}

async fn download_file(client: &Client, url: &str, dest: &Path) -> Result<(), String> {
    let response = client.get(url).send().await
        .map_err(|e| format!("Failed to start download: {}", e))?;
        
    let bytes = response.bytes().await
        .map_err(|e| format!("Failed to download bytes: {}", e))?;
        
    let mut file = fs::File::create(dest)
        .map_err(|e| format!("Failed to create file: {}", e))?;
        
    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write to file: {}", e))?;
        
    Ok(())
}
