#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, custom_attribute,
           extern_types, main)]
extern crate libc;
#[header_src = "internal:0"]
pub mod internal {
    #[src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
    use super::libc;
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:18"]
pub mod _types_h {
    #[src_loc = "46:1"]
    pub type __int64_t = libc::c_longlong;
    #[src_loc = "98:1"]
    pub type __darwin_va_list = __builtin_va_list;
    use super::libc;
    use super::internal::__builtin_va_list;
    /* _BSD_I386__TYPES_H_ */
    /* time() */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:18"]
pub mod sys__types_h {
    #[src_loc = "71:1"]
    pub type __darwin_off_t = __int64_t;
    use super::_types_h::__int64_t;
    /* _SYS__TYPES_H_ */
    /* (gcc >= 3.5) */
    /* !(gcc >= 3.5) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h:18"]
pub mod _va_list_h {
    /*
 * Copyright (c) 2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    /*===---- stdarg.h - Variable argument handling ----------------------------===
 *
 * Copyright (c) 2008 Eli Friedman
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 *===-----------------------------------------------------------------------===
 */
    #[src_loc = "32:1"]
    pub type va_list = __darwin_va_list;
    use super::_types_h::__darwin_va_list;
    /* _VA_LIST_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:18"]
pub mod _stdio_h {
    /*
 * Copyright (c) 2000, 2005, 2007, 2009, 2010 Apple Inc. All rights reserved.
 *
 * @APPLE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this
 * file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_LICENSE_HEADER_END@
 */
/*-
 * Copyright (c) 1990, 1993
 *	The Regents of the University of California.  All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Chris Torek.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 *	@(#)stdio.h	8.5 (Berkeley) 4/29/95
 */
    /*
 * Common header for stdio.h and xlocale/_stdio.h
 */
    /* DO NOT REMOVE THIS COMMENT: fixincludes needs to see:
 * __gnuc_va_list and include <stdarg.h> */
    #[src_loc = "81:1"]
    pub type fpos_t = __darwin_off_t;
    /* Define for new stdio with functions. */
    /*
 * NB: to fit things in six character monocase externals, the stdio
 * code uses the prefix `__s' for stdio objects, typically followed
 * by a three-character attempt at a mnemonic.
 */
    /* stdio buffers */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "92:1"]
    pub struct __sbuf {
        pub _base: *mut libc::c_uchar,
        pub _size: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "126:9"]
    pub struct __sFILE {
        pub _p: *mut libc::c_uchar,
        pub _r: libc::c_int,
        pub _w: libc::c_int,
        pub _flags: libc::c_short,
        pub _file: libc::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: libc::c_int,
        pub _cookie: *mut libc::c_void,
        pub _close: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                               -> libc::c_int>,
        pub _read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: *mut libc::c_char,
                                               _: libc::c_int)
                              -> libc::c_int>,
        pub _seek: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: fpos_t, _: libc::c_int)
                              -> fpos_t>,
        pub _write: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *const libc::c_char,
                                                _: libc::c_int)
                               -> libc::c_int>,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: libc::c_int,
        pub _ubuf: [libc::c_uchar; 3],
        pub _nbuf: [libc::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: libc::c_int,
        pub _offset: fpos_t,
    }
    /*
 * stdio state variables.
 *
 * The following always hold:
 *
 *	if (_flags&(__SLBF|__SWR)) == (__SLBF|__SWR),
 *		_lbfsize is -_bf._size, else _lbfsize is 0
 *	if _flags&__SRD, _w is 0
 *	if _flags&__SWR, _r is 0
 *
 * This ensures that the getc and putc macros (or inline functions) never
 * try to write or read from a file that is in `read' or `write' mode.
 * (Moreover, they can, and do, automatically switch from read mode to
 * write mode, and back, on "r+" and "w+" files.)
 *
 * _lbfsize is used only to make the inline line-buffered output stream
 * code as compact as possible.
 *
 * _ub, _up, and _ur are used when ungetc() pushes back more characters
 * than fit in the current _bf, or when ungetc() pushes back a character
 * that does not match the previous one in _bf.  When this happens,
 * _ub._base becomes non-nil (i.e., a stream has ungetc() data iff
 * _ub._base!=NULL) and _up and _ur save the current values of _p and _r.
 *
 * NB: see WARNING above before changing the layout of this structure!
 */
    #[src_loc = "126:1"]
    pub type FILE = __sFILE;
    use super::sys__types_h::__darwin_off_t;
    use super::libc;
    extern "C" {
        /* current position in (some) buffer */
        /* read space left for getc() */
        /* write space left for putc() */
        /* flags, below; this FILE is free if 0 */
        /* fileno, if Unix descriptor, else -1 */
        /* the buffer (at least 1 byte, if !NULL) */
        /* 0 or -_bf._size, for inline putc */
        /* operations */
        /* cookie passed to io functions */
        /* separate buffer for long sequences of ungetc() */
        /* ungetc buffer */
        /* additions to FILE to not break ABI */
        /* saved _r when _r is counting ungetc data */
        /* tricks to meet minimum requirements even when malloc() fails */
        /* guarantee an ungetc() buffer */
        /* guarantee a getc() buffer */
        /* separate buffer for fgetln() when line crosses buffer boundary */
        /* buffer for fgetln() */
        /* Unix stdio files get aligned to block boundaries on fseek() */
        /* stat.st_blksize (may be != _bf._size) */
        /* current lseek offset (see WARNING) */
        /* hold a buncha junk that would grow the ABI */
        #[src_loc = "98:1"]
        pub type __sFILEX;
    }
    /* __STDIO_H_ */
}
#[header_src = "/Users/cameron/github/openconnect/openconnect.h:29"]
pub mod openconnect_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "221:1"]
    pub struct oc_form_opt {
        pub next: *mut oc_form_opt,
        pub type_0: libc::c_int,
        pub name: *mut libc::c_char,
        pub label: *mut libc::c_char,
        pub _value: *mut libc::c_char,
        pub flags: libc::c_uint,
        pub reserved: *mut libc::c_void,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "235:1"]
    pub struct oc_choice {
        pub name: *mut libc::c_char,
        pub label: *mut libc::c_char,
        pub auth_type: *mut libc::c_char,
        pub override_name: *mut libc::c_char,
        pub override_label: *mut libc::c_char,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "249:1"]
    pub struct oc_form_opt_select {
        pub form: oc_form_opt,
        pub nr_choices: libc::c_int,
        pub choices: *mut *mut oc_choice,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "256:1"]
    pub struct oc_auth_form {
        pub banner: *mut libc::c_char,
        pub message: *mut libc::c_char,
        pub error: *mut libc::c_char,
        pub auth_id: *mut libc::c_char,
        pub method: *mut libc::c_char,
        pub action: *mut libc::c_char,
        pub opts: *mut oc_form_opt,
        pub authgroup_opt: *mut oc_form_opt_select,
        pub authgroup_selection: libc::c_int,
    }
    #[src_loc = "610:1"]
    pub type openconnect_validate_peer_cert_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char) -> libc::c_int>;
    #[src_loc = "619:1"]
    pub type openconnect_write_new_config_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char, _: libc::c_int)
                   -> libc::c_int>;
    #[src_loc = "627:1"]
    pub type openconnect_process_auth_form_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut oc_auth_form) -> libc::c_int>;
    #[src_loc = "630:1"]
    pub type openconnect_progress_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int,
                                    _: *const libc::c_char, _: ...) -> ()>;
    use super::libc;
    extern "C" {
        #[src_loc = "340:1"]
        pub type openconnect_info;
        #[no_mangle]
        #[src_loc = "386:1"]
        pub fn openconnect_get_peer_cert_hash(vpninfo: *mut openconnect_info)
         -> *const libc::c_char;
        #[no_mangle]
        #[src_loc = "426:1"]
        pub fn openconnect_obtain_cookie(vpninfo: *mut openconnect_info)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "427:1"]
        pub fn openconnect_init_ssl() -> libc::c_int;
        #[no_mangle]
        #[src_loc = "502:1"]
        pub fn openconnect_set_system_trust(vpninfo: *mut openconnect_info,
                                            val: libc::c_uint);
        #[no_mangle]
        #[src_loc = "633:1"]
        pub fn openconnect_vpninfo_new(useragent: *const libc::c_char,
                                       _: openconnect_validate_peer_cert_vfn,
                                       _: openconnect_write_new_config_vfn,
                                       _: openconnect_process_auth_form_vfn,
                                       _: openconnect_progress_vfn,
                                       privdata: *mut libc::c_void)
         -> *mut openconnect_info;
        #[no_mangle]
        #[src_loc = "541:1"]
        pub fn openconnect_parse_url(vpninfo: *mut openconnect_info,
                                     url: *const libc::c_char) -> libc::c_int;
    }
    /* __OPENCONNECT_H__ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:18"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    use super::libc;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[src_loc = "69:14"]
        pub static mut __stderrp: *mut FILE;
        /* (DARWIN_UNLIMITED_STREAMS || _DARWIN_C_SOURCE) */
        #[no_mangle]
        #[src_loc = "155:6"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "170:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "190:6"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
    }
    /* _STDIO_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:19"]
pub mod stdlib_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "145:7"]
        pub fn exit(_: libc::c_int) -> !;
    }
    /* _STDLIB_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* Poison the following routines if -fshort-wchar is set */
    /* !_ANSI_SOURCE && !_POSIX_SOURCE */
    /* valloc is now declared in _malloc.h */
    /* getsubopt(3) external variable */
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::_types_h::{__int64_t, __darwin_va_list};
pub use self::sys__types_h::__darwin_off_t;
pub use self::_va_list_h::va_list;
pub use self::_stdio_h::{fpos_t, __sbuf, __sFILE, FILE, __sFILEX};
pub use self::openconnect_h::{oc_form_opt, oc_choice, oc_form_opt_select,
                              oc_auth_form,
                              openconnect_validate_peer_cert_vfn,
                              openconnect_write_new_config_vfn,
                              openconnect_process_auth_form_vfn,
                              openconnect_progress_vfn, openconnect_info,
                              openconnect_get_peer_cert_hash,
                              openconnect_obtain_cookie, openconnect_init_ssl,
                              openconnect_set_system_trust,
                              openconnect_vpninfo_new, openconnect_parse_url};
