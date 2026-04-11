use anyhow::Result;
use clap::Parser;
use pcompact::cli::Cli;
use pcompact::compactor::{
    combine_parquet_files,
    compact_files,
    split_one_large_file_into_small_files,
    CompactionOptions,
};
use pcompact::config::AppConfig;
use pcompact::size_parser;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

fn init_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("info")
    } else {
        EnvFilter::new("warn")
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logging(cli.verbose);

    if cli.combine_only {
        let input = cli
            .input
            .clone()
            .unwrap_or_else(|| "sample_data/raw".to_string());

        let output = cli
            .output
            .clone()
            .unwrap_or_else(|| "sample_data/combined/all_months_combined.parquet".to_string());

        info!("Running combine-only mode");
        combine_parquet_files(&input, &output)?;
        return Ok(());
    }

    if cli.split_one_file {
        let input = cli
            .input
            .clone()
            .unwrap_or_else(|| "sample_data/combined/all_months_combined.parquet".to_string());

        let output = cli
            .output
            .clone()
            .unwrap_or_else(|| "sample_data/small_files".to_string());

        let rows_per_file = cli.rows_per_file.unwrap_or(50_000);

        info!("Running split-one-file mode");
        split_one_large_file_into_small_files(&input, &output, rows_per_file)?;
        return Ok(());
    }

    let file_config = if let Some(config_path) = &cli.config {
        Some(AppConfig::from_file(config_path)?)
    } else {
        None
    };

    let merged = AppConfig::merge(file_config, &cli)?;
    let target_bytes = size_parser::parse_size(&merged.target_size)?;

    let options = CompactionOptions {
    input_dir: merged.input.clone(),
    output_dir: merged.output.clone(),
    target_size_bytes: target_bytes,
    dry_run: merged.dry_run,
    verbose: merged.verbose,
    threads: merged.threads,
    };

    match compact_files(options) {
        Ok(summary) => {
            println!("{}", summary.pretty_print());
            return Ok(());
        }
        Err(e) => {
            error!("Compaction failed: {:?}", e);
            Err(e)
        }
    }
}