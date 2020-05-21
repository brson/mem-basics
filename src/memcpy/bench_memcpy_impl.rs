use self::memcpy as memcpy_impl;

use criterion::{black_box, criterion_group, Criterion};
use crate::tools;
use std::iter;

fn bench_memcpy(c: &mut Criterion, len: usize) {
    let src = tools::bytes(len);
    let src_ptr = src.as_ptr();
    let mut dst = iter::repeat(0).take(len).collect::<Vec<_>>();
    let dst_ptr = dst.as_mut_ptr();

    let name = &format!("{}/memcpy_{}", module_path!(), len);
    c.bench_function(name, |b| b.iter(|| {
        unsafe { memcpy_impl(dst_ptr, src_ptr, len) };
        black_box(&dst);
    }));
}

fn bench_memcpy_0(c: &mut Criterion) {
    bench_memcpy(c, 0);
}

fn bench_memcpy_16(c: &mut Criterion) {
    bench_memcpy(c, 16);
}

fn bench_memcpy_1024(c: &mut Criterion) {
    bench_memcpy(c, 1024);
}

criterion_group!(
    benches,
    bench_memcpy_0,
    bench_memcpy_16,
    bench_memcpy_1024,
);
