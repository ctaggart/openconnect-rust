#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
}
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
pub type uint8_t = libc::c_uchar;
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
pub type uint32_t = libc::c_uint;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct LZOContext {
    pub in_0: *const uint8_t,
    pub in_end: *const uint8_t,
    pub out_start: *mut uint8_t,
    pub out: *mut uint8_t,
    pub out_end: *mut uint8_t,
    pub error: libc::c_int,
}
/* *
 * @}
 */
#[derive ( Copy, Clone )]
#[repr(C, packed)]
pub struct lzo_packed_uint32 {
    pub d: uint32_t,
}
#[inline]
unsafe extern "C" fn av_memcpy_backptr(mut dst: *mut libc::c_uchar,
                                       mut back: libc::c_int,
                                       mut cnt: libc::c_int) {
    loop  {
        let fresh0 = cnt;
        cnt = cnt - 1;
        if !(fresh0 != 0) { break ; }
        *dst = *dst.offset(-(back as isize));
        dst = dst.offset(1)
    };
}
/* *
 * @brief Reads one byte from the input buffer, avoiding an overrun.
 * @return byte read
 */
#[inline]
unsafe extern "C" fn get_byte(mut c: *mut LZOContext) -> libc::c_int {
    if (*c).in_0 < (*c).in_end {
        let fresh1 = (*c).in_0;
        (*c).in_0 = (*c).in_0.offset(1);
        return *fresh1 as libc::c_int
    }
    (*c).error |= 1i32;
    return 1i32;
}
/* *
 * @brief Decodes a length value in the coding used by lzo.
 * @param x previous byte value
 * @param mask bits used from x
 * @return decoded length value
 */
#[inline]
unsafe extern "C" fn get_len(mut c: *mut LZOContext, mut x: libc::c_int,
                             mut mask: libc::c_int) -> libc::c_int {
    let mut cnt: libc::c_int = x & mask;
    if cnt == 0 {
        loop  {
            x = get_byte(c);
            if !(x == 0) { break ; }
            if cnt >= 65535i32 {
                (*c).error |= 8i32;
                break ;
            } else { cnt += 255i32 }
        }
        cnt += mask + x
    }
    return cnt;
}
/* *
 * @brief Copies bytes from input to output buffer with checking.
 * @param cnt number of bytes to copy, must be >= 0
 */
#[inline]
unsafe extern "C" fn copy(mut c: *mut LZOContext, mut cnt: libc::c_int) {
    let mut src: *const uint8_t = (*c).in_0;
    let mut dst: *mut uint8_t = (*c).out;
    /* Should never happen */
    if cnt < 0i32 { (*c).error |= 8i32; return }
    if cnt as libc::c_long >
           (*c).in_end.wrapping_offset_from(src) as libc::c_long {
        cnt =
            ({
                 let mut _x: libc::c_long =
                     (*c).in_end.wrapping_offset_from(src) as libc::c_long;
                 let mut _y: libc::c_int = 0i32;
                 if _x > _y as libc::c_long { _x } else { _y as libc::c_long }
             }) as libc::c_int;
        (*c).error |= 1i32
    }
    if cnt as libc::c_long >
           (*c).out_end.wrapping_offset_from(dst) as libc::c_long {
        cnt =
            ({
                 let mut _x: libc::c_long =
                     (*c).out_end.wrapping_offset_from(dst) as libc::c_long;
                 let mut _y: libc::c_int = 0i32;
                 if _x > _y as libc::c_long { _x } else { _y as libc::c_long }
             }) as libc::c_int;
        (*c).error |= 2i32
    }
    (*(dst as *mut lzo_packed_uint32)).d =
        (*(src as *mut lzo_packed_uint32)).d;
    src = src.offset(4);
    dst = dst.offset(4);
    cnt -= 4i32;
    if cnt > 0i32 {
        memcpy(dst as *mut libc::c_void, src as *const libc::c_void,
               cnt as libc::c_ulong);
    }
    (*c).in_0 = src.offset(cnt as isize);
    (*c).out = dst.offset(cnt as isize);
}
/* *
 * @brief Copies previously decoded bytes to current position.
 * @param back how many bytes back we start, must be > 0
 * @param cnt number of bytes to copy, must be > 0
 *
 * cnt > back is valid, this will copy the bytes we just copied,
 * thus creating a repeating pattern with a period length of back.
 */
#[inline]
unsafe extern "C" fn copy_backptr(mut c: *mut LZOContext,
                                  mut back: libc::c_int,
                                  mut cnt: libc::c_int) {
    let mut dst: *mut uint8_t = (*c).out;
    if cnt <= 0i32 { (*c).error |= 8i32; return }
    if (dst.wrapping_offset_from((*c).out_start) as libc::c_long) <
           back as libc::c_long {
        (*c).error |= 4i32;
        return
    }
    if cnt as libc::c_long >
           (*c).out_end.wrapping_offset_from(dst) as libc::c_long {
        cnt =
            ({
                 let mut _x: libc::c_long =
                     (*c).out_end.wrapping_offset_from(dst) as libc::c_long;
                 let mut _y: libc::c_int = 0i32;
                 if _x > _y as libc::c_long { _x } else { _y as libc::c_long }
             }) as libc::c_int;
        (*c).error |= 2i32
    }
    av_memcpy_backptr(dst, back, cnt);
    (*c).out = dst.offset(cnt as isize);
}
/*
 * LZO 1x decompression
 * copyright (c) 2006 Reimar Doeffinger
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */
/* *
 * @defgroup lavu_lzo LZO
 * @ingroup lavu_crypto
 *
 * @{
 */
