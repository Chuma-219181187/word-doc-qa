use std::path::Path;
use crate::config::{ModelConfig, TrainingConfig, DataConfig};
use crate::data::{DocumentLoader, Dataset, QAExample, TextCleaner, Tokenizer};
use crate::training::Trainer;

pub struct TrainCommand;

impl TrainCommand {
    pub fn execute(
        data_dir: &Path,
        checkpoint_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Document QA System - Training Mode ===\n");

        // Load configuration
        let model_config = ModelConfig::default();
        let training_config = TrainingConfig::default();
        let data_config = DataConfig::default();

        println!("Configuration loaded:");
        println!("  Model: {} layers, {} dim, {} heads", 
            model_config.num_layers, model_config.embedding_size, model_config.attention_heads);
        println!("  Training: {} epochs, batch size {}", 
            training_config.epochs, training_config.batch_size);

        // Load documents
        println!("\nLoading documents from: {:?}", data_dir);
        let documents = DocumentLoader::load_documents(data_dir)?;
        println!("Loaded {} documents", documents.len());

        if documents.is_empty() {
            println!("Loaded {} documents", 0);

             // List what files are in the directory
    if let Ok(entries) = std::fs::read_dir(data_dir) {
        println!("Files found in directory:");
        for entry in entries {
            if let Ok(entry) = entry {
                println!("  - {:?}", entry.path());
            }
        }
    }
}

println!("Loaded {} documents", documents.len());
        

        // Create dataset
        let mut dataset = Dataset::new();

        let tokenizer = Tokenizer::new(30000, model_config.max_seq_length);

        for (_doc_idx, doc) in documents.iter().enumerate() {
            let cleaned = TextCleaner::clean(doc);
            let chunks = TextCleaner::split_into_chunks(&cleaned, data_config.chunk_size);

            for (chunk_idx, chunk) in chunks.iter().enumerate() {
                let tokens = tokenizer.tokenize(chunk);

                let example = QAExample {
                    context: chunk.clone(),
                    question: format!("What is mentioned in section {}?", chunk_idx),
                    answer_start: 0,
                    answer_text: chunk.split_whitespace().take(3).collect::<Vec<_>>().join(" "),
                    tokens,
                };

                dataset.add_example(example);
            }
        }

        println!("Created dataset with {} examples", dataset.size());

        // Shuffle and split
        dataset.shuffle();
        dataset.split(data_config.train_split);

        println!("Train: {}, Validation: {}", dataset.train_size(), dataset.val_size());

        // Train
        let mut trainer = Trainer::new(model_config, training_config);
        trainer.train(&dataset, checkpoint_dir)?;

        println!("\nTraining completed successfully!");
        Ok(())
    }

    fn create_sample_dataset(checkpoint_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("\nCreating sample dataset for demonstration...");

        let sample_texts = vec![
            "The 2026 graduation ceremony will be held on June 15 at the main auditorium. All students must arrive by 8:00 AM.",
            "The academic calendar includes four semesters. Semester 1 begins in February and ends in April.",
            "HDC meetings are held monthly on the first Tuesday of each month. In 2024, there were 12 meetings.",
            "The university library is open from 8:00 AM to 8:00 PM on weekdays and 10:00 AM to 4:00 PM on weekends.",
            "Student enrollment for the next academic year is now open. The deadline is December 31, 2025.",
        ];

        let mut dataset = Dataset::new();
        let tokenizer = Tokenizer::new(30000, 512);

        for text in sample_texts {
            let tokens = tokenizer.tokenize(text);
            let example = QAExample {
                context: text.to_string(),
                question: "What is the main topic?".to_string(),
                answer_start: 0,
                answer_text: text.split_whitespace().take(5).collect::<Vec<_>>().join(" "),
                tokens,
            };
            dataset.add_example(example);
        }

        dataset.shuffle();
        dataset.split(0.8);

        println!("Sample dataset created: {} examples", dataset.size());

        // Train on sample data
        let mut trainer = Trainer::new(ModelConfig::default(), TrainingConfig::default());
        trainer.train(&dataset, checkpoint_dir)?;

        println!("\nSample training completed!");
        Ok(())
    }
}
