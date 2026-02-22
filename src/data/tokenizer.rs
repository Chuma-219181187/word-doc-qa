use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizedSequence {
    pub input_ids: Vec<u32>,
    pub attention_mask: Vec<u32>,
    pub token_type_ids: Vec<u32>,
}

pub struct Tokenizer {
    vocab_size: usize,
    max_length: usize,
}

impl Tokenizer {
    pub fn new(vocab_size: usize, max_length: usize) -> Self {
        Self {
            vocab_size,
            max_length,
        }
    }

    /// Simple tokenizer: split by spaces and convert to token IDs
    pub fn tokenize(&self, text: &str) -> TokenizedSequence {
        let tokens: Vec<&str> = text.split_whitespace().collect();

        // Simple hash-based token ID generation
        let mut input_ids: Vec<u32> = tokens
            .iter()
            .take(self.max_length)
            .map(|token| {
                let hash = token.chars().fold(0u32, |acc, c| {
                    acc.wrapping_mul(31).wrapping_add(c as u32)
                });
                (hash % self.vocab_size as u32) + 1
            })
            .collect();

        let original_len = input_ids.len();

        // Pad to max_length
        while input_ids.len() < self.max_length {
            input_ids.push(0); // 0 is padding token
        }

        // Create attention mask
        let mut attention_mask = vec![1u32; original_len];
        while attention_mask.len() < self.max_length {
            attention_mask.push(0);
        }

        // Token type IDs (all 0s for single sequence)
        let token_type_ids = vec![0u32; self.max_length];

        TokenizedSequence {
            input_ids,
            attention_mask,
            token_type_ids,
        }
    }

    pub fn batch_tokenize(&self, texts: &[String]) -> Vec<TokenizedSequence> {
        texts
            .iter()
            .map(|text| self.tokenize(text))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokenizer = Tokenizer::new(30000, 512);
        let seq = tokenizer.tokenize("hello world test");
        assert_eq!(seq.input_ids.len(), 512);
        assert_eq!(seq.attention_mask.len(), 512);
    }
}
