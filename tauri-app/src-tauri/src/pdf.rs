// src-tauri/src/pdf.rs
use std::path::Path;
use anyhow::{Result, anyhow};

pub struct PdfReader;

impl PdfReader {
    pub fn extract_text<P: AsRef<Path>>(path: P) -> Result<String> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(anyhow!("File does not exist: {:?}", path));
        }

        // Use pdf-extract to get all text
        // Note: For production, a more robust parser like `lopdf` with layout 
        // preservation or a cross-platform wrapper for `pdftotext` might be better.
        match pdf_extract::extract_text(path) {
            Ok(text) => Ok(text),
            Err(e) => Err(anyhow!("Failed to extract text from PDF: {}", e)),
        }
    }

    pub fn extract_text_sample<P: AsRef<Path>>(path: P, max_chars: usize) -> Result<String> {
        let text = Self::extract_text(path)?;
        Ok(text.chars().take(max_chars).collect())
    }
}
