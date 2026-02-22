use crate::config::{ModelConfig, TrainingConfig};
use crate::data::{Dataset, QAExample};
use crate::training::metrics::MetricsTracker;
use crate::training::checkpoint::{Checkpoint, CheckpointManager};
use std::path::Path;

pub struct Trainer {
    model_config: ModelConfig,
    training_config: TrainingConfig,
    metrics: MetricsTracker,
}

impl Trainer {
    pub fn new(model_config: ModelConfig, training_config: TrainingConfig) -> Self {
        Self {
            model_config,
            training_config,
            metrics: MetricsTracker::new(),
        }
    }

    pub fn train(
        &mut self,
        dataset: &Dataset,
        checkpoint_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Starting Training ===");
        println!("Epochs: {}", self.training_config.epochs);
        println!("Batch Size: {}", self.training_config.batch_size);
        println!("Learning Rate: {}", self.training_config.learning_rate);
        println!("Training Samples: {}", dataset.train_size());
        println!("Validation Samples: {}", dataset.val_size());

        for epoch in 0..self.training_config.epochs {
            self.metrics.start_epoch();
            println!("\n--- Epoch {}/{} ---", epoch + 1, self.training_config.epochs);

            // Simulate training loss
            let train_loss = self.simulate_epoch_loss(&dataset.train_examples, epoch);

            // Simulate validation loss
            let val_loss = self.simulate_epoch_loss(&dataset.val_examples, epoch);

            // Simulate accuracy
            let accuracy = 0.5 + (epoch as f32 * 0.05);

            // Record metrics
            self.metrics.record_epoch(epoch, train_loss, val_loss, accuracy);

            println!(
                "Train Loss: {:.4}, Val Loss: {:.4}, Accuracy: {:.4}",
                train_loss, val_loss, accuracy
            );

            // Save checkpoint
            let checkpoint = Checkpoint {
                epoch,
                train_loss,
                val_loss,
                accuracy,
            };

            CheckpointManager::save_checkpoint(&checkpoint, epoch, checkpoint_dir)?;
            println!("Checkpoint saved for epoch {}", epoch);
        }

        self.metrics.print_summary();
        Ok(())
    }

    fn simulate_epoch_loss(&self, examples: &[QAExample], epoch: usize) -> f32 {
        let base_loss = 3.0;
        let decay = 0.1 * (epoch as f32 + 1.0);
        (base_loss - decay).max(0.1)
    }

    pub fn get_metrics(&self) -> &MetricsTracker {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trainer_creation() {
        let trainer = Trainer::new(ModelConfig::default(), TrainingConfig::default());
        assert_eq!(trainer.training_config.epochs, 3);
    }
}
