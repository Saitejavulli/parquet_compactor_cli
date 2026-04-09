use anyhow::{Context, Result};
use polars::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn read_parquet(path: &Path) -> Result<DataFrame> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open parquet file: {}", path.display()))?;

    let df = ParquetReader::new(file)
        .finish()
        .with_context(|| format!("Failed to read parquet file: {}", path.display()))?;

    Ok(df)
}

pub fn write_parquet(path: &Path, df: &mut DataFrame) -> Result<()> {
    let file = File::create(path)
        .with_context(|| format!("Failed to create parquet file: {}", path.display()))?;

    ParquetWriter::new(file)
        .finish(df)
        .with_context(|| format!("Failed to write parquet file: {}", path.display()))?;

    Ok(())
}