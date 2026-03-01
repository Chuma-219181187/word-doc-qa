use std::path::Path;
use crate::data::DocumentLoader;
use crate::inference::AnswerEngine;

pub struct AskCommand;

impl AskCommand {
    pub fn execute(
        question: &str,
        data_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Document QA System - Inference Mode ===\n");
        println!("Question: {}\n", question);

        // Load documents
        println!("Loading documents from: {:?}", data_dir);
        let documents = DocumentLoader::load_documents(data_dir)?;

        if documents.is_empty() {
            println!("No documents found. Using sample data for demonstration...");
            return Self::demo_answer(question);
        }

        // Get all chunks from documents
        let mut all_chunks = Vec::new();
        for doc in documents {
            let chunks: Vec<String> = doc
                .split('.')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
                .collect();
            all_chunks.extend(chunks);
        }

        println!("Using {} document chunks", all_chunks.len());

        // Get answer
        match AnswerEngine::answer(question, &all_chunks) {
            Ok(answer) => {
                println!("\n=== Answer ===");
                println!("Text: {}", answer.text);
                println!("Confidence: {:.2}%", answer.confidence * 100.0);
                println!("Source: {}", answer.source_chunk.chars().take(100).collect::<String>());
            }
            Err(e) => {
                println!("Error finding answer: {}", e);
            }
        }

        Ok(())
    }

    fn demo_answer(question: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Using Sample Data ===\n");

        let sample_documents = vec![
            "The 2026 graduation ceremony will be held on June 15 at the main auditorium.".to_string(),
            "The academic year starts in February and ends in January of the next year.".to_string(),
            "HDC meetings occur monthly on the first Tuesday. In 2024 there were 12 meetings.".to_string(),
            "The semester ends on April 30 with final examinations.".to_string(),
            "Students can register for courses starting from December 1st.".to_string(),
        ];

        match AnswerEngine::answer(question, &sample_documents) {
            Ok(answer) => {
                println!("=== Answer ===");
                println!("Text: {}", answer.text);
                println!("Confidence: {:.2}%", answer.confidence * 100.0);
                println!("Source: {}", answer.source_chunk);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        Ok(())
    }
}
