use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_memcpy(c: &mut Criterion) {
    c.bench_function("memcpy", |b| b.iter(|| {
        black_box(());
    }));
}

criterion_group!(benches, bench_memcpy);
criterion_main!(benches);
