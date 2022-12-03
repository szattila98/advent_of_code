# Advent of Code 2022

## Requirements

- **rustup** >=1.25.1
- **rustc** >=1.65.0

## How to run

- Run with `cargo run --release` to run the optimized version.
- Or build it with `cargo build --release` and run the executable located in the target directory.
- Benchmark with `cargo bench` to see benchmarks, defined in `benches/benchmark.rs` with criterion benchmark library.
- If input files change, run `cargo clean` to cause a rebuild, because cargo does not pick up the changes in the input files and loads cached artifacts, so changes won't be visible.
