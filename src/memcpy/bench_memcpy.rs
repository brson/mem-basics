use criterion::criterion_main;

mod offset {
    use memcpy::memcpy_offset::memcpy;
    include!("bench_memcpy_impl.rs");
}

mod offset_rev {
    use memcpy::memcpy_offset_rev::memcpy;
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

mod copyto {
    use memcpy::memcpy_copyto::memcpy;
    include!("bench_memcpy_impl.rs");
}

mod copyfrom {
    use memcpy::memcpy_copyfrom::memcpy;
    include!("bench_memcpy_impl.rs");
}

criterion_main!(
    offset::benches,
    offset_rev::benches,
    accumulate::benches,
    ptrcopy::benches,
    copyto::benches,
    copyfrom::benches,
);
