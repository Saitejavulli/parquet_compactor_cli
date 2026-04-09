use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "pcompact",
    version,
    about = "Parquet Compactor CLI: combine, split, and compact parquet files"
)]
pub struct Cli {
    #[arg(short, long, help = "Input file or directory path")]
    pub input: Option<String>,

    #[arg(short, long, help = "Output file or directory path")]
    pub output: Option<String>,

    #[arg(long = "target-size", help = "Target output size, e.g. 128MB")]
    pub target_size: Option<String>,

    #[arg(long, help = "Dry run mode")]
    pub dry_run: bool,

    #[arg(long, help = "Enable verbose logging")]
    pub verbose: bool,

    #[arg(long, help = "Path to config TOML file")]
    pub config: Option<String>,

    #[arg(long, help = "Optional thread count")]
    pub threads: Option<usize>,

    #[arg(long, help = "Combine all parquet files from input directory into one parquet file")]
    pub combine_only: bool,

    #[arg(long, help = "Split one large parquet file into many smaller parquet files")]
    pub split_one_file: bool,

    #[arg(long, help = "Rows per file when splitting one large parquet file")]
    pub rows_per_file: Option<usize>,
}