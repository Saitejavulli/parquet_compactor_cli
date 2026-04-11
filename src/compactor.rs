use indicatif::{ProgressBar, ProgressStyle};
use crate::errors::PcompactError;
use crate::file_discovery::{discover_parquet_files, ensure_input_file_exists, file_size_bytes};
use crate::parquet_io::{read_parquet, write_parquet};
use crate::summary::CompactionSummary;
use anyhow::{bail, Context, Result};
use polars::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct CompactionOptions {
    pub input_dir: String,
    pub output_dir: String,
    pub target_size_bytes: u64,
    pub dry_run: bool,
    pub verbose: bool,
    pub threads: Option<usize>,
}

pub fn combine_parquet_files(input_dir: &str, output_file: &str) -> Result<()> {
    let files = discover_parquet_files(input_dir)?;
    if let Err(e) = validate_schema_consistency(&files) {
    tracing::error!("Schema validation failed during combine: {}", e);
    return Err(e);
}
    let mut combined: Option<DataFrame> = None;
    let mut total_rows = 0usize;

    if let Some(parent) = Path::new(output_file).parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
    }

    for file in files {
        info!("Reading {}", file.display());
        let df = match read_parquet(&file) {
        Ok(df) => df,
        Err(e) => {
        warn!("Skipping corrupted file: {} -> {}", file.display(), e);
        continue;
    }
};
        total_rows += df.height();

        combined = match combined {
            None => Some(df),
            Some(mut existing) => {
                existing.vstack_mut(&df)?;
                Some(existing)
            }
        };
    }

    let mut final_df = combined.context("No input parquet data found to combine")?;
    write_parquet(Path::new(output_file), &mut final_df)?;

    info!("Combined parquet written to {}", output_file);
    info!("Total rows combined: {}", total_rows);

    Ok(())
}

pub fn split_one_large_file_into_small_files(
    input_file: &str,
    output_dir: &str,
    rows_per_file: usize,
) -> Result<()> {
    ensure_input_file_exists(input_file)?;

    if rows_per_file == 0 {
        return Err(PcompactError::InvalidRowsPerFile.into());
    }

    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir))?;

    let df = read_parquet(Path::new(input_file))?;
    let total_rows = df.height();

    info!("Splitting file {}", input_file);
    info!("Total rows in large file: {}", total_rows);

    let mut start = 0usize;
    let mut file_counter = 0usize;

    while start < total_rows {
        let len = rows_per_file.min(total_rows - start);
        let offset = i64::try_from(start).context("Row offset does not fit into i64")?;
        let mut chunk = df.slice(offset, len);

        let out_path = Path::new(output_dir).join(format!("small_{:05}.parquet", file_counter));
        write_parquet(&out_path, &mut chunk)?;

        info!("Wrote {}", out_path.display());

        file_counter += 1;
        start += len;
    }

    info!("Created {} small parquet files", file_counter);
    Ok(())
}

pub fn compact_files(options: CompactionOptions) -> Result<CompactionSummary> {
    let start_time = Instant::now();

    if options.target_size_bytes == 0 {
        bail!("Target size cannot be zero");
    }

    fs::create_dir_all(&options.output_dir)
        .with_context(|| format!("Failed to create output directory: {}", options.output_dir))?;

    if options.verbose {
        info!("Verbose mode enabled");
    }
    info!("Starting compaction");
    info!("Using {} threads", options.threads.unwrap_or(1));
    info!("Target size: {} bytes", options.target_size_bytes);
    info!("Input directory: {}", options.input_dir);
    info!("Output directory: {}", options.output_dir);

    let files = discover_parquet_files(&options.input_dir)?;
    if let Err(e) = validate_schema_consistency(&files) {
    tracing::error!("Schema validation failed: {}", e);
    return Err(e);
}
    let groups = group_files_by_target_size(&files, options.target_size_bytes)?;
    let pb = ProgressBar::new(groups.len() as u64);
pb.set_style(
    ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")
        .unwrap()
);
    let mut summary = CompactionSummary::default();
    summary.input_files_found = files.len();
    summary.target_size_bytes = options.target_size_bytes;
    summary.dry_run = options.dry_run;

    if options.dry_run {
    println!("Dry run mode:");
    println!("Discovered {} parquet files", files.len());
    println!("Estimated {} output groups", groups.len());

    for (i, group) in groups.iter().enumerate() {
        println!(
            "  Would create compacted_{:03}.parquet from {} files",
            i + 1,
            group.len()
        );
    }

    summary.elapsed = start_time.elapsed();
    return Ok(summary);
}

    for (group_idx, group) in groups.iter().enumerate() {
    pb.inc(1);
    info!("Processing group {} with {} files", group_idx + 1, group.len());
        let mut combined: Option<DataFrame> = None;

        for file in group {
            let size = file_size_bytes(file)?;
            summary.input_bytes += size;

            let df = match read_parquet(file) {
    Ok(df) => df,
    Err(e) => {
        warn!("Skipping corrupted file: {} -> {}", file.display(), e);
        continue;
    }
};

summary.rows_processed += df.height();

            combined = match combined {
                None => Some(df),
                Some(mut existing) => {
                    existing.vstack_mut(&df)?;
                    Some(existing)
                }
            };
        }

        let mut final_df = combined.context("Encountered empty compaction group")?;
        let output_file =
            Path::new(&options.output_dir).join(format!("compacted_{:03}.parquet", group_idx + 1));

        write_parquet(&output_file, &mut final_df)?;
        summary.output_files_written += 1;
        summary.output_bytes += fs::metadata(&output_file)?.len();

        info!("Wrote {}", output_file.display());
    }

    pb.finish();

    summary.elapsed = start_time.elapsed();
    Ok(summary)
}

fn group_files_by_target_size(files: &[PathBuf], target_size: u64) -> Result<Vec<Vec<PathBuf>>> {
    let mut groups: Vec<Vec<PathBuf>> = Vec::new();
    let mut current_group: Vec<PathBuf> = Vec::new();
    let mut current_size = 0u64;

    for file in files {
        let size = file_size_bytes(file)?;

        if size > target_size {
            warn!(
                "File {} is larger than target size; placing it in its own group",
                file.display()
            );

            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
                current_size = 0;
            }

            groups.push(vec![file.clone()]);
            continue;
        }

        if current_size + size > target_size && !current_group.is_empty() {
            groups.push(current_group);
            current_group = Vec::new();
            current_size = 0;
        }

        current_group.push(file.clone());
        current_size += size;
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    Ok(groups)
}
fn validate_schema_consistency(files: &[PathBuf]) -> Result<()> {
    let mut base_schema = None;

    for file in files {
        let df = read_parquet(file)
            .with_context(|| format!("Failed while validating schema for {}", file.display()))?;

        match &base_schema {
            None => {
                base_schema = Some(df.schema().clone());
            }
            Some(schema) => {
                if df.schema() != schema {
                    return Err(anyhow::anyhow!(
                        "Schema mismatch detected in file: {}",
                        file.display()
                    ));
                }
            }
        }
    }

    Ok(())
} 