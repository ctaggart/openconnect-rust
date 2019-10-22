#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(custom_attribute)]
extern crate libc;
#[no_mangle]
#[src_loc = "1:13"]
pub static mut openconnect_version_str: *const libc::c_char =
    b"v8.05\x00" as *const u8 as *const libc::c_char;
