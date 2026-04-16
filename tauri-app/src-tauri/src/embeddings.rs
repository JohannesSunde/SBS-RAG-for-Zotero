// src-tauri/src/embeddings.rs
use anyhow::{Result, anyhow};
use ort::session::{Session, builder::GraphOptimizationLevel};
use ort::inputs;
use ort::value::TensorRef;
use std::path::{Path};
use tokenizers::Tokenizer;
use ndarray::{Array2};

pub struct EmbeddingModel {
    session: Session,
    tokenizer: Tokenizer,
    dimension: usize,
}

impl EmbeddingModel {
    pub fn new<P: AsRef<Path>>(model_path: P, tokenizer_path: P) -> Result<Self> {
        let session = Session::builder()
            .map_err(|e| anyhow!("Failed to create builder: {}", e))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| anyhow!("Failed to set optimization: {}", e))?
            .with_intra_threads(4)
            .map_err(|e| anyhow!("Failed to set threads: {}", e))?
            .commit_from_file(model_path)
            .map_err(|e| anyhow!("Failed to load model: {}", e))?;

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow!("Failed to load tokenizer: {}", e))?;

        Ok(Self {
            session,
            tokenizer,
            dimension: 768,
        })
    }

    pub fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow!("Tokenization failed: {}", e))?;
        
        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&x| x as i64).collect();
        let attention_mask: Vec<i64> = encoding.get_attention_mask().iter().map(|&x| x as i64).collect();
        let token_type_ids: Vec<i64> = encoding.get_type_ids().iter().map(|&x| x as i64).collect();

        let batch_size = 1;
        let seq_len = input_ids.len();

        let input_ids_tensor = Array2::from_shape_vec((batch_size, seq_len), input_ids)?
            .into_dyn();
        let attention_mask_tensor = Array2::from_shape_vec((batch_size, seq_len), attention_mask)?
            .into_dyn();
        let token_type_ids_tensor = Array2::from_shape_vec((batch_size, seq_len), token_type_ids)?
            .into_dyn();

        // ort 2.0: inputs! returns a Vec directly.
        let session_inputs = inputs![
            "input_ids" => TensorRef::from_array_view(&input_ids_tensor)
                .map_err(|e| anyhow!("Failed to create input_ids tensor: {}", e))?,
            "attention_mask" => TensorRef::from_array_view(&attention_mask_tensor)
                .map_err(|e| anyhow!("Failed to create attention_mask tensor: {}", e))?,
            "token_type_ids" => TensorRef::from_array_view(&token_type_ids_tensor)
                .map_err(|e| anyhow!("Failed to create token_type_ids tensor: {}", e))?,
        ];

        let outputs = self.session.run(session_inputs)
            .map_err(|e| anyhow!("Inference failed: {}", e))?;
        
        let tensor = outputs["last_hidden_state"]
            .try_extract_tensor::<f32>()
            .map_err(|e| anyhow!("Failed to extract tensor: {}", e))?;

        let (shape, data) = tensor;
        
        let mut embedding = vec![0.0f32; self.dimension];
        let num_tokens = seq_len as f32;
        
        for s in 0..seq_len {
            for d in 0..self.dimension {
                embedding[d] += data[s * self.dimension + d] / num_tokens;
            }
        }

        let norm = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in embedding.iter_mut() {
                *x /= norm;
            }
        }

        Ok(embedding)
    }
}
