use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub vocab_size: usize,
    pub embedding_dim: usize,
}

impl Embedding {
    pub fn new(vocab_size: usize, embedding_dim: usize) -> Self {
        Self {
            vocab_size,
            embedding_dim,
        }
    }

    pub fn forward(&self, input_ids: Vec<u32>) -> Vec<Vec<f32>> {
        input_ids
            .iter()
            .map(|id| {
                vec![0.1; self.embedding_dim]
            })
            .collect()
    }
}
