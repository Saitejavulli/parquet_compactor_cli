# Parquet Compactor CLI (Rust)

A high-performance Command Line Interface (CLI) tool built in Rust to address the small file problem in data engineering by compacting multiple small Parquet files into optimized larger files.

---

## Problem Statement

In modern data platforms such as Apache Spark, Hive, and Presto, large numbers of small Parquet files lead to:

* High metadata overhead
* Slower query execution
* Increased I/O operations
* Inefficient distributed processing

This is commonly referred to as the small file problem.

---

## Solution

This tool:

* Reads a directory of small Parquet files
* Validates schema consistency across files
* Groups files based on a target output size (e.g., 128MB)
* Merges them into fewer optimized Parquet files
* Supports a dry-run mode to simulate execution without writing output

---

## Features

### Core Features

* Directory-based Parquet ingestion
* File compaction based on configurable target size
* Schema validation across input files
* Efficient processing of large datasets
* Summary report generation

### Advanced Features

* Dry-run mode for safe simulation
* Progress tracking using a progress bar
* Structured logging
* Graceful handling of corrupted or invalid files
* Thread configuration support
* Config file support (TOML)
* Benchmarking using Criterion
* Optional performance visualization using Gnuplot

---

## Technology Stack

* Rust
* Polars (Parquet processing)
* Clap (CLI argument parsing)
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
│   └── size_tests.rs
├── benches/
│   └── compaction_bench.rs
├── config.toml
├── Cargo.toml
└── README.md
```

---

## Usage

### Using CLI Arguments

```
cargo run -- -i sample_data/small_files -o sample_data/compacted --target-size 128MB --verbose
```

### Using Config File

```
cargo run -- --config config.toml
```

---

## Configuration Example

```
input = "sample_data/small_files"
output = "sample_data/compacted"
target_size = "128MB"
dry_run = false
verbose = true
threads = 4
```

---

## Dry Run Mode

Run with:

```
dry_run = true
```

Example output:

```
Would create compacted_001.parquet
Would create compacted_002.parquet
```

This mode allows validation of grouping logic without writing files.

---

## Real Execution

Run with:

```
dry_run = false
```

Example output:

```
Wrote compacted_001.parquet
Wrote compacted_002.parquet
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

## Benchmarking

Run:

```
cargo bench
```

Example output:

```
compaction_small time: [859 ms 873 ms 887 ms]
```

---

## Benchmark Visualization

If Gnuplot is installed:

```
target/criterion/compaction_small/report/index.html
```

Open this file in a browser to view performance graphs.

---

## Data Engineering Perspective

This project demonstrates:

* Reduction in file count to improve query planning
* Lower metadata overhead
* Improved scan efficiency
* Efficient batching and processing strategies
* Practical system-level optimization using Rust

---

## Limitations

* Assumes consistent schema across all input files
* Partition-aware compaction is not implemented
* Threading support is basic and not fully parallel

---

## Future Enhancements

* Partition-aware compaction
* Full parallel processing
* Integration with cloud storage (S3, Azure Blob)
* Incremental compaction support
* Schema evolution handling

---

## Author

Saiteja Vulli
Master’s in Computer Science
Aspiring Software Engineer

---

## Conclusion

This project provides a practical implementation of a data engineering optimization technique using Rust, focusing on performance, reliability, and scalability.
