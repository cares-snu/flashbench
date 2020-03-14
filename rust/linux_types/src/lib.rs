#![no_std]
#![cfg(target_arch = "x86_64")]
#![allow(non_camel_case_types)]

pub type c_void = core::ffi::c_void;
pub type c_char = i8;
pub type c_uchar = u8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;
