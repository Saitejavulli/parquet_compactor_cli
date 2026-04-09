use thiserror::Error;

#[derive(Error, Debug)]
pub enum PcompactError {
    #[error("Input directory does not exist: {0}")]
    InputDirMissing(String),

    #[error("Input file does not exist: {0}")]
    InputFileMissing(String),

    #[error("No parquet files found in input directory: {0}")]
    NoParquetFiles(String),

    #[error("Invalid target size: {0}")]
    InvalidTargetSize(String),

    #[error("Output directory is invalid: {0}")]
    InvalidOutputDir(String),

    #[error("Rows per file must be greater than zero")]
    InvalidRowsPerFile,
}