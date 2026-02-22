use serde::{Deserialize, Serialize};
use crate::inference::question_processor::QuestionProcessor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub text: String,
    pub confidence: f32,
    pub source_chunk: String,
}

pub struct AnswerEngine;

impl AnswerEngine {
    pub fn answer(
        question: &str,
        document_chunks: &[String],
    ) -> Result<Answer, Box<dyn std::error::Error>> {
        // Process question
        let processed_q = QuestionProcessor::process(question);

        // Find relevant chunks
        let relevant_chunks = QuestionProcessor::find_relevant_chunks(&processed_q, document_chunks, 3);

        if relevant_chunks.is_empty() {
            return Err("No relevant content found".into());
        }

        // Get the most relevant chunk
        let (source_chunk, relevance_score) = relevant_chunks.first().unwrap();

        // Extract answer from the chunk (simple approach)
        let answer_text = Self::extract_answer(&processed_q, source_chunk);

        Ok(Answer {
            text: answer_text,
            confidence: relevance_score * 0.8,
            source_chunk: source_chunk.clone(),
        })
    }

    fn extract_answer(question: &str, chunk: &str) -> String {
    let question_words: Vec<&str> = question.split_whitespace().collect();

    // Check if chunk contains any question keywords
    let has_keywords = question_words.iter().any(|qw| {
        chunk.to_lowercase().contains(&qw.to_lowercase())
    });

    if has_keywords {
        // Split by lines to preserve formatting
        let lines: Vec<&str> = chunk.lines().collect();
        
        // Take up to 5 lines for a cleaner answer
        let answer_lines: Vec<&str> = lines
            .iter()
            .take(10)
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        // Join lines with newline character
        answer_lines.join("\n")
    } else {
        "No answer found in the provided context.".to_string()
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_engine() {
        let question = "graduation ceremony";
        let chunks = vec!["The graduation ceremony is on June 15 at the main hall.".to_string()];
        let result = AnswerEngine::answer(&question, &chunks);
        assert!(result.is_ok());
    }
}
