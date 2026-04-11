use criterion::{criterion_group, criterion_main, Criterion};
use pcompact::compactor::{compact_files, CompactionOptions};

fn benchmark_compaction(c: &mut Criterion) {
    c.bench_function("compaction_small", |b| {
        b.iter(|| {
            let options = CompactionOptions {
                input_dir: "sample_data/small_files".to_string(),
                output_dir: "sample_data/bench_output".to_string(),
                target_size_bytes: 128 * 1024 * 1024,
                dry_run: true,
                verbose: false,
                threads: Some(1),
            };

            let _ = compact_files(options);
        });
    });
}

criterion_group!(benches, benchmark_compaction);
criterion_main!(benches);