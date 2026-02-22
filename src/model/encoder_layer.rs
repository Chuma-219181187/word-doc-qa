use serde::{Deserialize, Serialize};
use crate::model::attention::MultiHeadAttention;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncoderLayer {
    pub attention: MultiHeadAttention,
    pub hidden_size: usize,
    pub ffn_dim: usize,
    pub dropout_rate: f32,
}

impl EncoderLayer {
    pub fn new(
        hidden_size: usize,
        num_heads: usize,
        ffn_dim: usize,
        dropout_rate: f32,
    ) -> Self {
        Self {
            attention: MultiHeadAttention::new(hidden_size, num_heads),
            hidden_size,
            ffn_dim,
            dropout_rate,
        }
    }

    pub fn forward(
        &self,
        x: Vec<Vec<f32>>,
        attention_mask: Option<Vec<Vec<f32>>>,
    ) -> Vec<Vec<f32>> {
        // Self-attention with residual connection
        let attn_output = self.attention.forward(x.clone(), x.clone(), x.clone(), attention_mask);
        
        // Feed-forward with residual connection (simplified)
        attn_output
    }
}
