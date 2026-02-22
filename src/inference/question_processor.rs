use crate::data::{Tokenizer, TextCleaner};

pub struct QuestionProcessor;

impl QuestionProcessor {
    pub fn process(question: &str) -> String {
        let cleaned = TextCleaner::clean(question);
        cleaned.trim().to_string()
    }

    pub fn find_relevant_chunks(
        question: &str,
        chunks: &[String],
        top_k: usize,
    ) -> Vec<(String, f32)> {
        let question_words: Vec<&str> = question.split_whitespace().collect();

        let mut scored_chunks: Vec<(String, f32)> = chunks
            .iter()
            .map(|chunk| {
                let chunk_words: Vec<&str> = chunk.split_whitespace().collect();

                // Simple overlap-based scoring
                let overlap = question_words
                    .iter()
                    .filter(|qw| chunk_words.contains(qw))
                    .count();

                let score = overlap as f32 / question_words.len().max(1) as f32;
                (chunk.clone(), score)
            })
            .collect();

        scored_chunks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scored_chunks.into_iter().take(top_k).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_question() {
        let question = "What is the GRADUATION date?";
        let processed = QuestionProcessor::process(question);
        assert!(processed.len() > 0);
        assert!(processed.contains("graduation"));
    }

    #[test]
    fn test_find_relevant_chunks() {
        let question = "graduation ceremony";
        let chunks = vec![
            "The graduation ceremony is on June 15".to_string(),
            "The weather is sunny today".to_string(),
        ];
        let relevant = QuestionProcessor::find_relevant_chunks(&question, &chunks, 1);
        assert_eq!(relevant.len(), 1);
    }
}
