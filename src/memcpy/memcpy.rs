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

    let dst_does_not_overflow = (dst as usize).checked_add(bytes).is_some();
    let src_does_not_overflow = (src as usize).checked_add(bytes).is_some();
    assert!(dst_does_not_overflow);
    assert!(src_does_not_overflow);
}

pub mod memcpy_offset;
pub mod memcpy_offset_rev;
pub mod memcpy_accumulate;
pub mod memcpy_ptrcopy;
pub mod memcpy_copyto;
pub mod memcpy_copyfrom;

/// Ensure sure all implementations have the same type
#[allow(unused)]
static ALL_MEMCPYS: &[Memcpy] = &[
    memcpy_offset::memcpy,
    memcpy_offset_rev::memcpy,
    memcpy_offset_rev::memcpy,
    memcpy_accumulate::memcpy,
    memcpy_ptrcopy::memcpy,
    memcpy_copyto::memcpy,
    memcpy_copyfrom::memcpy,
];
