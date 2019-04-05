#![feature(libc)]
#![feature(extern_types)]
#![feature(asm)]
#![feature(ptr_wrapping_offset_from)]
#![feature(label_break_value)]
#![feature(const_raw_ptr_to_usize_cast)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]
#![feature(custom_attribute)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

/// The low-level API obtained by applying c2rust to https://github.com/Cyan4973/FiniteStateEntropy/commit/510d22b221c8c02f281c35f9edeb606baacef27b
pub mod converted;
