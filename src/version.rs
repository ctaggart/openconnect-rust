use libc;
#[no_mangle]
#[src_loc = "1:13"]
pub static mut openconnect_version_str: *const libc::c_char =
    b"v8.05\x00" as *const u8 as *const libc::c_char;
