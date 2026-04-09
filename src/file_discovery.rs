use crate::errors::PcompactError;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn discover_parquet_files(input_dir: &str) -> Result<Vec<PathBuf>> {
    let path = Path::new(input_dir);

    if !path.exists() {
        return Err(PcompactError::InputDirMissing(input_dir.to_string()).into());
    }

    let mut files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_file() {
            if let Some(ext) = p.extension() {
                if ext.to_string_lossy().to_lowercase() == "parquet" {
                    files.push(p.to_path_buf());
                }
            }
        }
    }

    files.sort();

    if files.is_empty() {
        return Err(PcompactError::NoParquetFiles(input_dir.to_string()).into());
    }

    Ok(files)
}

pub fn ensure_input_file_exists(path: &str) -> Result<()> {
    let p = Path::new(path);
    if !p.exists() || !p.is_file() {
        return Err(PcompactError::InputFileMissing(path.to_string()).into());
    }
    Ok(())
}

pub fn file_size_bytes(path: &Path) -> Result<u64> {
    Ok(fs::metadata(path)?.len())
}