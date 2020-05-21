mod offset {
    use memcpy::memcpy_offset::memcpy;
    include!("test_memcpy_impl.rs");
}

mod accumulate {
    use memcpy::memcpy_accumulate::memcpy;
    include!("test_memcpy_impl.rs");
}

mod ptrcopy {
    use memcpy::memcpy_ptrcopy::memcpy;
    include!("test_memcpy_impl.rs");
}
