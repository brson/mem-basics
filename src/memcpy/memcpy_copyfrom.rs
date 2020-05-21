pub unsafe fn memcpy(dst: *mut u8, src: *const u8, bytes: usize) {
    crate::memcpy_assert(dst, src, bytes);

    dst.copy_from(src, bytes);
}
