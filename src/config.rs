use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub embedding_size: usize,
    pub hidden_size: usize,
    pub attention_heads: usize,
    pub num_layers: usize,
    pub feedforward_dim: usize,
    pub dropout: f32,
    pub max_seq_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub learning_rate: f32,
    pub batch_size: usize,
    pub epochs: usize,
    pub optimizer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataConfig {
    pub chunk_size: usize,
    pub train_split: f32,
    pub shuffle: bool,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            embedding_size: 512,
            hidden_size: 512,
            attention_heads: 8,
            num_layers: 6,
            feedforward_dim: 2048,
            dropout: 0.1,
            max_seq_length: 512,
        }
    }
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.0003,
            batch_size: 16,
            epochs: 3,
            optimizer: "adamw".to_string(),
        }
    }
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            chunk_size: 512,
            train_split: 0.8,
            shuffle: true,
        }
    }
}
