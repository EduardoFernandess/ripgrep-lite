# ripgrep-lite

Recursive regex search over a directory tree. Walks files (honoring `.gitignore`), then scans them in parallel with Rayon.

## Stack

- **Rust** — CLI binary
- **Clap** — argument parsing
- **regex** — pattern matching
- **ignore** — gitignore-aware directory walks
- **Rayon** — parallel file scanning

## What was built

- Case-sensitive and case-insensitive search (`-i`)
- Optional thread count (`-j`)
- Parallel matching across files for faster trees
- Exit status `1` when nothing matches (grep-compatible behavior)
- Unit tests for search helpers

## Run

```bash
cargo run --release -- "fn main" ./src
cargo run --release -- -i TODO .
cargo run --release -- -j 8 pattern ./path
```

### Tests

```bash
cargo test
```
