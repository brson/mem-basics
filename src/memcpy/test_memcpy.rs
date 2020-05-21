use proptest::prelude::*;
use std::iter;

use memcpy::memcpy_accumulate::memcpy as memcpy_impl;

proptest! {
    #[test]
    fn memcpy(src: Vec<u8>) {
        let mut dst: Vec<_> = iter::repeat(0).take(src.len()).collect();
        let src_ptr = src.as_ptr();
        let dst_ptr = dst.as_mut_ptr();
        let bytes = src.len();
        unsafe {
            memcpy_impl(dst_ptr, src_ptr, bytes);
        }
        assert_eq!(src, dst);
    }
}
