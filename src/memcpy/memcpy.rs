#![no_std]

/// The type signature of a `memcpy` function
pub type Memcpy = unsafe fn(dst: *mut u8, src: *const u8, bytes: usize);

/// Assert the arguments of `memcpy` are correct
fn memcpy_assert(dst: *mut u8, src: *const u8, bytes: usize) {
    let src_before_dst = (src as usize + bytes) <= dst as usize;
    let dst_before_src = (dst as usize + bytes) <= src as usize;
    let buffers_do_not_overlap = src_before_dst || dst_before_src;
    debug_assert!(buffers_do_not_overlap);

    // Buffers larger than isize::max_value are bogus.
    // See https://doc.rust-lang.org/std/primitive.pointer.html#method.offset

    let size_fits_in_signed_offset = bytes <= isize::max_value() as usize;
    debug_assert!(size_fits_in_signed_offset);
}

pub mod memcpy_loop_offset;

/// Ensure sure all implementations have the same type
#[allow(unused)]
static ALL_MEMCPYS: &[Memcpy] = &[
    memcpy_loop_offset::memcpy,
];
