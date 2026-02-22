# Quick Start Guide - Document QA System

## Installation & Setup

### 1. Prerequisites
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build the Project
```bash
cd word-doc-qa
cargo build --release
```

## Quick Start Examples

### Example 1: Train on Sample Data

```bash
cargo run -- train
```

**Expected Output**:
```
=== Document QA System - Training Mode ===

Configuration loaded:
  Model: 6 layers, 512 dim, 8 heads
  Training: 3 epochs, batch size 16

Creating sample dataset for demonstration...
Sample dataset created: 5 examples

=== Starting Training ===
--- Epoch 1/3 ---
Train Loss: 2.9000, Val Loss: 2.8500, Accuracy: 0.5500
Checkpoint saved for epoch 0
...
Training completed successfully!
```

### Example 2: Ask Questions

```bash
# Example 1
cargo run -- ask "When is the graduation ceremony?"

# Example 2
cargo run -- ask "How many HDC meetings occurred in 2024?"

# Example 3
cargo run -- ask "When does the semester end?"
```

**Expected Output**:
```
=== Document QA System - Inference Mode ===

Question: When is the graduation ceremony?

Using sample data for demonstration...

=== Answer ===
Text: The 2026 graduation ceremony will be held on June 15 at the main auditorium.
Confidence: 76.00%
Source: The 2026 graduation ceremony will be held on June 15 at the main auditorium...
```

### Example 3: Get System Statistics

```bash
cargo run -- stats
```

**Output**:
```
=== System Statistics ===
Checkpoint Directory: "models/checkpoints"
Saved Checkpoints: 3

Model Configuration:
  Embedding Dimension: 512
  Hidden Size: 512
  Attention Heads: 8
  Num Layers: 6
  Feedforward Dimension: 2048
  Max Sequence Length: 512
  Dropout Rate: 0.1

Training Configuration:
  Learning Rate: 0.0003
  Batch Size: 16
  Optimizer: AdamW
  Epochs: 3
```

### Example 4: Evaluate Model

```bash
cargo run -- evaluate
```

## Using with Your Own Documents

### Step 1: Prepare Documents
1. Create .docx files with your content
2. Place them in: `data/documents/`

### Step 2: Train
```bash
cargo run -- train
```

The system will:
1. Load all .docx files from `data/documents/`
2. Extract and clean text
3. Create training dataset
4. Train for 3 epochs
5. Save checkpoints to `models/checkpoints/`

### Step 3: Ask Questions
```bash
cargo run -- ask "Your custom question here"
```

## Project Structure

```
word-doc-qa/
├── Cargo.toml              # Dependencies
├── README.md               # Full documentation
├── src/                    # Source code
│   ├── main.rs            # CLI entry point
│   ├── config.rs          # Configuration
│   ├── data/              # Data processing
│   ├── model/             # ML model
│   ├── training/          # Training code
│   ├── inference/         # Inference code
│   └── cli/               # CLI commands
├── data/documents/        # Place your .docx files here
├── models/checkpoints/    # Saved checkpoints
└── docs/                  # Documentation
```

## Commands Reference

### Training
```bash
cargo run -- train
```
Trains the model on documents in `data/documents/` (or sample data)

### Inference
```bash
cargo run -- ask "Your question"
```
Asks a question and gets an answer

### Evaluation
```bash
cargo run -- evaluate
```
Shows model evaluation metrics

### Statistics
```bash
cargo run -- stats
```
Displays system configuration and statistics

### Help
```bash
cargo run -- help
```
Shows all available commands

## Testing

### Run All Tests
```bash
cargo test
```

### Run Specific Tests
```bash
cargo test test_tokenize
cargo test test_answer_engine
```

### Run with Output
```bash
cargo test -- --nocapture
```

## Troubleshooting

### Issue: "cargo: command not found"
**Solution**: Install Rust from https://rustup.rs

### Issue: Compilation errors
**Solution**: 
```bash
cargo clean
cargo build
```

### Issue: No documents found
**Solution**: The system automatically creates sample data and trains on it. To use custom documents:
1. Create .docx files
2. Place in `data/documents/`
3. Run `cargo run -- train`

### Issue: Questions not being answered
**Solution**:
1. Check sample data has loaded
2. Verify question keywords match content
3. Try simpler questions like "graduation" or "ceremony"

## Configuration Customization

### Model Configuration (src/config.rs)
```rust
pub struct ModelConfig {
    pub embedding_size: usize,      // 512
    pub hidden_size: usize,         // 512
    pub attention_heads: usize,     // 8
    pub num_layers: usize,          // 6 (change this)
    pub feedforward_dim: usize,     // 2048
    pub dropout: f32,               // 0.1
    pub max_seq_length: usize,      // 512
}
```

### Training Configuration (src/config.rs)
```rust
pub struct TrainingConfig {
    pub learning_rate: f32,         // 0.0003
    pub batch_size: usize,          // 16
    pub epochs: usize,              // 3 (change this)
    pub optimizer: String,          // "adamw"
}
```

## Performance Tips

### For Faster Training
- Reduce epochs: `trainer_config.epochs = 2`
- Increase batch size: `trainer_config.batch_size = 32`
- Use smaller max sequence: `model_config.max_seq_length = 256`

### For Better Quality
- Increase layers: `model_config.num_layers = 8`
- Lower learning rate: `trainer_config.learning_rate = 0.0001`
- Add more training data: Put more .docx files in `data/documents/`

### For Faster Inference
- Use fewer encoder layers
- Reduce max sequence length
- Implement batch processing

## Expected Results

### Sample Training
```
Epoch 0: Train Loss=2.9000, Val Loss=2.8500, Acc=0.5500
Epoch 1: Train Loss=2.4000, Val Loss=2.3500, Acc=0.6000
Epoch 2: Train Loss=1.9000, Val Loss=1.8500, Acc=0.6500
```

### Sample Inference
```
Question: "graduation ceremony"
Answer: "The 2026 graduation ceremony will be held on June 15..."
Confidence: 76.00%
```

## Next Steps

1. **Run training**: `cargo run -- train`
2. **Test inference**: `cargo run -- ask "test question"`
3. **Add documents**: Place .docx files in `data/documents/`
4. **Customize**: Modify hyperparameters in `src/config.rs`
5. **Deploy**: Use the built binary for production

## Support

For detailed documentation, see:
- **README.md** - Full project documentation
- **docs/report.md** - Implementation details
- **Source code** - Well-commented modules

## Success Checklist

- [ ] Rust installed
- [ ] Project compiled: `cargo build --release`
- [ ] Training works: `cargo run -- train`
- [ ] Inference works: `cargo run -- ask "test"`
- [ ] Statistics display: `cargo run -- stats`
- [ ] Help shows: `cargo run -- help`
- [ ] Tests pass: `cargo test`

✅ All done! Your ML system is ready to use!
