use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    pub learning_rate: f32,
    pub weight_decay: f32,
    pub beta1: f32,
    pub beta2: f32,
}

pub struct OptimizerFactory;

impl OptimizerFactory {
    pub fn create_adamw(learning_rate: f32) -> OptimizationConfig {
        OptimizationConfig {
            learning_rate,
            weight_decay: 0.01,
            beta1: 0.9,
            beta2: 0.999,
        }
    }

    pub fn create_sgd(learning_rate: f32) -> OptimizationConfig {
        OptimizationConfig {
            learning_rate,
            weight_decay: 0.0,
            beta1: 0.9,
            beta2: 0.999,
        }
    }
}
