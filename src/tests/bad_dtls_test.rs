#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main)]
extern crate libc;
extern "C" {
    pub type __sFILEX;
    pub type bignum_ctx;
    pub type bn_blinding_st;
    pub type engine_st;
    pub type ASN1_VALUE_st;
    pub type evp_pkey_ctx_st;
    pub type ec_key_st;
    pub type evp_pkey_asn1_method_st;
    pub type NAME_CONSTRAINTS_st;
    pub type stack_st_GENERAL_NAME;
    pub type stack_st_DIST_POINT;
    pub type X509_POLICY_CACHE_st;
    pub type AUTHORITY_KEYID_st;
    pub type x509_crl_method_st;
    pub type stack_st_GENERAL_NAMES;
    pub type ISSUING_DIST_POINT_st;
    pub type X509_POLICY_TREE_st;
    pub type X509_VERIFY_PARAM_ID_st;
    pub type ssl3_buf_freelist_st;
    pub type cert_st;
    pub type sess_cert_st;
    pub type ssl3_enc_method;
    pub type stack_st_OCSP_RESPID;
    pub type _pqueue;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /*
 * Copyright (c) 2018 Apple Computer, Inc. All rights reserved.
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
    /*
 * This header is included from <stdlib.h>, so the contents of this file have
 * broad source compatibility and POSIX conformance implications.
 * Be cautious about what is included and declared here.
 */
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn BIO_s_mem() -> *mut BIO_METHOD;
    #[no_mangle]
    fn BIO_ctrl(bp: *mut BIO, cmd: libc::c_int, larg: libc::c_long,
                parg: *mut libc::c_void) -> libc::c_long;
    #[no_mangle]
    fn BIO_write(b: *mut BIO, data: *const libc::c_void, len: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn BIO_new(type_0: *mut BIO_METHOD) -> *mut BIO;
    #[no_mangle]
    fn CRYPTO_memcmp(a: *const libc::c_void, b: *const libc::c_void,
                     len: size_t) -> libc::c_int;
    #[no_mangle]
    fn CRYPTO_malloc(num: libc::c_int, file: *const libc::c_char,
                     line: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn CRYPTO_free(ptr: *mut libc::c_void);
    #[no_mangle]
    fn EVP_MD_size(md: *const EVP_MD) -> libc::c_int;
    #[no_mangle]
    fn EVP_MD_CTX_md(ctx: *const EVP_MD_CTX) -> *const EVP_MD;
    #[no_mangle]
    fn EVP_Cipher(c: *mut EVP_CIPHER_CTX, out: *mut libc::c_uchar,
                  in_0: *const libc::c_uchar, inl: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn EVP_MD_CTX_create() -> *mut EVP_MD_CTX;
    #[no_mangle]
    fn EVP_MD_CTX_destroy(ctx: *mut EVP_MD_CTX);
    #[no_mangle]
    fn EVP_DigestInit_ex(ctx: *mut EVP_MD_CTX, type_0: *const EVP_MD,
                         impl_0: *mut ENGINE) -> libc::c_int;
    #[no_mangle]
    fn EVP_DigestUpdate(ctx: *mut EVP_MD_CTX, d: *const libc::c_void,
                        cnt: size_t) -> libc::c_int;
    #[no_mangle]
    fn EVP_DigestFinal_ex(ctx: *mut EVP_MD_CTX, md: *mut libc::c_uchar,
                          s: *mut libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn EVP_CipherInit_ex(ctx: *mut EVP_CIPHER_CTX, cipher: *const EVP_CIPHER,
                         impl_0: *mut ENGINE, key: *const libc::c_uchar,
                         iv: *const libc::c_uchar, enc: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
    #[no_mangle]
    fn EVP_CIPHER_CTX_free(a: *mut EVP_CIPHER_CTX);
    #[no_mangle]
    fn EVP_md5() -> *const EVP_MD;
    #[no_mangle]
    fn EVP_sha1() -> *const EVP_MD;
    #[no_mangle]
    fn EVP_aes_128_cbc() -> *const EVP_CIPHER;
    #[no_mangle]
    fn EVP_cleanup();
    #[no_mangle]
    fn time(_: *mut time_t) -> time_t;
    #[no_mangle]
    fn SSL_library_init() -> libc::c_int;
    #[no_mangle]
    fn SSL_set_connect_state(s: *mut SSL);
    #[no_mangle]
    fn SSL_do_handshake(s: *mut SSL) -> libc::c_int;
    #[no_mangle]
    fn DTLSv1_client_method() -> *const SSL_METHOD;
    #[no_mangle]
    fn SSL_get_error(s: *const SSL, ret_code: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SSL_CTX_ctrl(ctx: *mut SSL_CTX, cmd: libc::c_int, larg: libc::c_long,
                    parg: *mut libc::c_void) -> libc::c_long;
    #[no_mangle]
    fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void, num: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SSL_free(ssl: *mut SSL);
    #[no_mangle]
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    #[no_mangle]
    fn d2i_SSL_SESSION(a: *mut *mut SSL_SESSION,
                       pp: *mut *const libc::c_uchar, length: libc::c_long)
     -> *mut SSL_SESSION;
    #[no_mangle]
    fn SSL_set_session(to: *mut SSL, session: *mut SSL_SESSION)
     -> libc::c_int;
    #[no_mangle]
    fn SSL_SESSION_free(ses: *mut SSL_SESSION);
    #[no_mangle]
    fn SSL_load_error_strings();
    #[no_mangle]
    fn SSL_set_bio(s: *mut SSL, rbio: *mut BIO, wbio: *mut BIO);
    #[no_mangle]
    fn SSL_CTX_free(_: *mut SSL_CTX);
    #[no_mangle]
    fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
    #[no_mangle]
    fn SSL_CTX_set_cipher_list(_: *mut SSL_CTX, str: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn HMAC_CTX_init(ctx: *mut HMAC_CTX);
    #[no_mangle]
    fn HMAC_CTX_cleanup(ctx: *mut HMAC_CTX);
    #[no_mangle]
    fn HMAC_Init_ex(ctx: *mut HMAC_CTX, key: *const libc::c_void,
                    len: libc::c_int, md: *const EVP_MD, impl_0: *mut ENGINE)
     -> libc::c_int;
    #[no_mangle]
    fn HMAC_Update(ctx: *mut HMAC_CTX, data: *const libc::c_uchar,
                   len: size_t) -> libc::c_int;
    #[no_mangle]
    fn HMAC_Final(ctx: *mut HMAC_CTX, md: *mut libc::c_uchar,
                  len: *mut libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn HMAC(evp_md: *const EVP_MD, key: *const libc::c_void,
            key_len: libc::c_int, d: *const libc::c_uchar, n: size_t,
            md: *mut libc::c_uchar, md_len: *mut libc::c_uint)
     -> *mut libc::c_uchar;
    #[no_mangle]
    fn ERR_print_errors_fp(fp: *mut FILE);
    #[no_mangle]
    fn ERR_free_strings();
    #[no_mangle]
    fn RAND_bytes(buf: *mut libc::c_uchar, num: libc::c_int) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
/* !__GNUC__ */
/* !__GNUC__ */
/*
 * The rune type below is declared to be an ``int'' instead of the more natural
 * ``unsigned long'' or ``long''.  Two things are happening here.  It is not
 * unsigned so that EOF (-1) can be naturally assigned to it and used.  Also,
 * it looks like 10646 will be a 31 bit standard.  This means that if your
 * ints cannot hold 32 bits, you will be in trouble.  The reason an int was
 * chosen over a long is that the is*() and to*() routines take ints (says
 * ANSI C), but they use __darwin_ct_rune_t instead of int.  By changing it
 * here, you lose a bit of ANSI conformance, but your programs will still
 * work.
 *
 * NOTE: rune_t is not covered by ANSI nor other standards, and should not
 * be instantiated outside of lib/libc/locale.  Use wchar_t.  wchar_t and
 * rune_t must be the same type.  Also wint_t must be no narrower than
 * wchar_t, and should also be able to hold all members of the largest
 * character set plus one extra value (WEOF). wint_t must be at least 16 bits.
 */
/* ct_rune_t */
/*
 * mbstate_t is an opaque object to keep conversion state, during multibyte
 * stream conversions.  The content must not be referenced by user programs.
 */
/* for alignment */
/* mbstate_t */
/* ptr1 - ptr2 */
/* __GNUC__ */
/* sizeof() */
/* va_list */
/* wchar_t */
/* rune_t */
/* wint_t */
/* clock() */
/* socklen_t (duh) */
/* byte count or error */
pub type __darwin_time_t = libc::c_long;
/* [???] Some file attributes */
pub type __darwin_off_t = __int64_t;
/* [???] signal set */
pub type __darwin_suseconds_t = __int32_t;
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
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
/* __darwin_size_t */
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
                                           _: libc::c_int) -> libc::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: fpos_t,
                                           _: libc::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *const libc::c_char,
                                            _: libc::c_int) -> libc::c_int>,
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
/* Define for new stdio with functions. */
/*
 * NB: to fit things in six character monocase externals, the stdio
 * code uses the prefix `__s' for stdio objects, typically followed
 * by a three-character attempt at a mnemonic.
 */
/* stdio buffers */
/* hold a buncha junk that would grow the ABI */
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
pub type FILE = __sFILE;
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
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
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
/* __darwin_time_t */
/* __darwin_suseconds_t */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
    /* and microseconds */
}
/* crypto/stack/stack.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st {
    pub num: libc::c_int,
    pub data: *mut *mut libc::c_char,
    pub sorted: libc::c_int,
    pub num_alloc: libc::c_int,
    pub comp: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                          _: *const libc::c_void)
                         -> libc::c_int>,
}
pub type _STACK = stack_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct asn1_string_st {
    pub length: libc::c_int,
    pub type_0: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub flags: libc::c_long,
    /* If placed in pkcs12.h, we end up with a circular depency with pkcs7.h */
    /* Nothing */
    /* Nothing */
    /* Callback types for crypto.h */
    /* def HEADER_OPENSSL_TYPES_H */
}
/* ====================================================================
 * Copyright (c) 1998-2001 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.openssl.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@openssl.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.openssl.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
pub type ASN1_INTEGER = asn1_string_st;
pub type ASN1_ENUMERATED = asn1_string_st;
pub type ASN1_BIT_STRING = asn1_string_st;
pub type ASN1_OCTET_STRING = asn1_string_st;
pub type ASN1_PRINTABLESTRING = asn1_string_st;
pub type ASN1_T61STRING = asn1_string_st;
pub type ASN1_IA5STRING = asn1_string_st;
pub type ASN1_GENERALSTRING = asn1_string_st;
pub type ASN1_UNIVERSALSTRING = asn1_string_st;
pub type ASN1_BMPSTRING = asn1_string_st;
pub type ASN1_UTCTIME = asn1_string_st;
pub type ASN1_TIME = asn1_string_st;
pub type ASN1_GENERALIZEDTIME = asn1_string_st;
pub type ASN1_VISIBLESTRING = asn1_string_st;
pub type ASN1_UTF8STRING = asn1_string_st;
pub type ASN1_STRING = asn1_string_st;
pub type ASN1_BOOLEAN = libc::c_int;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct asn1_object_st {
    pub sn: *const libc::c_char,
    pub ln: *const libc::c_char,
    pub nid: libc::c_int,
    pub length: libc::c_int,
    pub data: *const libc::c_uchar,
    pub flags: libc::c_int,
}
pub type ASN1_OBJECT = asn1_object_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bignum_st {
    pub d: *mut libc::c_ulong,
    pub top: libc::c_int,
    pub dmax: libc::c_int,
    pub neg: libc::c_int,
    pub flags: libc::c_int,
}
pub type BIGNUM = bignum_st;
pub type BN_CTX = bignum_ctx;
pub type BN_BLINDING = bn_blinding_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bn_mont_ctx_st {
    pub ri: libc::c_int,
    pub RR: BIGNUM,
    pub N: BIGNUM,
    pub Ni: BIGNUM,
    pub n0: [libc::c_ulong; 2],
    pub flags: libc::c_int,
}
pub type BN_MONT_CTX = bn_mont_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bn_gencb_st {
    pub ver: libc::c_uint,
    pub arg: *mut libc::c_void,
    pub cb: C2RustUnnamed,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub cb_1: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                          _: *mut libc::c_void) -> ()>,
    pub cb_2: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                          _: *mut BN_GENCB) -> libc::c_int>,
}
pub type BN_GENCB = bn_gencb_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct buf_mem_st {
    pub length: size_t,
    pub data: *mut libc::c_char,
    pub max: size_t,
}
pub type BUF_MEM = buf_mem_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct evp_cipher_st {
    pub nid: libc::c_int,
    pub block_size: libc::c_int,
    pub key_len: libc::c_int,
    pub iv_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                          _: *const libc::c_uchar,
                                          _: *const libc::c_uchar,
                                          _: libc::c_int) -> libc::c_int>,
    pub do_cipher: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                               _: *mut libc::c_uchar,
                                               _: *const libc::c_uchar,
                                               _: size_t) -> libc::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX)
                            -> libc::c_int>,
    pub ctx_size: libc::c_int,
    pub set_asn1_parameters: Option<unsafe extern "C" fn(_:
                                                             *mut EVP_CIPHER_CTX,
                                                         _: *mut ASN1_TYPE)
                                        -> libc::c_int>,
    pub get_asn1_parameters: Option<unsafe extern "C" fn(_:
                                                             *mut EVP_CIPHER_CTX,
                                                         _: *mut ASN1_TYPE)
                                        -> libc::c_int>,
    pub ctrl: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                          _: libc::c_int, _: libc::c_int,
                                          _: *mut libc::c_void)
                         -> libc::c_int>,
    pub app_data: *mut libc::c_void,
}
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct evp_cipher_ctx_st {
    pub cipher: *const EVP_CIPHER,
    pub engine: *mut ENGINE,
    pub encrypt: libc::c_int,
    pub buf_len: libc::c_int,
    pub oiv: [libc::c_uchar; 16],
    pub iv: [libc::c_uchar; 16],
    pub buf: [libc::c_uchar; 32],
    pub num: libc::c_int,
    pub app_data: *mut libc::c_void,
    pub key_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub cipher_data: *mut libc::c_void,
    pub final_used: libc::c_int,
    pub block_mask: libc::c_int,
    pub final_0: [libc::c_uchar; 32],
}
pub type ENGINE = engine_st;
pub type EVP_CIPHER = evp_cipher_st;
pub type ASN1_TYPE = asn1_type_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct asn1_type_st {
    pub type_0: libc::c_int,
    pub value: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_0 {
    pub ptr: *mut libc::c_char,
    pub boolean: ASN1_BOOLEAN,
    pub asn1_string: *mut ASN1_STRING,
    pub object: *mut ASN1_OBJECT,
    pub integer: *mut ASN1_INTEGER,
    pub enumerated: *mut ASN1_ENUMERATED,
    pub bit_string: *mut ASN1_BIT_STRING,
    pub octet_string: *mut ASN1_OCTET_STRING,
    pub printablestring: *mut ASN1_PRINTABLESTRING,
    pub t61string: *mut ASN1_T61STRING,
    pub ia5string: *mut ASN1_IA5STRING,
    pub generalstring: *mut ASN1_GENERALSTRING,
    pub bmpstring: *mut ASN1_BMPSTRING,
    pub universalstring: *mut ASN1_UNIVERSALSTRING,
    pub utctime: *mut ASN1_UTCTIME,
    pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
    pub visiblestring: *mut ASN1_VISIBLESTRING,
    pub utf8string: *mut ASN1_UTF8STRING,
    pub set: *mut ASN1_STRING,
    pub sequence: *mut ASN1_STRING,
    pub asn1_value: *mut ASN1_VALUE,
}
pub type ASN1_VALUE = ASN1_VALUE_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct env_md_st {
    pub type_0: libc::c_int,
    pub pkey_type: libc::c_int,
    pub md_size: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX) -> libc::c_int>,
    pub update: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                            _: *const libc::c_void, _: size_t)
                           -> libc::c_int>,
    pub final_0: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                             _: *mut libc::c_uchar)
                            -> libc::c_int>,
    pub copy: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                          _: *const EVP_MD_CTX)
                         -> libc::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX)
                            -> libc::c_int>,
    pub sign: Option<unsafe extern "C" fn(_: libc::c_int,
                                          _: *const libc::c_uchar,
                                          _: libc::c_uint,
                                          _: *mut libc::c_uchar,
                                          _: *mut libc::c_uint,
                                          _: *mut libc::c_void)
                         -> libc::c_int>,
    pub verify: Option<unsafe extern "C" fn(_: libc::c_int,
                                            _: *const libc::c_uchar,
                                            _: libc::c_uint,
                                            _: *const libc::c_uchar,
                                            _: libc::c_uint,
                                            _: *mut libc::c_void)
                           -> libc::c_int>,
    pub required_pkey_type: [libc::c_int; 5],
    pub block_size: libc::c_int,
    pub ctx_size: libc::c_int,
    pub md_ctrl: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                             _: libc::c_int, _: libc::c_int,
                                             _: *mut libc::c_void)
                            -> libc::c_int>,
}
pub type EVP_MD_CTX = env_md_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct env_md_ctx_st {
    pub digest: *const EVP_MD,
    pub engine: *mut ENGINE,
    pub flags: libc::c_ulong,
    pub md_data: *mut libc::c_void,
    pub pctx: *mut EVP_PKEY_CTX,
    pub update: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                            _: *const libc::c_void, _: size_t)
                           -> libc::c_int>,
}
pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
pub type EVP_MD = env_md_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct evp_pkey_st {
    pub type_0: libc::c_int,
    pub save_type: libc::c_int,
    pub references: libc::c_int,
    pub ameth: *const EVP_PKEY_ASN1_METHOD,
    pub engine: *mut ENGINE,
    pub pkey: C2RustUnnamed_1,
    pub save_parameters: libc::c_int,
    pub attributes: *mut stack_st_X509_ATTRIBUTE,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_ATTRIBUTE {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_1 {
    pub ptr: *mut libc::c_char,
    pub rsa: *mut rsa_st,
    pub dsa: *mut dsa_st,
    pub dh: *mut dh_st,
    pub ec: *mut ec_key_st,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dh_st {
    pub pad: libc::c_int,
    pub version: libc::c_int,
    pub p: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub length: libc::c_long,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub q: *mut BIGNUM,
    pub j: *mut BIGNUM,
    pub seed: *mut libc::c_uchar,
    pub seedlen: libc::c_int,
    pub counter: *mut BIGNUM,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DH_METHOD,
    pub engine: *mut ENGINE,
}
pub type DH_METHOD = dh_method;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dh_method {
    pub name: *const libc::c_char,
    pub generate_key: Option<unsafe extern "C" fn(_: *mut DH) -> libc::c_int>,
    pub compute_key: Option<unsafe extern "C" fn(_: *mut libc::c_uchar,
                                                 _: *const BIGNUM, _: *mut DH)
                                -> libc::c_int>,
    pub bn_mod_exp: Option<unsafe extern "C" fn(_: *const DH, _: *mut BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *mut BN_CTX,
                                                _: *mut BN_MONT_CTX)
                               -> libc::c_int>,
    pub init: Option<unsafe extern "C" fn(_: *mut DH) -> libc::c_int>,
    pub finish: Option<unsafe extern "C" fn(_: *mut DH) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub generate_params: Option<unsafe extern "C" fn(_: *mut DH,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut BN_GENCB)
                                    -> libc::c_int>,
}
pub type DH = dh_st;
pub type CRYPTO_EX_DATA = crypto_ex_data_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct crypto_ex_data_st {
    pub sk: *mut stack_st_void,
    pub dummy: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_void {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub write_params: libc::c_int,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub kinv: *mut BIGNUM,
    pub r: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DSA_METHOD,
    pub engine: *mut ENGINE,
}
pub type DSA_METHOD = dsa_method;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dsa_method {
    pub name: *const libc::c_char,
    pub dsa_do_sign: Option<unsafe extern "C" fn(_: *const libc::c_uchar,
                                                 _: libc::c_int, _: *mut DSA)
                                -> *mut DSA_SIG>,
    pub dsa_sign_setup: Option<unsafe extern "C" fn(_: *mut DSA,
                                                    _: *mut BN_CTX,
                                                    _: *mut *mut BIGNUM,
                                                    _: *mut *mut BIGNUM)
                                   -> libc::c_int>,
    pub dsa_do_verify: Option<unsafe extern "C" fn(_: *const libc::c_uchar,
                                                   _: libc::c_int,
                                                   _: *mut DSA_SIG,
                                                   _: *mut DSA)
                                  -> libc::c_int>,
    pub dsa_mod_exp: Option<unsafe extern "C" fn(_: *mut DSA, _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BN_CTX,
                                                 _: *mut BN_MONT_CTX)
                                -> libc::c_int>,
    pub bn_mod_exp: Option<unsafe extern "C" fn(_: *mut DSA, _: *mut BIGNUM,
                                                _: *mut BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *mut BN_CTX,
                                                _: *mut BN_MONT_CTX)
                               -> libc::c_int>,
    pub init: Option<unsafe extern "C" fn(_: *mut DSA) -> libc::c_int>,
    pub finish: Option<unsafe extern "C" fn(_: *mut DSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub dsa_paramgen: Option<unsafe extern "C" fn(_: *mut DSA, _: libc::c_int,
                                                  _: *const libc::c_uchar,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_int,
                                                  _: *mut libc::c_ulong,
                                                  _: *mut BN_GENCB)
                                 -> libc::c_int>,
    pub dsa_keygen: Option<unsafe extern "C" fn(_: *mut DSA) -> libc::c_int>,
}
pub type DSA = dsa_st;
pub type DSA_SIG = DSA_SIG_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct DSA_SIG_st {
    pub r: *mut BIGNUM,
    pub s: *mut BIGNUM,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub meth: *const RSA_METHOD,
    pub engine: *mut ENGINE,
    pub n: *mut BIGNUM,
    pub e: *mut BIGNUM,
    pub d: *mut BIGNUM,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub dmp1: *mut BIGNUM,
    pub dmq1: *mut BIGNUM,
    pub iqmp: *mut BIGNUM,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub _method_mod_n: *mut BN_MONT_CTX,
    pub _method_mod_p: *mut BN_MONT_CTX,
    pub _method_mod_q: *mut BN_MONT_CTX,
    pub bignum_data: *mut libc::c_char,
    pub blinding: *mut BN_BLINDING,
    pub mt_blinding: *mut BN_BLINDING,
}
pub type RSA_METHOD = rsa_meth_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rsa_meth_st {
    pub name: *const libc::c_char,
    pub rsa_pub_enc: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_uchar,
                                                 _: *mut libc::c_uchar,
                                                 _: *mut RSA, _: libc::c_int)
                                -> libc::c_int>,
    pub rsa_pub_dec: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_uchar,
                                                 _: *mut libc::c_uchar,
                                                 _: *mut RSA, _: libc::c_int)
                                -> libc::c_int>,
    pub rsa_priv_enc: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: *mut RSA, _: libc::c_int)
                                 -> libc::c_int>,
    pub rsa_priv_dec: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: *mut RSA, _: libc::c_int)
                                 -> libc::c_int>,
    pub rsa_mod_exp: Option<unsafe extern "C" fn(_: *mut BIGNUM,
                                                 _: *const BIGNUM,
                                                 _: *mut RSA, _: *mut BN_CTX)
                                -> libc::c_int>,
    pub bn_mod_exp: Option<unsafe extern "C" fn(_: *mut BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *mut BN_CTX,
                                                _: *mut BN_MONT_CTX)
                               -> libc::c_int>,
    pub init: Option<unsafe extern "C" fn(_: *mut RSA) -> libc::c_int>,
    pub finish: Option<unsafe extern "C" fn(_: *mut RSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub rsa_sign: Option<unsafe extern "C" fn(_: libc::c_int,
                                              _: *const libc::c_uchar,
                                              _: libc::c_uint,
                                              _: *mut libc::c_uchar,
                                              _: *mut libc::c_uint,
                                              _: *const RSA) -> libc::c_int>,
    pub rsa_verify: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: *const libc::c_uchar,
                                                _: libc::c_uint,
                                                _: *const libc::c_uchar,
                                                _: libc::c_uint,
                                                _: *const RSA)
                               -> libc::c_int>,
    pub rsa_keygen: Option<unsafe extern "C" fn(_: *mut RSA, _: libc::c_int,
                                                _: *mut BIGNUM,
                                                _: *mut BN_GENCB)
                               -> libc::c_int>,
}
pub type RSA = rsa_st;
pub type EVP_PKEY_ASN1_METHOD = evp_pkey_asn1_method_st;
pub type EVP_PKEY = evp_pkey_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_st {
    pub cert_info: *mut X509_CINF,
    pub sig_alg: *mut X509_ALGOR,
    pub signature: *mut ASN1_BIT_STRING,
    pub valid: libc::c_int,
    pub references: libc::c_int,
    pub name: *mut libc::c_char,
    pub ex_data: CRYPTO_EX_DATA,
    pub ex_pathlen: libc::c_long,
    pub ex_pcpathlen: libc::c_long,
    pub ex_flags: libc::c_ulong,
    pub ex_kusage: libc::c_ulong,
    pub ex_xkusage: libc::c_ulong,
    pub ex_nscert: libc::c_ulong,
    pub skid: *mut ASN1_OCTET_STRING,
    pub akid: *mut AUTHORITY_KEYID,
    pub policy_cache: *mut X509_POLICY_CACHE,
    pub crldp: *mut stack_st_DIST_POINT,
    pub altname: *mut stack_st_GENERAL_NAME,
    pub nc: *mut NAME_CONSTRAINTS,
    pub sha1_hash: [libc::c_uchar; 20],
    pub aux: *mut X509_CERT_AUX,
}
pub type X509_CERT_AUX = x509_cert_aux_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_cert_aux_st {
    pub trust: *mut stack_st_ASN1_OBJECT,
    pub reject: *mut stack_st_ASN1_OBJECT,
    pub alias: *mut ASN1_UTF8STRING,
    pub keyid: *mut ASN1_OCTET_STRING,
    pub other: *mut stack_st_X509_ALGOR,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_ALGOR {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_ASN1_OBJECT {
    pub stack: _STACK,
}
pub type NAME_CONSTRAINTS = NAME_CONSTRAINTS_st;
pub type X509_POLICY_CACHE = X509_POLICY_CACHE_st;
pub type AUTHORITY_KEYID = AUTHORITY_KEYID_st;
pub type X509_ALGOR = X509_algor_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_algor_st {
    pub algorithm: *mut ASN1_OBJECT,
    pub parameter: *mut ASN1_TYPE,
}
pub type X509_CINF = x509_cinf_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_cinf_st {
    pub version: *mut ASN1_INTEGER,
    pub serialNumber: *mut ASN1_INTEGER,
    pub signature: *mut X509_ALGOR,
    pub issuer: *mut X509_NAME,
    pub validity: *mut X509_VAL,
    pub subject: *mut X509_NAME,
    pub key: *mut X509_PUBKEY,
    pub issuerUID: *mut ASN1_BIT_STRING,
    pub subjectUID: *mut ASN1_BIT_STRING,
    pub extensions: *mut stack_st_X509_EXTENSION,
    pub enc: ASN1_ENCODING,
}
pub type ASN1_ENCODING = ASN1_ENCODING_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ASN1_ENCODING_st {
    pub enc: *mut libc::c_uchar,
    pub len: libc::c_long,
    pub modified: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_EXTENSION {
    pub stack: _STACK,
}
pub type X509_PUBKEY = X509_pubkey_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_pubkey_st {
    pub algor: *mut X509_ALGOR,
    pub public_key: *mut ASN1_BIT_STRING,
    pub pkey: *mut EVP_PKEY,
}
pub type X509_NAME = X509_name_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_name_st {
    pub entries: *mut stack_st_X509_NAME_ENTRY,
    pub modified: libc::c_int,
    pub bytes: *mut BUF_MEM,
    pub canon_enc: *mut libc::c_uchar,
    pub canon_enclen: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_NAME_ENTRY {
    pub stack: _STACK,
}
pub type X509_VAL = X509_val_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_val_st {
    pub notBefore: *mut ASN1_TIME,
    pub notAfter: *mut ASN1_TIME,
}
pub type X509 = x509_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_crl_st {
    pub crl: *mut X509_CRL_INFO,
    pub sig_alg: *mut X509_ALGOR,
    pub signature: *mut ASN1_BIT_STRING,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub akid: *mut AUTHORITY_KEYID,
    pub idp: *mut ISSUING_DIST_POINT,
    pub idp_flags: libc::c_int,
    pub idp_reasons: libc::c_int,
    pub crl_number: *mut ASN1_INTEGER,
    pub base_crl_number: *mut ASN1_INTEGER,
    pub sha1_hash: [libc::c_uchar; 20],
    pub issuers: *mut stack_st_GENERAL_NAMES,
    pub meth: *const X509_CRL_METHOD,
    pub meth_data: *mut libc::c_void,
}
pub type X509_CRL_METHOD = x509_crl_method_st;
pub type ISSUING_DIST_POINT = ISSUING_DIST_POINT_st;
pub type X509_CRL_INFO = X509_crl_info_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_crl_info_st {
    pub version: *mut ASN1_INTEGER,
    pub sig_alg: *mut X509_ALGOR,
    pub issuer: *mut X509_NAME,
    pub lastUpdate: *mut ASN1_TIME,
    pub nextUpdate: *mut ASN1_TIME,
    pub revoked: *mut stack_st_X509_REVOKED,
    pub extensions: *mut stack_st_X509_EXTENSION,
    pub enc: ASN1_ENCODING,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_REVOKED {
    pub stack: _STACK,
}
pub type X509_CRL = X509_crl_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_store_st {
    pub cache: libc::c_int,
    pub objs: *mut stack_st_X509_OBJECT,
    pub get_cert_methods: *mut stack_st_X509_LOOKUP,
    pub param: *mut X509_VERIFY_PARAM,
    pub verify: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                           -> libc::c_int>,
    pub verify_cb: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut X509_STORE_CTX)
                              -> libc::c_int>,
    pub get_issuer: Option<unsafe extern "C" fn(_: *mut *mut X509,
                                                _: *mut X509_STORE_CTX,
                                                _: *mut X509) -> libc::c_int>,
    pub check_issued: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509, _: *mut X509)
                                 -> libc::c_int>,
    pub check_revocation: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                                     -> libc::c_int>,
    pub get_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                             _: *mut *mut X509_CRL,
                                             _: *mut X509) -> libc::c_int>,
    pub check_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                               _: *mut X509_CRL)
                              -> libc::c_int>,
    pub cert_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                              _: *mut X509_CRL, _: *mut X509)
                             -> libc::c_int>,
    pub lookup_certs: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509_NAME)
                                 -> *mut stack_st_X509>,
    pub lookup_crls: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                 _: *mut X509_NAME)
                                -> *mut stack_st_X509_CRL>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                            -> libc::c_int>,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
}
pub type X509_STORE_CTX = x509_store_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_store_ctx_st {
    pub ctx: *mut X509_STORE,
    pub current_method: libc::c_int,
    pub cert: *mut X509,
    pub untrusted: *mut stack_st_X509,
    pub crls: *mut stack_st_X509_CRL,
    pub param: *mut X509_VERIFY_PARAM,
    pub other_ctx: *mut libc::c_void,
    pub verify: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                           -> libc::c_int>,
    pub verify_cb: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut X509_STORE_CTX)
                              -> libc::c_int>,
    pub get_issuer: Option<unsafe extern "C" fn(_: *mut *mut X509,
                                                _: *mut X509_STORE_CTX,
                                                _: *mut X509) -> libc::c_int>,
    pub check_issued: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509, _: *mut X509)
                                 -> libc::c_int>,
    pub check_revocation: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                                     -> libc::c_int>,
    pub get_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                             _: *mut *mut X509_CRL,
                                             _: *mut X509) -> libc::c_int>,
    pub check_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                               _: *mut X509_CRL)
                              -> libc::c_int>,
    pub cert_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                              _: *mut X509_CRL, _: *mut X509)
                             -> libc::c_int>,
    pub check_policy: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                                 -> libc::c_int>,
    pub lookup_certs: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509_NAME)
                                 -> *mut stack_st_X509>,
    pub lookup_crls: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                 _: *mut X509_NAME)
                                -> *mut stack_st_X509_CRL>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                            -> libc::c_int>,
    pub valid: libc::c_int,
    pub last_untrusted: libc::c_int,
    pub chain: *mut stack_st_X509,
    pub tree: *mut X509_POLICY_TREE,
    pub explicit_policy: libc::c_int,
    pub error_depth: libc::c_int,
    pub error: libc::c_int,
    pub current_cert: *mut X509,
    pub current_issuer: *mut X509,
    pub current_crl: *mut X509_CRL,
    pub current_crl_score: libc::c_int,
    pub current_reasons: libc::c_uint,
    pub parent: *mut X509_STORE_CTX,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type X509_POLICY_TREE = X509_POLICY_TREE_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509 {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_CRL {
    pub stack: _STACK,
}
pub type X509_VERIFY_PARAM = X509_VERIFY_PARAM_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_VERIFY_PARAM_st {
    pub name: *mut libc::c_char,
    pub check_time: time_t,
    pub inh_flags: libc::c_ulong,
    pub flags: libc::c_ulong,
    pub purpose: libc::c_int,
    pub trust: libc::c_int,
    pub depth: libc::c_int,
    pub policies: *mut stack_st_ASN1_OBJECT,
    pub id: *mut X509_VERIFY_PARAM_ID,
}
pub type X509_VERIFY_PARAM_ID = X509_VERIFY_PARAM_ID_st;
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
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
/* __darwin_time_t */
pub type time_t = __darwin_time_t;
pub type X509_STORE = x509_store_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_LOOKUP {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_OBJECT {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_st {
    pub version: libc::c_int,
    pub type_0: libc::c_int,
    pub method: *const SSL_METHOD,
    pub rbio: *mut BIO,
    pub wbio: *mut BIO,
    pub bbio: *mut BIO,
    pub rwstate: libc::c_int,
    pub in_handshake: libc::c_int,
    pub handshake_func: Option<unsafe extern "C" fn(_: *mut SSL)
                                   -> libc::c_int>,
    pub server: libc::c_int,
    pub new_session: libc::c_int,
    pub quiet_shutdown: libc::c_int,
    pub shutdown: libc::c_int,
    pub state: libc::c_int,
    pub rstate: libc::c_int,
    pub init_buf: *mut BUF_MEM,
    pub init_msg: *mut libc::c_void,
    pub init_num: libc::c_int,
    pub init_off: libc::c_int,
    pub packet: *mut libc::c_uchar,
    pub packet_length: libc::c_uint,
    pub s2: *mut ssl2_state_st,
    pub s3: *mut ssl3_state_st,
    pub d1: *mut dtls1_state_st,
    pub read_ahead: libc::c_int,
    pub msg_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *const libc::c_void,
                                                  _: size_t, _: *mut SSL,
                                                  _: *mut libc::c_void)
                                 -> ()>,
    pub msg_callback_arg: *mut libc::c_void,
    pub hit: libc::c_int,
    pub param: *mut X509_VERIFY_PARAM,
    pub cipher_list: *mut stack_st_SSL_CIPHER,
    pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
    pub mac_flags: libc::c_int,
    pub enc_read_ctx: *mut EVP_CIPHER_CTX,
    pub read_hash: *mut EVP_MD_CTX,
    pub expand: *mut COMP_CTX,
    pub enc_write_ctx: *mut EVP_CIPHER_CTX,
    pub write_hash: *mut EVP_MD_CTX,
    pub compress: *mut COMP_CTX,
    pub cert: *mut cert_st,
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub session: *mut SSL_SESSION,
    pub generate_session_id: GEN_SESSION_CB,
    pub verify_mode: libc::c_int,
    pub verify_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut X509_STORE_CTX)
                                    -> libc::c_int>,
    pub info_callback: Option<unsafe extern "C" fn(_: *const SSL,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub error: libc::c_int,
    pub error_code: libc::c_int,
    pub psk_client_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _:
                                                             *const libc::c_char,
                                                         _: *mut libc::c_char,
                                                         _: libc::c_uint,
                                                         _:
                                                             *mut libc::c_uchar,
                                                         _: libc::c_uint)
                                        -> libc::c_uint>,
    pub psk_server_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _:
                                                             *const libc::c_char,
                                                         _:
                                                             *mut libc::c_uchar,
                                                         _: libc::c_uint)
                                        -> libc::c_uint>,
    pub ctx: *mut SSL_CTX,
    pub debug: libc::c_int,
    pub verify_result: libc::c_long,
    pub ex_data: CRYPTO_EX_DATA,
    pub client_CA: *mut stack_st_X509_NAME,
    pub references: libc::c_int,
    pub options: libc::c_ulong,
    pub mode: libc::c_ulong,
    pub max_cert_list: libc::c_long,
    pub first_packet: libc::c_int,
    pub client_version: libc::c_int,
    pub max_send_fragment: libc::c_uint,
    pub tlsext_debug_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut libc::c_uchar,
                                                     _: libc::c_int,
                                                     _: *mut libc::c_void)
                                    -> ()>,
    pub tlsext_debug_arg: *mut libc::c_void,
    pub tlsext_hostname: *mut libc::c_char,
    pub servername_done: libc::c_int,
    pub tlsext_status_type: libc::c_int,
    pub tlsext_status_expected: libc::c_int,
    pub tlsext_ocsp_ids: *mut stack_st_OCSP_RESPID,
    pub tlsext_ocsp_exts: *mut X509_EXTENSIONS,
    pub tlsext_ocsp_resp: *mut libc::c_uchar,
    pub tlsext_ocsp_resplen: libc::c_int,
    pub tlsext_ticket_expected: libc::c_int,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
    pub tlsext_opaque_prf_input: *mut libc::c_void,
    pub tlsext_opaque_prf_input_len: size_t,
    pub tlsext_session_ticket: *mut TLS_SESSION_TICKET_EXT,
    pub tls_session_ticket_ext_cb: tls_session_ticket_ext_cb_fn,
    pub tls_session_ticket_ext_cb_arg: *mut libc::c_void,
    pub tls_session_secret_cb: tls_session_secret_cb_fn,
    pub tls_session_secret_cb_arg: *mut libc::c_void,
    pub initial_ctx: *mut SSL_CTX,
    pub next_proto_negotiated: *mut libc::c_uchar,
    pub next_proto_negotiated_len: libc::c_uchar,
    pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
    pub srtp_profile: *mut SRTP_PROTECTION_PROFILE,
    pub tlsext_heartbeat: libc::c_uint,
    pub tlsext_hb_pending: libc::c_uint,
    pub tlsext_hb_seq: libc::c_uint,
    pub renegotiate: libc::c_int,
    pub srp_ctx: SRP_CTX,
    pub alpn_client_proto_list: *mut libc::c_uchar,
    pub alpn_client_proto_list_len: libc::c_uint,
}
pub type SRP_CTX = srp_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct srp_ctx_st {
    pub SRP_cb_arg: *mut libc::c_void,
    pub TLS_ext_srp_username_callback: Option<unsafe extern "C" fn(_:
                                                                       *mut SSL,
                                                                   _:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> libc::c_int>,
    pub SRP_verify_param_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> libc::c_int>,
    pub SRP_give_srp_client_pwd_callback: Option<unsafe extern "C" fn(_:
                                                                          *mut SSL,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> *mut libc::c_char>,
    pub login: *mut libc::c_char,
    pub N: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub s: *mut BIGNUM,
    pub B: *mut BIGNUM,
    pub A: *mut BIGNUM,
    pub a: *mut BIGNUM,
    pub b: *mut BIGNUM,
    pub v: *mut BIGNUM,
    pub info: *mut libc::c_char,
    pub strength: libc::c_int,
    pub srp_Mask: libc::c_ulong,
}
pub type SSL = ssl_st;
pub type SRTP_PROTECTION_PROFILE = srtp_protection_profile_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct srtp_protection_profile_st {
    pub name: *const libc::c_char,
    pub id: libc::c_ulong,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_SRTP_PROTECTION_PROFILE {
    pub stack: _STACK,
}
pub type SSL_CTX = ssl_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_ctx_st {
    pub method: *const SSL_METHOD,
    pub cipher_list: *mut stack_st_SSL_CIPHER,
    pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
    pub cert_store: *mut x509_store_st,
    pub sessions: *mut lhash_st_SSL_SESSION,
    pub session_cache_size: libc::c_ulong,
    pub session_cache_head: *mut ssl_session_st,
    pub session_cache_tail: *mut ssl_session_st,
    pub session_cache_mode: libc::c_int,
    pub session_timeout: libc::c_long,
    pub new_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_st,
                                                    _: *mut SSL_SESSION)
                                   -> libc::c_int>,
    pub remove_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_ctx_st,
                                                       _: *mut SSL_SESSION)
                                      -> ()>,
    pub get_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_st,
                                                    _: *mut libc::c_uchar,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_int)
                                   -> *mut SSL_SESSION>,
    pub stats: C2RustUnnamed_2,
    pub references: libc::c_int,
    pub app_verify_callback: Option<unsafe extern "C" fn(_:
                                                             *mut X509_STORE_CTX,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int>,
    pub app_verify_arg: *mut libc::c_void,
    pub default_passwd_callback: Option<unsafe extern "C" fn(_:
                                                                 *mut libc::c_char,
                                                             _: libc::c_int,
                                                             _: libc::c_int,
                                                             _:
                                                                 *mut libc::c_void)
                                            -> libc::c_int>,
    pub default_passwd_callback_userdata: *mut libc::c_void,
    pub client_cert_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                    _: *mut *mut X509,
                                                    _: *mut *mut EVP_PKEY)
                                   -> libc::c_int>,
    pub app_gen_cookie_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                       _: *mut libc::c_uchar,
                                                       _: *mut libc::c_uint)
                                      -> libc::c_int>,
    pub app_verify_cookie_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                          _:
                                                              *mut libc::c_uchar,
                                                          _: libc::c_uint)
                                         -> libc::c_int>,
    pub ex_data: CRYPTO_EX_DATA,
    pub rsa_md5: *const EVP_MD,
    pub md5: *const EVP_MD,
    pub sha1: *const EVP_MD,
    pub extra_certs: *mut stack_st_X509,
    pub comp_methods: *mut stack_st_SSL_COMP,
    pub info_callback: Option<unsafe extern "C" fn(_: *const SSL,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub client_CA: *mut stack_st_X509_NAME,
    pub options: libc::c_ulong,
    pub mode: libc::c_ulong,
    pub max_cert_list: libc::c_long,
    pub cert: *mut cert_st,
    pub read_ahead: libc::c_int,
    pub msg_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *const libc::c_void,
                                                  _: size_t, _: *mut SSL,
                                                  _: *mut libc::c_void)
                                 -> ()>,
    pub msg_callback_arg: *mut libc::c_void,
    pub verify_mode: libc::c_int,
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub default_verify_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _:
                                                                 *mut X509_STORE_CTX)
                                            -> libc::c_int>,
    pub generate_session_id: GEN_SESSION_CB,
    pub param: *mut X509_VERIFY_PARAM,
    pub quiet_shutdown: libc::c_int,
    pub max_send_fragment: libc::c_uint,
    pub client_cert_engine: *mut ENGINE,
    pub tlsext_servername_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                                _:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> libc::c_int>,
    pub tlsext_servername_arg: *mut libc::c_void,
    pub tlsext_tick_key_name: [libc::c_uchar; 16],
    pub tlsext_tick_hmac_key: [libc::c_uchar; 16],
    pub tlsext_tick_aes_key: [libc::c_uchar; 16],
    pub tlsext_ticket_key_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                          _:
                                                              *mut libc::c_uchar,
                                                          _:
                                                              *mut libc::c_uchar,
                                                          _:
                                                              *mut EVP_CIPHER_CTX,
                                                          _: *mut HMAC_CTX,
                                                          _: libc::c_int)
                                         -> libc::c_int>,
    pub tlsext_status_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                      _: *mut libc::c_void)
                                     -> libc::c_int>,
    pub tlsext_status_arg: *mut libc::c_void,
    pub tlsext_opaque_prf_input_callback: Option<unsafe extern "C" fn(_:
                                                                          *mut SSL,
                                                                      _:
                                                                          *mut libc::c_void,
                                                                      _:
                                                                          size_t,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> libc::c_int>,
    pub tlsext_opaque_prf_input_callback_arg: *mut libc::c_void,
    pub psk_identity_hint: *mut libc::c_char,
    pub psk_client_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _:
                                                             *const libc::c_char,
                                                         _: *mut libc::c_char,
                                                         _: libc::c_uint,
                                                         _:
                                                             *mut libc::c_uchar,
                                                         _: libc::c_uint)
                                        -> libc::c_uint>,
    pub psk_server_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _:
                                                             *const libc::c_char,
                                                         _:
                                                             *mut libc::c_uchar,
                                                         _: libc::c_uint)
                                        -> libc::c_uint>,
    pub freelist_max_len: libc::c_uint,
    pub wbuf_freelist: *mut ssl3_buf_freelist_st,
    pub rbuf_freelist: *mut ssl3_buf_freelist_st,
    pub srp_ctx: SRP_CTX,
    pub next_protos_advertised_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                               _:
                                                                   *mut *const libc::c_uchar,
                                                               _:
                                                                   *mut libc::c_uint,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> libc::c_int>,
    pub next_protos_advertised_cb_arg: *mut libc::c_void,
    pub next_proto_select_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                          _:
                                                              *mut *mut libc::c_uchar,
                                                          _:
                                                              *mut libc::c_uchar,
                                                          _:
                                                              *const libc::c_uchar,
                                                          _: libc::c_uint,
                                                          _:
                                                              *mut libc::c_void)
                                         -> libc::c_int>,
    pub next_proto_select_cb_arg: *mut libc::c_void,
    pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
    pub alpn_select_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                    _:
                                                        *mut *const libc::c_uchar,
                                                    _: *mut libc::c_uchar,
                                                    _: *const libc::c_uchar,
                                                    _: libc::c_uint,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
    pub alpn_select_cb_arg: *mut libc::c_void,
    pub alpn_client_proto_list: *mut libc::c_uchar,
    pub alpn_client_proto_list_len: libc::c_uint,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
}
pub type HMAC_CTX = hmac_ctx_st;
/* crypto/hmac/hmac.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* largest known is SHA512 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct hmac_ctx_st {
    pub md: *const EVP_MD,
    pub md_ctx: EVP_MD_CTX,
    pub i_ctx: EVP_MD_CTX,
    pub o_ctx: EVP_MD_CTX,
    pub key_length: libc::c_uint,
    pub key: [libc::c_uchar; 128],
}
pub type GEN_SESSION_CB
    =
    Option<unsafe extern "C" fn(_: *const SSL, _: *mut libc::c_uchar,
                                _: *mut libc::c_uint) -> libc::c_int>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_NAME {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_SSL_COMP {
    pub stack: _STACK,
}
/* crypto/pem/pem.h */
/* Copyright (C) 1995-1997 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/*
   * Note that this structure is initialised by PEM_SealInit and cleaned up
   * by PEM_SealFinal (at least for now)
   */
