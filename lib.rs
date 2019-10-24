#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(custom_attribute)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]




pub mod src {
pub mod auth;
pub mod auth_common;
pub mod auth_globalprotect;
pub mod auth_juniper;
pub mod compat;
pub mod cstp;
pub mod digest;
pub mod dtls;
pub mod esp;
pub mod esp_seqno;
pub mod gpst;
pub mod gssapi;
pub mod http;
pub mod http_auth;
pub mod iconv;
pub mod library;
pub mod lzo;
pub mod lzs;
pub mod main;
pub mod mainloop;
pub mod ntlm;
pub mod oath;
pub mod oncp;
pub mod openssl;
pub mod openssl_dtls;
pub mod openssl_esp;
pub mod openssl_pkcs11;
pub mod pulse;
pub mod script;
pub mod ssl;
pub mod stoken;
pub mod tests {
pub mod bad_dtls_test;
pub mod lzstest;
pub mod seqtest;
pub mod serverhash;
} // mod tests
pub mod tun;
pub mod version;
pub mod xml;
pub mod yubikey;
} // mod src

