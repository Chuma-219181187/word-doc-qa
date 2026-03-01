use serde::{Deserialize, Serialize};
use crate::model::embeddings::Embedding;
use crate::model::encoder_layer::EncoderLayer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformerEncoder {
    pub embeddings: Embedding,
    pub layers: Vec<EncoderLayer>,
    pub num_layers: usize,
}

impl TransformerEncoder {
    pub fn new(
        vocab_size: usize,
        embedding_dim: usize,
        hidden_size: usize,
        num_heads: usize,
        num_layers: usize,
        ffn_dim: usize,
        dropout: f32,
        _max_seq_length: usize,
    ) -> Self {
        let embeddings = Embedding::new(vocab_size, embedding_dim);

        let mut layers = Vec::new();
        for _ in 0..num_layers {
            layers.push(EncoderLayer::new(hidden_size, num_heads, ffn_dim, dropout));
        }

        Self {
            embeddings,
            layers,
            num_layers,
        }
    }

    pub fn forward(
        &self,
        input_ids: Vec<u32>,
        attention_mask: Option<Vec<Vec<f32>>>,
    ) -> Vec<Vec<f32>> {
        // Embed inputs
        let mut hidden_states = self.embeddings.forward(input_ids);

        // Pass through encoder layers
        for layer in &self.layers {
            hidden_states = layer.forward(hidden_states, attention_mask.clone());
        }

        hidden_states
    }
}