use self::stdio_h::{__stderrp, fprintf, printf, vfprintf};
use self::stdlib_h::exit;
/*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2008-2016 Intel Corporation.
 *
 * Author: David Woodhouse <dwmw2@infradead.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * version 2.1, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 */
/* Normally it's nice for header files to automatically include anything
 * they need. But winsock is a horrid can of worms; we 're not going to
 * make openconnect.h include anything for itself. So just do this... */
#[src_loc = "31:1"]
unsafe extern "C" fn progress(mut privdata: *mut libc::c_void,
                              mut level: libc::c_int,
                              mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    if level > 0i32 { return }
    args_0 = args.clone();
    vfprintf(__stderrp, fmt, args_0.as_va_list());
}
#[src_loc = "43:1"]
unsafe extern "C" fn validate_peer_cert(mut _vpninfo: *mut libc::c_void,
                                        mut reason: *const libc::c_char)
 -> libc::c_int {
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           openconnect_get_peer_cert_hash(_vpninfo as *mut openconnect_info));
    exit(0i32);
}
/* We do this in a separate test tool because we *really* don't want
 * people scripting it to recover the --no-cert-check functionality.
 * Validate your server certs properly, people! */
#[src_loc = "52:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut vpninfo: *mut openconnect_info = 0 as *mut openconnect_info;
    if argc != 2i32 {
        fprintf(__stderrp,
                b"usage: serverhash <server>\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(1i32);
    }
    openconnect_init_ssl();
    vpninfo =
        openconnect_vpninfo_new(0 as *const libc::c_char,
                                Some(validate_peer_cert as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_void,
                                                              _:
                                                                  *const libc::c_char)
                                             -> libc::c_int), None, None,
                                Some(progress as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_void,
                                                              _: libc::c_int,
                                                              _:
                                                                  *const libc::c_char,
                                                              _: ...) -> ()),
                                0 as *mut libc::c_void);
    if openconnect_parse_url(vpninfo, *argv.offset(1)) != 0 {
        fprintf(__stderrp,
                b"Failed to parse URL\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(1i32);
    }
    openconnect_set_system_trust(vpninfo, 0i32 as libc::c_uint);
    openconnect_obtain_cookie(vpninfo);
    return -1i32;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
