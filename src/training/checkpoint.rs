use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub epoch: usize,
    pub train_loss: f32,
    pub val_loss: f32,
    pub accuracy: f32,
}

pub struct CheckpointManager;

impl CheckpointManager {
    pub fn save_checkpoint(
        checkpoint: &Checkpoint,
        epoch: usize,
        checkpoint_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(checkpoint_dir)?;

        let checkpoint_path = checkpoint_dir.join(format!("model_epoch_{}.json", epoch));
        let json = serde_json::to_string_pretty(checkpoint)?;
        fs::write(checkpoint_path, json)?;

        Ok(())
    }

    pub fn load_checkpoint(
        checkpoint_path: &Path,
    ) -> Result<Checkpoint, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(checkpoint_path)?;
        let checkpoint = serde_json::from_str(&json)?;
        Ok(checkpoint)
    }

    pub fn list_checkpoints(checkpoint_dir: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut checkpoints = Vec::new();

        if checkpoint_dir.exists() {
            for entry in fs::read_dir(checkpoint_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    if let Some(filename) = path.file_name() {
                        checkpoints.push(filename.to_string_lossy().to_string());
                    }
                }
            }
        }

        checkpoints.sort();
        Ok(checkpoints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_checkpoint_save() {
        let temp_dir = TempDir::new().unwrap();
        let checkpoint = Checkpoint {
            epoch: 0,
            train_loss: 2.5,
            val_loss: 2.3,
            accuracy: 0.75,
        };

        let result = CheckpointManager::save_checkpoint(&checkpoint, 0, temp_dir.path());
        assert!(result.is_ok());
    }
}
