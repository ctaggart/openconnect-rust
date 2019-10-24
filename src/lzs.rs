use libc;
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint16_t.h:22"]
pub mod _uint16_t_h {
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
    #[src_loc = "31:1"]
    pub type uint16_t = libc::c_ushort;
    /* _UINT16_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:22"]
pub mod _uint32_t_h {
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
    #[src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
    /* _UINT32_T */
}
#[header_src = "/Users/cameron/github/openconnect/openconnect-internal.h:24"]
pub mod openconnect_internal_h {
    #[derive ( Copy, Clone )]
    #[repr(C, packed)]
    #[src_loc = "1189:1"]
    pub struct oc_packed_uint16_t {
        pub d: uint16_t,
    }
    use super::_uint16_t_h::uint16_t;
    /* __OPENCONNECT_INTERNAL_H__ */
    /* !Not known to be little-endian */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:21"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[src_loc = "71:6"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
    /* _STRING_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
    /* Some functions historically defined in string.h were placed in strings.h
 * by SUS.  We are using "strings.h" instead of <strings.h> to avoid an issue
 * where /Developer/Headers/FlatCarbon/Strings.h could be included instead on
 * case-insensitive file systems.
 */
}
pub use self::_uint16_t_h::uint16_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::openconnect_internal_h::oc_packed_uint16_t;
use self::string_h::{memcmp, memset};
/*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2008-2015 Intel Corporation.
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
/* Strictly speaking, this check ought to be on			\
	 * (srclen < 1 + (bits_left < bits)). However, when bits == 9	\
	 * the (bits_left < bits) comparison is always true so it	\
	 * always comes out as (srclen < 2).				\
	 * And bits is only anything *other* than 9 when we're reading	\
	 * reading part of a match encoding. And in that case, there	\
	 * damn well ought to be an end marker (7 more bits) after	\
	 * what we're reading now, so it's perfectly OK to use		\
	 * (srclen < 2) in that case too. And a *lot* cheaper. */
/* Explicit comparison with 8 to optimise it into a tautology	\
	 * in the the bits == 9 case, because the compiler doesn't	\
	 * know that bits_left can never be larger than 8. */
/* We need *all* the bits that are left in the current	\
		 * byte. Take them and bump the input pointer. */
/* We need bits from the next byte too... */
/* ...if we used *all* of them then (which can	\
			 * only happen if bits > 8), then bump the	\
			 * input pointer again so we never leave	\
			 * bits_left == 0. */