/* enc_type is one off */
/*      char iv[8]; unused and wrong size */
/* what type of object */
/*-
        unused, and wrong size
        unsigned char iv[8]; */
/*-
    XXX(ben): don#t think this is used!
        STACK *x509_chain;      / * certificate chain */
/* signature type */
/* is the md encrypted or not? */
/* length of md_data */
/* message digest, could be pkey encrypted */
/* date encryption cipher */
/* key length */
/* key */
/*-
    unused, and wrong size
    unsigned char iv[8]; */
/* is the data encrypted */
/*
 * These macros make the PEM_read/PEM_write functions easier to maintain and
 * write. Now they are all implemented with either: IMPLEMENT_PEM_rw(...) or
 * IMPLEMENT_PEM_rw_cb(...)
 */
/* These are the same except they are for the declarations */
/* "userdata": new with OpenSSL 0.9.4 */
pub type pem_password_cb
    =
    unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int,
                         _: *mut libc::c_void) -> libc::c_int;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub sess_connect: libc::c_int,
    pub sess_connect_renegotiate: libc::c_int,
    pub sess_connect_good: libc::c_int,
    pub sess_accept: libc::c_int,
    pub sess_accept_renegotiate: libc::c_int,
    pub sess_accept_good: libc::c_int,
    pub sess_miss: libc::c_int,
    pub sess_timeout: libc::c_int,
    pub sess_cache_full: libc::c_int,
    pub sess_hit: libc::c_int,
    pub sess_cb_hit: libc::c_int,
}
pub type SSL_SESSION = ssl_session_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_session_st {
    pub ssl_version: libc::c_int,
    pub key_arg_length: libc::c_uint,
    pub key_arg: [libc::c_uchar; 8],
    pub master_key_length: libc::c_int,
    pub master_key: [libc::c_uchar; 48],
    pub session_id_length: libc::c_uint,
    pub session_id: [libc::c_uchar; 32],
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub psk_identity_hint: *mut libc::c_char,
    pub psk_identity: *mut libc::c_char,
    pub not_resumable: libc::c_int,
    pub sess_cert: *mut sess_cert_st,
    pub peer: *mut X509,
    pub verify_result: libc::c_long,
    pub references: libc::c_int,
    pub timeout: libc::c_long,
    pub time: libc::c_long,
    pub compress_meth: libc::c_uint,
    pub cipher: *const SSL_CIPHER,
    pub cipher_id: libc::c_ulong,
    pub ciphers: *mut stack_st_SSL_CIPHER,
    pub ex_data: CRYPTO_EX_DATA,
    pub prev: *mut ssl_session_st,
    pub next: *mut ssl_session_st,
    pub tlsext_hostname: *mut libc::c_char,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
    pub tlsext_tick: *mut libc::c_uchar,
    pub tlsext_ticklen: size_t,
    pub tlsext_tick_lifetime_hint: libc::c_long,
    pub srp_username: *mut libc::c_char,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_SSL_CIPHER {
    pub stack: _STACK,
}
pub type SSL_CIPHER = ssl_cipher_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_cipher_st {
    pub valid: libc::c_int,
    pub name: *const libc::c_char,
    pub id: libc::c_ulong,
    pub algorithm_mkey: libc::c_ulong,
    pub algorithm_auth: libc::c_ulong,
    pub algorithm_enc: libc::c_ulong,
    pub algorithm_mac: libc::c_ulong,
    pub algorithm_ssl: libc::c_ulong,
    pub algo_strength: libc::c_ulong,
    pub algorithm2: libc::c_ulong,
    pub strength_bits: libc::c_int,
    pub alg_bits: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct lhash_st_SSL_SESSION {
    pub dummy: libc::c_int,
}
pub type SSL_METHOD = ssl_method_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_method_st {
    pub version: libc::c_int,
    pub ssl_new: Option<unsafe extern "C" fn(_: *mut SSL) -> libc::c_int>,
    pub ssl_clear: Option<unsafe extern "C" fn(_: *mut SSL) -> ()>,
    pub ssl_free: Option<unsafe extern "C" fn(_: *mut SSL) -> ()>,
    pub ssl_accept: Option<unsafe extern "C" fn(_: *mut SSL) -> libc::c_int>,
    pub ssl_connect: Option<unsafe extern "C" fn(_: *mut SSL) -> libc::c_int>,
    pub ssl_read: Option<unsafe extern "C" fn(_: *mut SSL,
                                              _: *mut libc::c_void,
                                              _: libc::c_int) -> libc::c_int>,
    pub ssl_peek: Option<unsafe extern "C" fn(_: *mut SSL,
                                              _: *mut libc::c_void,
                                              _: libc::c_int) -> libc::c_int>,
    pub ssl_write: Option<unsafe extern "C" fn(_: *mut SSL,
                                               _: *const libc::c_void,
                                               _: libc::c_int)
                              -> libc::c_int>,
    pub ssl_shutdown: Option<unsafe extern "C" fn(_: *mut SSL)
                                 -> libc::c_int>,
    pub ssl_renegotiate: Option<unsafe extern "C" fn(_: *mut SSL)
                                    -> libc::c_int>,
    pub ssl_renegotiate_check: Option<unsafe extern "C" fn(_: *mut SSL)
                                          -> libc::c_int>,
    pub ssl_get_message: Option<unsafe extern "C" fn(_: *mut SSL,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_long,
                                                     _: *mut libc::c_int)
                                    -> libc::c_long>,
    pub ssl_read_bytes: Option<unsafe extern "C" fn(_: *mut SSL,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_uchar,
                                                    _: libc::c_int,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub ssl_write_bytes: Option<unsafe extern "C" fn(_: *mut SSL,
                                                     _: libc::c_int,
                                                     _: *const libc::c_void,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
    pub ssl_dispatch_alert: Option<unsafe extern "C" fn(_: *mut SSL)
                                       -> libc::c_int>,
    pub ssl_ctrl: Option<unsafe extern "C" fn(_: *mut SSL, _: libc::c_int,
                                              _: libc::c_long,
                                              _: *mut libc::c_void)
                             -> libc::c_long>,
    pub ssl_ctx_ctrl: Option<unsafe extern "C" fn(_: *mut SSL_CTX,
                                                  _: libc::c_int,
                                                  _: libc::c_long,
                                                  _: *mut libc::c_void)
                                 -> libc::c_long>,
    pub get_cipher_by_char: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_uchar)
                                       -> *const SSL_CIPHER>,
    pub put_cipher_by_char: Option<unsafe extern "C" fn(_: *const SSL_CIPHER,
                                                        _: *mut libc::c_uchar)
                                       -> libc::c_int>,
    pub ssl_pending: Option<unsafe extern "C" fn(_: *const SSL)
                                -> libc::c_int>,
    pub num_ciphers: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub get_cipher: Option<unsafe extern "C" fn(_: libc::c_uint)
                               -> *const SSL_CIPHER>,
    pub get_ssl_method: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *const ssl_method_st>,
    pub get_timeout: Option<unsafe extern "C" fn() -> libc::c_long>,
    pub ssl3_enc: *mut ssl3_enc_method,
    pub ssl_version: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub ssl_callback_ctrl: Option<unsafe extern "C" fn(_: *mut SSL,
                                                       _: libc::c_int,
                                                       _:
                                                           Option<unsafe extern "C" fn()
                                                                      -> ()>)
                                      -> libc::c_long>,
    pub ssl_ctx_callback_ctrl: Option<unsafe extern "C" fn(_: *mut SSL_CTX,
                                                           _: libc::c_int,
                                                           _:
                                                               Option<unsafe extern "C" fn()
                                                                          ->
                                                                              ()>)
                                          -> libc::c_long>,
}
pub type tls_session_secret_cb_fn
    =
    Option<unsafe extern "C" fn(_: *mut SSL, _: *mut libc::c_void,
                                _: *mut libc::c_int,
                                _: *mut stack_st_SSL_CIPHER,
                                _: *mut *mut SSL_CIPHER, _: *mut libc::c_void)
               -> libc::c_int>;
pub type tls_session_ticket_ext_cb_fn
    =
    Option<unsafe extern "C" fn(_: *mut SSL, _: *const libc::c_uchar,
                                _: libc::c_int, _: *mut libc::c_void)
               -> libc::c_int>;
pub type TLS_SESSION_TICKET_EXT = tls_session_ticket_ext_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct tls_session_ticket_ext_st {
    pub length: libc::c_ushort,
    pub data: *mut libc::c_void,
}
pub type X509_EXTENSIONS = stack_st_X509_EXTENSION;
pub type COMP_CTX = comp_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct comp_ctx_st {
    pub meth: *mut COMP_METHOD,
    pub compress_in: libc::c_ulong,
    pub compress_out: libc::c_ulong,
    pub expand_in: libc::c_ulong,
    pub expand_out: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type COMP_METHOD = comp_method_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct comp_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(_: *mut COMP_CTX) -> libc::c_int>,
    pub finish: Option<unsafe extern "C" fn(_: *mut COMP_CTX) -> ()>,
    pub compress: Option<unsafe extern "C" fn(_: *mut COMP_CTX,
                                              _: *mut libc::c_uchar,
                                              _: libc::c_uint,
                                              _: *mut libc::c_uchar,
                                              _: libc::c_uint)
                             -> libc::c_int>,
    pub expand: Option<unsafe extern "C" fn(_: *mut COMP_CTX,
                                            _: *mut libc::c_uchar,
                                            _: libc::c_uint,
                                            _: *mut libc::c_uchar,
                                            _: libc::c_uint) -> libc::c_int>,
    pub ctrl: Option<unsafe extern "C" fn() -> libc::c_long>,
    pub callback_ctrl: Option<unsafe extern "C" fn() -> libc::c_long>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dtls1_state_st {
    pub send_cookie: libc::c_uint,
    pub cookie: [libc::c_uchar; 256],
    pub rcvd_cookie: [libc::c_uchar; 256],
    pub cookie_len: libc::c_uint,
    pub r_epoch: libc::c_ushort,
    pub w_epoch: libc::c_ushort,
    pub bitmap: DTLS1_BITMAP,
    pub next_bitmap: DTLS1_BITMAP,
    pub handshake_write_seq: libc::c_ushort,
    pub next_handshake_write_seq: libc::c_ushort,
    pub handshake_read_seq: libc::c_ushort,
    pub last_write_sequence: [libc::c_uchar; 8],
    pub unprocessed_rcds: record_pqueue,
    pub processed_rcds: record_pqueue,
    pub buffered_messages: pqueue,
    pub sent_messages: pqueue,
    pub buffered_app_data: record_pqueue,
    pub listen: libc::c_uint,
    pub link_mtu: libc::c_uint,
    pub mtu: libc::c_uint,
    pub w_msg_hdr: hm_header_st,
    pub r_msg_hdr: hm_header_st,
    pub timeout: dtls1_timeout_st,
    pub next_timeout: timeval,
    pub timeout_duration: libc::c_ushort,
    pub alert_fragment: [libc::c_uchar; 2],
    pub alert_fragment_len: libc::c_uint,
    pub handshake_fragment: [libc::c_uchar; 12],
    pub handshake_fragment_len: libc::c_uint,
    pub retransmitting: libc::c_uint,
    pub change_cipher_spec_ok: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dtls1_timeout_st {
    pub read_timeouts: libc::c_uint,
    pub write_timeouts: libc::c_uint,
    pub num_alerts: libc::c_uint,
}
/* ssl/dtls1.h */
/*
 * DTLS implementation written by Nagendra Modadugu
 * (nagendra@cs.stanford.edu) for the OpenSSL project 2005.
 */
/* ====================================================================
 * Copyright (c) 1999-2005 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.OpenSSL.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@OpenSSL.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.OpenSSL.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
/* Special value for method supporting multiple versions */
/* lengths of messages */
/* Max MTU overhead we know about so far is 40 for IPv6 + 8 for UDP */
/* track 32 packets on 32-bit systems and 64
                                 * - on 64-bit systems */
/* max record number seen so far, 64-bit
                                   * value in big-endian encoding */
/* cryptographic state */
/* used for mac generation */
/* compression */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct hm_header_st {
    pub type_0: libc::c_uchar,
    pub msg_len: libc::c_ulong,
    pub seq: libc::c_ushort,
    pub frag_off: libc::c_ulong,
    pub frag_len: libc::c_ulong,
    pub is_ccs: libc::c_uint,
    pub saved_retransmit_state: dtls1_retransmit_state,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dtls1_retransmit_state {
    pub enc_write_ctx: *mut EVP_CIPHER_CTX,
    pub write_hash: *mut EVP_MD_CTX,
    pub compress: *mut COMP_CTX,
    pub session: *mut SSL_SESSION,
    pub epoch: libc::c_ushort,
}
pub type record_pqueue = record_pqueue_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct record_pqueue_st {
    pub epoch: libc::c_ushort,
    pub q: pqueue,
}
/* crypto/pqueue/pqueue.h */
/*
 * DTLS implementation written by Nagendra Modadugu
 * (nagendra@cs.stanford.edu) for the OpenSSL project 2005.
 */
/* ====================================================================
 * Copyright (c) 1999-2005 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.OpenSSL.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@OpenSSL.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.OpenSSL.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
pub type pqueue = *mut _pqueue;
pub type DTLS1_BITMAP = dtls1_bitmap_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dtls1_bitmap_st {
    pub map: libc::c_ulong,
    pub max_seq_num: [libc::c_uchar; 8],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl3_state_st {
    pub flags: libc::c_long,
    pub delay_buf_pop_ret: libc::c_int,
    pub read_sequence: [libc::c_uchar; 8],
    pub read_mac_secret_size: libc::c_int,
    pub read_mac_secret: [libc::c_uchar; 64],
    pub write_sequence: [libc::c_uchar; 8],
    pub write_mac_secret_size: libc::c_int,
    pub write_mac_secret: [libc::c_uchar; 64],
    pub server_random: [libc::c_uchar; 32],
    pub client_random: [libc::c_uchar; 32],
    pub need_empty_fragments: libc::c_int,
    pub empty_fragment_done: libc::c_int,
    pub init_extra: libc::c_int,
    pub rbuf: SSL3_BUFFER,
    pub wbuf: SSL3_BUFFER,
    pub rrec: SSL3_RECORD,
    pub wrec: SSL3_RECORD,
    pub alert_fragment: [libc::c_uchar; 2],
    pub alert_fragment_len: libc::c_uint,
    pub handshake_fragment: [libc::c_uchar; 4],
    pub handshake_fragment_len: libc::c_uint,
    pub wnum: libc::c_uint,
    pub wpend_tot: libc::c_int,
    pub wpend_type: libc::c_int,
    pub wpend_ret: libc::c_int,
    pub wpend_buf: *const libc::c_uchar,
    pub handshake_buffer: *mut BIO,
    pub handshake_dgst: *mut *mut EVP_MD_CTX,
    pub change_cipher_spec: libc::c_int,
    pub warn_alert: libc::c_int,
    pub fatal_alert: libc::c_int,
    pub alert_dispatch: libc::c_int,
    pub send_alert: [libc::c_uchar; 2],
    pub renegotiate: libc::c_int,
    pub total_renegotiations: libc::c_int,
    pub num_renegotiations: libc::c_int,
    pub in_read_app_data: libc::c_int,
    pub client_opaque_prf_input: *mut libc::c_void,
    pub client_opaque_prf_input_len: size_t,
    pub server_opaque_prf_input: *mut libc::c_void,
    pub server_opaque_prf_input_len: size_t,
    pub tmp: C2RustUnnamed_3,
    pub previous_client_finished: [libc::c_uchar; 64],
    pub previous_client_finished_len: libc::c_uchar,
    pub previous_server_finished: [libc::c_uchar; 64],
    pub previous_server_finished_len: libc::c_uchar,
    pub send_connection_binding: libc::c_int,
    pub next_proto_neg_seen: libc::c_int,
    pub is_probably_safari: libc::c_char,
    pub alpn_selected: *mut libc::c_uchar,
    pub alpn_selected_len: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub cert_verify_md: [libc::c_uchar; 128],
    pub finish_md: [libc::c_uchar; 128],
    pub finish_md_len: libc::c_int,
    pub peer_finish_md: [libc::c_uchar; 128],
    pub peer_finish_md_len: libc::c_int,
    pub message_size: libc::c_ulong,
    pub message_type: libc::c_int,
    pub new_cipher: *const SSL_CIPHER,
    pub dh: *mut DH,
    pub ecdh: *mut EC_KEY,
    pub next_state: libc::c_int,
    pub reuse_message: libc::c_int,
    pub cert_req: libc::c_int,
    pub ctype_num: libc::c_int,
    pub ctype: [libc::c_char; 9],
    pub ca_names: *mut stack_st_X509_NAME,
    pub use_rsa_tmp: libc::c_int,
    pub key_block_length: libc::c_int,
    pub key_block: *mut libc::c_uchar,
    pub new_sym_enc: *const EVP_CIPHER,
    pub new_hash: *const EVP_MD,
    pub new_mac_pkey_type: libc::c_int,
    pub new_mac_secret_size: libc::c_int,
    pub new_compression: *const SSL_COMP,
    pub cert_request: libc::c_int,
}
pub type SSL_COMP = ssl_comp_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_comp_st {
    pub id: libc::c_int,
    pub name: *const libc::c_char,
    pub method: *mut COMP_METHOD,
}
pub type EC_KEY = ec_key_st;
pub type BIO = bio_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bio_st {
    pub method: *mut BIO_METHOD,
    pub callback: Option<unsafe extern "C" fn(_: *mut bio_st, _: libc::c_int,
                                              _: *const libc::c_char,
                                              _: libc::c_int, _: libc::c_long,
                                              _: libc::c_long)
                             -> libc::c_long>,
    pub cb_arg: *mut libc::c_char,
    pub init: libc::c_int,
    pub shutdown: libc::c_int,
    pub flags: libc::c_int,
    pub retry_reason: libc::c_int,
    pub num: libc::c_int,
    pub ptr: *mut libc::c_void,
    pub next_bio: *mut bio_st,
    pub prev_bio: *mut bio_st,
    pub references: libc::c_int,
    pub num_read: libc::c_ulong,
    pub num_write: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
    /*
 * This stuff is basically class callback functions The current classes are
 * SSL_CTX, SSL, SSL_SESSION, and a few more
 */
    /* Arbitary long */
    /* Arbitary void * */
    /*
 * Per class, we have a STACK of CRYPTO_EX_DATA_FUNCS for each CRYPTO_EX_DATA
 * entry.
 */
}
pub type BIO_METHOD = bio_method_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bio_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub bwrite: Option<unsafe extern "C" fn(_: *mut BIO,
                                            _: *const libc::c_char,
                                            _: libc::c_int) -> libc::c_int>,
    pub bread: Option<unsafe extern "C" fn(_: *mut BIO, _: *mut libc::c_char,
                                           _: libc::c_int) -> libc::c_int>,
    pub bputs: Option<unsafe extern "C" fn(_: *mut BIO,
                                           _: *const libc::c_char)
                          -> libc::c_int>,
    pub bgets: Option<unsafe extern "C" fn(_: *mut BIO, _: *mut libc::c_char,
                                           _: libc::c_int) -> libc::c_int>,
    pub ctrl: Option<unsafe extern "C" fn(_: *mut BIO, _: libc::c_int,
                                          _: libc::c_long,
                                          _: *mut libc::c_void)
                         -> libc::c_long>,
    pub create: Option<unsafe extern "C" fn(_: *mut BIO) -> libc::c_int>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut BIO) -> libc::c_int>,
    pub callback_ctrl: Option<unsafe extern "C" fn(_: *mut BIO,
                                                   _: libc::c_int,
                                                   _:
                                                       Option<unsafe extern "C" fn(_:
                                                                                       *mut bio_st,
                                                                                   _:
                                                                                       libc::c_int,
                                                                                   _:
                                                                                       *const libc::c_char,
                                                                                   _:
                                                                                       libc::c_int,
                                                                                   _:
                                                                                       libc::c_long,
                                                                                   _:
                                                                                       libc::c_long)
                                                                  -> ()>)
                                  -> libc::c_long>,
}
pub type bio_info_cb
    =
    unsafe extern "C" fn(_: *mut bio_st, _: libc::c_int,
                         _: *const libc::c_char, _: libc::c_int,
                         _: libc::c_long, _: libc::c_long) -> ();
/* ssl/ssl3.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* ====================================================================
 * Copyright (c) 1998-2002 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.openssl.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@openssl.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.openssl.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
/* ====================================================================
 * Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
 * ECC cipher suite support in OpenSSL originally developed by
 * SUN MICROSYSTEMS, INC., and contributed to the OpenSSL project.
 */
/*
 * Signalling cipher suite value from RFC 5746
 * (TLS_EMPTY_RENEGOTIATION_INFO_SCSV)
 */
/*
 * Signalling cipher suite value from draft-ietf-tls-downgrade-scsv-00
 * (TLS_FALLBACK_SCSV)
 */
/*
 * VRS Additional Kerberos5 entries
 */
/*
 * This next block of six "EDH" labels is for backward compatibility with
 * older versions of OpenSSL.  New code should use the six "DHE" labels above
 * instead:
 */
/*
  * Some will argue that this increases memory footprint, but it's not
  * actually true. Point is that malloc has to return at least 64-bit aligned
  * pointers, meaning that allocating 5 bytes wastes 3 bytes in either case.
  * Suggested pre-gaping simply moves these wasted bytes from the end of
  * allocated region to its front, but makes data payload aligned, which
  * improves performance:-)
  */
/*
 * This is the maximum MAC (digest) size used by the SSL library. Currently
 * maximum of 20 is used by SHA1, but we reserve for future extension for
 * 512-bit hashes.
 */
/*
 * Maximum block size used in all ciphersuites. Currently 16 for AES.
 */
/* Maximum plaintext length: defined by SSL/TLS standards */
/* Maximum compression overhead: defined by SSL/TLS standards */
/*
 * The standards give a maximum encryption overhead of 1024 bytes. In
 * practice the value is lower than this. The overhead is the maximum number
 * of padding bytes (256) plus the mac size.
 */
/*
 * OpenSSL currently only uses a padding length of at most one block so the
 * send overhead is smaller.
 */
/* If compression isn't used don't include the compression overhead */
/* Pseudo content types to indicate additional parameters */
/* Pseudo content type for SSL/TLS header info */
/* fatal */
/* fatal */
/* fatal */
/* fatal */
/* fatal */
pub type SSL3_RECORD = ssl3_record_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl3_record_st {
    pub type_0: libc::c_int,
    pub length: libc::c_uint,
    pub off: libc::c_uint,
    pub data: *mut libc::c_uchar,
    pub input: *mut libc::c_uchar,
    pub comp: *mut libc::c_uchar,
    pub epoch: libc::c_ulong,
    pub seq_num: [libc::c_uchar; 8],
}
pub type SSL3_BUFFER = ssl3_buffer_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl3_buffer_st {
    pub buf: *mut libc::c_uchar,
    pub len: size_t,
    pub offset: libc::c_int,
    pub left: libc::c_int,
}
/* type of record */
    /*
     * r
     */
/* How many bytes available */
    /*
     * rw
     */
/* read/write offset into 'buf' */
    /*
     * r
     */
/* pointer to the record data */
    /*
     * rw
     */
/* where the decode bytes are */
    /*
     * rw
     */
/* only used with decompression - malloc()ed */
    /*
     * r
     */
/* epoch number, needed by DTLS1 */
    /*
     * r
     */
/* sequence number, needed by DTLS1 */
    /*
     * r
     */
/* ssl/ssl2.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* Protocol Version Codes */
/* #define SSL2_CLIENT_VERSION  0x0002 */
/* #define SSL2_SERVER_VERSION  0x0002 */
/* Protocol Message Codes */
/* Error Message Codes */
/* Cipher Kind Values */
/* v3 */
/* v3 */
/* v3 */
/* MS hack */
/* SSLeay */
/* SSLeay */
/* Flags for the SSL_CIPHER.algorithm2 field */
/* Certificate Type Codes */
/* Authentication Type Code */
/* Upper/Lower Bounds */
/* 2^15-1 */
/* 2^14-1 */
/*
 * #define SSL2_CHALLENGE_LENGTH 32
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl2_state_st {
    pub three_byte_header: libc::c_int,
    pub clear_text: libc::c_int,
    pub escape: libc::c_int,
    pub ssl2_rollback: libc::c_int,
    pub wnum: libc::c_uint,
    pub wpend_tot: libc::c_int,
    pub wpend_buf: *const libc::c_uchar,
    pub wpend_off: libc::c_int,
    pub wpend_len: libc::c_int,
    pub wpend_ret: libc::c_int,
    pub rbuf_left: libc::c_int,
    pub rbuf_offs: libc::c_int,
    pub rbuf: *mut libc::c_uchar,
    pub wbuf: *mut libc::c_uchar,
    pub write_ptr: *mut libc::c_uchar,
    pub padding: libc::c_uint,
    pub rlength: libc::c_uint,
    pub ract_data_length: libc::c_int,
    pub wlength: libc::c_uint,
    pub wact_data_length: libc::c_int,
    pub ract_data: *mut libc::c_uchar,
    pub wact_data: *mut libc::c_uchar,
    pub mac_data: *mut libc::c_uchar,
    pub read_key: *mut libc::c_uchar,
    pub write_key: *mut libc::c_uchar,
    pub challenge_length: libc::c_uint,
    pub challenge: [libc::c_uchar; 32],
    pub conn_id_length: libc::c_uint,
    pub conn_id: [libc::c_uchar; 16],
    pub key_material_length: libc::c_uint,
    pub key_material: [libc::c_uchar; 48],
    pub read_sequence: libc::c_ulong,
    pub write_sequence: libc::c_ulong,
    pub tmp: C2RustUnnamed_4,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub conn_id_length: libc::c_uint,
    pub cert_type: libc::c_uint,
    pub cert_length: libc::c_uint,
    pub csl: libc::c_uint,
    pub clear: libc::c_uint,
    pub enc: libc::c_uint,
    pub ccl: [libc::c_uchar; 32],
    pub cipher_spec_length: libc::c_uint,
    pub session_id_length: libc::c_uint,
    pub clen: libc::c_uint,
    pub rlen: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct PACKET {
    pub curr: *const libc::c_uchar,
    pub remaining: size_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub seq: libc::c_ulong,
    pub drop: libc::c_int,
}
/* Internal unchecked shorthand; don't use outside this file. */
#[inline]
unsafe extern "C" fn packet_forward(mut pkt: *mut PACKET, mut len: size_t) {
    (*pkt).curr = (*pkt).curr.offset(len as isize);
    (*pkt).remaining =
        ((*pkt).remaining as libc::c_ulong).wrapping_sub(len) as size_t as
            size_t;
}
/*
 * Returns the number of bytes remaining to be read in the PACKET
 */
#[inline]
unsafe extern "C" fn PACKET_remaining(mut pkt: *const PACKET) -> size_t {
    return (*pkt).remaining;
}
/*
 * Initialise a PACKET with |len| bytes held in |buf|. This does not make a
 * copy of the data so |buf| must be present for the whole time that the PACKET
 * is being used.
 */
#[inline]
unsafe extern "C" fn PACKET_buf_init(mut pkt: *mut PACKET,
                                     mut buf: *const libc::c_uchar,
                                     mut len: size_t) -> libc::c_int {
    /* Sanity check for negative values. */
    if len > 65536i32 as size_t { return 0i32 }
    (*pkt).curr = buf;
    (*pkt).remaining = len;
    return 1i32;
}
/*
 * Returns 1 if the packet has length |num| and its contents equal the |num|
 * bytes read from |ptr|. Returns 0 otherwise (lengths or contents not equal).
 * If lengths are equal, performs the comparison in constant time.
 */
#[inline]
unsafe extern "C" fn PACKET_equal(mut pkt: *const PACKET,
                                  mut ptr: *const libc::c_void,
                                  mut num: size_t) -> libc::c_int {
    if PACKET_remaining(pkt) != num { return 0i32 }
    return (CRYPTO_memcmp((*pkt).curr as *const libc::c_void, ptr, num) ==
                0i32) as libc::c_int;
}
/*
 * Peek ahead at 2 bytes in network order from |pkt| and store the value in
 * |*data|
 */
#[inline]
unsafe extern "C" fn PACKET_peek_net_2(mut pkt: *const PACKET,
                                       mut data: *mut libc::c_uint)
 -> libc::c_int {
    if PACKET_remaining(pkt) < 2i32 as libc::c_ulong { return 0i32 }
    *data = (*(*pkt).curr as libc::c_uint) << 8i32;
    *data |= *(*pkt).curr.offset(1) as libc::c_uint;
    return 1i32;
}
/* Equivalent of n2s */
/* Get 2 bytes in network order from |pkt| and store the value in |*data| */
#[inline]
unsafe extern "C" fn PACKET_get_net_2(mut pkt: *mut PACKET,
                                      mut data: *mut libc::c_uint)
 -> libc::c_int {
    if PACKET_peek_net_2(pkt, data) == 0 { return 0i32 }
    packet_forward(pkt, 2i32 as size_t);
    return 1i32;
}
/* Peek ahead at 1 byte from |pkt| and store the value in |*data| */
#[inline]
unsafe extern "C" fn PACKET_peek_1(mut pkt: *const PACKET,
                                   mut data: *mut libc::c_uint)
 -> libc::c_int {
    if PACKET_remaining(pkt) == 0 { return 0i32 }
    *data = *(*pkt).curr as libc::c_uint;
    return 1i32;
}
/* Get 1 byte from |pkt| and store the value in |*data| */
#[inline]
unsafe extern "C" fn PACKET_get_1(mut pkt: *mut PACKET,
                                  mut data: *mut libc::c_uint)
 -> libc::c_int {
    if PACKET_peek_1(pkt, data) == 0 { return 0i32 }
    packet_forward(pkt, 1i32 as size_t);
    return 1i32;
}
/*
 * Peek ahead at |len| bytes from the |pkt| and store a pointer to them in
 * |*data|. This just points at the underlying buffer that |pkt| is using. The
 * caller should not free this data directly (it will be freed when the
 * underlying buffer gets freed
 */
#[inline]
unsafe extern "C" fn PACKET_peek_bytes(mut pkt: *const PACKET,
                                       mut data: *mut *const libc::c_uchar,
                                       mut len: size_t) -> libc::c_int {
    if PACKET_remaining(pkt) < len { return 0i32 }
    *data = (*pkt).curr;
    return 1i32;
}
/*
 * Read |len| bytes from the |pkt| and store a pointer to them in |*data|. This
 * just points at the underlying buffer that |pkt| is using. The caller should
 * not free this data directly (it will be freed when the underlying buffer gets
 * freed
 */
#[inline]
unsafe extern "C" fn PACKET_get_bytes(mut pkt: *mut PACKET,
                                      mut data: *mut *const libc::c_uchar,
                                      mut len: size_t) -> libc::c_int {
    if PACKET_peek_bytes(pkt, data, len) == 0 { return 0i32 }
    packet_forward(pkt, len);
    return 1i32;
}
/* Peek ahead at |len| bytes from |pkt| and copy them to |data| */
#[inline]
unsafe extern "C" fn PACKET_peek_copy_bytes(mut pkt: *const PACKET,
                                            mut data: *mut libc::c_uchar,
                                            mut len: size_t) -> libc::c_int {
    if PACKET_remaining(pkt) < len { return 0i32 }
    memcpy(data as *mut libc::c_void, (*pkt).curr as *const libc::c_void,
           len);
    return 1i32;
}
/*
 * Read |len| bytes from |pkt| and copy them to |data|.
 * The caller is responsible for ensuring that |data| can hold |len| bytes.
 */
#[inline]
unsafe extern "C" fn PACKET_copy_bytes(mut pkt: *mut PACKET,
                                       mut data: *mut libc::c_uchar,
                                       mut len: size_t) -> libc::c_int {
    if PACKET_peek_copy_bytes(pkt, data, len) == 0 { return 0i32 }
    packet_forward(pkt, len);
    return 1i32;
}
/* Move the current reading position forward |len| bytes */
#[inline]
unsafe extern "C" fn PACKET_forward(mut pkt: *mut PACKET, mut len: size_t)
 -> libc::c_int {
    if PACKET_remaining(pkt) < len { return 0i32 }
    packet_forward(pkt, len);
    return 1i32;
}
/*
 * Reads a variable-length vector prefixed with a one-byte length, and stores
 * the contents in |subpkt|. |pkt| can equal |subpkt|.
 * Data is not copied: the |subpkt| packet will share its underlying buffer with
 * the original |pkt|, so data wrapped by |pkt| must outlive the |subpkt|.
 * Upon failure, the original |pkt| and |subpkt| are not modified.
 */
#[inline]
unsafe extern "C" fn PACKET_get_length_prefixed_1(mut pkt: *mut PACKET,
                                                  mut subpkt: *mut PACKET)
 -> libc::c_int {
    let mut length: libc::c_uint = 0;
    let mut data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut tmp: PACKET = *pkt;
    if PACKET_get_1(&mut tmp, &mut length) == 0 ||
           PACKET_get_bytes(&mut tmp, &mut data, length as size_t) == 0 {
        return 0i32
    }
    *pkt = tmp;
    (*subpkt).curr = data;
    (*subpkt).remaining = length as size_t;
    return 1i32;
}
static mut client_random: [libc::c_uchar; 32] = [0; 32];
static mut server_random: [libc::c_uchar; 32] = [0; 32];
/* These are all generated locally, sized purely according to our own whim */
static mut session_id: [libc::c_uchar; 32] = [0; 32];
static mut master_secret: [libc::c_uchar; 48] = [0; 48];
static mut cookie: [libc::c_uchar; 20] = [0; 20];
/* We've hard-coded the cipher suite; we know it's 104 bytes */
static mut key_block: [libc::c_uchar; 104] = [0; 104];
static mut handshake_md5: *mut EVP_MD_CTX =
    0 as *const EVP_MD_CTX as *mut EVP_MD_CTX;
static mut handshake_sha1: *mut EVP_MD_CTX =
    0 as *const EVP_MD_CTX as *mut EVP_MD_CTX;
#[inline]
unsafe extern "C" fn HMAC_CTX_new() -> *mut HMAC_CTX {
    let mut ret: *mut HMAC_CTX =
        malloc(::std::mem::size_of::<HMAC_CTX>() as libc::c_ulong) as
            *mut HMAC_CTX;
    HMAC_CTX_init(ret);
    return ret;
}
#[inline]
unsafe extern "C" fn HMAC_CTX_free(mut ctx: *mut HMAC_CTX) {
    HMAC_CTX_cleanup(ctx);
    free(ctx as *mut libc::c_void);
}
unsafe extern "C" fn tls1_P_hash(mut md: *const EVP_MD,
                                 mut sec: *const libc::c_uchar,
                                 mut sec_len: libc::c_int,
                                 mut seed1: *const libc::c_void,
                                 mut seed1_len: libc::c_int,
                                 mut seed2: *const libc::c_void,
                                 mut seed2_len: libc::c_int,
                                 mut seed3: *const libc::c_void,
                                 mut seed3_len: libc::c_int,
                                 mut out: *mut libc::c_uchar,
                                 mut olen: libc::c_int) -> libc::c_int {
    let mut A1: [libc::c_uchar; 64] = [0; 64];
    let mut ctx: *mut HMAC_CTX = HMAC_CTX_new();
    let mut chunk: libc::c_uint = 0;
    let mut i: libc::c_int = 0i32;
    HMAC_Init_ex(ctx, sec as *const libc::c_void, sec_len, md,
                 0 as *mut ENGINE);
    loop  {
        if i != 0 { HMAC_Update(ctx, A1.as_mut_ptr(), chunk as size_t); }
        if !seed1.is_null() {
            HMAC_Update(ctx, seed1 as *const libc::c_uchar,
                        seed1_len as size_t);
        }
        if !seed2.is_null() {
            HMAC_Update(ctx, seed2 as *const libc::c_uchar,
                        seed2_len as size_t);
        }
        if !seed3.is_null() {
            HMAC_Update(ctx, seed3 as *const libc::c_uchar,
                        seed3_len as size_t);
        }
        /* First generate A1 from the seed */
        if i == 0 {
            HMAC_Final(ctx, A1.as_mut_ptr(), &mut chunk);
        } else if (i as libc::c_uint).wrapping_mul(chunk) <=
                      olen as libc::c_uint {
            HMAC_Final(ctx,
                       out.offset(((i - 1i32) as
                                       libc::c_uint).wrapping_mul(chunk) as
                                      isize), 0 as *mut libc::c_uint);
            /* calculate A(n+1) value */
            HMAC(md, sec as *const libc::c_void, sec_len, A1.as_mut_ptr(),
                 chunk as size_t, A1.as_mut_ptr(), 0 as *mut libc::c_uint);
        } else {
            HMAC_Final(ctx, A1.as_mut_ptr(), 0 as *mut libc::c_uint);
            memcpy(out.offset(((i - 1i32) as libc::c_uint).wrapping_mul(chunk)
                                  as isize) as *mut libc::c_void,
                   A1.as_mut_ptr() as *const libc::c_void,
                   (olen as libc::c_uint).wrapping_rem(chunk) as
                       libc::c_ulong);
            break ;
        }
        HMAC_Init_ex(ctx, 0 as *const libc::c_void, 0i32, 0 as *const EVP_MD,
                     0 as *mut ENGINE);
        i += 1;
        if !(((i - 1i32) as libc::c_uint).wrapping_mul(chunk) <=
                 olen as libc::c_uint) {
            break ;
        }
    }
    HMAC_CTX_free(ctx);
    return 1i32;
}
/* seed1 through seed5 are virtually concatenated */
unsafe extern "C" fn do_PRF(mut seed1: *const libc::c_void,
                            mut seed1_len: libc::c_int,
                            mut seed2: *const libc::c_void,
                            mut seed2_len: libc::c_int,
                            mut seed3: *const libc::c_void,
                            mut seed3_len: libc::c_int,
                            mut out: *mut libc::c_uchar,
                            mut olen: libc::c_int) -> libc::c_int {
    let mut out2: [libc::c_uchar; 104] = [0; 104];
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if olen >
           ::std::mem::size_of::<[libc::c_uchar; 104]>() as libc::c_ulong as
               libc::c_int {
        return 0i32
    }
    len =
        (::std::mem::size_of::<[libc::c_uchar; 48]>() as
             libc::c_ulong).wrapping_div(2i32 as libc::c_ulong) as
            libc::c_int;
    if tls1_P_hash(EVP_md5(), master_secret.as_mut_ptr(), len, seed1,
                   seed1_len, seed2, seed2_len, seed3, seed3_len, out, olen)
           == 0 ||
           tls1_P_hash(EVP_sha1(),
                       master_secret.as_mut_ptr().offset(len as isize), len,
                       seed1, seed1_len, seed2, seed2_len, seed3, seed3_len,
                       out2.as_mut_ptr(), olen) == 0 {
        return 0i32
    }
    i = 0i32;
    while i < olen {
        let ref mut fresh0 = *out.offset(i as isize);
        *fresh0 =
            (*fresh0 as libc::c_int ^ out2[i as usize] as libc::c_int) as
                libc::c_uchar;
        i += 1
    }
    return 1i32;
}
unsafe extern "C" fn client_session() -> *mut SSL_SESSION {
    static mut session_asn1: [libc::c_uchar; 97] =
        [0x30i32 as libc::c_uchar, 0x5fi32 as libc::c_uchar,
         0x2i32 as libc::c_uchar, 0x1i32 as libc::c_uchar,
         0x1i32 as libc::c_uchar, 0x2i32 as libc::c_uchar,
         0x2i32 as libc::c_uchar, 0x1i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0x4i32 as libc::c_uchar,
         0x2i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x2fi32 as libc::c_uchar, 0x4i32 as libc::c_uchar,
         0x20i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0x4i32 as libc::c_uchar,
         0x30i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar];
    let mut p: *const libc::c_uchar = session_asn1.as_mut_ptr();
    /* Copy the randomly-generated fields into the above ASN1 */
    memcpy(session_asn1.as_mut_ptr().offset(15) as *mut libc::c_void,
           session_id.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong);
    memcpy(session_asn1.as_mut_ptr().offset(49) as *mut libc::c_void,
           master_secret.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_uchar; 48]>() as libc::c_ulong);
    return d2i_SSL_SESSION(0 as *mut *mut SSL_SESSION, &mut p,
                           ::std::mem::size_of::<[libc::c_uchar; 97]>() as
                               libc::c_ulong as libc::c_long);
}
/* Returns 1 for initial ClientHello, 2 for ClientHello with cookie */
unsafe extern "C" fn validate_client_hello(mut wbio: *mut BIO)
 -> libc::c_int {
    let mut pkt: PACKET =
        PACKET{curr: 0 as *const libc::c_uchar, remaining: 0,};
    let mut pkt2: PACKET =
        PACKET{curr: 0 as *const libc::c_uchar, remaining: 0,};
    let mut len: libc::c_long = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cookie_found: libc::c_int = 0i32;
    let mut u: libc::c_uint = 0;
    len =
        BIO_ctrl(wbio, 3i32, 0i32 as libc::c_long,
                 &mut data as *mut *mut libc::c_uchar as
                     *mut *mut libc::c_char as *mut libc::c_char as
                     *mut libc::c_void);
    if PACKET_buf_init(&mut pkt, data, len as size_t) == 0 { return 0i32 }
    /* Check record header type */
    if PACKET_get_1(&mut pkt, &mut u) == 0 || u != 22i32 as libc::c_uint {
        return 0i32
    }
    /* Version */
    if PACKET_get_net_2(&mut pkt, &mut u) == 0 ||
           u != 0x100i32 as libc::c_uint {
        return 0i32
    }
    /* Skip the rest of the record header */
    if PACKET_forward(&mut pkt, (13i32 - 3i32) as size_t) == 0 { return 0i32 }
    /* Check it's a ClientHello */
    if PACKET_get_1(&mut pkt, &mut u) == 0 || u != 1i32 as libc::c_uint {
        return 0i32
    }
    /* Skip the rest of the handshake message header */
    if PACKET_forward(&mut pkt, (12i32 - 1i32) as size_t) == 0 { return 0i32 }
    /* Check client version */
    if PACKET_get_net_2(&mut pkt, &mut u) == 0 ||
           u != 0x100i32 as libc::c_uint {
        return 0i32
    }
    /* Store random */
    if PACKET_copy_bytes(&mut pkt, client_random.as_mut_ptr(),
                         32i32 as size_t) == 0 {
        return 0i32
    }
    /* Check session id length and content */
    if PACKET_get_length_prefixed_1(&mut pkt, &mut pkt2) == 0 ||
           PACKET_equal(&mut pkt2,
                        session_id.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                            libc::c_ulong) == 0 {
        return 0i32
    }
    /* Check cookie */
    if PACKET_get_length_prefixed_1(&mut pkt, &mut pkt2) == 0 { return 0i32 }
    if PACKET_remaining(&mut pkt2) != 0 {
        if PACKET_equal(&mut pkt2, cookie.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[libc::c_uchar; 20]>() as
                            libc::c_ulong) == 0 {
            return 0i32
        }
        cookie_found = 1i32
    }
    /* Skip ciphers */
    if PACKET_get_net_2(&mut pkt, &mut u) == 0 ||
           PACKET_forward(&mut pkt, u as size_t) == 0 {
        return 0i32
    }
    /* Skip compression */
    if PACKET_get_1(&mut pkt, &mut u) == 0 ||
           PACKET_forward(&mut pkt, u as size_t) == 0 {
        return 0i32
    }
    /* Skip extensions */
    if PACKET_get_net_2(&mut pkt, &mut u) == 0 ||
           PACKET_forward(&mut pkt, u as size_t) == 0 {
        return 0i32
    }
    /* Now we are at the end */
    if PACKET_remaining(&mut pkt) != 0 { return 0i32 }
    /* Update handshake MAC for second ClientHello (with cookie) */
    if cookie_found != 0 &&
           (EVP_DigestUpdate(handshake_md5,
                             data.offset((13i32 + 12i32) as isize) as
                                 *const libc::c_void,
                             (len - (13i32 + 12i32) as libc::c_long) as
                                 size_t) == 0 ||
                EVP_DigestUpdate(handshake_sha1,
                                 data.offset((13i32 + 12i32) as isize) as
                                     *const libc::c_void,
                                 (len - (13i32 + 12i32) as libc::c_long) as
                                     size_t) == 0) {
        printf(b"EVP_DigestUpdate() failed\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    BIO_ctrl(wbio, 1i32, 0i32 as libc::c_long, 0 as *mut libc::c_void);
    return 1i32 + cookie_found;
}
unsafe extern "C" fn send_hello_verify(mut rbio: *mut BIO) -> libc::c_int {
    static mut hello_verify: [libc::c_uchar; 48] =
        [0x16i32 as libc::c_uchar, 0x1i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0x23i32 as libc::c_uchar,
         0x3i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0x17i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0x17i32 as libc::c_uchar,
         0x1i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x14i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar];
    memcpy(hello_verify.as_mut_ptr().offset(28) as *mut libc::c_void,
           cookie.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong);
    BIO_write(rbio, hello_verify.as_mut_ptr() as *const libc::c_void,
              ::std::mem::size_of::<[libc::c_uchar; 48]>() as libc::c_ulong as
                  libc::c_int);
    return 1i32;
}
unsafe extern "C" fn send_server_hello(mut rbio: *mut BIO) -> libc::c_int {
    static mut server_hello: [libc::c_uchar; 95] =
        [0x16i32 as libc::c_uchar, 0x1i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x1i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x52i32 as libc::c_uchar, 0x2i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x46i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x1i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0x46i32 as libc::c_uchar,
         0x1i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x20i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x2fi32 as libc::c_uchar, 0i32 as libc::c_uchar];
    static mut change_cipher_spec: [libc::c_uchar; 16] =
        [0x14i32 as libc::c_uchar, 0x1i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x2i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0x3i32 as libc::c_uchar, 0x1i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0x2i32 as libc::c_uchar];
    memcpy(server_hello.as_mut_ptr().offset(27) as *mut libc::c_void,
           server_random.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong);
    memcpy(server_hello.as_mut_ptr().offset(60) as *mut libc::c_void,
           session_id.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong);
    if EVP_DigestUpdate(handshake_md5,
                        server_hello.as_mut_ptr().offset((13i32 + 12i32) as
                                                             isize) as
                            *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_uchar; 95]>() as
                             libc::c_ulong).wrapping_sub((13i32 + 12i32) as
                                                             libc::c_ulong))
           == 0 ||
           EVP_DigestUpdate(handshake_sha1,
                            server_hello.as_mut_ptr().offset((13i32 + 12i32)
                                                                 as isize) as
                                *const libc::c_void,
                            (::std::mem::size_of::<[libc::c_uchar; 95]>() as
                                 libc::c_ulong).wrapping_sub((13i32 + 12i32)
                                                                 as
                                                                 libc::c_ulong))
               == 0 {
        printf(b"EVP_DigestUpdate() failed\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    BIO_write(rbio, server_hello.as_mut_ptr() as *const libc::c_void,
              ::std::mem::size_of::<[libc::c_uchar; 95]>() as libc::c_ulong as
                  libc::c_int);
    BIO_write(rbio, change_cipher_spec.as_mut_ptr() as *const libc::c_void,
              ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong as
                  libc::c_int);
    return 1i32;
}
/* Create header, HMAC, pad, encrypt and send a record */
unsafe extern "C" fn send_record(mut rbio: *mut BIO,
                                 mut type_0: libc::c_uchar,
                                 mut seqnr: libc::c_ulong,
                                 mut msg: *const libc::c_void,
                                 mut len: size_t) -> libc::c_int {
    /* Note that the order of the record header fields on the wire,
     * and in the HMAC, is different. So we just keep them in separate
     * variables and handle them individually. */
    static mut epoch: [libc::c_uchar; 2] =
        [0i32 as libc::c_uchar, 0x1i32 as libc::c_uchar]; /* DTLS1_BAD_VER */
    static mut seq: [libc::c_uchar; 6] =
        [0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar];
    static mut ver: [libc::c_uchar; 2] =
        [0x1i32 as libc::c_uchar, 0i32 as libc::c_uchar];
    let mut lenbytes: [libc::c_uchar; 2] = [0; 2];
    let mut ctx: *mut HMAC_CTX = HMAC_CTX_new();
    let mut enc_ctx: *mut EVP_CIPHER_CTX = EVP_CIPHER_CTX_new();
    let mut iv: [libc::c_uchar; 16] = [0; 16];
    let mut pad: libc::c_uchar = 0;
    let mut enc: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    seq[0] = (seqnr >> 40i32 & 0xffi32 as libc::c_ulong) as libc::c_uchar;
    seq[1] = (seqnr >> 32i32 & 0xffi32 as libc::c_ulong) as libc::c_uchar;
    seq[2] = (seqnr >> 24i32 & 0xffi32 as libc::c_ulong) as libc::c_uchar;
    seq[3] = (seqnr >> 16i32 & 0xffi32 as libc::c_ulong) as libc::c_uchar;
    seq[4] = (seqnr >> 8i32 & 0xffi32 as libc::c_ulong) as libc::c_uchar;
    seq[5] = (seqnr & 0xffi32 as libc::c_ulong) as libc::c_uchar;
    pad =
        (15i32 as
             libc::c_ulong).wrapping_sub(len.wrapping_add(20i32 as
                                                              libc::c_ulong).wrapping_rem(16i32
                                                                                              as
                                                                                              libc::c_ulong))
            as libc::c_uchar;
    enc =
        CRYPTO_malloc(len as libc::c_int + 20i32 + 1i32 + pad as libc::c_int,
                      b"bad_dtls_test.c\x00" as *const u8 as
                          *const libc::c_char, 595i32) as *mut libc::c_uchar;
    if enc.is_null() { return 0i32 }
    /* Copy record to encryption buffer */
    memcpy(enc as *mut libc::c_void, msg, len);
    /* Append HMAC to data */
    HMAC_Init_ex(ctx,
                 key_block.as_mut_ptr().offset(20) as *const libc::c_void,
                 20i32, EVP_sha1(), 0 as *mut ENGINE); /* Version */
    HMAC_Update(ctx, epoch.as_mut_ptr(), 2i32 as size_t); /* Length */
    HMAC_Update(ctx, seq.as_mut_ptr(),
                6i32 as size_t); /* Finally the data itself */
    HMAC_Update(ctx, &mut type_0, 1i32 as size_t);
    HMAC_Update(ctx, ver.as_mut_ptr(), 2i32 as size_t);
    lenbytes[0] = (len >> 8i32) as libc::c_uchar;
    lenbytes[1] = (len & 0xffi32 as libc::c_ulong) as libc::c_uchar;
    HMAC_Update(ctx, lenbytes.as_mut_ptr(), 2i32 as size_t);
    HMAC_Update(ctx, enc, len);
    HMAC_Final(ctx, enc.offset(len as isize), 0 as *mut libc::c_uint);
    HMAC_CTX_free(ctx);
    /* Append padding bytes */
    len =
        (len as libc::c_ulong).wrapping_add(20i32 as libc::c_ulong) as size_t
            as size_t;
    loop  {
        let fresh1 = len;
        len = len.wrapping_add(1);
        *enc.offset(fresh1 as isize) = pad;
        if !(len.wrapping_rem(16i32 as libc::c_ulong) != 0) { break ; }
    }
    /* Generate IV, and encrypt */
    RAND_bytes(iv.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong
                   as libc::c_int);
    //    EVP_CIPHER_CTX_init(enc_ctx);
    EVP_CipherInit_ex(enc_ctx, EVP_aes_128_cbc(), 0 as *mut ENGINE,
                      key_block.as_mut_ptr().offset(56), iv.as_mut_ptr(),
                      1i32);
    EVP_Cipher(enc_ctx, enc, enc, len as libc::c_uint);
    EVP_CIPHER_CTX_free(enc_ctx);
    /* Finally write header (from fragmented variables), IV and encrypted record */
    BIO_write(rbio, &mut type_0 as *mut libc::c_uchar as *const libc::c_void,
              1i32);
    BIO_write(rbio, ver.as_mut_ptr() as *const libc::c_void, 2i32);
    BIO_write(rbio, epoch.as_mut_ptr() as *const libc::c_void, 2i32);
    BIO_write(rbio, seq.as_mut_ptr() as *const libc::c_void, 6i32);
    lenbytes[0] =
        (len.wrapping_add(::std::mem::size_of::<[libc::c_uchar; 16]>() as
                              libc::c_ulong) >> 8i32) as libc::c_uchar;
    lenbytes[1] =
        (len.wrapping_add(::std::mem::size_of::<[libc::c_uchar; 16]>() as
                              libc::c_ulong) & 0xffi32 as libc::c_ulong) as
            libc::c_uchar;
    BIO_write(rbio, lenbytes.as_mut_ptr() as *const libc::c_void, 2i32);
    BIO_write(rbio, iv.as_mut_ptr() as *const libc::c_void,
              ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong as
                  libc::c_int);
    BIO_write(rbio, enc as *const libc::c_void, len as libc::c_int);
    CRYPTO_free(enc as *mut libc::c_void);
    return 1i32;
}
unsafe extern "C" fn send_finished(mut s: *mut SSL, mut rbio: *mut BIO)
 -> libc::c_int {
    static mut finished_msg: [libc::c_uchar; 24] =
        [0x14i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0xci32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0x3i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0xci32 as libc::c_uchar, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut handshake_hash: [libc::c_uchar; 128] = [0; 128];
    /* Derive key material */
    do_PRF(b"key expansion\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void, 13i32,
           server_random.as_mut_ptr() as *const libc::c_void, 32i32,
           client_random.as_mut_ptr() as *const libc::c_void, 32i32,
           key_block.as_mut_ptr(),
           ::std::mem::size_of::<[libc::c_uchar; 104]>() as libc::c_ulong as
               libc::c_int);
    /* Generate Finished MAC */
    if EVP_DigestFinal_ex(handshake_md5, handshake_hash.as_mut_ptr(),
                          0 as *mut libc::c_uint) == 0 ||
           EVP_DigestFinal_ex(handshake_sha1,
                              handshake_hash.as_mut_ptr().offset(EVP_MD_size(EVP_MD_CTX_md(handshake_md5))
                                                                     as
                                                                     isize),
                              0 as *mut libc::c_uint) == 0 {
        printf(b"EVP_DigestFinal_ex() failed\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    do_PRF(b"server finished\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void, 15i32,
           handshake_hash.as_mut_ptr() as *const libc::c_void,
           EVP_MD_size(EVP_MD_CTX_md(handshake_md5)) +
               EVP_MD_size(EVP_MD_CTX_md(handshake_sha1)),
           0 as *const libc::c_void, 0i32,
           finished_msg.as_mut_ptr().offset(12), 12i32);
    return send_record(rbio, 22i32 as libc::c_uchar, 0i32 as libc::c_ulong,
                       finished_msg.as_mut_ptr() as *const libc::c_void,
                       ::std::mem::size_of::<[libc::c_uchar; 24]>() as
                           libc::c_ulong);
}
unsafe extern "C" fn validate_ccs(mut wbio: *mut BIO) -> libc::c_int {
    let mut pkt: PACKET =
        PACKET{curr: 0 as *const libc::c_uchar, remaining: 0,};
    let mut len: libc::c_long = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut u: libc::c_uint = 0;
    len =
        BIO_ctrl(wbio, 3i32, 0i32 as libc::c_long,
                 &mut data as *mut *mut libc::c_uchar as
                     *mut *mut libc::c_char as *mut libc::c_char as
                     *mut libc::c_void);
    if PACKET_buf_init(&mut pkt, data, len as size_t) == 0 { return 0i32 }
    /* Check record header type */
    if PACKET_get_1(&mut pkt, &mut u) == 0 || u != 20i32 as libc::c_uint {
        return 0i32
    }
    /* Version */
    if PACKET_get_net_2(&mut pkt, &mut u) == 0 ||
           u != 0x100i32 as libc::c_uint {
        return 0i32
    }
    /* Skip the rest of the record header */
    if PACKET_forward(&mut pkt, (13i32 - 3i32) as size_t) == 0 { return 0i32 }
    /* Check ChangeCipherSpec message */
    if PACKET_get_1(&mut pkt, &mut u) == 0 || u != 1i32 as libc::c_uint {
        return 0i32
    }
    /* A DTLS1_BAD_VER ChangeCipherSpec also contains the
     * handshake sequence number (which is 2 here) */
    if PACKET_get_net_2(&mut pkt, &mut u) == 0 || u != 0x2i32 as libc::c_uint
       {
        return 0i32
    }
    /* Now check the Finished packet */
    if PACKET_get_1(&mut pkt, &mut u) == 0 || u != 22i32 as libc::c_uint {
        return 0i32
    }
    if PACKET_get_net_2(&mut pkt, &mut u) == 0 ||
           u != 0x100i32 as libc::c_uint {
        return 0i32
    }
    /* Check epoch is now 1 */
    if PACKET_get_net_2(&mut pkt, &mut u) == 0 || u != 0x1i32 as libc::c_uint
       {
        return 0i32
    }
    /* That'll do for now. If OpenSSL accepted *our* Finished packet
     * then it's evidently remembered that DTLS1_BAD_VER doesn't
     * include the handshake header in the MAC. There's not a lot of
     * point in implementing decryption here, just to check that it
     * continues to get it right for one more packet. */
    return 1i32;
}
static mut tests: [C2RustUnnamed_5; 35] =
    [{ let mut init = C2RustUnnamed_5{seq: 1u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 3u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 2u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x1234u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x1230u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x1235u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0xffffu64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x10001u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0xfffeu64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x10000u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x10001u64, drop: 1i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0xffu64, drop: 1i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x100000u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x800000u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x7fffe1u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0xffffffu64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x1000000u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0xfffffeu64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0xffffffu64, drop: 1i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x1000010u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0xfffffdu64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x1000011u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x12u64, drop: 1i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x1000012u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x1ffffffu64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x2000000u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x1ff00feu64, drop: 1i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x2000001u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x20fffffu64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x2105500u64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x20ffffeu64, drop: 1i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x21054ffu64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x211ffffu64, drop: 0i32,}; init },
     { let mut init = C2RustUnnamed_5{seq: 0x2110000u64, drop: 1i32,}; init },
     {
         let mut init = C2RustUnnamed_5{seq: 0x2120000u64, drop: 0i32,};
         init
     }];
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut sess: *mut SSL_SESSION = 0 as *mut SSL_SESSION;
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut con: *mut SSL = 0 as *mut SSL;
    let mut rbio: *mut BIO = 0 as *mut BIO;
    let mut wbio: *mut BIO = 0 as *mut BIO;
    let mut testresult: libc::c_int = 0i32;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    SSL_library_init();
    SSL_load_error_strings();
    RAND_bytes(session_id.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong
                   as libc::c_int);
    RAND_bytes(master_secret.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_uchar; 48]>() as libc::c_ulong
                   as libc::c_int);
    RAND_bytes(cookie.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong
                   as libc::c_int);
    RAND_bytes(server_random.as_mut_ptr().offset(4),
               (::std::mem::size_of::<[libc::c_uchar; 32]>() as
                    libc::c_ulong).wrapping_sub(4i32 as libc::c_ulong) as
                   libc::c_int);
    time(server_random.as_mut_ptr() as *mut libc::c_void as *mut time_t);
    sess = client_session();
    if sess.is_null() {
        printf(b"Failed to generate SSL_SESSION\n\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        handshake_md5 = EVP_MD_CTX_create();
        handshake_sha1 = EVP_MD_CTX_create();
        if EVP_DigestInit_ex(handshake_md5, EVP_md5(), 0 as *mut ENGINE) == 0
               ||
               EVP_DigestInit_ex(handshake_sha1, EVP_sha1(), 0 as *mut ENGINE)
                   == 0 {
            printf(b"Failed to initialise handshake_md\n\x00" as *const u8 as
                       *const libc::c_char);
        } else {
            ctx = SSL_CTX_new(DTLSv1_client_method());
            if ctx.is_null() {
                printf(b"Failed to allocate SSL_CTX\n\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                SSL_CTX_ctrl(ctx, 32i32, 0x8000i64, 0 as *mut libc::c_void);
                if SSL_CTX_set_cipher_list(ctx,
                                           b"AES128-SHA\x00" as *const u8 as
                                               *const libc::c_char) == 0 {
                    printf(b"SSL_CTX_set_cipher_list() failed\n\x00" as
                               *const u8 as *const libc::c_char);
                } else {
                    con = SSL_new(ctx);
                    if SSL_set_session(con, sess) == 0 {
                        printf(b"SSL_set_session() failed\n\x00" as *const u8
                                   as *const libc::c_char);
                    } else {
                        SSL_SESSION_free(sess);
                        rbio = BIO_new(BIO_s_mem());
                        wbio = BIO_new(BIO_s_mem());
                        BIO_ctrl(rbio, 102i32, 1i32 as libc::c_long,
                                 0 as *mut libc::c_void);
                        BIO_ctrl(wbio, 102i32, 1i32 as libc::c_long,
                                 0 as *mut libc::c_void);
                        SSL_set_bio(con, rbio, wbio);
                        SSL_set_connect_state(con);
                        /* Send initial ClientHello */
                        ret = SSL_do_handshake(con);
                        if ret > 0i32 || SSL_get_error(con, ret) != 2i32 {
                            printf(b"Unexpected handshake result at initial call!\n\x00"
                                       as *const u8 as *const libc::c_char);
                        } else if validate_client_hello(wbio) != 1i32 {
                            printf(b"Initial ClientHello failed validation\n\x00"
                                       as *const u8 as *const libc::c_char);
                        } else if send_hello_verify(rbio) != 1i32 {
                            printf(b"Failed to send HelloVerify\n\x00" as
                                       *const u8 as *const libc::c_char);
                        } else {
                            ret = SSL_do_handshake(con);
                            if ret > 0i32 || SSL_get_error(con, ret) != 2i32 {
                                printf(b"Unexpected handshake result after HelloVerify!\n\x00"
                                           as *const u8 as
                                           *const libc::c_char);
                            } else if validate_client_hello(wbio) != 2i32 {
                                printf(b"Second ClientHello failed validation\n\x00"
                                           as *const u8 as
                                           *const libc::c_char);
                            } else if send_server_hello(rbio) != 1i32 {
                                printf(b"Failed to send ServerHello\n\x00" as
                                           *const u8 as *const libc::c_char);
                            } else {
                                ret = SSL_do_handshake(con);
                                if ret > 0i32 ||
                                       SSL_get_error(con, ret) != 2i32 {
                                    printf(b"Unexpected handshake result after ServerHello!\n\x00"
                                               as *const u8 as
                                               *const libc::c_char);
                                } else if send_finished(con, rbio) != 1i32 {
                                    printf(b"Failed to send Finished\n\x00" as
                                               *const u8 as
                                               *const libc::c_char);
                                } else {
                                    ret = SSL_do_handshake(con);
                                    if ret < 1i32 {
                                        printf(b"Handshake not successful after Finished!\n\x00"
                                                   as *const u8 as
                                                   *const libc::c_char);
                                    } else if validate_ccs(wbio) != 1i32 {
                                        printf(b"Failed to validate client CCS/Finished\n\x00"
                                                   as *const u8 as
                                                   *const libc::c_char);
                                    } else {
                                        /* While we're here and crafting packets by hand, we might as well do a
       bit of a stress test on the DTLS record replay handling. Not Cisco-DTLS
       specific but useful anyway for the general case. It's been broken
       before, and in fact was broken even for a basic 0, 2, 1 test case
       when this test was first added.... */
                                        i = 0i32;
                                        loop  {
                                            if !(i <
                                                     (::std::mem::size_of::<[C2RustUnnamed_5; 35]>()
                                                          as
                                                          libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_5>()
                                                                                          as
                                                                                          libc::c_ulong)
                                                         as libc::c_int) {
                                                current_block =
                                                    11796148217846552555;
                                                break ;
                                            }
                                            let mut recv_buf:
                                                    [libc::c_ulong; 2] =
                                                [0; 2];
                                            if send_record(rbio,
                                                           23i32 as
                                                               libc::c_uchar,
                                                           tests[i as
                                                                     usize].seq,
                                                           &mut (*tests.as_mut_ptr().offset(i
                                                                                                as
                                                                                                isize)).seq
                                                               as
                                                               *mut libc::c_ulong
                                                               as
                                                               *const libc::c_void,
                                                           ::std::mem::size_of::<libc::c_ulong>()
                                                               as
                                                               libc::c_ulong)
                                                   != 1i32 {
                                                printf(b"Failed to send data seq #0x%lx (%d)\n\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       tests[i as usize].seq,
                                                       i);
                                                current_block =
                                                    12503479768206912604;
                                                break ;
                                            } else {
                                                if !(tests[i as usize].drop !=
                                                         0) {
                                                    ret =
                                                        SSL_read(con,
                                                                 recv_buf.as_mut_ptr()
                                                                     as
                                                                     *mut libc::c_void,
                                                                 (2i32 as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ulong>()
                                                                                                      as
                                                                                                      libc::c_ulong)
                                                                     as
                                                                     libc::c_int);
                                                    if ret as libc::c_ulong !=
                                                           ::std::mem::size_of::<libc::c_ulong>()
                                                               as
                                                               libc::c_ulong {
                                                        printf(b"SSL_read failed or wrong size on seq#0x%lx (%d)\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               tests[i as
                                                                         usize].seq,
                                                               i);
                                                        current_block =
                                                            12503479768206912604;
                                                        break ;
                                                    } else if recv_buf[0] !=
                                                                  tests[i as
                                                                            usize].seq
                                                     {
                                                        printf(b"Wrong data packet received (0x%lx not 0x%lx) at packet %d\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               recv_buf[0],
                                                               tests[i as
                                                                         usize].seq,
                                                               i);
                                                        current_block =
                                                            12503479768206912604;
                                                        break ;
                                                    }
                                                }
                                                i += 1
                                            }
                                        }
                                        match current_block {
                                            12503479768206912604 => { }
                                            _ => {
                                                if tests[(i - 1i32) as
                                                             usize].drop != 0
                                                   {
                                                    printf(b"Error: last test cannot be DROP()\n\x00"
                                                               as *const u8 as
                                                               *const libc::c_char);
                                                } else { testresult = 1i32 }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    SSL_free(con);
                }
                SSL_CTX_free(ctx);
            }
            EVP_MD_CTX_destroy(handshake_md5);
            EVP_MD_CTX_destroy(handshake_sha1);
        }
    }
    ERR_print_errors_fp(__stderrp);
    if testresult == 0 {
        printf(b"Cisco BadDTLS test: FAILED\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    ERR_free_strings();
    EVP_cleanup();
    return if testresult != 0 { 0i32 } else { 1i32 };
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