/* * @name Error flags returned by av_lzo1x_decode
 * @{ */
// / end of the input buffer reached before decoding finished
// / decoded data did not fit into output buffer
// / a reference to previously decoded data was wrong
// / a non-specific error in the compressed bitstream
/* * @} */
/* *
 * @brief Decodes LZO 1x compressed data.
 * @param out output buffer
 * @param outlen size of output buffer, number of bytes left are returned here
 * @param in input buffer
 * @param inlen size of input buffer, number of bytes left are returned here
 * @return 0 on success, otherwise a combination of the error flags above
 *
 * Make sure all buffers are appropriately padded, in must provide
 * AV_LZO_INPUT_PADDING, out must provide AV_LZO_OUTPUT_PADDING additional bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn av_lzo1x_decode(mut out: *mut libc::c_void,
                                         mut outlen: *mut libc::c_int,
                                         mut in_0: *const libc::c_void,
                                         mut inlen: *mut libc::c_int)
 -> libc::c_int {
    let mut state: libc::c_int = 0i32;
    let mut x: libc::c_int = 0;
    let mut c: LZOContext =
        LZOContext{in_0: 0 as *const uint8_t,
                   in_end: 0 as *const uint8_t,
                   out_start: 0 as *mut uint8_t,
                   out: 0 as *mut uint8_t,
                   out_end: 0 as *mut uint8_t,
                   error: 0,};
    if *outlen <= 0i32 || *inlen <= 0i32 {
        let mut res: libc::c_int = 0i32;
        if *outlen <= 0i32 { res |= 2i32 }
        if *inlen <= 0i32 { res |= 1i32 }
        return res
    }
    c.in_0 = in_0 as *const uint8_t;
    c.in_end = (in_0 as *const uint8_t).offset(*inlen as isize);
    c.out_start = out as *mut uint8_t;
    c.out = c.out_start;
    c.out_end = (out as *mut uint8_t).offset(*outlen as isize);
    c.error = 0i32;
    let fresh2 = c.in_0;
    c.in_0 = c.in_0.offset(1);
    x = *fresh2 as libc::c_int;
    if x > 17i32 {
        copy(&mut c, x - 17i32);
        let fresh3 = c.in_0;
        c.in_0 = c.in_0.offset(1);
        x = *fresh3 as libc::c_int;
        if x < 16i32 { c.error |= 8i32 }
    }
    if c.in_0 > c.in_end { c.error |= 1i32 }
    while c.error == 0 {
        let mut cnt: libc::c_int = 0;
        let mut back: libc::c_int = 0;
        if x > 15i32 {
            if x > 63i32 {
                /* cccbbbnn BBBBBBBB */
                cnt =
                    (x >> 5i32) -
                        1i32; /* 0001bccc (cccccccc...) bbbbbbnn BBBBBBBB */
                let fresh4 = c.in_0;
                c.in_0 = c.in_0.offset(1);
                back =
                    ((*fresh4 as libc::c_int) << 3i32) + (x >> 2i32 & 7i32) +
                        1i32
            } else if x > 31i32 {
                /* 001ccccc (cccccccc...) bbbbbbnn BBBBBBBB */
                cnt = get_len(&mut c, x, 31i32); /* 0000bbnn BBBBBBBB ) */
                let fresh5 = c.in_0;
                c.in_0 = c.in_0.offset(1);
                x = *fresh5 as libc::c_int;
                let fresh6 = c.in_0;
                c.in_0 = c.in_0.offset(1);
                back = ((*fresh6 as libc::c_int) << 6i32) + (x >> 2i32) + 1i32
            } else {
                cnt = get_len(&mut c, x, 7i32);
                back = (1i32 << 14i32) + ((x & 8i32) << 11i32);
                let fresh7 = c.in_0;
                c.in_0 = c.in_0.offset(1);
                x = *fresh7 as libc::c_int;
                let fresh8 = c.in_0;
                c.in_0 = c.in_0.offset(1);
                back += ((*fresh8 as libc::c_int) << 6i32) + (x >> 2i32);
                if back == 1i32 << 14i32 {
                    if cnt != 1i32 { c.error |= 8i32 }
                    break ;
                }
            }
        } else if state == 0 {
            /* 0000llll (llllllll...) { literal... } ( 0000bbnn BBBBBBBB ) */
            cnt = get_len(&mut c, x, 15i32);
            copy(&mut c, cnt + 3i32);
            let fresh9 = c.in_0;
            c.in_0 = c.in_0.offset(1);
            x = *fresh9 as libc::c_int;
            if x > 15i32 { continue ; }
            cnt = 1i32;
            let fresh10 = c.in_0;
            c.in_0 = c.in_0.offset(1);
            back =
                (1i32 << 11i32) + ((*fresh10 as libc::c_int) << 2i32) +
                    (x >> 2i32) + 1i32
        } else {
            cnt = 0i32;
            let fresh11 = c.in_0;
            c.in_0 = c.in_0.offset(1);
            back = ((*fresh11 as libc::c_int) << 2i32) + (x >> 2i32) + 1i32
        }
        copy_backptr(&mut c, back, cnt + 2i32);
        cnt = x & 3i32;
        state = cnt;
        copy(&mut c, cnt);
        let fresh12 = c.in_0;
        c.in_0 = c.in_0.offset(1);
        x = *fresh12 as libc::c_int
    }
    *inlen =
        c.in_end.wrapping_offset_from(c.in_0) as libc::c_long as libc::c_int;
    if c.in_0 > c.in_end { *inlen = 0i32 }
    *outlen =
        c.out_end.wrapping_offset_from(c.out) as libc::c_long as libc::c_int;
    return c.error;
}
