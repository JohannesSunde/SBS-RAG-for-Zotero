// src-tauri/src/vector_db.rs
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkMetadata {
    pub item_id: String,
    pub chunk_idx: i32,
    pub title: String,
    pub authors: String,
    pub tags: String,
    pub collections: String,
    pub year: String,
    pub pdf_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct VectorStore {
    vectors: Vec<Vec<f32>>,
    metadatas: Vec<ChunkMetadata>,
    db_path: PathBuf,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            vectors: Vec::new(),
            metadatas: Vec::new(),
            db_path: PathBuf::new(),
        }
    }

    pub fn add_chunks(&mut self, embeddings: Vec<Vec<f32>>, metas: Vec<ChunkMetadata>) -> Result<()> {
        self.vectors.extend(embeddings);
        self.metadatas.extend(metas);
        Ok(())
    }

    pub fn search(&self, query_vector: &[f32], limit: usize) -> Vec<(f32, &ChunkMetadata)> {
        let mut results: Vec<(f32, usize)> = self.vectors.iter().enumerate()
            .map(|(i, v)| {
                let score = dot_product(query_vector, v);
                (score, i)
            })
            .collect();
            
        // Sort by score descending (higher is better for cosine with normalized vectors)
        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        results.iter()
            .take(limit)
            .map(|(score, idx)| (*score, &self.metadatas[*idx]))
            .collect()
    }
}

fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
