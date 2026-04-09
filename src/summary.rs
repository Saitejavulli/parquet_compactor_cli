use std::time::Duration;

#[derive(Debug, Default)]
pub struct CompactionSummary {
    pub input_files_found: usize,
    pub output_files_written: usize,
    pub rows_processed: usize,
    pub input_bytes: u64,
    pub output_bytes: u64,
    pub target_size_bytes: u64,
    pub dry_run: bool,
    pub elapsed: Duration,
}

impl CompactionSummary {
    pub fn pretty_print(&self) -> String {
        format!(
            "\nCompaction Summary
------------------
Input files found: {}
Output files written: {}
Rows processed: {}
Input bytes: {}
Output bytes: {}
Target size bytes: {}
Dry run: {}
Elapsed time: {:.2?}
",
            self.input_files_found,
            self.output_files_written,
            self.rows_processed,
            self.input_bytes,
            self.output_bytes,
            self.target_size_bytes,
            self.dry_run,
            self.elapsed
        )
    }
}