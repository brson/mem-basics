#![no_std]
#![allow(unused)]

use cfg_if::cfg_if;

pub mod memcpy_trivial;

pub type Memcpy = unsafe fn(dst: *mut u8, src: *mut u8, bytes: usize);

pub static ALL_MEMCPYS: &[Memcpy] = &[
    memcpy_trivial::memcpy,
];
