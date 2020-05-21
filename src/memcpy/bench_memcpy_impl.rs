use self::memcpy as memcpy_impl;

use criterion::{black_box, criterion_group, Criterion};

fn bench_memcpy_empty(c: &mut Criterion) {
    let mut dst = Vec::new();
    let src = Vec::new();
    let dst_ptr = dst.as_mut_ptr();
    let src_ptr = src.as_ptr();

    let name = &format!("{}/memcpy_empty", module_path!());
    c.bench_function(name, |b| b.iter(|| {
        unsafe { memcpy_impl(dst_ptr, src_ptr, 0) };
        black_box(&dst);
    }));
}

criterion_group!(benches, bench_memcpy_empty);
