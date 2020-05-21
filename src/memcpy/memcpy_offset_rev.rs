pub unsafe fn memcpy(dst: *mut u8, src: *const u8, bytes: usize) {
    crate::memcpy_assert(dst, src, bytes);

    for i in (0..bytes).rev() {
        let dst_byte = dst.add(i);
        let src_byte = src.add(i);
        *dst_byte = *src_byte;
    }
}
