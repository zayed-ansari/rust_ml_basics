# BPE Tokenizer in Rust

The same sub-word tokenization algorithm used by GPT  built from scratch in Rust.

## What it does
Learns to merge frequent character pairs from a corpus, then uses those merges to tokenize unseen words into meaningful sub-word units.

## Example output
```
Merge 1: ("e", "s")
Merge 2: ("es", "t")
...
Merge 9: ("low", "</w>")

Tokenizing unseen words:
'lowest' → ["low", "est</w>"]
'low'    → ["low</w>"]
```

## Why this matters
GPT does not tokenize word by word. It uses BPE to break text into sub-word pieces so "lowest" becomes ["low", "est"] even if it was never seen during training.

## Run it
```bash
cargo run
```

## Built with
- Rust 
- Zero external dependencies
