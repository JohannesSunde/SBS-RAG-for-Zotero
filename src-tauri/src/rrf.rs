use std::collections::HashMap;

/// Result structure for search engines
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub metadata: HashMap<String, String>, // Placeholder for LanceDB/BM25 metadata
}

/// Applies Reciprocal Rank Fusion (RRF) to combine dense and sparse search results
pub fn reciprocal_rank_fusion(
    dense_results: Vec<SearchResult>,
    sparse_results: Vec<SearchResult>,
    k: f32, // Typically 60
) -> Vec<SearchResult> {
    let mut scores: HashMap<String, (f32, Option<SearchResult>)> = HashMap::new();

    // Process dense results
    for (rank, result) in dense_results.into_iter().enumerate() {
        let entry = scores.entry(result.id.clone()).or_insert((0.0, Some(result)));
        entry.0 += 1.0 / (k + (rank as f32) + 1.0);
    }

    // Process sparse results
    for (rank, result) in sparse_results.into_iter().enumerate() {
        let entry = scores.entry(result.id.clone()).or_insert((0.0, Some(result)));
        entry.0 += 1.0 / (k + (rank as f32) + 1.0);
    }

    // Convert back to list and sort
    let mut final_results: Vec<SearchResult> = scores
        .into_iter()
        .filter_map(|(_, (score, result))| {
            result.map(|mut r| {
                r.score = score;
                r
            })
        })
        .collect();

    // Sort descending by score
    final_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    final_results
}