/* We need fewer bits than are left in the current byte */
#[no_mangle]
#[src_loc = "69:1"]
pub unsafe extern "C" fn lzs_decompress(mut dst: *mut libc::c_uchar,
                                        mut dstlen: libc::c_int,
                                        mut src: *const libc::c_uchar,
                                        mut srclen: libc::c_int)
 -> libc::c_int {
    let mut outlen: libc::c_int =
        0i32; /* Bits left in the current byte at *src */
    let mut bits_left: libc::c_int = 8i32;
    let mut data: uint32_t = 0;
    let mut offset: uint16_t = 0;
    let mut length: uint16_t = 0;
    loop  {
        /* Get 9 bits, which is the minimum and a common case */
        if srclen < 2i32 { return -22i32 }
        if 9i32 >= 8i32 || 9i32 >= bits_left {
            data =
                ((*src.offset(0) as libc::c_int) << 9i32 - bits_left &
                     (1i32 << 9i32) - 1i32) as uint32_t;
            src = src.offset(1);
            srclen -= 1;
            bits_left += 8i32 - 9i32;
            if 9i32 > 8i32 || bits_left < 8i32 {
                data |=
                    (*src.offset(0) as libc::c_int >> bits_left) as
                        libc::c_uint;
                if 9i32 > 8i32 && bits_left == 0 {
                    bits_left = 8i32;
                    src = src.offset(1);
                    srclen -= 1
                }
            }
        } else {
            data =
                ((*src.offset(0) as libc::c_int >> bits_left - 9i32) as
                     libc::c_ulonglong &
                     (1u64 << 9i32).wrapping_sub(1i32 as libc::c_ulonglong))
                    as uint32_t;
            bits_left -= 9i32
        }
        /* 0bbbbbbbb is a literal byte. The loop gives a hint to
		 * the compiler that we expect to see a few of these. */
        while data < 0x100i32 as libc::c_uint {
            if outlen == dstlen { return -27i32 }
            let fresh0 = outlen;
            outlen = outlen + 1;
            *dst.offset(fresh0 as isize) = data as libc::c_uchar;
            if srclen < 2i32 { return -22i32 }
            if 9i32 >= 8i32 || 9i32 >= bits_left {
                data =
                    ((*src.offset(0) as libc::c_int) << 9i32 - bits_left &
                         (1i32 << 9i32) - 1i32) as uint32_t;
                src = src.offset(1);
                srclen -= 1;
                bits_left += 8i32 - 9i32;
                if 9i32 > 8i32 || bits_left < 8i32 {
                    data |=
                        (*src.offset(0) as libc::c_int >> bits_left) as
                            libc::c_uint;
                    if 9i32 > 8i32 && bits_left == 0 {
                        bits_left = 8i32;
                        src = src.offset(1);
                        srclen -= 1
                    }
                }
            } else {
                data =
                    ((*src.offset(0) as libc::c_int >> bits_left - 9i32) as
                         libc::c_ulonglong &
                         (1u64 <<
                              9i32).wrapping_sub(1i32 as libc::c_ulonglong))
                        as uint32_t;
                bits_left -= 9i32
            }
        }
        /* 110000000 is the end marker */
        if data == 0x180i32 as libc::c_uint { return outlen }
        /* 11bbbbbbb is a 7-bit offset */
        offset = (data & 0x7fi32 as libc::c_uint) as uint16_t;
        /* 10bbbbbbbbbbb is an 11-bit offset, so get the next 4 bits */
        if data < 0x180i32 as libc::c_uint {
            if srclen < 2i32 { return -22i32 }
            if 4i32 >= 8i32 || 4i32 >= bits_left {
                data =
                    ((*src.offset(0) as libc::c_int) << 4i32 - bits_left &
                         (1i32 << 4i32) - 1i32) as uint32_t;
                src = src.offset(1);
                srclen -= 1;
                bits_left += 8i32 - 4i32;
                if 4i32 > 8i32 || bits_left < 8i32 {
                    data |=
                        (*src.offset(0) as libc::c_int >> bits_left) as
                            libc::c_uint;
                    if 4i32 > 8i32 && bits_left == 0 {
                        bits_left = 8i32;
                        src = src.offset(1);
                        srclen -= 1
                    }
                }
            } else {
                data =
                    ((*src.offset(0) as libc::c_int >> bits_left - 4i32) as
                         libc::c_ulonglong &
                         (1u64 <<
                              4i32).wrapping_sub(1i32 as libc::c_ulonglong))
                        as uint32_t;
                bits_left -= 4i32
            }
            offset = ((offset as libc::c_int) << 4i32) as uint16_t;
            offset = (offset as libc::c_uint | data) as uint16_t
        }
        /* This is a compressed sequence; now get the length */
        if srclen < 2i32 { return -22i32 }
        if 2i32 >= 8i32 || 2i32 >= bits_left {
            data =
                ((*src.offset(0) as libc::c_int) << 2i32 - bits_left &
                     (1i32 << 2i32) - 1i32) as uint32_t;
            src = src.offset(1);
            srclen -= 1;
            bits_left += 8i32 - 2i32;
            if 2i32 > 8i32 || bits_left < 8i32 {
                data |=
                    (*src.offset(0) as libc::c_int >> bits_left) as
                        libc::c_uint;
                if 2i32 > 8i32 && bits_left == 0 {
                    bits_left = 8i32;
                    src = src.offset(1);
                    srclen -= 1
                }
            }
        } else {
            data =
                ((*src.offset(0) as libc::c_int >> bits_left - 2i32) as
                     libc::c_ulonglong &
                     (1u64 << 2i32).wrapping_sub(1i32 as libc::c_ulonglong))
                    as uint32_t;
            bits_left -= 2i32
        }
        if data != 3i32 as libc::c_uint {
            /* 00, 01, 10 ==> 2, 3, 4 */
            length = data.wrapping_add(2i32 as libc::c_uint) as uint16_t
        } else {
            if srclen < 2i32 { return -22i32 }
            if 2i32 >= 8i32 || 2i32 >= bits_left {
                data =
                    ((*src.offset(0) as libc::c_int) << 2i32 - bits_left &
                         (1i32 << 2i32) - 1i32) as uint32_t;
                src = src.offset(1);
                srclen -= 1;
                bits_left += 8i32 - 2i32;
                if 2i32 > 8i32 || bits_left < 8i32 {
                    data |=
                        (*src.offset(0) as libc::c_int >> bits_left) as
                            libc::c_uint;
                    if 2i32 > 8i32 && bits_left == 0 {
                        bits_left = 8i32;
                        src = src.offset(1);
                        srclen -= 1
                    }
                }
            } else {
                data =
                    ((*src.offset(0) as libc::c_int >> bits_left - 2i32) as
                         libc::c_ulonglong &
                         (1u64 <<
                              2i32).wrapping_sub(1i32 as libc::c_ulonglong))
                        as uint32_t;
                bits_left -= 2i32
            }
            if data != 3i32 as libc::c_uint {
                /* 1100, 1101, 1110 => 5, 6, 7 */
                length = data.wrapping_add(5i32 as libc::c_uint) as uint16_t
            } else {
                /* For each 1111 prefix add 15 to the length. Then add
				   the value of final nybble. */
                length = 8i32 as uint16_t;
                loop  {
                    if srclen < 2i32 { return -22i32 }
                    if 4i32 >= 8i32 || 4i32 >= bits_left {
                        data =
                            ((*src.offset(0) as libc::c_int) <<
                                 4i32 - bits_left & (1i32 << 4i32) - 1i32) as
                                uint32_t;
                        src = src.offset(1);
                        srclen -= 1;
                        bits_left += 8i32 - 4i32;
                        if 4i32 > 8i32 || bits_left < 8i32 {
                            data |=
                                (*src.offset(0) as libc::c_int >> bits_left)
                                    as libc::c_uint;
                            if 4i32 > 8i32 && bits_left == 0 {
                                bits_left = 8i32;
                                src = src.offset(1);
                                srclen -= 1
                            }
                        }
                    } else {
                        data =
                            ((*src.offset(0) as libc::c_int >>
                                  bits_left - 4i32) as libc::c_ulonglong &
                                 (1u64 <<
                                      4i32).wrapping_sub(1i32 as
                                                             libc::c_ulonglong))
                                as uint32_t;
                        bits_left -= 4i32
                    }
                    if data != 15i32 as libc::c_uint {
                        length =
                            (length as libc::c_uint).wrapping_add(data) as
                                uint16_t as uint16_t;
                        break ;
                    } else {
                        length = (length as libc::c_int + 15i32) as uint16_t
                    }
                }
            }
        }
        if offset as libc::c_int > outlen { return -22i32 }
        if length as libc::c_int + outlen > dstlen { return -27i32 }
        while length != 0 {
            *dst.offset(outlen as isize) =
                *dst.offset((outlen - offset as libc::c_int) as isize);
            outlen += 1;
            length = length.wrapping_sub(1)
        }
    };
}
/* Key material for DTLS-PSK */
/* Packet types */
/* Uncompressed data */
/* Dead Peer Detection */
/* DPD response */
/* Client disconnection notice */
/* Keepalive */
/* Compressed data */
/* Server kick */
/* Encryption and HMAC algorithms (matching Juniper/Pulse binary encoding) */
/* SHA256 */
/* Including the next-header field */
/* ***************************************************************************/
/* Oh Solaris how we hate thee! */
/* For systems that don't support O_CLOEXEC, just don't bother.
   We don't keep files open for long anyway. */
