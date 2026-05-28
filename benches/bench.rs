use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_wig_to_bed(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-wig-to-bed");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let wig = manifest.join("tests/golden/signal.wig");

    c.bench_function("rsomics-wig-to-bed golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([wig.to_str().unwrap()])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_wig_to_bed);
criterion_main!(benches);
