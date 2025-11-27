# Rust Wordcount

This is a Rust implementation of the wordcount function from the parent MoonBit project, using Rust's `regex` crate.

## Building and Running

```bash
# Build the project
cargo build --release

# Run the program
cargo run --release

# Run tests
cargo test

# Run benchmarks (使用 Criterion)
cargo bench
```

## Benchmarking

This project uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for benchmarking. Criterion provides:
- Statistical analysis of performance
- Detection of performance regressions
- HTML reports with graphs

After running `cargo bench`, you can find detailed HTML reports in `target/criterion/`.

## Implementation Details

The `wordcount` function uses Rust's regex library to match:
- `(\n)` - newlines (increments line count)
- `([^ \t\r\n]+)` - words (non-whitespace sequences, increments word count and adds to char count)
- `(.)` - any other character (increments char count)

This implementation mirrors the logic from the MoonBit `lexmatch` version in the parent directory.
