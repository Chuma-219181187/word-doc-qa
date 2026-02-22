use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    pub epoch: usize,
    pub train_loss: f32,
    pub val_loss: f32,
    pub accuracy: f32,
    pub epoch_time_secs: f32,
}

#[derive(Debug, Clone, Default)]
pub struct MetricsTracker {
    pub metrics: Vec<TrainingMetrics>,
    epoch_start: Option<Instant>,
}

impl MetricsTracker {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
            epoch_start: None,
        }
    }

    pub fn start_epoch(&mut self) {
        self.epoch_start = Some(Instant::now());
    }

    pub fn record_epoch(
        &mut self,
        epoch: usize,
        train_loss: f32,
        val_loss: f32,
        accuracy: f32,
    ) {
        let epoch_time = self
            .epoch_start
            .map(|start| start.elapsed().as_secs_f32())
            .unwrap_or(0.0);

        self.metrics.push(TrainingMetrics {
            epoch,
            train_loss,
            val_loss,
            accuracy,
            epoch_time_secs: epoch_time,
        });
    }

    pub fn get_best_epoch(&self) -> Option<&TrainingMetrics> {
        self.metrics.iter().min_by(|a, b| {
            a.val_loss
                .partial_cmp(&b.val_loss)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn print_summary(&self) {
        println!("\n=== Training Summary ===");
        for metric in &self.metrics {
            println!(
                "Epoch {}: Train Loss={:.4}, Val Loss={:.4}, Acc={:.4}, Time={:.2}s",
                metric.epoch, metric.train_loss, metric.val_loss, metric.accuracy, metric.epoch_time_secs
            );
        }

        if let Some(best) = self.get_best_epoch() {
            println!("\nBest Epoch: {} with Val Loss: {:.4}", best.epoch, best.val_loss);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_tracker() {
        let mut tracker = MetricsTracker::new();
        tracker.start_epoch();
        tracker.record_epoch(0, 2.5, 2.3, 0.75);
        assert_eq!(tracker.metrics.len(), 1);
    }
}
