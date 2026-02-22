# Example Outputs

## Training Example

Command:
```bash
cargo run -- train
```

Output:
```
=== Document QA System - Training Mode ===

Configuration loaded:
  Model: 6 layers, 512 dim, 8 heads
  Training: 3 epochs, batch size 16

Loading documents from: "data/documents"
Loaded 1 documents
Created dataset with X examples
Train: X, Validation: X

=== Starting Training ===
Epochs: 3
Batch Size: 16
Learning Rate: 0.0003

--- Epoch 1/3 ---
Train Loss: 2.9000, Val Loss: 2.8500, Accuracy: 0.5500
Checkpoint saved for epoch 0

--- Epoch 2/3 ---
Train Loss: 2.4000, Val Loss: 2.3500, Accuracy: 0.6000
Checkpoint saved for epoch 1

--- Epoch 3/3 ---
Train Loss: 1.9000, Val Loss: 1.8500, Accuracy: 0.6500
Checkpoint saved for epoch 2

=== Training Summary ===
Best Epoch: 2 with Val Loss: 1.8500
Training completed successfully!
```

## Inference Examples

### Example 1: Semester End Date

Command:
```bash
cargo run -- ask "When does semester end?"
```

Output:
```
=== Document QA System - Inference Mode ===

Question: When does semester end?

Loading documents from: "data/documents"
Using 10 document chunks

=== Answer ===
Text: The semester ends on April 30, 2026.
Confidence: 80.00%
Source: ACADEMIC CALENDAR AND IMPORTANT DATES
Semester End Date
The semester ends on April 30, 2026.
```

### Example 2: Graduation Date

Command:
```bash
cargo run -- ask "When is graduation?"
```

Output:
```
=== Answer ===
Text: The 2026 graduation ceremony will be held on June 15 at the main auditorium.
Confidence: 85.00%
```

### Example 3: Academic Calendar

Command:
```bash
cargo run -- ask "What is the academic calendar?"
```

Output:
```
=== Answer ===
Text: The academic year consists of four semesters.
Semester 1 runs from February to April.
Semester 2 runs from May to July.
Confidence: 78.00%
```

## System Statistics

Command:
```bash
cargo run -- stats
```

Output:
```
=== System Statistics ===
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

## Help Command

Command:
```bash
cargo run -- help
```

Output:
```
=== Document Question Answering System ===

Usage:
  cargo run -- train              # Train the model
  cargo run -- ask "question"     # Ask a question
  cargo run -- evaluate           # Evaluate the model
  cargo run -- stats              # Show statistics
  cargo run -- help               # Show this help message
```