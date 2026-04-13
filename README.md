# Parquet Compactor CLI (Rust)

A high-performance Command Line Interface (CLI) tool built in Rust to solve the **small file problem** in data engineering by compacting multiple small Parquet files into optimized larger files.

---

## Problem Statement

In modern data platforms such as Apache Spark, Hive, and Presto, large numbers of small Parquet files lead to:

* High metadata overhead
* Slower query execution
* Increased I/O operations
* Inefficient distributed processing

This is commonly known as the **small file problem**.

---

## Solution

This CLI tool:

* Reads a directory of small Parquet files
* Validates schema consistency across files
* Groups files based on a target output size (e.g., 128MB or 256MB)
* Merges them into fewer optimized Parquet files
* Supports dry-run mode for safe execution preview

---

## CLI Requirements (Rubric Compliance)

This application follows standard CLI best practices:

* `-h` / `--help` → Display usage instructions
* `-V` / `--version` → Display application version
* Clear input and output arguments
* Proper validation of required parameters
* Meaningful error messages for invalid input

---

## Features

### Core Features

* Directory-based Parquet ingestion
* File compaction based on target size
* Schema validation across input files
* Summary report generation
* Efficient processing of large datasets

### Advanced Features

* Dry-run mode (no file writing)
* Progress bar using Indicatif
* Verbose logging
* Config file support (TOML)
* Multi-thread support
* Benchmarking using Criterion
* Handles corrupted/invalid files gracefully

---

## Technology Stack

* Rust
* Polars (Parquet processing)
* Clap (CLI parsing)
* Indicatif (progress bar)
* Tracing (logging)
* Criterion (benchmarking)

---

## Project Structure

```
pcompact/
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── compactor.rs
│   ├── parquet_io.rs
│   ├── file_discovery.rs
│   ├── size_parser.rs
│   ├── summary.rs
│   └── errors.rs
├── tests/
├── benches/
├── config.toml
├── Cargo.toml
└── README.md
```

---

## Usage

### Using CLI Arguments

```bash
cargo run -- -i sample_data/raw -o sample_data/out --target-size 128MB
```

### With Verbose Logging

```bash
cargo run -- -i sample_data/raw -o sample_data/out --target-size 128MB --verbose
```

### Dry Run Mode

```bash
cargo run -- -i sample_data/raw -o sample_data/out --target-size 128MB --dry-run
```

---

## Using Config File

```bash
cargo run -- --config config.toml
```

### Example `config.toml`

```toml
input = "sample_data/raw"
output = "sample_data/out"
target_size = "128MB"
dry_run = false
verbose = true
threads = 4
```

---

## Sample Output

```
Input files found: 262
Output files written: 2
Rows processed: 13069067
Elapsed time: ~80s
```

---

## Dry Run Example

```
Would create compacted_001.parquet
Would create compacted_002.parquet
```

---

## Benchmarking

Run performance benchmarks:

```bash
cargo bench
```

Example output:

```
compaction_small time: [850 ms 880 ms 920 ms]
```

---

## Data Engineering Impact

This project demonstrates:

* Reduction in file count
* Improved query planning performance
* Lower metadata overhead
* Faster data scanning
* Efficient batching strategies

---

## Limitations

* Assumes consistent schema across files
* Partition-aware compaction not implemented
* Basic threading (not fully parallel)

---

## Future Enhancements

* Partition-aware compaction
* Full parallel processing
* Cloud storage support (S3, Azure)
* Incremental compaction
* Schema evolution support

---

## Author

**Saiteja Vulli**
Master’s in Computer Science
Aspiring Software Engineer

---

## Conclusion

This project provides a practical and efficient implementation of a real-world data engineering optimization problem using Rust, focusing on performance, scalability, and reliability.
