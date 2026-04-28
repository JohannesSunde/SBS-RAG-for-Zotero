use std::path::Path;
use lancedb::connection::Connection;
use lancedb::query::{ExecutableQuery, QueryBase};
use lancedb::index::{Index, scalar::FtsIndexBuilder, scalar::FullTextSearchQuery};
use futures::TryStreamExt;
use crate::rrf::{reciprocal_rank_fusion, SearchResult};

pub struct VectorStore {
    conn: Connection,
}

impl VectorStore {
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, String> {
        let uri = db_path.as_ref().to_str().ok_or("Invalid path")?;
        let conn = lancedb::connect(uri).execute().await
            .map_err(|e| format!("Failed to connect to LanceDB: {}", e))?;
            
        Ok(Self { conn })
    }

    /// Create an FTS index on the specified column
    pub async fn create_fts_index(&self, collection_name: &str, column_name: &str) -> Result<(), String> {
        let table = self.conn.open_table(collection_name).execute().await
            .map_err(|e| format!("Failed to open table: {}", e))?;
            
        table.create_index(&[column_name], Index::FTS(FtsIndexBuilder::default()))
            .execute()
            .await
            .map_err(|e| format!("Failed to create FTS index: {}", e))?;
            
        Ok(())
    }

    /// Hybrid search combining vector and keyword search
    /// Hybrid search combining vector and keyword search
    pub async fn hybrid_search(
        &self, 
        collection_name: &str, 
        query_text: &str, 
        query_vector: Vec<f32>, 
        limit: usize
    ) -> Result<Vec<String>, String> {
        let collection = self.conn.open_table(collection_name).execute().await
            .map_err(|e| format!("Failed to open collection: {}", e))?;
            
        // 1. Vector Search (Dense)
        let vector_results_stream = collection.vector_search(query_vector)
            .map_err(|e| format!("Vector search setup failed: {}", e))?
            .limit(limit * 2)
            .execute()
            .await
            .map_err(|e| format!("Vector search execution failed: {}", e))?;
            
        let vector_results_batches = vector_results_stream.try_collect::<Vec<_>>().await
            .map_err(|e| format!("Vector search streaming failed: {}", e))?;
            
        // 2. Keyword Search (FTS)
        let fts_results_stream = collection.query()
            .full_text_search(FullTextSearchQuery::new(query_text.to_string()))
            .limit(limit * 2)
            .execute()
            .await
            .map_err(|e| format!("FTS search execution failed: {}", e))?;
            
        let fts_results_batches = fts_results_stream.try_collect::<Vec<_>>().await
            .map_err(|e| format!("FTS search streaming failed: {}", e))?;

        // 3. Extract IDs and Scores
        let dense_results = self.extract_results(vector_results_batches, "_distance")?;
        let sparse_results = self.extract_results(fts_results_batches, "_score")?;

        // 4. Combine with RRF
        let fused_results = reciprocal_rank_fusion(dense_results, sparse_results, 60.0);
        
        // 5. Return top IDs
        Ok(fused_results.into_iter().take(limit).map(|r| r.id).collect())
    }

    fn extract_results(&self, batches: Vec<arrow::record_batch::RecordBatch>, score_col: &str) -> Result<Vec<SearchResult>, String> {
        use arrow::array::{StringArray, Float32Array};
        use std::collections::HashMap;

        let mut results = Vec::new();
        for batch in batches {
            let id_array = batch.column_by_name("id")
                .ok_or("ID column missing")?
                .as_any().downcast_ref::<StringArray>()
                .ok_or("Failed to downcast ID column")?;
                
            let score_array = batch.column_by_name(score_col)
                .ok_or_else(|| format!("Score column {} missing", score_col))?
                .as_any().downcast_ref::<Float32Array>()
                .ok_or("Failed to downcast score column")?;

            for i in 0..batch.num_rows() {
                results.push(SearchResult {
                    id: id_array.value(i).to_string(),
                    score: score_array.value(i),
                    metadata: HashMap::new(),
                });
            }
        }
        Ok(results)
    }

    /// Search the vector store (deprecated in favor of hybrid_search)
    pub async fn search(&self, collection_name: &str, query_vector: Vec<f32>, limit: usize) -> Result<Vec<String>, String> {
        let collection = self.conn.open_table(collection_name).execute().await
            .map_err(|e| format!("Failed to open collection: {}", e))?;
            
        let _batches = collection.vector_search(query_vector)
            .map_err(|e| format!("Search setup failed: {}", e))?
            .limit(limit)
            .execute()
            .await
            .map_err(|e| format!("Search failed: {}", e))?;
            
        Ok(Vec::new())
    }
}
