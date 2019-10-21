#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(custom_attribute, main)]
extern crate libc;
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:20"]
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
    use super::libc;
    /* _UINT32_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:20"]
pub mod _uint64_t_h {
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
    pub type uint64_t = libc::c_ulonglong;
    use super::libc;
    /* _UINT64_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:21"]
pub mod stdio_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "170:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
    /* _STDIO_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src = "/Users/cameron/github/openconnect/esp-seqno.c:37"]
pub mod esp_seqno_c {
    /* Eventually we're going to have to have more than one incoming ESP
   context at a time, to allow for the overlap period during a rekey.
   So pass the 'esp' even though for now it's redundant. */
    #[no_mangle]
    #[src_loc = "32:1"]
    pub unsafe extern "C" fn verify_packet_seqno(mut vpninfo:
                                                     *mut openconnect_info,
                                                 mut esp: *mut esp,
                                                 mut seq: uint32_t)
     -> libc::c_int {
        /*
	 * For incoming, esp->seq is the next *expected* packet, being
	 * the sequence number *after* the latest we have received.
	 *
	 * Since it must always be true that packet esp->seq-1 has been
	 * received, so there's no need to explicitly record that.
	 *
	 * So the backlog bitmap covers the 64 packets prior to that,
	 * with the LSB representing packet (esp->seq - 2), and the MSB
	 * representing (esp->seq - 65). A received packet is represented
	 * by a zero bit, and a missing packet is represented by a one.
	 *
	 * Thus we can allow out-of-order reception of packets that are
	 * within a reasonable interval of the latest packet received.
	 */
        if seq as libc::c_ulonglong == (*esp).seq {
            /* The common case. This is the packet we expected next. */
            (*esp).seq_backlog <<= 1i32;
            /* This might reach a value higher than the 32-bit ESP sequence
		 * numbers can actually reach. Which is fine. When that
		 * happens, we'll do the right thing and just not accept any
		 * newer packets. Someone needs to start a new epoch. */
            (*esp).seq = (*esp).seq.wrapping_add(1);
            printf(b"Accepting expected ESP packet with seq %u\n\x00" as
                       *const u8 as *const libc::c_char, seq);
            return 0i32
        } else if seq as libc::c_ulonglong > (*esp).seq {
            /* The packet we were expecting has gone missing; this one is newer.
		 * We always advance the window to accommodate it. */
            let mut delta: uint32_t =
                (seq as libc::c_ulonglong).wrapping_sub((*esp).seq) as
                    uint32_t;
            if delta >= 64i32 as libc::c_uint {
                /* We jumped a long way into the future. We have not seen
			 * any of the previous 32 packets so set the backlog bitmap
			 * to all ones. */
                (*esp).seq_backlog = 0xffffffffffffffffu64
            } else if delta == 63i32 as libc::c_uint {
                /* Avoid undefined behaviour that shifting by 64 would incur.
			 * The (clear) top bit represents the packet which is currently
			 * esp->seq - 1, which we know was already received. */
                (*esp).seq_backlog = 0xffffffffffffffffu64 >> 1i32
            } else {
                /* We have missed (delta) packets. Shift the backlog by that
			 * amount *plus* the one we would have shifted it anyway if
			 * we'd received the packet we were expecting. The zero bit
			 * representing the packet which is currently esp->seq - 1,
			 * which we know has been received, ends up at bit position
			 * (1<<delta). Then we set all the bits lower than that, which
			 * represent the missing packets. */
                (*esp).seq_backlog <<=
                    delta.wrapping_add(1i32 as libc::c_uint);
                (*esp).seq_backlog |=
                    (1u64 << delta).wrapping_sub(1i32 as libc::c_ulonglong)
            }
            printf(b"Accepting later-than-expected ESP packet with seq %u (expected %llu)\n\x00"
                       as *const u8 as *const libc::c_char, seq, (*esp).seq);
            (*esp).seq =
                (seq as uint64_t).wrapping_add(1i32 as libc::c_ulonglong);
            return 0i32
        } else {
            /* This packet is older than the one we were expecting. By how much...? */
            let mut delta_0: uint32_t =
                (*esp).seq.wrapping_sub(seq as libc::c_ulonglong) as uint32_t;
            /* delta==0 is the overflow case where esp->seq is 0x100000000 and seq is 0 */
            if delta_0 > 65i32 as libc::c_uint ||
                   delta_0 == 0i32 as libc::c_uint {
                /* Too old. We can't know if it's a replay. */
                if (*vpninfo).esp_replay_protect != 0 {
                    printf(b"Discarding ancient ESP packet with seq %u (expected %llu)\n\x00"
                               as *const u8 as *const libc::c_char, seq,
                           (*esp).seq);
                    return -22i32
                } else {
                    printf(b"Tolerating ancient ESP packet with seq %u (expected %llu)\n\x00"
                               as *const u8 as *const libc::c_char, seq,
                           (*esp).seq);
                    return 0i32
                }
            } else {
                if !(delta_0 == 1i32 as libc::c_uint) {
                    /* Within the backlog window, so we remember whether we've seen it or not. */
                    let mut mask: uint64_t =
                        1u64 << delta_0.wrapping_sub(2i32 as libc::c_uint);
                    if !((*esp).seq_backlog & mask == 0) {
                        (*esp).seq_backlog &= !mask;
                        printf(b"Accepting out-of-order ESP packet with seq %u (expected %llu)\n\x00"
                                   as *const u8 as *const libc::c_char, seq,
                               (*esp).seq);
                        return 0i32
                    }
                }
                /* Not in the bitmask since it is by definition already received. */
                if (*vpninfo).esp_replay_protect != 0 {
                    printf(b"Discarding replayed ESP packet with seq %u\n\x00"
                               as *const u8 as *const libc::c_char, seq);
                    return -22i32
                } else {
                    printf(b"Tolerating replayed ESP packet with seq %u\n\x00"
                               as *const u8 as *const libc::c_char, seq);
                    return 0i32
                }
            }
        };
    }
    use super::{openconnect_info, esp, libc};
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::stdio_h::printf;
}
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
use self::stdio_h::printf;
pub use self::esp_seqno_c::verify_packet_seqno;
/*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2016 Intel Corporation.
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
#[derive ( Copy, Clone )]
#[repr(C)]
#[src_loc = "28:1"]
pub struct openconnect_info {
    pub esp_replay_protect: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
#[src_loc = "32:1"]
pub struct esp {
    pub seq_backlog: uint64_t,
    pub seq: uint64_t,
}
#[src_loc = "40:1"]
unsafe fn main_0() -> libc::c_int {
    let mut esptest: esp =
        {
            let mut init =
                esp{seq_backlog: 0i32 as uint64_t, seq: 0i32 as uint64_t,};
            init
        };
    let mut vpninfo: openconnect_info =
        { let mut init = openconnect_info{esp_replay_protect: 1i32,}; init };
    if verify_packet_seqno(&mut vpninfo, &mut esptest, 0i32 as uint32_t) != 0
           ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 2i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 1i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 64i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 65i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 65i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 66i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 67i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 68i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 68i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 2i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 3i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 4i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 164i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 99i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 100i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 200i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 264i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 199i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 200i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 265i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 210i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 201i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 270i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 206i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 210i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 333i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 268i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 269i32 as uint32_t)
               != 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 270i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0xfffffffdu32) != 0
           ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 1i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0xffffffc1u32) != 0
           ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0xfffffffcu32) != 0
           ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0xffffffffu32) != 0
           ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0i32 as uint32_t)
               == 0 ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0xffffffbeu32) == 0
           ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0xffffffbfu32) != 0
           ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0xffffffc1u32) == 0
           ||
           verify_packet_seqno(&mut vpninfo, &mut esptest, 0xffffffc0u32) != 0
       {
        return 1i32
    }
    return 0i32;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
