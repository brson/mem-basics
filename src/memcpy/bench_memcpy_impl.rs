use self::memcpy as memcpy_impl;

use criterion::{black_box, criterion_group, Criterion};
use crate::tools;
use std::iter;

fn bench_memcpy_empty(c: &mut Criterion) {
    let src = tools::bytes(0);
    let src_ptr = src.as_ptr();
    let mut dst = iter::repeat(0).take(src.len()).collect::<Vec<_>>();
    let dst_ptr = dst.as_mut_ptr();

    let name = &format!("{}/memcpy_empty", module_path!());
    c.bench_function(name, |b| b.iter(|| {
        unsafe { memcpy_impl(dst_ptr, src_ptr, 0) };
        black_box(&dst);
    }));
}

fn bench_memcpy_1024(c: &mut Criterion) {
    let src = tools::bytes(1024);
    let src_ptr = src.as_ptr();
    let mut dst = iter::repeat(0).take(src.len()).collect::<Vec<_>>();
    let dst_ptr = dst.as_mut_ptr();
    
    let name = &format!("{}/memcpy_1024", module_path!());
    c.bench_function(name, |b| b.iter(|| {
        unsafe { memcpy_impl(dst_ptr, src_ptr, 0) };
        black_box(&dst);
    }));
}

criterion_group!(
    benches,
    bench_memcpy_empty,
    bench_memcpy_1024,
);