/* I always coded as if it worked like this. Now it does. */
/* ***************************************************************************/
/* iconv.c */
/* script.c */
/* tun.c / tun-win32.c */
/* {gnutls,openssl}-dtls.c */
/* dtls.c */
/* cstp.c */
/* auth-juniper.c */
/* oncp.c */
/* pulse.c */
/* auth-globalprotect.c */
/* gpst.c */
/* lzs.c */
/*
 * Much of the compression algorithm used here is based very loosely on ideas
 * from isdn_lzscomp.c by Andre Beck: http://micky.ibh.de/~beck/stuff/lzs4i4l/
 */
#[no_mangle]
#[src_loc = "166:1"]
pub unsafe extern "C" fn lzs_compress(mut dst: *mut libc::c_uchar,
                                      mut dstlen: libc::c_int,
                                      mut src: *const libc::c_uchar,
                                      mut srclen: libc::c_int)
 -> libc::c_int {
    let mut length: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut inpos: libc::c_int = 0i32;
    let mut outpos: libc::c_int = 0i32;
    let mut longest_match_len: uint16_t = 0;
    let mut hofs: uint16_t = 0;
    let mut longest_match_ofs: uint16_t = 0;
    let mut hash: uint16_t = 0;
    let mut outbits: uint32_t = 0i32 as uint32_t;
    let mut nr_outbits: libc::c_int = 0i32;
    /*
	 * This is theoretically a hash. But RAM is cheap and just loading the
	 * 16-bit value and using it as a hash is *much* faster.
	 */
    /*
	 * There are two data structures for tracking the history. The first
	 * is the true hash table, an array indexed by the hash value described
	 * above. It yields the offset in the input buffer at which the given
	 * hash was most recently seen. We use INVALID_OFS (0xffff) for none
	 * since we know IP packets are limited to 64KiB and we can never be
	 * *starting* a match at the penultimate byte of the packet.
	 */
    let mut hash_table: [uint16_t; 65536] =
        [0; 65536]; /* Buffer offset for first match */
    /*
	 * The second data structure allows us to find the previous occurrences
	 * of the same hash value. It is a ring buffer containing links only for
	 * the latest MAX_HISTORY bytes of the input. The lookup for a given
	 * offset will yield the previous offset at which the same data hash
	 * value was found.
	 */
    /* Highest offset LZS can represent is 11 bits */
    let mut hash_chain: [uint16_t; 2048] = [0; 2048];
    /* Just in case anyone tries to use this in a more general-purpose
	 * scenario... */
    if srclen > 0xffffi32 + 1i32 { return -27i32 }
    /* No need to initialise hash_chain since we can only ever follow
	 * links to it that have already been initialised. */
    memset(hash_table.as_mut_ptr() as *mut libc::c_void, 0xffi32,
           ::std::mem::size_of::<[uint16_t; 65536]>() as libc::c_ulong);
    while inpos < srclen - 2i32 {
        hash = (*(src.offset(inpos as isize) as *mut oc_packed_uint16_t)).d;
        hofs = hash_table[hash as usize];
        hash_chain[(inpos & (1i32 << 11i32) - 1i32) as usize] = hofs;
        hash_table[hash as usize] = inpos as uint16_t;
        if hofs as libc::c_int == 0xffffi32 ||
               hofs as libc::c_int + (1i32 << 11i32) <= inpos {
            outbits <<= 9i32;
            outbits |= *src.offset(inpos as isize) as libc::c_uint;
            nr_outbits += 9i32;
            if 9i32 > 8i32 {
                nr_outbits -= 8i32;
                if outpos == dstlen { return -27i32 }
                let fresh1 = outpos;
                outpos = outpos + 1;
                *dst.offset(fresh1 as isize) =
                    (outbits >> nr_outbits) as libc::c_uchar
            }
            if nr_outbits >= 8i32 {
                nr_outbits -= 8i32;
                if outpos == dstlen { return -27i32 }
                let fresh2 = outpos;
                outpos = outpos + 1;
                *dst.offset(fresh2 as isize) =
                    (outbits >> nr_outbits) as libc::c_uchar
            }
            inpos += 1
        } else {
            /* Since the hash is 16-bits, we *know* the first two bytes match */
            longest_match_len = 2i32 as uint16_t;
            longest_match_ofs = hofs;
            's_159:
                while hofs as libc::c_int != 0xffffi32 &&
                          hofs as libc::c_int + (1i32 << 11i32) > inpos {
                    /* We only get here if longest_match_len is >= 2. We need to find
			   a match of longest_match_len + 1 for it to be interesting. */
                    if memcmp(src.offset(hofs as libc::c_int as
                                             isize).offset(2) as
                                  *const libc::c_void,
                              src.offset(inpos as isize).offset(2) as
                                  *const libc::c_void,
                              (longest_match_len as libc::c_int - 1i32) as
                                  libc::c_ulong) == 0 {
                        longest_match_ofs = hofs;
                        loop  {
                            longest_match_len =
                                longest_match_len.wrapping_add(1);
                            /* If we cannot *have* a longer match because we're at the
					 * end of the input, stop looking */
                            if longest_match_len as libc::c_int + inpos ==
                                   srclen {
                                break 's_159 ;
                            }
                            if !(*src.offset((longest_match_len as libc::c_int
                                                  + inpos) as isize) as
                                     libc::c_int ==
                                     *src.offset((longest_match_len as
                                                      libc::c_int +
                                                      hofs as libc::c_int) as
                                                     isize) as libc::c_int) {
                                break ;
                            }
                        }
                    }
                    hofs =
                        hash_chain[(hofs as libc::c_int &
                                        (1i32 << 11i32) - 1i32) as usize]
                    /* Typical compressor tuning would have a break out of the loop
			   here depending on the number of potential match locations we've
			   tried, or a value of longest_match_len that's considered "good
			   enough" so we stop looking for something better. We could also
			   do a hybrid where we count the total bytes compared, so 5
			   attempts to find a match better than 10 bytes is worth the same
			   as 10 attempts to find a match better than 5 bytes. Or
			   something. Anyway, we currently don't give up until we run out
			   of reachable history — maximal compression. */
                }
            /* Output offset, as 7-bit or 11-bit as appropriate */
            offset = inpos - longest_match_ofs as libc::c_int;
            length = longest_match_len as libc::c_int;
            if offset < 0x80i32 {
                outbits <<= 9i32;
                outbits |= (0x180i32 | offset) as libc::c_uint;
                nr_outbits += 9i32;
                if 9i32 > 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh3 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh3 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
                if nr_outbits >= 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh4 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh4 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
            } else {
                outbits <<= 13i32;
                outbits |= (0x1000i32 | offset) as libc::c_uint;
                nr_outbits += 13i32;
                if 13i32 > 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh5 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh5 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
                if nr_outbits >= 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh6 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh6 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
            }
            /* Output length */
            if length < 5i32 {
                outbits <<= 2i32;
                outbits |= (length - 2i32) as libc::c_uint;
                nr_outbits += 2i32;
                if 2i32 > 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh7 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh7 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
                if nr_outbits >= 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh8 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh8 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
            } else if length < 8i32 {
                outbits <<= 4i32;
                outbits |= (length + 7i32) as libc::c_uint;
                nr_outbits += 4i32;
                if 4i32 > 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh9 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh9 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
                if nr_outbits >= 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh10 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh10 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
            } else {
                length += 7i32;
                while length >= 30i32 {
                    outbits <<= 8i32;
                    outbits |= 0xffi32 as libc::c_uint;
                    nr_outbits += 8i32;
                    if 8i32 > 8i32 {
                        nr_outbits -= 8i32;
                        if outpos == dstlen { return -27i32 }
                        let fresh11 = outpos;
                        outpos = outpos + 1;
                        *dst.offset(fresh11 as isize) =
                            (outbits >> nr_outbits) as libc::c_uchar
                    }
                    if nr_outbits >= 8i32 {
                        nr_outbits -= 8i32;
                        if outpos == dstlen { return -27i32 }
                        let fresh12 = outpos;
                        outpos = outpos + 1;
                        *dst.offset(fresh12 as isize) =
                            (outbits >> nr_outbits) as libc::c_uchar
                    }
                    length -= 30i32
                }
                if length >= 15i32 {
                    outbits <<= 8i32;
                    outbits |= (0xf0i32 + length - 15i32) as libc::c_uint;
                    nr_outbits += 8i32;
                    if 8i32 > 8i32 {
                        nr_outbits -= 8i32;
                        if outpos == dstlen { return -27i32 }
                        let fresh13 = outpos;
                        outpos = outpos + 1;
                        *dst.offset(fresh13 as isize) =
                            (outbits >> nr_outbits) as libc::c_uchar
                    }
                    if nr_outbits >= 8i32 {
                        nr_outbits -= 8i32;
                        if outpos == dstlen { return -27i32 }
                        let fresh14 = outpos;
                        outpos = outpos + 1;
                        *dst.offset(fresh14 as isize) =
                            (outbits >> nr_outbits) as libc::c_uchar
                    }
                } else {
                    outbits <<= 4i32;
                    outbits |= length as libc::c_uint;
                    nr_outbits += 4i32;
                    if 4i32 > 8i32 {
                        nr_outbits -= 8i32;
                        if outpos == dstlen { return -27i32 }
                        let fresh15 = outpos;
                        outpos = outpos + 1;
                        *dst.offset(fresh15 as isize) =
                            (outbits >> nr_outbits) as libc::c_uchar
                    }
                    if nr_outbits >= 8i32 {
                        nr_outbits -= 8i32;
                        if outpos == dstlen { return -27i32 }
                        let fresh16 = outpos;
                        outpos = outpos + 1;
                        *dst.offset(fresh16 as isize) =
                            (outbits >> nr_outbits) as libc::c_uchar
                    }
                }
            }
            /* If we're already done, don't bother updating the hash tables. */
            if inpos + longest_match_len as libc::c_int >= srclen - 2i32 {
                inpos += longest_match_len as libc::c_int;
                break ;
            } else {
                /* We already added the first byte to the hash tables. Add the rest. */
                inpos += 1;
                loop  {
                    longest_match_len = longest_match_len.wrapping_sub(1);
                    if !(longest_match_len != 0) { break ; }
                    hash =
                        (*(src.offset(inpos as isize) as
                               *mut oc_packed_uint16_t)).d;
                    hash_chain[(inpos & (1i32 << 11i32) - 1i32) as usize] =
                        hash_table[hash as usize];
                    let fresh17 = inpos;
                    inpos = inpos + 1;
                    hash_table[hash as usize] = fresh17 as uint16_t
                }
            }
        }
    }
    /* Special cases at the end */
    if inpos == srclen - 2i32 {
        hash = (*(src.offset(inpos as isize) as *mut oc_packed_uint16_t)).d;
        hofs = hash_table[hash as usize];
        if hofs as libc::c_int != 0xffffi32 &&
               hofs as libc::c_int + (1i32 << 11i32) > inpos {
            offset = inpos - hofs as libc::c_int;
            if offset < 0x80i32 {
                outbits <<= 9i32;
                outbits |= (0x180i32 | offset) as libc::c_uint;
                nr_outbits += 9i32;
                if 9i32 > 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh18 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh18 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
                if nr_outbits >= 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh19 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh19 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
            } else {
                outbits <<= 13i32;
                outbits |= (0x1000i32 | offset) as libc::c_uint;
                nr_outbits += 13i32;
                if 13i32 > 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh20 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh20 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
                if nr_outbits >= 8i32 {
                    nr_outbits -= 8i32;
                    if outpos == dstlen { return -27i32 }
                    let fresh21 = outpos;
                    outpos = outpos + 1;
                    *dst.offset(fresh21 as isize) =
                        (outbits >> nr_outbits) as libc::c_uchar
                }
            }
            /* The length is 2 bytes */
            outbits <<= 2i32;
            outbits |= 0i32 as libc::c_uint;
            nr_outbits += 2i32;
            if 2i32 > 8i32 {
                nr_outbits -= 8i32;
                if outpos == dstlen { return -27i32 }
                let fresh22 = outpos;
                outpos = outpos + 1;
                *dst.offset(fresh22 as isize) =
                    (outbits >> nr_outbits) as libc::c_uchar
            }
            if nr_outbits >= 8i32 {
                nr_outbits -= 8i32;
                if outpos == dstlen { return -27i32 }
                let fresh23 = outpos;
                outpos = outpos + 1;
                *dst.offset(fresh23 as isize) =
                    (outbits >> nr_outbits) as libc::c_uchar
            }
        } else {
            outbits <<= 9i32;
            outbits |= *src.offset(inpos as isize) as libc::c_uint;
            nr_outbits += 9i32;
            if 9i32 > 8i32 {
                nr_outbits -= 8i32;
                if outpos == dstlen { return -27i32 }
                let fresh24 = outpos;
                outpos = outpos + 1;
                *dst.offset(fresh24 as isize) =
                    (outbits >> nr_outbits) as libc::c_uchar
            }
            if nr_outbits >= 8i32 {
                nr_outbits -= 8i32;
                if outpos == dstlen { return -27i32 }
                let fresh25 = outpos;
                outpos = outpos + 1;
                *dst.offset(fresh25 as isize) =
                    (outbits >> nr_outbits) as libc::c_uchar
            }
            outbits <<= 9i32;
            outbits |= *src.offset((inpos + 1i32) as isize) as libc::c_uint;
            nr_outbits += 9i32;
            if 9i32 > 8i32 {
                nr_outbits -= 8i32;
                if outpos == dstlen { return -27i32 }
                let fresh26 = outpos;
                outpos = outpos + 1;
                *dst.offset(fresh26 as isize) =
                    (outbits >> nr_outbits) as libc::c_uchar
            }
            if nr_outbits >= 8i32 {
                nr_outbits -= 8i32;
                if outpos == dstlen { return -27i32 }
                let fresh27 = outpos;
                outpos = outpos + 1;
                *dst.offset(fresh27 as isize) =
                    (outbits >> nr_outbits) as libc::c_uchar
            }
        }
    } else if inpos == srclen - 1i32 {
        outbits <<= 9i32;
        outbits |= *src.offset(inpos as isize) as libc::c_uint;
        nr_outbits += 9i32;
        if 9i32 > 8i32 {
            nr_outbits -= 8i32;
            if outpos == dstlen { return -27i32 }
            let fresh28 = outpos;
            outpos = outpos + 1;
            *dst.offset(fresh28 as isize) =
                (outbits >> nr_outbits) as libc::c_uchar
        }
        if nr_outbits >= 8i32 {
            nr_outbits -= 8i32;
            if outpos == dstlen { return -27i32 }
            let fresh29 = outpos;
            outpos = outpos + 1;
            *dst.offset(fresh29 as isize) =
                (outbits >> nr_outbits) as libc::c_uchar
        }
    }
    /* End marker, with 7 trailing zero bits to ensure that it's flushed. */
    outbits <<= 16i32;
    outbits |= 0xc000i32 as libc::c_uint;
    nr_outbits += 16i32;
    if 16i32 > 8i32 {
        nr_outbits -= 8i32;
        if outpos == dstlen { return -27i32 }
        let fresh30 = outpos;
        outpos = outpos + 1;
        *dst.offset(fresh30 as isize) =
            (outbits >> nr_outbits) as libc::c_uchar
    }
    if nr_outbits >= 8i32 {
        nr_outbits -= 8i32;
        if outpos == dstlen { return -27i32 }
        let fresh31 = outpos;
        outpos = outpos + 1;
        *dst.offset(fresh31 as isize) =
            (outbits >> nr_outbits) as libc::c_uchar
    }
    return outpos;
}
