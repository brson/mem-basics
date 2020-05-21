pub unsafe fn memcpy(dst: *mut u8, src: *const u8, bytes: usize) {
    crate::memcpy_assert(dst, src, bytes);

    core::ptr::copy_nonoverlapping(src, dst, bytes);
}
