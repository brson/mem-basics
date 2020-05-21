pub unsafe fn memcpy(dst: *mut u8, src: *const u8, bytes: usize) {
    crate::memcpy_assert(dst, src, bytes);

    let mut dst = dst;
    let mut src = src;

    for _ in 0..bytes {
        *dst = *src;
        dst = dst.add(1);
        src = src.add(1);
    }
}
