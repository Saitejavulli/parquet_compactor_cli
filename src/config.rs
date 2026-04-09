use crate::cli::Cli;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub input: String,
    pub output: String,
    pub target_size: String,
    pub dry_run: bool,
    pub verbose: bool,
    pub threads: Option<usize>,
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;

        let cfg: AppConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config TOML: {}", path))?;

        Ok(cfg)
    }

    pub fn merge(file_cfg: Option<Self>, cli: &Cli) -> Result<Self> {
        let base = file_cfg.unwrap_or(AppConfig {
            input: "sample_data/small_files".to_string(),
            output: "sample_data/compacted".to_string(),
            target_size: "128MB".to_string(),
            dry_run: false,
            verbose: false,
            threads: Some(4),
        });

        Ok(AppConfig {
            input: cli.input.clone().unwrap_or(base.input),
            output: cli.output.clone().unwrap_or(base.output),
            target_size: cli.target_size.clone().unwrap_or(base.target_size),
            dry_run: if cli.dry_run { true } else { base.dry_run },
            verbose: if cli.verbose { true } else { base.verbose },
            threads: cli.threads.or(base.threads),
        })
    }
}