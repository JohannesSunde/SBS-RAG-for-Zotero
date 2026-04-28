use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use ort::value::Value;
use std::path::PathBuf;
use tokenizers::Tokenizer;

pub struct EmbeddingEngine {
    session: Session,
    tokenizer: Tokenizer,
}

impl EmbeddingEngine {
    pub fn new(model_path: PathBuf, tokenizer_path: PathBuf) -> Result<Self, String> {
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

        let session = Session::builder()
            .map_err(|e| format!("Failed to create ORT builder: {}", e))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| format!("Failed to set opt level: {}", e))?
            .with_intra_threads(4)
            .map_err(|e| format!("Failed to set threads: {}", e))?
            .commit_from_file(&model_path)
            .map_err(|e| format!("Failed to load ONNX model: {}", e))?;

        Ok(Self { session, tokenizer })
    }

    pub fn generate_embedding(&mut self, text: &str) -> Result<Vec<f32>, String> {
        // Encode text
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| format!("Tokenizer error: {}", e))?;

        let input_ids = encoding.get_ids().iter().map(|&x| x as i64).collect::<Vec<_>>();
        let attention_mask = encoding.get_attention_mask().iter().map(|&x| x as i64).collect::<Vec<_>>();
        
        let batch_size = 1;
        let seq_len = input_ids.len();

        let input_ids_array = ndarray::Array2::from_shape_vec((batch_size, seq_len), input_ids).unwrap();
        let attention_mask_array = ndarray::Array2::from_shape_vec((batch_size, seq_len), attention_mask).unwrap();

        // Run inference
        let inputs = ort::inputs![
            "input_ids" => Value::from_array(input_ids_array).map_err(|e| format!("Value error: {}", e))?,
            "attention_mask" => Value::from_array(attention_mask_array).map_err(|e| format!("Value error: {}", e))?,
        ];

        let outputs = self.session.run(inputs)
            .map_err(|e| format!("Inference error: {}", e))?;

        // Extract last_hidden_state
        let (shape, data) = outputs["last_hidden_state"]
            .try_extract_tensor::<f32>()
            .map_err(|e| format!("Output extraction error: {}", e))?;

        // For BGE-base, we usually want the CLS token (first token of the sequence)
        // shape is [batch_size, seq_len, hidden_size]
        let hidden_size = shape[2] as usize;
        let cls_embedding = data[0..hidden_size].to_vec();

        // Normalize
        let norm: f32 = cls_embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        let normalized = cls_embedding.iter().map(|x| x / norm).collect();

        Ok(normalized)
    }
}
