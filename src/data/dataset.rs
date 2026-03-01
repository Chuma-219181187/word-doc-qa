use serde::{Deserialize, Serialize};
use crate::data::tokenizer::TokenizedSequence;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAExample {
    pub context: String,
    pub question: String,
    pub answer_start: usize,
    pub answer_text: String,
    pub tokens: TokenizedSequence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub examples: Vec<QAExample>,
    pub train_examples: Vec<QAExample>,
    pub val_examples: Vec<QAExample>,
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            examples: Vec::new(),
            train_examples: Vec::new(),
            val_examples: Vec::new(),
        }
    }

    pub fn add_example(&mut self, example: QAExample) {
        self.examples.push(example);
    }

    pub fn split(&mut self, train_ratio: f32) {
        let split_idx = (self.examples.len() as f32 * train_ratio) as usize;
        self.train_examples = self.examples[..split_idx].to_vec();
        self.val_examples = self.examples[split_idx..].to_vec();
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.examples.shuffle(&mut rng);
    }

    pub fn size(&self) -> usize {
        self.examples.len()
    }

    pub fn train_size(&self) -> usize {
        self.train_examples.len()
    }

    pub fn val_size(&self) -> usize {
        self.val_examples.len()
    }
}

impl Default for Dataset {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_operations() {
        let dataset = Dataset::new();
        assert_eq!(dataset.size(), 0);
    }
}
