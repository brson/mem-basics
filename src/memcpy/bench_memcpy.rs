use criterion::criterion_main;

mod offset {
    use memcpy::memcpy_offset::memcpy;
    include!("bench_memcpy_impl.rs");
}

mod accumulate {
    use memcpy::memcpy_accumulate::memcpy;
    include!("bench_memcpy_impl.rs");
}

mod ptrcopy {
    use memcpy::memcpy_ptrcopy::memcpy;
    include!("bench_memcpy_impl.rs");
}

criterion_main!(
    offset::benches,
    accumulate::benches,
    ptrcopy::benches,
);
