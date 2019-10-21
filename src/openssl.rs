#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(custom_attribute, extern_types)]
extern crate libc;
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:21"]
pub mod _types_h {
    /*
 * Copyright (c) 2000-2003 Apple Computer, Inc. All rights reserved.
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
    /*
 * This header file contains integer types.  It's intended to also contain
 * flotaing point and other arithmetic types, as needed, later.
 */
    /* !__GNUC__ */
    /* !__GNUC__ */
    #[src_loc = "41:1"]
    pub type __uint8_t = libc::c_uchar;
    #[src_loc = "44:1"]
    pub type __int32_t = libc::c_int;
    #[src_loc = "45:1"]
    pub type __uint32_t = libc::c_uint;
    #[src_loc = "46:1"]
    pub type __int64_t = libc::c_longlong;
    #[src_loc = "92:1"]
    pub type __darwin_size_t = libc::c_ulong;
    #[src_loc = "118:1"]
    pub type __darwin_socklen_t = __uint32_t;
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
    #[src_loc = "120:1"]
    pub type __darwin_time_t = libc::c_long;
    use super::libc;
    /* _BSD_I386__TYPES_H_ */
    /* time() */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:21"]
pub mod sys__types_h {
    /* Used by statvfs and fstatvfs */
    #[src_loc = "60:1"]
    pub type __darwin_gid_t = __uint32_t;
    /* [???] Some file attributes */
    #[src_loc = "71:1"]
    pub type __darwin_off_t = __int64_t;
    /* [???] signal set */
    #[src_loc = "74:1"]
    pub type __darwin_suseconds_t = __int32_t;
    /* [???] microseconds */
    #[src_loc = "75:1"]
    pub type __darwin_uid_t = __uint32_t;
    use super::_types_h::{__uint32_t, __int64_t, __int32_t};
    /* _SYS__TYPES_H_ */
    /* (gcc >= 3.5) */
    /* !(gcc >= 3.5) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:21"]
pub mod _size_t_h {
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
    #[src_loc = "31:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
    /* _SIZE_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_gid_t.h:22"]
pub mod _gid_t_h {
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
    /* __darwin_gid_t */
    #[src_loc = "31:1"]
    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:22"]
pub mod _uid_t_h {
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
    /* __darwin_uid_t */
    #[src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
    /* _UID_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_time_t.h:22"]
pub mod _time_t_h {
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
    #[src_loc = "31:1"]
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
    /* _TIME_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_fd_def.h:22"]
pub mod _fd_def_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "49:9"]
    pub struct fd_set {
        pub fds_bits: [__int32_t; 32],
    }
    use super::_types_h::__int32_t;
    /* _FD_SET */
    /*
 * Use the built-in bzero function instead of the library version so that
 * we do not pollute the namespace or introduce prototype warnings.
 */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:25"]
pub mod _sa_family_t_h {
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
    /* __uint8_t */
    #[src_loc = "31:1"]
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
    /* _SA_FAMILY_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:25"]
pub mod _socklen_t_h {
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
    /* __darwin_socklen_t */
    #[src_loc = "31:1"]
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:25"]
pub mod socket_h {
    /*
 * Copyright (c) 2000-2019 Apple Inc. All rights reserved.
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
/* Copyright (c) 1998, 1999 Apple Computer, Inc. All Rights Reserved */
/* Copyright (c) 1995 NeXT Computer, Inc. All Rights Reserved */
/*
 * Copyright (c) 1982, 1985, 1986, 1988, 1993, 1994
 *	The Regents of the University of California.  All rights reserved.
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
 *	@(#)socket.h	8.4 (Berkeley) 2/21/94
 * $FreeBSD: src/sys/sys/socket.h,v 1.39.2.7 2001/07/03 11:02:01 ume Exp $
 */
/*
 * NOTICE: This file was modified by SPARTA, Inc. in 2005 to introduce
 * support for mandatory and extensible security protections.  This notice
 * is included in support of clause 2.2 (b) of the Apple Public License,
 * Version 2.0.
 */
    /*
 * Definitions related to sockets: types, address families, options.
 */
    /*
 * Data types.
 */
    /* XXX Not explicitly defined by POSIX, but function return types are */
    /* XXX Not explicitly defined by POSIX, but function return types are */
    /*
 * [XSI] The iovec structure shall be defined as described in <sys/uio.h>.
 */
    /*
 * Types
 */
    /* stream socket */
    /* datagram socket */
    /* raw-protocol interface */
    /* reliably-delivered message */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* sequenced packet stream */
    /*
 * Option flags per-socket.
 */
    /* turn on debugging info recording */
    /* socket has had listen() */
    /* allow local address reuse */
    /* keep connections alive */
    /* just use interface addresses */
    /* permit sending of broadcast msgs */
    /* bypass hardware when possible */
    /* linger on close if data present (in ticks) */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* leave received OOB data in line */
    /* allow local address & port reuse */
    /* timestamp received dgram traffic */
    /* Monotonically increasing timestamp on rcvd dgram */
    /* APPLE: Retain unread data */
                                        /*  (ATOMIC proto) */
    /* APPLE: Give hint when more data ready */
    /* APPLE: Want OOB in MSG_FLAG on receive */
    /* (!__APPLE__) */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /*
 * Additional options, not kept in so_options.
 */
    /* send buffer size */
    /* receive buffer size */
    /* send low-water mark */
    /* receive low-water mark */
    /* send timeout */
    /* receive timeout */
    /* get error status and clear */
    /* get socket type */
    /* socket's MAC label */
    /* socket's peer MAC label */
    /* APPLE: get 1st-packet byte count */
    /* APPLE: Install socket-level NKE */
    /* APPLE: No SIGPIPE on EPIPE */
    /* APPLE: Returns EADDRNOTAVAIL when src is not available anymore */
    /* APPLE: Get number of bytes currently in send socket buffer */
    /* APPLE: Allow reuse of port/socket by different userids */
    /* APPLE: send notification if there is a bind on a port which is already in use */
    /* APPLE: block on close until an upcall returns */
    /* linger on close if data present (in seconds) */
    /* APPLE: request local port randomization */
    /* To turn off some POSIX behavior */
    /* number of datagrams in receive socket buffer */
    /* Network service type */
    /* Get QoS marking in effect for socket */
    /*
 * Network Service Type for option SO_NET_SERVICE_TYPE
 *
 * The vast majority of sockets should use Best Effort that is the default
 * Network Service Type. Other Network Service Types have to be used only if
 * the traffic actually matches the description of the Network Service Type.
 *
 * Network Service Types do not represent priorities but rather describe
 * different categories of delay, jitter and loss parameters.
 * Those parameters may influence protocols from layer 4 protocols like TCP
 * to layer 2 protocols like Wi-Fi. The Network Service Type can determine
 * how the traffic is queued and scheduled by the host networking stack and
 * by other entities on the network like switches and routers. For example
 * for Wi-Fi, the Network Service Type can select the marking of the
 * layer 2 packet with the appropriate WMM Access Category.
 *
 * There is no point in attempting to game the system and use
 * a Network Service Type that does not correspond to the actual
 * traffic characteristic but one that seems to have a higher precedence.
 * The reason is that for service classes that have lower tolerance
 * for delay and jitter, the queues size is lower than for service
 * classes that are more tolerant to delay and jitter.
 *
 * For example using a voice service type for bulk data transfer will lead
 * to disastrous results as soon as congestion happens because the voice
 * queue overflows and packets get dropped. This is not only bad for the bulk
 * data transfer but it is also bad for VoIP apps that legitimately are using
 * the voice  service type.
 *
 * The characteristics of the Network Service Types are based on the service
 * classes defined in RFC 4594 "Configuration Guidelines for DiffServ Service
 * Classes"
 *
 * When system detects the outgoing interface belongs to a DiffServ domain
 * that follows the recommendation of the IETF draft "Guidelines for DiffServ to
 * IEEE 802.11 Mapping", the packet will marked at layer 3 with a DSCP value
 * that corresponds to Network Service Type.
 *
 * NET_SERVICE_TYPE_BE
 *	"Best Effort", unclassified/standard.  This is the default service
 *	class and cover the majority of the traffic.
 *
 * NET_SERVICE_TYPE_BK
 *	"Background", high delay tolerant, loss tolerant. elastic flow,
 *	variable size & long-lived. E.g: non-interactive network bulk transfer
 *	like synching or backup.
 *
 * NET_SERVICE_TYPE_RD
 *	"Responsive Data", a notch higher than "Best Effort", medium delay
 *	tolerant, elastic & inelastic flow, bursty, long-lived. E.g. email,
 *	instant messaging, for which there is a sense of interactivity and
 *	urgency (user waiting for output).
 *
 * NET_SERVICE_TYPE_OAM
 *	"Operations, Administration, and Management", medium delay tolerant,
 *	low-medium loss tolerant, elastic & inelastic flows, variable size.
 *	E.g. VPN tunnels.
 *
 * NET_SERVICE_TYPE_AV
 *	"Multimedia Audio/Video Streaming", medium delay tolerant, low-medium
 *	loss tolerant, elastic flow, constant packet interval, variable rate
 *	and size. E.g. video and audio playback with buffering.
 *
 * NET_SERVICE_TYPE_RV
 *	"Responsive Multimedia Audio/Video", low delay tolerant, low-medium
 *	loss tolerant, elastic flow, variable packet interval, rate and size.
 *	E.g. screen sharing.
 *
 * NET_SERVICE_TYPE_VI
 *	"Interactive Video", low delay tolerant, low-medium loss tolerant,
 *	elastic flow, constant packet interval, variable rate & size. E.g.
 *	video telephony.
 *
 * NET_SERVICE_TYPE_SIG
 *	"Signaling", low delay tolerant, low loss tolerant, inelastic flow,
 *	jitter tolerant, rate is bursty but short, variable size. E.g. SIP.
 *
 * NET_SERVICE_TYPE_VO
 *	"Interactive Voice", very low delay tolerant, very low loss tolerant,
 *	inelastic flow, constant packet rate, somewhat fixed size.
 *	E.g. VoIP.
 */
    /* Best effort */
    /* Background system initiated */
    /* Signaling */
    /* Interactive Video */
    /* Interactive Voice */
    /* Responsive Multimedia Audio/Video */
    /* Multimedia Audio/Video Streaming */
    /* Operations, Administration, and Management */
    /* Responsive Data */
    /* These are supported values for SO_NETSVC_MARKING_LEVEL */
    /* The outgoing network interface is not known */
    /* Default marking at layer 2 (for example Wi-Fi WMM) */
    /* Layer 3 DSCP marking and layer 2 marking for all Network Service Types */
    /* The system policy limits layer 3 DSCP marking and layer 2 marking
	                                         * to background Network Service Types */
    /* connectx() flag parameters */
    /* resume connect() on read/write */
    /* data is idempotent */
    /* data includes security that replaces the TFO-cookie */
    /* sockaddr endpoints */
    /* optional source interface */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "305:8"]
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::_types_h::__uint8_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::libc;
    /* !_SYS_SOCKET_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timeval.h:25"]
pub mod _timeval_h {
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
    #[src_loc = "34:1"]
    pub struct timeval {
        pub tv_sec: __darwin_time_t,
        pub tv_usec: __darwin_suseconds_t,
        /* and microseconds */
    }
    use super::_types_h::__darwin_time_t;
    use super::sys__types_h::__darwin_suseconds_t;
    /* _STRUCT_TIMEVAL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:25"]
pub mod _uint8_t_h {
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
    pub type uint8_t = libc::c_uchar;
    use super::libc;
    /* _UINT8_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:25"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:25"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netdb.h:25"]
pub mod netdb_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "147:1"]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_canonname: *mut libc::c_char,
        pub ai_addr: *mut sockaddr,
        pub ai_next: *mut addrinfo,
        /* next structure in linked list */
    }
    use super::libc;
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
    /* !_NETDB_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src = "/Users/cameron/github/openconnect/openconnect.h:25"]
pub mod openconnect_h {
    /* ***************************************************************************/
    /* Authentication form processing */
    /* char * fields are static (owned by XML parser) and don't need to be
   freed by the form handling code — except for value, which for TEXT
   and PASSWORD options is allocated by openconnect_set_option_value()
   when process_form() interacts with the user and must be freed. */
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
    /* All fields are static, owned by the XML parser */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "235:1"]
    pub struct oc_choice {
        pub name: *mut libc::c_char,
        pub label: *mut libc::c_char,
        pub auth_type: *mut libc::c_char,
        pub override_name: *mut libc::c_char,
        pub override_label: *mut libc::c_char,
        pub second_auth: libc::c_int,
        pub secondary_username: *mut libc::c_char,
        pub secondary_username_editable: libc::c_int,
        pub noaaa: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "249:1"]
    pub struct oc_form_opt_select {
        pub form: oc_form_opt,
        pub nr_choices: libc::c_int,
        pub choices: *mut *mut oc_choice,
    }
    /* All char * fields are static, owned by the XML parser */
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
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "268:1"]
    pub struct oc_split_include {
        pub route: *const libc::c_char,
        pub next: *mut oc_split_include,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "273:1"]
    pub struct oc_ip_info {
        pub addr: *const libc::c_char,
        pub netmask: *const libc::c_char,
        pub addr6: *const libc::c_char,
        pub netmask6: *const libc::c_char,
        pub dns: [*const libc::c_char; 3],
        pub nbns: [*const libc::c_char; 3],
        pub domain: *const libc::c_char,
        pub proxy_pac: *const libc::c_char,
        pub mtu: libc::c_int,
        pub split_dns: *mut oc_split_include,
        pub split_includes: *mut oc_split_include,
        pub split_excludes: *mut oc_split_include,
        pub gateway_addr: *mut libc::c_char,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "294:1"]
    pub struct oc_vpn_option {
        pub option: *mut libc::c_char,
        pub value: *mut libc::c_char,
        pub next: *mut oc_vpn_option,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "300:1"]
    pub struct oc_stats {
        pub tx_pkts: uint64_t,
        pub tx_bytes: uint64_t,
        pub rx_pkts: uint64_t,
        pub rx_bytes: uint64_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "307:1"]
    pub struct oc_cert {
        pub der_len: libc::c_int,
        pub der_data: *mut libc::c_uchar,
        pub reserved: *mut libc::c_void,
    }
    /* ***************************************************************************/
    /* Byte commands to write into the cmd_fd:
 *
 *  CANCEL closes network connections, logs off the session (cookie)
 *    and shuts down the tun device.
 *  PAUSE closes network connections and returns. The caller is expected
 *    to call openconnect_mainloop() again soon.
 *  DETACH closes network connections and shuts down the tun device.
 *    It is not legal to call openconnect_mainloop() again after this,
 *    but a new instance of openconnect can be started using the same
 *    cookie.
 *  STATS calls the stats_handler.
 */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "340:1"]
    pub struct openconnect_info {
        pub proto: *const vpn_proto,
        pub ic_legacy_to_utf8: iconv_t,
        pub ic_utf8_to_legacy: iconv_t,
        pub redirect_url: *mut libc::c_char,
        pub redirect_type: libc::c_int,
        pub esp_hmac: libc::c_uchar,
        pub esp_enc: libc::c_uchar,
        pub esp_compr: libc::c_uchar,
        pub esp_replay_protect: uint32_t,
        pub esp_lifetime_bytes: uint32_t,
        pub esp_lifetime_seconds: uint32_t,
        pub esp_ssl_fallback: uint32_t,
        pub current_esp_in: libc::c_int,
        pub old_esp_maxseq: libc::c_int,
        pub esp_in: [esp; 2],
        pub esp_out: esp,
        pub enc_key_len: libc::c_int,
        pub hmac_key_len: libc::c_int,
        pub hmac_out_len: libc::c_int,
        pub esp_magic: uint32_t,
        pub tncc_fd: libc::c_int,
        pub csd_xmltag: *const libc::c_char,
        pub csd_nostub: libc::c_int,
        pub platname: *mut libc::c_char,
        pub mobile_platform_version: *mut libc::c_char,
        pub mobile_device_type: *mut libc::c_char,
        pub mobile_device_uniqueid: *mut libc::c_char,
        pub csd_token: *mut libc::c_char,
        pub csd_ticket: *mut libc::c_char,
        pub csd_stuburl: *mut libc::c_char,
        pub csd_starturl: *mut libc::c_char,
        pub csd_waiturl: *mut libc::c_char,
        pub csd_preurl: *mut libc::c_char,
        pub csd_scriptname: *mut libc::c_char,
        pub opaque_srvdata: *mut xmlNode,
        pub profile_url: *mut libc::c_char,
        pub profile_sha1: *mut libc::c_char,
        pub proxy_factory: *mut pxProxyFactory,
        pub proxy_type: *mut libc::c_char,
        pub proxy: *mut libc::c_char,
        pub proxy_port: libc::c_int,
        pub proxy_fd: libc::c_int,
        pub proxy_user: *mut libc::c_char,
        pub proxy_pass: *mut libc::c_char,
        pub proxy_close_during_auth: libc::c_int,
        pub retry_on_auth_fail: libc::c_int,
        pub try_http_auth: libc::c_int,
        pub http_auth: [http_auth_state; 4],
        pub proxy_auth: [http_auth_state; 4],
        pub localname: *mut libc::c_char,
        pub hostname: *mut libc::c_char,
        pub unique_hostname: *mut libc::c_char,
        pub port: libc::c_int,
        pub urlpath: *mut libc::c_char,
        pub cert_expire_warning: libc::c_int,
        pub cert: *mut libc::c_char,
        pub sslkey: *mut libc::c_char,
        pub cert_password: *mut libc::c_char,
        pub cafile: *mut libc::c_char,
        pub no_system_trust: libc::c_uint,
        pub xmlconfig: *const libc::c_char,
        pub xmlsha1: [libc::c_char; 41],
        pub authgroup: *mut libc::c_char,
        pub nopasswd: libc::c_int,
        pub xmlpost: libc::c_int,
        pub dtls_ciphers: *mut libc::c_char,
        pub dtls12_ciphers: *mut libc::c_char,
        pub csd_wrapper: *mut libc::c_char,
        pub no_http_keepalive: libc::c_int,
        pub dump_http_traffic: libc::c_int,
        pub token_mode: libc::c_int,
        pub token_bypassed: libc::c_int,
        pub token_tries: libc::c_int,
        pub token_time: time_t,
        pub stoken_ctx: *mut stoken_ctx,
        pub stoken_pin: *mut libc::c_char,
        pub stoken_concat_pin: libc::c_int,
        pub stoken_interval: libc::c_int,
        pub oath_secret: *mut libc::c_char,
        pub oath_secret_len: size_t,
        pub oath_hmac_alg: C2RustUnnamed_13,
        pub hotp_secret_format: C2RustUnnamed_12,
        pub pcsc: *mut oc_pcsc_ctx,
        pub yubikey_pwhash: [libc::c_uchar; 16],
        pub lock_token: openconnect_lock_token_vfn,
        pub unlock_token: openconnect_unlock_token_vfn,
        pub tok_cbdata: *mut libc::c_void,
        pub peer_cert: *mut libc::c_void,
        pub peer_cert_sha1_raw: [uint8_t; 20],
        pub peer_cert_sha256_raw: [uint8_t; 32],
        pub peer_cert_hash: *mut libc::c_char,
        pub cert_list_handle: *mut libc::c_void,
        pub cert_list_size: libc::c_int,
        pub cookie: *mut libc::c_char,
        pub cookies: *mut oc_vpn_option,
        pub cstp_options: *mut oc_vpn_option,
        pub dtls_options: *mut oc_vpn_option,
        pub script_env: *mut oc_vpn_option,
        pub csd_env: *mut oc_vpn_option,
        pub pfs: libc::c_uint,
        pub no_tls13: libc::c_uint,
        pub cert_x509: *mut X509,
        pub https_ctx: *mut SSL_CTX,
        pub https_ssl: *mut SSL,
        pub ttls_bio_meth: *mut BIO_METHOD,
        pub ttls_pushbuf: *mut oc_text_buf,
        pub ttls_eap_ident: uint8_t,
        pub ttls_recvbuf: *mut libc::c_uchar,
        pub ttls_recvpos: libc::c_int,
        pub ttls_recvlen: libc::c_int,
        pub pin_cache: *mut pin_cache,
        pub ssl_times: keepalive_info,
        pub owe_ssl_dpd_response: libc::c_int,
        pub deflate_pkt_size: libc::c_int,
        pub deflate_pkt: *mut pkt,
        pub pending_deflated_pkt: *mut pkt,
        pub current_ssl_pkt: *mut pkt,
        pub oncp_control_queue: pkt_q,
        pub oncp_rec_size: libc::c_int,
        pub cstp_pkt: *mut pkt,
        pub dtls_pkt: *mut pkt,
        pub tun_pkt: *mut pkt,
        pub pkt_trailer: libc::c_int,
        pub inflate_strm: z_stream,
        pub inflate_adler32: uint32_t,
        pub deflate_strm: z_stream,
        pub deflate_adler32: uint32_t,
        pub disable_ipv6: libc::c_int,
        pub reconnect_timeout: libc::c_int,
        pub reconnect_interval: libc::c_int,
        pub dtls_attempt_period: libc::c_int,
        pub new_dtls_started: time_t,
        pub dtls_ctx: *mut SSL_CTX,
        pub dtls_ssl: *mut SSL,
        pub cstp_cipher: *mut libc::c_char,
        pub dtls_state: libc::c_int,
        pub dtls_need_reconnect: libc::c_int,
        pub dtls_times: keepalive_info,
        pub dtls_session_id: [libc::c_uchar; 32],
        pub dtls_secret: [libc::c_uchar; 48],
        pub dtls_app_id: [libc::c_uchar; 32],
        pub dtls_app_id_size: libc::c_uint,
        pub ift_seq: uint32_t,
        pub cisco_dtls12: libc::c_int,
        pub dtls_cipher: *mut libc::c_char,
        pub vpnc_script: *mut libc::c_char,
        pub uid_csd_given: libc::c_int,
        pub uid_csd: uid_t,
        pub gid_csd: gid_t,
        pub uid: uid_t,
        pub gid: gid_t,
        pub use_tun_script: libc::c_int,
        pub script_tun: libc::c_int,
        pub ifname: *mut libc::c_char,
        pub cmd_ifname: *mut libc::c_char,
        pub reqmtu: libc::c_int,
        pub basemtu: libc::c_int,
        pub banner: *const libc::c_char,
        pub ip_info: oc_ip_info,
        pub cstp_basemtu: libc::c_int,
        pub idle_timeout: libc::c_int,
        pub _select_nfds: libc::c_int,
        pub _select_rfds: fd_set,
        pub _select_wfds: fd_set,
        pub _select_efds: fd_set,
        pub tun_fd: libc::c_int,
        pub ssl_fd: libc::c_int,
        pub dtls_fd: libc::c_int,
        pub dtls_tos_current: libc::c_int,
        pub dtls_pass_tos: libc::c_int,
        pub dtls_tos_proto: libc::c_int,
        pub dtls_tos_optname: libc::c_int,
        pub cmd_fd: libc::c_int,
        pub cmd_fd_write: libc::c_int,
        pub got_cancel_cmd: libc::c_int,
        pub got_pause_cmd: libc::c_int,
        pub cancel_type: libc::c_char,
        pub incoming_queue: pkt_q,
        pub outgoing_queue: pkt_q,
        pub max_qlen: libc::c_int,
        pub stats: oc_stats,
        pub stats_handler: openconnect_stats_vfn,
        pub peer_addrlen: socklen_t,
        pub peer_addr: *mut sockaddr,
        pub dtls_addr: *mut sockaddr,
        pub dtls_local_port: libc::c_int,
        pub req_compr: libc::c_int,
        pub cstp_compr: libc::c_int,
        pub dtls_compr: libc::c_int,
        pub is_dyndns: libc::c_int,
        pub useragent: *mut libc::c_char,
        pub version_string: *mut libc::c_char,
        pub quit_reason: *const libc::c_char,
        pub verbose: libc::c_int,
        pub cbdata: *mut libc::c_void,
        pub validate_peer_cert: openconnect_validate_peer_cert_vfn,
        pub write_new_config: openconnect_write_new_config_vfn,
        pub process_auth_form: openconnect_process_auth_form_vfn,
        pub progress: openconnect_progress_vfn,
        pub protect_socket: openconnect_protect_socket_vfn,
        pub getaddrinfo_override: openconnect_getaddrinfo_vfn,
        pub setup_tun: openconnect_setup_tun_vfn,
        pub reconnected: openconnect_reconnected_vfn,
        pub ssl_read: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                  _: *mut libc::c_char,
                                                  _: size_t) -> libc::c_int>,
        pub ssl_gets: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                  _: *mut libc::c_char,
                                                  _: size_t) -> libc::c_int>,
        pub ssl_write: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                   _: *mut libc::c_char,
                                                   _: size_t) -> libc::c_int>,
        /* All strings are UTF-8. If operating in a legacy environment where
   nl_langinfo(CODESET) returns anything other than UTF-8, or on Windows,
   the library will take appropriate steps to convert back to the legacy
   character set (or UTF-16) for file handling and wherever else it is
   appropriate to do so. Library functions may (but probably don't yet)
   return -EILSEQ if passed invalid UTF-8 strings. */
        /* Unlike previous versions of openconnect, no functions will take ownership
   of the provided strings. */
        /* Provide environment variables to be set in the CSD trojan environment
   before spawning it. Some callers may need to set $TMPDIR, $PATH and
   other such things if not running from a standard UNIX-like environment.
   To ensure that a variable is unset, pass its name with value==NULL.
   To clear all settings and allow the CSD trojan to inherit an unmodified
   environment, call with name==NULL. */
    }
    /* This string is static, valid only while the connection lasts. If you
 * are going to cache this to remember which certs the user has accepted,
 * make sure you also store the host/port for which it was accepted and
 * don't just accept this cert from *anywhere*. Also use use the check
 * function below instead of manually comparing. When this function
 * returns a string which *doesn't* match the previously-stored hash
 * matched with openconnect_check_peer_cert_hash(), you should store
 * the new result from this function in place of the old. It means
 * we have upgraded to a better hash function. */
    /* Check if the current peer certificate matches a hash previously
 * obtained from openconect_get_peer_cert_hash(). Clients should not
 * attempt to do this using strcmp() and the *current* result of
 * openconnect_get_peer_cert_hash() because it might use
 * a different hash function today. This function will get it right.
 * Returns 0 on match; 1 on mismatch, -errno on failure. */
    /* The buffers returned by these two functions must be freed with
   openconnect_free_cert_info(), especially on Windows. */
    /* Returns the length of the created DER output, in a newly-allocated buffer
   that will need to be freed by openconnect_free_cert_info(). */
    /* Creates a list of all certs in the peer's chain, returning the
   number of certs in the chain (or <0 on error). Only valid inside the
   validate_peer_cert callback. The caller should free the chain,
   but should not modify the contents. */
    /* Contains a comma-separated list of authentication methods to enabled.
   Currently supported: Negotiate,NTLM,Digest,Basic */
    /* These are strictly cosmetic. The strings differ depending on
 * whether OpenSSL or GnuTLS is being used. And even depending on the
 * version of GnuTLS. Do *not* attempt to do anything meaningful based
 * on matching these strings; if you want to do something like that then
 * ask for an API that *does* offer you what you need. */
    /* These return a descriptive string of the compression algorithm 
 * in use (LZS, LZ4, ...). If no compression then NULL is returned. */
    /* Returns the IP address of the exact host to which the connection
 * was made. In --cookieonly mode or in any other scenario involving
 * a "two stage" connection, it is important to reconnect by IP because
 * the server side may be using DNS trickery for load balancing.
 *
 * If the IP address is unavailable due to the use of a proxy, this will
 * fall back to returning the DNS name. */
    /* Returns the hostname parsed out of the server name URL. This is
 * intended to be used by the validate_peer_cert callback to check that
 * the certificate matches the server name. */
    /* Some software tokens, such as HOTP tokens, include a counter which
 * needs to be stored in persistent storage.
 *
 * For such tokens, the lock function is first invoked to obtain a lock
 * on the storage because we're about to generate a new code. It is
 * permitted to call openconnect_set_token_mode() from the lock function,
 * if the token storage has been updated since it was first loaded. The
 * token mode must not change; only the token secret.
 *
 * The unlock function is called when a token code has been generated,
 * with a new token secret to be written to the persistent storage. The
 * secret will be in the same format as it was originally received by
 * openconnect_set_token_mode(). The new token may be NULL if an error
 * was encountered generating the code, in which case it is only
 * necessary for the callback function to unlock the storage.
 */
    /* Legacy stoken-only function; do not use */
    /* The size must be 41 bytes, since that's the size of a 20-byte SHA1
   represented as hex with a trailing NUL. */
    /* call this function to disable the system trust from being used to
 * verify the server certificate. @val is a boolean value.
 *
 * For backwards compatibility reasons this is enabled by default.
 */
    /* Valid choices are: "linux", "linux-64", "win", "mac-intel",
   "android", and "apple-ios". This also selects the corresponding CSD
   trojan binary. */
    /* The returned structures are owned by the library and may be freed/replaced
   due to rekey or reconnect. Assume that once the mainloop starts, the
   pointers are no longer valid. For similar reasons, it is unsafe to call
   this function from another thread. */
    /* If this is set, then openconnect_obtain_cookie() will abort and return
   failure if the file descriptor is readable. Typically a user may create
   a pair of pipes with the pipe(2) system call, hand the readable one to
   this function, and then write a byte to the other end if it ever wants
   to cancel the connection. This way, a multi-threaded UI (which will be
   running openconnect_obtain_cookie() in a separate thread since it blocks)
   has the ability to cancel that call, reap its thread and free the
   vpninfo structure (or retry). An 'fd' argument of -1 will render the
   cancellation mechanism inactive. */
    /* Create a nonblocking pipe used to send cancellations and other commands
   to the library. This returns a file descriptor to the write side of
   the pipe. Both sides will be closed by openconnect_vpninfo_free().
   This replaces openconnect_set_cancel_fd(). */
    /* Open CSTP connection; on success, IP information will be available. */
    /* Create a tun device through the OS kernel (typical use case). Both
   strings are optional and can be NULL if desired. */
    /* Pass traffic to a script program (no tun device). */
    /* Caller will provide a file descriptor for the tunnel traffic. */
    /* Optional call to enable DTLS on the connection. */
    /* Start the main loop; exits if OC_CMD_CANCEL is received on cmd_fd or
   the remote site aborts. */
    /* The first (privdata) argument to each of these functions is either
   the privdata argument provided to openconnect_vpninfo_new_with_cbdata(),
   or if that argument was NULL then it'll be the vpninfo itself. */
    /* When the server's certificate fails validation via the normal means,
   this function is called with the offending certificate along with
   a textual reason for the failure (which may not be translated, if
   it comes directly from OpenSSL, but will be if it is rejected for
   "certificate does not match hostname", because that check is done
   in OpenConnect and *is* translated). The function shall return zero
   if the certificate is (or has in the past been) explicitly accepted
   by the user, and non-zero to abort the connection. */
    /* On a successful connection, the server may provide us with a new XML
   configuration file. This contains the list of servers that can be
   chosen by the user to connect to, amongst other stuff that we mostly
   ignore. By "new", we mean that the SHA1 indicated by the server does
   not match the SHA1 set with the openconnect_set_xmlsha1() above. If
   they don't match, or openconnect_set_xmlsha1() has not been called,
   then the new XML is downloaded and this function is invoked. */
    /* Handle an authentication form, requesting input from the user.
 * Return value:
 *  < 0, on error
 *  = 0, when form was parsed and POST required
 *  = 1, when response was cancelled by user
 */
    /* Logging output which the user *may* want to see. */
    /* Callback to allow binding a newly created socket's file descriptor to
   a specific interface, e.g. with SO_BINDTODEVICE. This tells the kernel
   not to route the traffic in question over the VPN tunnel. */
    /* Callback for obtaining traffic stats via OC_CMD_STATS.
 */
    /* SSL certificate capabilities. openconnect_has_pkcs11_support() means that we
   can accept PKCS#11 URLs in place of filenames, for the certificate and key. */
    /* The OpenSSL TPM ENGINE stores keys in a PEM file labelled with the string
   -----BEGIN TSS KEY BLOB-----. */
    /* Software token capabilities. */
    /* Query and select from among supported protocols */
    /* Callback for configuring the interface after MTU detection finishes. */
    /* Callback for indicating that a TCP reconnection succeeded. */
    #[src_loc = "690:1"]
    pub type openconnect_reconnected_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    #[src_loc = "685:1"]
    pub type openconnect_setup_tun_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    #[src_loc = "680:1"]
    pub type openconnect_getaddrinfo_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char,
                                    _: *const addrinfo, _: *mut *mut addrinfo)
                   -> libc::c_int>;
    #[src_loc = "644:1"]
    pub type openconnect_protect_socket_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int)
                   -> ()>;
    #[src_loc = "630:1"]
    pub type openconnect_progress_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int,
                                    _: *const libc::c_char, _: ...) -> ()>;
    #[src_loc = "627:1"]
    pub type openconnect_process_auth_form_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut oc_auth_form) -> libc::c_int>;
    #[src_loc = "619:1"]
    pub type openconnect_write_new_config_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char, _: libc::c_int)
                   -> libc::c_int>;
    #[src_loc = "610:1"]
    pub type openconnect_validate_peer_cert_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char) -> libc::c_int>;
    #[src_loc = "654:1"]
    pub type openconnect_stats_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const oc_stats)
                   -> ()>;
    #[src_loc = "478:1"]
    pub type openconnect_unlock_token_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char) -> libc::c_int>;
    #[src_loc = "477:1"]
    pub type openconnect_lock_token_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
    use super::libc;
    use super::_uint64_t_h::uint64_t;
    use super::openconnect_internal_h::{vpn_proto, esp, http_auth_state,
                                        C2RustUnnamed_13, C2RustUnnamed_12,
                                        oc_pcsc_ctx, oc_text_buf, pin_cache,
                                        keepalive_info, pkt, pkt_q};
    use super::iconv_h::iconv_t;
    use super::_uint32_t_h::uint32_t;
    use super::tree_h::xmlNode;
    use super::proxy_h::pxProxyFactory;
    use super::_time_t_h::time_t;
    use super::stoken_h::stoken_ctx;
    use super::_size_t_h::size_t;
    use super::_uint8_t_h::uint8_t;
    use super::ossl_typ_h::{X509, SSL_CTX, SSL};
    use super::bio_h::BIO_METHOD;
    use super::zlib_h::z_stream;
    use super::_uid_t_h::uid_t;
    use super::_gid_t_h::gid_t;
    use super::_fd_def_h::fd_set;
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
    use super::netdb_h::addrinfo;
    /* __OPENCONNECT_H__ */
}
#[header_src = "/Users/cameron/github/openconnect/openconnect-internal.h:25"]
pub mod openconnect_internal_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "304:1"]
    pub struct pkt_q {
        pub head: *mut pkt,
        pub tail: *mut *mut pkt,
        pub count: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "123:1"]
    pub struct pkt {
        pub len: libc::c_int,
        pub next: *mut pkt,
        pub c2rust_unnamed: C2RustUnnamed,
        pub data: [libc::c_uchar; 0],
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "126:2"]
    pub union C2RustUnnamed {
        pub esp: C2RustUnnamed_4,
        pub oncp: C2RustUnnamed_3,
        pub cstp: C2RustUnnamed_2,
        pub gpst: C2RustUnnamed_1,
        pub pulse: C2RustUnnamed_0,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "146:3"]
    pub struct C2RustUnnamed_0 {
        pub pad: [libc::c_uchar; 8],
        pub vendor: uint32_t,
        pub type_0: uint32_t,
        pub len: uint32_t,
        pub ident: uint32_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "142:3"]
    pub struct C2RustUnnamed_1 {
        pub pad: [libc::c_uchar; 8],
        pub hdr: [libc::c_uchar; 16],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "138:3"]
    pub struct C2RustUnnamed_2 {
        pub pad: [libc::c_uchar; 16],
        pub hdr: [libc::c_uchar; 8],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "133:3"]
    pub struct C2RustUnnamed_3 {
        pub pad: [libc::c_uchar; 2],
        pub rec: [libc::c_uchar; 2],
        pub kmp: [libc::c_uchar; 20],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "127:3"]
    pub struct C2RustUnnamed_4 {
        pub spi: uint32_t,
        pub seq: uint32_t,
        pub iv: [libc::c_uchar; 16],
        pub payload: [libc::c_uchar; 0],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "189:1"]
    pub struct keepalive_info {
        pub dpd: libc::c_int,
        pub keepalive: libc::c_int,
        pub rekey: libc::c_int,
        pub rekey_method: libc::c_int,
        pub last_rekey: time_t,
        pub last_tx: time_t,
        pub last_rx: time_t,
        pub last_dpd: time_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "200:1"]
    pub struct pin_cache {
        pub next: *mut pin_cache,
        pub token: *mut libc::c_char,
        pub pin: *mut libc::c_char,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "206:1"]
    pub struct oc_text_buf {
        pub data: *mut libc::c_char,
        pub pos: libc::c_int,
        pub buf_len: libc::c_int,
        pub error: libc::c_int,
    }
    #[src_loc = "471:2"]
    pub type C2RustUnnamed_12 = libc::c_uint;
    #[src_loc = "475:3"]
    pub const HOTP_SECRET_PSKC: C2RustUnnamed_12 = 4;
    #[src_loc = "474:3"]
    pub const HOTP_SECRET_HEX: C2RustUnnamed_12 = 3;
    #[src_loc = "473:3"]
    pub const HOTP_SECRET_RAW: C2RustUnnamed_12 = 2;
    #[src_loc = "472:3"]
    pub const HOTP_SECRET_BASE32: C2RustUnnamed_12 = 1;
    #[src_loc = "466:2"]
    pub type C2RustUnnamed_13 = libc::c_uint;
    #[src_loc = "469:3"]
    pub const OATH_ALG_HMAC_SHA512: C2RustUnnamed_13 = 2;
    #[src_loc = "468:3"]
    pub const OATH_ALG_HMAC_SHA256: C2RustUnnamed_13 = 1;
    #[src_loc = "467:3"]
    pub const OATH_ALG_HMAC_SHA1: C2RustUnnamed_13 = 0;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "237:1"]
    pub struct http_auth_state {
        pub state: libc::c_int,
        pub challenge: *mut libc::c_char,
        pub c2rust_unnamed: C2RustUnnamed_14,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "240:2"]
    pub union C2RustUnnamed_14 {
        pub c2rust_unnamed: C2RustUnnamed_16,
        pub c2rust_unnamed_0: C2RustUnnamed_15,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "258:3"]
    pub struct C2RustUnnamed_15 {
        pub ntlm_helper_fd: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "242:3"]
    pub struct C2RustUnnamed_16 {
        pub gss_target_name: gss_name_t,
        pub gss_context: gss_ctx_id_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "347:1"]
    pub struct esp {
        pub hmac: *mut HMAC_CTX,
        pub cipher: *mut EVP_CIPHER_CTX,
        pub seq_backlog: uint64_t,
        pub seq: uint64_t,
        pub spi: uint32_t,
        pub enc_key: [libc::c_uchar; 64],
        pub hmac_key: [libc::c_uchar; 64],
        pub iv: [libc::c_uchar; 16],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "265:1"]
    pub struct vpn_proto {
        pub name: *const libc::c_char,
        pub pretty_name: *const libc::c_char,
        pub description: *const libc::c_char,
        pub udp_protocol: *const libc::c_char,
        pub flags: libc::c_uint,
        pub vpn_close_session: Option<unsafe extern "C" fn(_:
                                                               *mut openconnect_info,
                                                           _:
                                                               *const libc::c_char)
                                          -> libc::c_int>,
        pub obtain_cookie: Option<unsafe extern "C" fn(_:
                                                           *mut openconnect_info)
                                      -> libc::c_int>,
        pub tcp_connect: Option<unsafe extern "C" fn(_: *mut openconnect_info)
                                    -> libc::c_int>,
        pub tcp_mainloop: Option<unsafe extern "C" fn(_:
                                                          *mut openconnect_info,
                                                      _: *mut libc::c_int,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub add_http_headers: Option<unsafe extern "C" fn(_:
                                                              *mut openconnect_info,
                                                          _: *mut oc_text_buf)
                                         -> ()>,
        pub udp_setup: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
        pub udp_mainloop: Option<unsafe extern "C" fn(_:
                                                          *mut openconnect_info,
                                                      _: *mut libc::c_int,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub udp_close: Option<unsafe extern "C" fn(_: *mut openconnect_info)
                                  -> ()>,
        pub udp_shutdown: Option<unsafe extern "C" fn(_:
                                                          *mut openconnect_info)
                                     -> ()>,
        pub udp_send_probes: Option<unsafe extern "C" fn(_:
                                                             *mut openconnect_info)
                                        -> libc::c_int>,
        pub udp_catch_probe: Option<unsafe extern "C" fn(_:
                                                             *mut openconnect_info,
                                                         _: *mut pkt)
                                        -> libc::c_int>,
    }
    #[derive ( Copy, Clone )]
    #[repr(C, packed)]
    #[src_loc = "1186:1"]
    pub struct oc_packed_uint32_t {
        pub d: uint32_t,
    }
    #[inline]
    #[src_loc = "1193:1"]
    pub unsafe extern "C" fn load_be32(mut _p: *const libc::c_void)
     -> uint32_t {
        let mut p: *const oc_packed_uint32_t =
            _p as *const oc_packed_uint32_t;
        return if 0 != 0 {
                   (((*p).d & 0xff000000u32) >> 24i32 |
                        ((*p).d & 0xff0000i32 as libc::c_uint) >> 8i32 |
                        ((*p).d & 0xff00i32 as libc::c_uint) << 8i32) |
                       ((*p).d & 0xffi32 as libc::c_uint) << 24i32
               } else { _OSSwapInt32((*p).d) };
    }
    use super::libc;
    use super::_uint32_t_h::uint32_t;
    use super::_time_t_h::time_t;
    use super::gssapi_h::{gss_name_t, gss_ctx_id_t};
    use super::hmac_h::HMAC_CTX;
    use super::ossl_typ_h::EVP_CIPHER_CTX;
    use super::_uint64_t_h::uint64_t;
    use super::openconnect_h::{openconnect_info, oc_auth_form};
    use super::_size_t_h::size_t;
    use super::_fd_def_h::fd_set;
    use super::_stdio_h::FILE;
    use super::_OSByteOrder_h::_OSSwapInt32;
    extern "C" {
        #[src_loc = "363:1"]
        pub type oc_pcsc_ctx;
        #[no_mangle]
        #[src_loc = "835:1"]
        pub fn openconnect_utf8_to_legacy(vpninfo: *mut openconnect_info,
                                          utf8: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[src_loc = "936:1"]
        pub fn openconnect_print_err_cb(str: *const libc::c_char, len: size_t,
                                        ptr: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "906:1"]
        pub fn pulse_eap_ttls_recv(vpninfo: *mut openconnect_info,
                                   data: *mut libc::c_void, len: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "905:1"]
        pub fn pulse_eap_ttls_send(vpninfo: *mut openconnect_info,
                                   data: *const libc::c_void,
                                   len: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "947:1"]
        pub fn is_cancel_pending(vpninfo: *mut openconnect_info,
                                 fds: *mut fd_set) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "945:1"]
        pub fn cmd_fd_set(vpninfo: *mut openconnect_info, fds: *mut fd_set,
                          maxfd: *mut libc::c_int);
        #[no_mangle]
        #[src_loc = "929:1"]
        pub fn string_is_hostname(str: *const libc::c_char) -> libc::c_uint;
        #[no_mangle]
        #[src_loc = "930:1"]
        pub fn connect_https_socket(vpninfo: *mut openconnect_info)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "931:1"]
        pub fn request_passphrase(vpninfo: *mut openconnect_info,
                                  label: *const libc::c_char,
                                  response: *mut *mut libc::c_char,
                                  fmt: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "951:1"]
        pub fn openconnect_fopen_utf8(vpninfo: *mut openconnect_info,
                                      fname: *const libc::c_char,
                                      mode: *const libc::c_char) -> *mut FILE;
        #[no_mangle]
        #[src_loc = "965:1"]
        pub fn load_pkcs11_key(vpninfo: *mut openconnect_info) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "966:1"]
        pub fn load_pkcs11_certificate(vpninfo: *mut openconnect_info)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1072:1"]
        pub fn free_pass(p: *mut *mut libc::c_char);
        #[no_mangle]
        #[src_loc = "1137:1"]
        pub fn process_auth_form(vpninfo: *mut openconnect_info,
                                 form: *mut oc_auth_form) -> libc::c_int;
    }
    /* __OPENCONNECT_INTERNAL_H__ */
    /* !Not known to be little-endian */
}
#[header_src =
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ossl_typ.h:25"]
pub mod ossl_typ_h {
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
    #[src_loc = "178:1"]
    pub type SSL = ssl_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "178:9"]
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
                                                         _:
                                                             *mut X509_STORE_CTX)
                                        -> libc::c_int>,
        pub info_callback: Option<unsafe extern "C" fn(_: *const SSL,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
        pub error: libc::c_int,
        pub error_code: libc::c_int,
        pub psk_client_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                             _:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_char,
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
                                                         _:
                                                             *mut libc::c_uchar,
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
        /* If placed in pkcs12.h, we end up with a circular depency with pkcs7.h */
        /* Nothing */
        /* Nothing */
        /* Callback types for crypto.h */
        /* def HEADER_OPENSSL_TYPES_H */
    }
    #[src_loc = "120:1"]
    pub type BIGNUM = bignum_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "120:9"]
    pub struct bignum_st {
        pub d: *mut libc::c_ulong,
        pub top: libc::c_int,
        pub dmax: libc::c_int,
        pub neg: libc::c_int,
        pub flags: libc::c_int,
    }
    #[src_loc = "179:1"]
    pub type SSL_CTX = ssl_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "179:9"]
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
                                                           _:
                                                               *mut SSL_SESSION)
                                          -> ()>,
        pub get_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_st,
                                                        _: *mut libc::c_uchar,
                                                        _: libc::c_int,
                                                        _: *mut libc::c_int)
                                       -> *mut SSL_SESSION>,
        pub stats: C2RustUnnamed_9,
        pub references: libc::c_int,
        pub app_verify_callback: Option<unsafe extern "C" fn(_:
                                                                 *mut X509_STORE_CTX,
                                                             _:
                                                                 *mut libc::c_void)
                                            -> libc::c_int>,
        pub app_verify_arg: *mut libc::c_void,
        pub default_passwd_callback: Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_char,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     *mut libc::c_void)
                                                -> libc::c_int>,
        pub default_passwd_callback_userdata: *mut libc::c_void,
        pub client_cert_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                        _: *mut *mut X509,
                                                        _: *mut *mut EVP_PKEY)
                                       -> libc::c_int>,
        pub app_gen_cookie_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                           _:
                                                               *mut libc::c_uchar,
                                                           _:
                                                               *mut libc::c_uint)
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
        pub default_verify_callback: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     *mut X509_STORE_CTX)
                                                -> libc::c_int>,
        pub generate_session_id: GEN_SESSION_CB,
        pub param: *mut X509_VERIFY_PARAM,
        pub quiet_shutdown: libc::c_int,
        pub max_send_fragment: libc::c_uint,
        pub client_cert_engine: *mut ENGINE,
        pub tlsext_servername_callback: Option<unsafe extern "C" fn(_:
                                                                        *mut SSL,
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
                                                              _:
                                                                  *mut HMAC_CTX,
                                                              _: libc::c_int)
                                             -> libc::c_int>,
        pub tlsext_status_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                          _:
                                                              *mut libc::c_void)
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
                                                             _:
                                                                 *mut libc::c_char,
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
        pub next_protos_advertised_cb: Option<unsafe extern "C" fn(_:
                                                                       *mut SSL,
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
                                                        _:
                                                            *const libc::c_uchar,
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
    #[src_loc = "132:1"]
    pub type EVP_MD_CTX = env_md_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "132:9"]
    pub struct env_md_ctx_st {
        pub digest: *const EVP_MD,
        pub engine: *mut ENGINE,
        pub flags: libc::c_ulong,
        pub md_data: *mut libc::c_void,
        pub pctx: *mut EVP_PKEY_CTX,
        pub update: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                                _: *const libc::c_void,
                                                _: size_t) -> libc::c_int>,
    }
    #[src_loc = "138:1"]
    pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
    #[src_loc = "177:1"]
    pub type ENGINE = engine_st;
    #[src_loc = "131:1"]
    pub type EVP_MD = env_md_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "131:9"]
    pub struct env_md_st {
        pub type_0: libc::c_int,
        pub pkey_type: libc::c_int,
        pub md_size: libc::c_int,
        pub flags: libc::c_ulong,
        pub init: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX)
                             -> libc::c_int>,
        pub update: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                                _: *const libc::c_void,
                                                _: size_t) -> libc::c_int>,
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
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: *mut libc::c_void)
                                -> libc::c_int>,
    }
    #[src_loc = "130:1"]
    pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "130:9"]
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
    #[src_loc = "129:1"]
    pub type EVP_CIPHER = evp_cipher_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "129:9"]
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
                                                             _:
                                                                 *mut ASN1_TYPE)
                                            -> libc::c_int>,
        pub get_asn1_parameters: Option<unsafe extern "C" fn(_:
                                                                 *mut EVP_CIPHER_CTX,
                                                             _:
                                                                 *mut ASN1_TYPE)
                                            -> libc::c_int>,
        pub ctrl: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                              _: libc::c_int, _: libc::c_int,
                                              _: *mut libc::c_void)
                             -> libc::c_int>,
        pub app_data: *mut libc::c_void,
    }
    #[src_loc = "98:1"]
    pub type ASN1_STRING = asn1_string_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "83:9"]
    pub struct asn1_string_st {
        pub length: libc::c_int,
        pub type_0: libc::c_int,
        pub data: *mut libc::c_uchar,
        pub flags: libc::c_long,
    }
    #[src_loc = "97:1"]
    pub type ASN1_UTF8STRING = asn1_string_st;
    #[src_loc = "96:1"]
    pub type ASN1_VISIBLESTRING = asn1_string_st;
    #[src_loc = "95:1"]
    pub type ASN1_GENERALIZEDTIME = asn1_string_st;
    #[src_loc = "93:1"]
    pub type ASN1_UTCTIME = asn1_string_st;
    #[src_loc = "91:1"]
    pub type ASN1_UNIVERSALSTRING = asn1_string_st;
    #[src_loc = "92:1"]
    pub type ASN1_BMPSTRING = asn1_string_st;
    #[src_loc = "90:1"]
    pub type ASN1_GENERALSTRING = asn1_string_st;
    #[src_loc = "89:1"]
    pub type ASN1_IA5STRING = asn1_string_st;
    #[src_loc = "88:1"]
    pub type ASN1_T61STRING = asn1_string_st;
    #[src_loc = "87:1"]
    pub type ASN1_PRINTABLESTRING = asn1_string_st;
    #[src_loc = "86:1"]
    pub type ASN1_OCTET_STRING = asn1_string_st;
    #[src_loc = "85:1"]
    pub type ASN1_BIT_STRING = asn1_string_st;
    #[src_loc = "84:1"]
    pub type ASN1_ENUMERATED = asn1_string_st;
    #[src_loc = "83:1"]
    pub type ASN1_INTEGER = asn1_string_st;
    #[src_loc = "103:1"]
    pub type ASN1_OBJECT = asn1_object_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "103:9"]
    pub struct asn1_object_st {
        pub sn: *const libc::c_char,
        pub ln: *const libc::c_char,
        pub nid: libc::c_int,
        pub length: libc::c_int,
        pub data: *const libc::c_uchar,
        pub flags: libc::c_int,
    }
    #[src_loc = "99:1"]
    pub type ASN1_BOOLEAN = libc::c_int;
    #[src_loc = "162:1"]
    pub type X509_STORE_CTX = x509_store_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "162:9"]
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
                                                    _: *mut X509)
                                   -> libc::c_int>,
        pub check_issued: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                      _: *mut X509,
                                                      _: *mut X509)
                                     -> libc::c_int>,
        pub check_revocation: Option<unsafe extern "C" fn(_:
                                                              *mut X509_STORE_CTX)
                                         -> libc::c_int>,
        pub get_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                 _: *mut *mut X509_CRL,
                                                 _: *mut X509)
                                -> libc::c_int>,
        pub check_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                   _: *mut X509_CRL)
                                  -> libc::c_int>,
        pub cert_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509_CRL,
                                                  _: *mut X509)
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
    #[src_loc = "197:1"]
    pub type CRYPTO_EX_DATA = crypto_ex_data_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "197:9"]
    pub struct crypto_ex_data_st {
        pub sk: *mut stack_st_void,
        pub dummy: libc::c_int,
    }
    #[src_loc = "156:1"]
    pub type X509_CRL = X509_crl_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "156:9"]
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
    #[src_loc = "157:1"]
    pub type X509_CRL_METHOD = x509_crl_method_st;
    #[src_loc = "190:1"]
    pub type ISSUING_DIST_POINT = ISSUING_DIST_POINT_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "190:9"]
    pub struct ISSUING_DIST_POINT_st {
        pub distpoint: *mut DIST_POINT_NAME,
        pub onlyuser: libc::c_int,
        pub onlyCA: libc::c_int,
        pub onlysomereasons: *mut ASN1_BIT_STRING,
        pub indirectCRL: libc::c_int,
        pub onlyattr: libc::c_int,
    }
    #[src_loc = "159:1"]
    pub type X509_NAME = X509_name_st;
    /* we always keep X509_NAMEs in 2 forms. */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "159:9"]
    pub struct X509_name_st {
        pub entries: *mut stack_st_X509_NAME_ENTRY,
        pub modified: libc::c_int,
        pub bytes: *mut BUF_MEM,
        pub canon_enc: *mut libc::c_uchar,
        pub canon_enclen: libc::c_int,
    }
    #[src_loc = "127:1"]
    pub type BUF_MEM = buf_mem_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "127:9"]
    pub struct buf_mem_st {
        pub length: size_t,
        pub data: *mut libc::c_char,
        pub max: size_t,
    }
    #[src_loc = "188:1"]
    pub type AUTHORITY_KEYID = AUTHORITY_KEYID_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "188:9"]
    pub struct AUTHORITY_KEYID_st {
        pub keyid: *mut ASN1_OCTET_STRING,
        pub issuer: *mut GENERAL_NAMES,
        pub serial: *mut ASN1_INTEGER,
    }
    #[src_loc = "155:1"]
    pub type X509_ALGOR = X509_algor_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "155:9"]
    pub struct X509_algor_st {
        pub algorithm: *mut ASN1_OBJECT,
        pub parameter: *mut ASN1_TYPE,
    }
    #[src_loc = "94:1"]
    pub type ASN1_TIME = asn1_string_st;
    #[src_loc = "154:1"]
    pub type X509 = x509_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "154:9"]
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
    #[src_loc = "191:1"]
    pub type NAME_CONSTRAINTS = NAME_CONSTRAINTS_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "191:9"]
    pub struct NAME_CONSTRAINTS_st {
        pub permittedSubtrees: *mut stack_st_GENERAL_SUBTREE,
        pub excludedSubtrees: *mut stack_st_GENERAL_SUBTREE,
    }
    #[src_loc = "186:1"]
    pub type X509_POLICY_CACHE = X509_POLICY_CACHE_st;
    #[src_loc = "160:1"]
    pub type X509_PUBKEY = X509_pubkey_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "160:9"]
    pub struct X509_pubkey_st {
        pub algor: *mut X509_ALGOR,
        pub public_key: *mut ASN1_BIT_STRING,
        pub pkey: *mut EVP_PKEY,
    }
    #[src_loc = "133:1"]
    pub type EVP_PKEY = evp_pkey_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "133:9"]
    pub struct evp_pkey_st {
        pub type_0: libc::c_int,
        pub save_type: libc::c_int,
        pub references: libc::c_int,
        pub ameth: *const EVP_PKEY_ASN1_METHOD,
        pub engine: *mut ENGINE,
        pub pkey: C2RustUnnamed_7,
        pub save_parameters: libc::c_int,
        pub attributes: *mut stack_st_X509_ATTRIBUTE,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "140:9"]
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
    #[src_loc = "141:1"]
    pub type DH_METHOD = dh_method;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "141:9"]
    pub struct dh_method {
        pub name: *const libc::c_char,
        pub generate_key: Option<unsafe extern "C" fn(_: *mut DH)
                                     -> libc::c_int>,
        pub compute_key: Option<unsafe extern "C" fn(_: *mut libc::c_uchar,
                                                     _: *const BIGNUM,
                                                     _: *mut DH)
                                    -> libc::c_int>,
        pub bn_mod_exp: Option<unsafe extern "C" fn(_: *const DH,
                                                    _: *mut BIGNUM,
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
    #[src_loc = "125:1"]
    pub type BN_GENCB = bn_gencb_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "125:9"]
    pub struct bn_gencb_st {
        pub ver: libc::c_uint,
        pub arg: *mut libc::c_void,
        pub cb: C2RustUnnamed_8,
    }
    #[src_loc = "140:1"]
    pub type DH = dh_st;
    #[src_loc = "123:1"]
    pub type BN_MONT_CTX = bn_mont_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "123:9"]
    pub struct bn_mont_ctx_st {
        pub ri: libc::c_int,
        pub RR: BIGNUM,
        pub N: BIGNUM,
        pub Ni: BIGNUM,
        pub n0: [libc::c_ulong; 2],
        pub flags: libc::c_int,
    }
    #[src_loc = "121:1"]
    pub type BN_CTX = bignum_ctx;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "143:9"]
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
    #[src_loc = "144:1"]
    pub type DSA_METHOD = dsa_method;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "144:9"]
    pub struct dsa_method {
        pub name: *const libc::c_char,
        pub dsa_do_sign: Option<unsafe extern "C" fn(_: *const libc::c_uchar,
                                                     _: libc::c_int,
                                                     _: *mut DSA)
                                    -> *mut DSA_SIG>,
        pub dsa_sign_setup: Option<unsafe extern "C" fn(_: *mut DSA,
                                                        _: *mut BN_CTX,
                                                        _: *mut *mut BIGNUM,
                                                        _: *mut *mut BIGNUM)
                                       -> libc::c_int>,
        pub dsa_do_verify: Option<unsafe extern "C" fn(_:
                                                           *const libc::c_uchar,
                                                       _: libc::c_int,
                                                       _: *mut DSA_SIG,
                                                       _: *mut DSA)
                                      -> libc::c_int>,
        pub dsa_mod_exp: Option<unsafe extern "C" fn(_: *mut DSA,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BN_CTX,
                                                     _: *mut BN_MONT_CTX)
                                    -> libc::c_int>,
        pub bn_mod_exp: Option<unsafe extern "C" fn(_: *mut DSA,
                                                    _: *mut BIGNUM,
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
        pub dsa_paramgen: Option<unsafe extern "C" fn(_: *mut DSA,
                                                      _: libc::c_int,
                                                      _: *const libc::c_uchar,
                                                      _: libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: *mut libc::c_ulong,
                                                      _: *mut BN_GENCB)
                                     -> libc::c_int>,
        pub dsa_keygen: Option<unsafe extern "C" fn(_: *mut DSA)
                                   -> libc::c_int>,
    }
    #[src_loc = "143:1"]
    pub type DSA = dsa_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "146:9"]
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
    #[src_loc = "122:1"]
    pub type BN_BLINDING = bn_blinding_st;
    #[src_loc = "147:1"]
    pub type RSA_METHOD = rsa_meth_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "147:9"]
    pub struct rsa_meth_st {
        pub name: *const libc::c_char,
        pub rsa_pub_enc: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *const libc::c_uchar,
                                                     _: *mut libc::c_uchar,
                                                     _: *mut RSA,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
        pub rsa_pub_dec: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *const libc::c_uchar,
                                                     _: *mut libc::c_uchar,
                                                     _: *mut RSA,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
        pub rsa_priv_enc: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: *mut RSA,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub rsa_priv_dec: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: *mut RSA,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub rsa_mod_exp: Option<unsafe extern "C" fn(_: *mut BIGNUM,
                                                     _: *const BIGNUM,
                                                     _: *mut RSA,
                                                     _: *mut BN_CTX)
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
                                                  _: *const RSA)
                                 -> libc::c_int>,
        pub rsa_verify: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *const libc::c_uchar,
                                                    _: libc::c_uint,
                                                    _: *const libc::c_uchar,
                                                    _: libc::c_uint,
                                                    _: *const RSA)
                                   -> libc::c_int>,
        pub rsa_keygen: Option<unsafe extern "C" fn(_: *mut RSA,
                                                    _: libc::c_int,
                                                    _: *mut BIGNUM,
                                                    _: *mut BN_GENCB)
                                   -> libc::c_int>,
    }
    #[src_loc = "146:1"]
    pub type RSA = rsa_st;
    #[src_loc = "135:1"]
    pub type EVP_PKEY_ASN1_METHOD = evp_pkey_asn1_method_st;
    #[src_loc = "185:1"]
    pub type X509_POLICY_TREE = X509_POLICY_TREE_st;
    #[src_loc = "161:1"]
    pub type X509_STORE = x509_store_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "161:9"]
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
                                                    _: *mut X509)
                                   -> libc::c_int>,
        pub check_issued: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                      _: *mut X509,
                                                      _: *mut X509)
                                     -> libc::c_int>,
        pub check_revocation: Option<unsafe extern "C" fn(_:
                                                              *mut X509_STORE_CTX)
                                         -> libc::c_int>,
        pub get_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                 _: *mut *mut X509_CRL,
                                                 _: *mut X509)
                                -> libc::c_int>,
        pub check_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                   _: *mut X509_CRL)
                                  -> libc::c_int>,
        pub cert_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509_CRL,
                                                  _: *mut X509)
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
    #[src_loc = "181:1"]
    pub type COMP_METHOD = comp_method_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "181:9"]
    pub struct comp_method_st {
        pub type_0: libc::c_int,
        pub name: *const libc::c_char,
        pub init: Option<unsafe extern "C" fn(_: *mut COMP_CTX)
                             -> libc::c_int>,
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
                                                _: libc::c_uint)
                               -> libc::c_int>,
        pub ctrl: Option<unsafe extern "C" fn() -> libc::c_long>,
        pub callback_ctrl: Option<unsafe extern "C" fn() -> libc::c_long>,
    }
    /* PKCS#8 private key info structure */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "164:9"]
    pub struct pkcs8_priv_key_info_st {
        pub broken: libc::c_int,
        pub version: *mut ASN1_INTEGER,
        pub pkeyalg: *mut X509_ALGOR,
        pub pkey: *mut ASN1_TYPE,
        pub attributes: *mut stack_st_X509_ATTRIBUTE,
    }
    #[src_loc = "164:1"]
    pub type PKCS8_PRIV_KEY_INFO = pkcs8_priv_key_info_st;
    #[src_loc = "172:1"]
    pub type UI = ui_st;
    #[src_loc = "173:1"]
    pub type UI_METHOD = ui_method_st;
    use super::libc;
    use super::ssl_h::{SSL_METHOD, ssl2_state_st, ssl3_state_st,
                       dtls1_state_st, stack_st_SSL_CIPHER, cert_st,
                       SSL_SESSION, GEN_SESSION_CB, stack_st_OCSP_RESPID,
                       TLS_SESSION_TICKET_EXT, tls_session_ticket_ext_cb_fn,
                       tls_session_secret_cb_fn,
                       stack_st_SRTP_PROTECTION_PROFILE,
                       SRTP_PROTECTION_PROFILE, SRP_CTX, lhash_st_SSL_SESSION,
                       ssl_session_st, C2RustUnnamed_9, stack_st_SSL_COMP,
                       ssl3_buf_freelist_st};
    use super::bio_h::BIO;
    use super::_size_t_h::size_t;
    use super::x509_vfy_h::{X509_VERIFY_PARAM, stack_st_X509_OBJECT,
                            stack_st_X509_LOOKUP};
    use super::comp_h::COMP_CTX;
    use super::x509_h::{stack_st_X509_NAME, X509_EXTENSIONS, stack_st_X509,
                        stack_st_X509_CRL, X509_CRL_INFO,
                        stack_st_GENERAL_NAMES, stack_st_X509_NAME_ENTRY,
                        X509_CINF, stack_st_DIST_POINT, stack_st_GENERAL_NAME,
                        X509_CERT_AUX};
    use super::pem_h::pem_password_cb;
    use super::hmac_h::HMAC_CTX;
    use super::asn1_h::ASN1_TYPE;
    use super::crypto_h::stack_st_void;
    use super::x509v3_h::{DIST_POINT_NAME, GENERAL_NAMES,
                          stack_st_GENERAL_SUBTREE};
    use super::evp_h::{C2RustUnnamed_7, stack_st_X509_ATTRIBUTE};
    use super::bn_h::C2RustUnnamed_8;
    use super::dsa_h::DSA_SIG;
    extern "C" {
        #[src_loc = "138:9"]
        pub type evp_pkey_ctx_st;
        #[src_loc = "177:9"]
        pub type engine_st;
        #[src_loc = "157:9"]
        pub type x509_crl_method_st;
        #[src_loc = "186:9"]
        pub type X509_POLICY_CACHE_st;
        #[src_loc = "121:9"]
        pub type bignum_ctx;
        #[src_loc = "122:9"]
        pub type bn_blinding_st;
        #[src_loc = "135:9"]
        pub type evp_pkey_asn1_method_st;
        #[src_loc = "185:9"]
        pub type X509_POLICY_TREE_st;
        #[src_loc = "172:9"]
        pub type ui_st;
        #[src_loc = "173:9"]
        pub type ui_method_st;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl.h:25"]
pub mod ssl_h {
    #[src_loc = "849:1"]
    pub type SRP_CTX = srp_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "849:9"]
    pub struct srp_ctx_st {
        pub SRP_cb_arg: *mut libc::c_void,
        pub TLS_ext_srp_username_callback: Option<unsafe extern "C" fn(_:
                                                                           *mut SSL,
                                                                       _:
                                                                           *mut libc::c_int,
                                                                       _:
                                                                           *mut libc::c_void)
                                                      -> libc::c_int>,
        pub SRP_verify_param_callback: Option<unsafe extern "C" fn(_:
                                                                       *mut SSL,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> libc::c_int>,
        pub SRP_give_srp_client_pwd_callback: Option<unsafe extern "C" fn(_:
                                                                              *mut SSL,
                                                                          _:
                                                                              *mut libc::c_void)
                                                         ->
                                                             *mut libc::c_char>,
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
    #[src_loc = "383:1"]
    pub type SRTP_PROTECTION_PROFILE = srtp_protection_profile_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "383:9"]
    pub struct srtp_protection_profile_st {
        pub name: *const libc::c_char,
        pub id: libc::c_ulong,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "388:1"]
    pub struct stack_st_SRTP_PROTECTION_PROFILE {
        pub stack: _STACK,
    }
    #[src_loc = "905:1"]
    pub type GEN_SESSION_CB
        =
        Option<unsafe extern "C" fn(_: *const SSL, _: *mut libc::c_uchar,
                                    _: *mut libc::c_uint) -> libc::c_int>;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "922:1"]
    pub struct stack_st_SSL_COMP {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "964:5"]
    pub struct C2RustUnnamed_9 {
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
    #[src_loc = "376:1"]
    pub type SSL_SESSION = ssl_session_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "376:9"]
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
    #[src_loc = "380:1"]
    pub struct stack_st_SSL_CIPHER {
        pub stack: _STACK,
    }
    #[src_loc = "375:1"]
    pub type SSL_CIPHER = ssl_cipher_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "375:9"]
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
    #[src_loc = "923:1"]
    pub struct lhash_st_SSL_SESSION {
        pub dummy: libc::c_int,
    }
    #[src_loc = "374:1"]
    pub type SSL_METHOD = ssl_method_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "374:9"]
    pub struct ssl_method_st {
        pub version: libc::c_int,
        pub ssl_new: Option<unsafe extern "C" fn(_: *mut SSL) -> libc::c_int>,
        pub ssl_clear: Option<unsafe extern "C" fn(_: *mut SSL) -> ()>,
        pub ssl_free: Option<unsafe extern "C" fn(_: *mut SSL) -> ()>,
        pub ssl_accept: Option<unsafe extern "C" fn(_: *mut SSL)
                                   -> libc::c_int>,
        pub ssl_connect: Option<unsafe extern "C" fn(_: *mut SSL)
                                    -> libc::c_int>,
        pub ssl_read: Option<unsafe extern "C" fn(_: *mut SSL,
                                                  _: *mut libc::c_void,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
        pub ssl_peek: Option<unsafe extern "C" fn(_: *mut SSL,
                                                  _: *mut libc::c_void,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
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
                                                         _:
                                                             *const libc::c_void,
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
        pub put_cipher_by_char: Option<unsafe extern "C" fn(_:
                                                                *const SSL_CIPHER,
                                                            _:
                                                                *mut libc::c_uchar)
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
                                                                          ->
                                                                              ()>)
                                          -> libc::c_long>,
        pub ssl_ctx_callback_ctrl: Option<unsafe extern "C" fn(_:
                                                                   *mut SSL_CTX,
                                                               _: libc::c_int,
                                                               _:
                                                                   Option<unsafe extern "C" fn()
                                                                              ->
                                                                                  ()>)
                                              -> libc::c_long>,
    }
    #[src_loc = "393:1"]
    pub type tls_session_secret_cb_fn
        =
        Option<unsafe extern "C" fn(_: *mut SSL, _: *mut libc::c_void,
                                    _: *mut libc::c_int,
                                    _: *mut stack_st_SSL_CIPHER,
                                    _: *mut *mut SSL_CIPHER,
                                    _: *mut libc::c_void) -> libc::c_int>;
    #[src_loc = "390:1"]
    pub type tls_session_ticket_ext_cb_fn
        =
        Option<unsafe extern "C" fn(_: *mut SSL, _: *const libc::c_uchar,
                                    _: libc::c_int, _: *mut libc::c_void)
                   -> libc::c_int>;
    #[src_loc = "373:1"]
    pub type TLS_SESSION_TICKET_EXT = tls_session_ticket_ext_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "373:9"]
    pub struct tls_session_ticket_ext_st {
        pub length: libc::c_ushort,
        pub data: *mut libc::c_void,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "1493:5"]
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
    /*
 * SSL3_CT_NUMBER is used to size arrays and it must be large enough to
 * contain all of the cert types defined either for SSLv3 and TLSv1.
 */
    /*
 * Set when the handshake is ready to process peer's ChangeCipherSpec message.
 * Cleared after the message has been processed.
 */
    /* SSL3_FLAGS_SGC_RESTART_DONE is no longer used */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "1492:5"]
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
        pub tmp: C2RustUnnamed_10,
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
    #[src_loc = "908:1"]
    pub type SSL_COMP = ssl_comp_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "908:9"]
    pub struct ssl_comp_st {
        pub id: libc::c_int,
        pub name: *const libc::c_char,
        pub method: *mut COMP_METHOD,
    }
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
    #[src_loc = "1491:5"]
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
        pub tmp: C2RustUnnamed_11,
    }
    use super::libc;
    use super::ossl_typ_h::{SSL, BIGNUM, X509, CRYPTO_EX_DATA, SSL_CTX,
                            EVP_MD_CTX, COMP_METHOD, X509_STORE,
                            X509_STORE_CTX, EVP_PKEY};
    use super::stack_h::_STACK;
    use super::_size_t_h::size_t;
    use super::dtls1_h::{DTLS1_BITMAP, record_pqueue, hm_header_st,
                         dtls1_timeout_st};
    use super::pqueue_h::pqueue;
    use super::_timeval_h::timeval;
    use super::ssl3_h::{SSL3_BUFFER, SSL3_RECORD, C2RustUnnamed_10};
    use super::bio_h::BIO;
    use super::ssl2_h::C2RustUnnamed_11;
    extern "C" {
        #[src_loc = "1114:5"]
        pub type ssl3_buf_freelist_st;
        #[src_loc = "1035:5"]
        pub type cert_st;
        #[src_loc = "531:5"]
        pub type sess_cert_st;
        #[src_loc = "466:5"]
        pub type ssl3_enc_method;
        #[src_loc = "1610:5"]
        pub type stack_st_OCSP_RESPID;
        /* OPENSSL_NO_TLSEXT */
        /* compatibility */
        /*
 * The following are the possible values for ssl->state are are used to
 * indicate where we are up to in the SSL connection establishment. The
 * macros that follow are about the only things you should need to use and
 * even then, only when using non-blocking IO. It can also be useful to work
 * out where you were when the connection failed
 */
        /* used in callback */
        /* Is the SSL_connection established? */
        /*
 * The following 2 states are kept in ssl->rstate when reads fail, you should
 * not need these
 */
        /*-
 * Obtain latest Finished message
 *   -- that we sent (SSL_get_finished)
 *   -- that we expected from peer (SSL_get_peer_finished).
 * Returns length (0 == no Finished so far), copies up to 'count' bytes.
 */
        /*
 * use either SSL_VERIFY_NONE or SSL_VERIFY_PEER, the last 2 options are
 * 'ored' with SSL_VERIFY_PEER if they are desired
 */
        /* this is for backward compatibility */
        /* NEW_SSLEAY */
        /* More backward compatibility */
        /* offset to get SSL_R_... value
                                              * from SSL_AD_... */
        /* These alert types are for SSLv3 and TLSv1 */
        /* fatal */
        /* fatal */
        /* fatal */
        /* fatal */
        /* Not for TLS */
        /* fatal */
        /* fatal */
        /* fatal */
        /* fatal */
        /* fatal */
        /* fatal */
        /* fatal */
        /* fatal */
        /* fatal */
        /* fatal */
        /* look at error stack/return
                                           * value/errno */
        /* only applies to datagram connections */
        /* Stats */
        /* see tls1.h for macros based on these */
        /* OPENSSL_NO_TLSEXT */
        /* Set serverinfo data for the current active cert. */
        /* NO_STDIO */
        /* PEM type */
        /* XXXXX: Better scheme needed! [was: #ifndef MAC_OS_pre_X] */
        #[no_mangle]
        #[src_loc = "2216:1"]
        pub fn SSL_load_error_strings();
        /* This sets the 'default' SSL version that SSL_new() will create */
        /* SSLv2 */
        /* SSLv2 */
        /* SSLv2 */
        /* SSLv3 */
        /* SSLv3 */
        /* SSLv3 */
        /* Negotiate highest available SSL/TLS
                                        * version */
        /* Negotiate highest available
                                               * SSL/TLS version */
        /* Negotiate highest available
                                               * SSL/TLS version */
        /* TLSv1.0 */
        /* TLSv1.0 */
        /* TLSv1.0 */
        /* TLSv1.1 */
        /* TLSv1.1 */
        /* TLSv1.1 */
        /* TLSv1.2 */
        /* TLSv1.2 */
        /* TLSv1.2 */
        /* DTLSv1.0 */
        /* DTLSv1.0 */
        /* DTLSv1.0 */
        /* DTLSv1.2 */
        /* DTLSv1.2 */
        /* DTLSv1.2 */
        /* DTLS 1.0 and 1.2 */
        /* DTLS 1.0 and 1.2 */
        /* DTLS 1.0 and 1.2 */
        #[no_mangle]
        #[src_loc = "2419:1"]
        pub fn SSL_library_init() -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2130:1"]
        pub fn SSL_CTX_set_cipher_list(_: *mut SSL_CTX,
                                       str: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2131:1"]
        pub fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
        #[no_mangle]
        #[src_loc = "2132:1"]
        pub fn SSL_CTX_free(_: *mut SSL_CTX);
        #[no_mangle]
        #[src_loc = "2135:1"]
        pub fn SSL_CTX_get_cert_store(_: *const SSL_CTX) -> *mut X509_STORE;
        #[no_mangle]
        #[src_loc = "2142:1"]
        pub fn SSL_get_current_cipher(s: *const SSL) -> *const SSL_CIPHER;
        #[no_mangle]
        #[src_loc = "2145:1"]
        pub fn SSL_CIPHER_get_name(c: *const SSL_CIPHER)
         -> *const libc::c_char;
        #[no_mangle]
        #[src_loc = "2161:1"]
        pub fn SSL_set_bio(s: *mut SSL, rbio: *mut BIO, wbio: *mut BIO);
        #[no_mangle]
        #[src_loc = "2170:1"]
        pub fn SSL_set_verify(s: *mut SSL, mode: libc::c_int,
                              callback:
                                  Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut X509_STORE_CTX)
                                             -> libc::c_int>);
        #[no_mangle]
        #[src_loc = "2265:1"]
        pub fn SSL_CTX_set_cert_verify_callback(ctx: *mut SSL_CTX,
                                                cb:
                                                    Option<unsafe extern "C" fn(_:
                                                                                    *mut X509_STORE_CTX,
                                                                                _:
                                                                                    *mut libc::c_void)
                                                               ->
                                                                   libc::c_int>,
                                                arg: *mut libc::c_void);
        #[no_mangle]
        #[src_loc = "2275:1"]
        pub fn SSL_CTX_use_PrivateKey(ctx: *mut SSL_CTX, pkey: *mut EVP_PKEY)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2278:1"]
        pub fn SSL_CTX_use_certificate(ctx: *mut SSL_CTX, x: *mut X509)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2285:1"]
        pub fn SSL_CTX_check_private_key(ctx: *const SSL_CTX) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2291:1"]
        pub fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
        #[no_mangle]
        #[src_loc = "2295:1"]
        pub fn SSL_CTX_set_purpose(s: *mut SSL_CTX, purpose: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2331:1"]
        pub fn SSL_free(ssl: *mut SSL);
        #[no_mangle]
        #[src_loc = "2333:1"]
        pub fn SSL_connect(ssl: *mut SSL) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2334:1"]
        pub fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void,
                        num: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2336:1"]
        pub fn SSL_write(ssl: *mut SSL, buf: *const libc::c_void,
                         num: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2337:1"]
        pub fn SSL_ctrl(ssl: *mut SSL, cmd: libc::c_int, larg: libc::c_long,
                        parg: *mut libc::c_void) -> libc::c_long;
        #[no_mangle]
        #[src_loc = "2339:1"]
        pub fn SSL_CTX_ctrl(ctx: *mut SSL_CTX, cmd: libc::c_int,
                            larg: libc::c_long, parg: *mut libc::c_void)
         -> libc::c_long;
        #[no_mangle]
        #[src_loc = "2342:1"]
        pub fn SSL_get_error(s: *const SSL, ret_code: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2364:1"]
        pub fn SSLv23_client_method() -> *const SSL_METHOD;
        #[no_mangle]
        #[src_loc = "2426:1"]
        pub fn SSL_get_certificate(ssl: *const SSL) -> *mut X509;
        #[no_mangle]
        #[src_loc = "2441:1"]
        pub fn SSL_CTX_set_default_verify_paths(ctx: *mut SSL_CTX)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2442:1"]
        pub fn SSL_CTX_load_verify_locations(ctx: *mut SSL_CTX,
                                             CAfile: *const libc::c_char,
                                             CApath: *const libc::c_char)
         -> libc::c_int;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the SSL functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/stack.h:25"]
pub mod stack_h {
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
    #[src_loc = "66:1"]
    pub type _STACK = stack_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "66:9"]
    pub struct stack_st {
        pub num: libc::c_int,
        pub data: *mut *mut libc::c_char,
        pub sorted: libc::c_int,
        pub num_alloc: libc::c_int,
        pub comp: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                              _: *const libc::c_void)
                             -> libc::c_int>,
    }
    use super::libc;
    extern "C" {
        /* Use STACK_OF(...) instead */
        #[no_mangle]
        #[src_loc = "78:1"]
        pub fn sk_value(_: *const _STACK, _: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "77:1"]
        pub fn sk_num(_: *const _STACK) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "83:1"]
        pub fn sk_new_null() -> *mut _STACK;
        #[no_mangle]
        #[src_loc = "85:1"]
        pub fn sk_pop_free(st: *mut _STACK,
                           func:
                               Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_void)
                                          -> ()>);
        #[no_mangle]
        #[src_loc = "92:1"]
        pub fn sk_push(st: *mut _STACK, data: *mut libc::c_void)
         -> libc::c_int;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/hmac.h:25"]
pub mod hmac_h {
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
    #[src_loc = "75:1"]
    pub type HMAC_CTX = hmac_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "75:9"]
    pub struct hmac_ctx_st {
        pub md: *const EVP_MD,
        pub md_ctx: EVP_MD_CTX,
        pub i_ctx: EVP_MD_CTX,
        pub o_ctx: EVP_MD_CTX,
        pub key_length: libc::c_uint,
        pub key: [libc::c_uchar; 128],
    }
    use super::ossl_typ_h::{EVP_MD, EVP_MD_CTX};
    use super::libc;
    use super::_size_t_h::size_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "98:1"]
        pub fn HMAC(evp_md: *const EVP_MD, key: *const libc::c_void,
                    key_len: libc::c_int, d: *const libc::c_uchar, n: size_t,
                    md: *mut libc::c_uchar, md_len: *mut libc::c_uint)
         -> *mut libc::c_uchar;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/asn1.h:25"]
pub mod asn1_h {
    #[src_loc = "524:1"]
    pub type ASN1_TYPE = asn1_type_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "524:9"]
    pub struct asn1_type_st {
        pub type_0: libc::c_int,
        pub value: C2RustUnnamed_5,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "526:5"]
    pub union C2RustUnnamed_5 {
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
    #[src_loc = "299:1"]
    pub type ASN1_VALUE = ASN1_VALUE_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "793:1"]
    pub struct stack_st_ASN1_OBJECT {
        pub stack: _STACK,
    }
    #[src_loc = "257:1"]
    pub type ASN1_ENCODING = ASN1_ENCODING_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "257:9"]
    pub struct ASN1_ENCODING_st {
        pub enc: *mut libc::c_uchar,
        pub len: libc::c_long,
        pub modified: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "162:1"]
    pub struct stack_st_X509_ALGOR {
        pub stack: _STACK,
    }
    use super::libc;
    use super::ossl_typ_h::{ASN1_BOOLEAN, ASN1_STRING, ASN1_OBJECT,
                            ASN1_INTEGER, ASN1_ENUMERATED, ASN1_BIT_STRING,
                            ASN1_OCTET_STRING, ASN1_PRINTABLESTRING,
                            ASN1_T61STRING, ASN1_IA5STRING,
                            ASN1_GENERALSTRING, ASN1_BMPSTRING,
                            ASN1_UNIVERSALSTRING, ASN1_UTCTIME,
                            ASN1_GENERALIZEDTIME, ASN1_VISIBLESTRING,
                            ASN1_UTF8STRING, ASN1_TIME};
    use super::stack_h::_STACK;
    use super::bio_h::BIO;
    extern "C" {
        #[src_loc = "299:9"]
        pub type ASN1_VALUE_st;
        #[no_mangle]
        #[src_loc = "1037:1"]
        pub fn ASN1_TIME_print(fp: *mut BIO, a: *const ASN1_TIME)
         -> libc::c_int;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the ASN1 functions. */
}
#[header_src =
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509_vfy.h:25"]
pub mod x509_vfy_h {
    /* crypto/x509/x509_vfy.h */
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
    /* number of paths to files or directories */
    /* the list of paths or directories */
    /* ******************************/
/*-
SSL_CTX -> X509_STORE
                -> X509_LOOKUP
                        ->X509_LOOKUP_METHOD
                -> X509_LOOKUP
                        ->X509_LOOKUP_METHOD

SSL     -> X509_STORE_CTX
                ->X509_STORE

The X509_STORE holds the tables etc for verification stuff.
A X509_STORE_CTX is used while validating a single certificate.
The X509_STORE has X509_LOOKUPs for looking up certs.
The X509_STORE then calls a function to actually verify the
certificate chain.
*/
    /* one of the above types */
    /* This is a static that defines the function interface */
    /*
 * This structure hold all parameters associated with a verify operation by
 * including an X509_VERIFY_PARAM structure in related structures the
 * parameters used can be customized
 */
    #[src_loc = "167:1"]
    pub type X509_VERIFY_PARAM = X509_VERIFY_PARAM_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "167:9"]
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
    #[src_loc = "159:1"]
    pub type X509_VERIFY_PARAM_ID = X509_VERIFY_PARAM_ID_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "136:1"]
    pub struct stack_st_X509_LOOKUP {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "137:1"]
    pub struct stack_st_X509_OBJECT {
        pub stack: _STACK,
    }
    use super::libc;
    use super::_time_t_h::time_t;
    use super::asn1_h::stack_st_ASN1_OBJECT;
    use super::stack_h::_STACK;
    use super::ossl_typ_h::{X509_STORE_CTX, X509_STORE, X509};
    use super::x509_h::stack_st_X509;
    extern "C" {
        #[src_loc = "159:9"]
        pub type X509_VERIFY_PARAM_ID_st;
        #[no_mangle]
        #[src_loc = "485:1"]
        pub fn X509_STORE_CTX_new() -> *mut X509_STORE_CTX;
        #[no_mangle]
        #[src_loc = "489:1"]
        pub fn X509_STORE_CTX_free(ctx: *mut X509_STORE_CTX);
        #[no_mangle]
        #[src_loc = "490:1"]
        pub fn X509_STORE_CTX_init(ctx: *mut X509_STORE_CTX,
                                   store: *mut X509_STORE, x509: *mut X509,
                                   chain: *mut stack_st_X509) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "543:1"]
        pub fn X509_STORE_CTX_get_error(ctx: *mut X509_STORE_CTX)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "568:1"]
        pub fn X509_STORE_CTX_get0_param(ctx: *mut X509_STORE_CTX)
         -> *mut X509_VERIFY_PARAM;
        #[no_mangle]
        #[src_loc = "581:1"]
        pub fn X509_VERIFY_PARAM_set_flags(param: *mut X509_VERIFY_PARAM,
                                           flags: libc::c_ulong)
         -> libc::c_int;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/crypto.h:25"]
pub mod crypto_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "293:5"]
    pub struct stack_st_void {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "290:9"]
    pub struct bio_st {
        pub method: *mut BIO_METHOD,
        pub callback: Option<unsafe extern "C" fn(_: *mut bio_st,
                                                  _: libc::c_int,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: libc::c_long,
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
        /* Time to use */
        /* Inheritance flags */
        /* Various verify flags */
        /* purpose to check untrusted certificates */
        /* trust setting to check */
        /* Verify depth */
        /* Permissible policies */
        /* opaque ID data */
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
    use super::stack_h::_STACK;
    use super::bio_h::BIO_METHOD;
    use super::libc;
    use super::ossl_typ_h::CRYPTO_EX_DATA;
    extern "C" {
        /*
 * Dynamically assigned indexes start from this value (don't use directly,
 * use via CRYPTO_ex_data_new_class).
 */
        /*
 * This is the default callbacks, but we can have others as well: this is
 * needed in Win32 where the application malloc and the library malloc may
 * not be the same.
 */
        /*
 * Set standard debugging functions (not done by default unless CRYPTO_MDEBUG
 * is defined)
 */
        /* for applications */
        /* for library-internal use */
        /* An opaque type representing an implementation of "ex_data" support */
        /* Return an opaque pointer to the current "ex_data" implementation */
        /* Sets the "ex_data" implementation to be used (if it's not too late) */
        /* Get a new "ex_data" class, and return the corresponding "class_index" */
        /* Within a given class, get/register a new index */
        /*
 * Initialise/duplicate/free CRYPTO_EX_DATA variables corresponding to a
 * given class (invokes whatever per-class callbacks are applicable)
 */
        /*
 * Get/set data in a CRYPTO_EX_DATA variable corresponding to a particular
 * index (relative to the class type involved)
 */
        /*
 * This function cleans up all "ex_data" state. It mustn't be called under
 * potential race-conditions.
 */
        /* return CRYPTO_NUM_LOCKS (shared libs!) */
        /* Don't use this structure directly. */
        /* Only use CRYPTO_THREADID_set_[numeric|pointer]() within callbacks */
        /*
 * CRYPTO_set_mem_functions includes CRYPTO_set_locked_mem_functions -- call
 * the latter last if you need different functions
 */
        #[no_mangle]
        #[src_loc = "536:1"]
        pub fn CRYPTO_free(ptr: *mut libc::c_void);
        #[no_mangle]
        #[src_loc = "468:1"]
        pub fn CRYPTO_add_lock(pointer: *mut libc::c_int, amount: libc::c_int,
                               type_0: libc::c_int, file: *const libc::c_char,
                               line: libc::c_int) -> libc::c_int;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the CRYPTO functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509.h:25"]
pub mod x509_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "175:1"]
    pub struct stack_st_X509_NAME_ENTRY {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "289:5"]
    pub struct stack_st_GENERAL_NAME {
        pub stack: _STACK,
    }
    #[src_loc = "441:1"]
    pub type X509_CRL_INFO = X509_crl_info_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "441:9"]
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
    #[src_loc = "202:9"]
    pub struct stack_st_X509_EXTENSION {
        pub stack: _STACK,
    }
    /* [0] */
    /* optional */
    /* Set up if indirect CRL */
    /* Revocation reason */
    /* load sequence */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "438:1"]
    pub struct stack_st_X509_REVOKED {
        pub stack: _STACK,
    }
    /*
 * This stuff is certificate "auxiliary info" it contains details which are
 * useful in certificate stores and databases. When used this is tagged onto
 * the end of the certificate itself
 */
    #[src_loc = "262:1"]
    pub type X509_CERT_AUX = x509_cert_aux_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "262:9"]
    pub struct x509_cert_aux_st {
        pub trust: *mut stack_st_ASN1_OBJECT,
        pub reject: *mut stack_st_ASN1_OBJECT,
        pub alias: *mut ASN1_UTF8STRING,
        pub keyid: *mut ASN1_OCTET_STRING,
        pub other: *mut stack_st_X509_ALGOR,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "288:5"]
    pub struct stack_st_DIST_POINT {
        pub stack: _STACK,
    }
    #[src_loc = "242:1"]
    pub type X509_CINF = x509_cinf_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "242:9"]
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
    #[src_loc = "152:1"]
    pub type X509_VAL = X509_val_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "152:9"]
    pub struct X509_val_st {
        pub notBefore: *mut ASN1_TIME,
        pub notAfter: *mut ASN1_TIME,
    }
    /* trusted uses */
    /* rejected uses */
    /* "friendly name" */
    /* key id of private key */
    /* other unspecified info */
    /* [ 0 ] default of v1 */
    /* [ 1 ] optional in v2 */
    /* [ 2 ] optional in v2 */
    /* [ 3 ] optional in v3 */
    /* These contain copies of various extension values */
    /* X509 */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "301:1"]
    pub struct stack_st_X509 {
        pub stack: _STACK,
    }
    /* actual signature */
    /* Copies of various extensions */
    /* Convenient breakdown of IDP */
    /* CRL and base CRL numbers for delta processing */
    /* X509_CRL */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "476:1"]
    pub struct stack_st_X509_CRL {
        pub stack: _STACK,
    }
    /* true if 'bytes' needs to be built */
    /*      unsigned long hash; Keep the hash around for lookups */
    /* X509_NAME */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "192:1"]
    pub struct stack_st_X509_NAME {
        pub stack: _STACK,
    }
    #[src_loc = "202:1"]
    pub type X509_EXTENSIONS = stack_st_X509_EXTENSION;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "163:9"]
    pub struct X509_sig_st {
        pub algor: *mut X509_ALGOR,
        pub digest: *mut ASN1_OCTET_STRING,
    }
    #[src_loc = "163:1"]
    pub type X509_SIG = X509_sig_st;
    use super::stack_h::_STACK;
    use super::ossl_typ_h::{ASN1_INTEGER, X509_ALGOR, X509_NAME, ASN1_TIME,
                            ASN1_UTF8STRING, ASN1_OCTET_STRING, X509_PUBKEY,
                            ASN1_BIT_STRING, X509, EVP_MD, EVP_PKEY,
                            X509_STORE_CTX, PKCS8_PRIV_KEY_INFO};
    use super::asn1_h::{ASN1_ENCODING, stack_st_ASN1_OBJECT,
                        stack_st_X509_ALGOR};
    use super::libc;
    use super::bio_h::BIO;
    use super::_stdio_h::FILE;
    use super::_time_t_h::time_t;
    extern "C" {
        #[src_loc = "471:5"]
        pub type stack_st_GENERAL_NAMES;
        /*
 * This one is only used so that a binary form can output, as in
 * i2d_X509_NAME(X509_get_X509_PUBKEY(x),&buf)
 */
        #[no_mangle]
        #[src_loc = "628:1"]
        pub fn X509_verify_cert_error_string(n: libc::c_long)
         -> *const libc::c_char;
        /* optional */
        #[no_mangle]
        #[src_loc = "999:1"]
        pub fn X509_print_ex(bp: *mut BIO, x: *mut X509,
                             nmflag: libc::c_ulong, cflag: libc::c_ulong)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "710:1"]
        pub fn i2d_X509_bio(bp: *mut BIO, x509: *mut X509) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "823:1"]
        pub fn i2d_X509(a: *mut X509, out: *mut *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "659:1"]
        pub fn X509_digest(data: *const X509, type_0: *const EVP_MD,
                           md: *mut libc::c_uchar, len: *mut libc::c_uint)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "696:1"]
        pub fn d2i_PKCS8_fp(fp: *mut FILE, p8: *mut *mut X509_SIG)
         -> *mut X509_SIG;
        #[no_mangle]
        #[src_loc = "703:1"]
        pub fn d2i_PrivateKey_fp(fp: *mut FILE, a: *mut *mut EVP_PKEY)
         -> *mut EVP_PKEY;
        #[no_mangle]
        #[src_loc = "743:1"]
        pub fn i2d_PUBKEY_bio(bp: *mut BIO, pkey: *mut EVP_PKEY)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "764:1"]
        pub fn X509_cmp_time(s: *const ASN1_TIME, t: *mut time_t)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "805:1"]
        pub fn X509_SIG_free(a: *mut X509_SIG);
        #[no_mangle]
        #[src_loc = "823:1"]
        pub fn X509_free(a: *mut X509);
        #[no_mangle]
        #[src_loc = "875:1"]
        pub fn X509_NAME_oneline(a: *mut X509_NAME, buf: *mut libc::c_char,
                                 size: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[src_loc = "907:1"]
        pub fn X509_get_subject_name(a: *mut X509) -> *mut X509_NAME;
        #[no_mangle]
        #[src_loc = "911:1"]
        pub fn X509_get_pubkey(x: *mut X509) -> *mut EVP_PKEY;
        #[no_mangle]
        #[src_loc = "978:1"]
        pub fn X509_cmp(a: *const X509, b: *const X509) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1173:1"]
        pub fn X509_verify_cert(ctx: *mut X509_STORE_CTX) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1200:1"]
        pub fn PKCS8_PRIV_KEY_INFO_free(a: *mut PKCS8_PRIV_KEY_INFO);
        #[no_mangle]
        #[src_loc = "1202:1"]
        pub fn EVP_PKCS82PKEY(p8: *mut PKCS8_PRIV_KEY_INFO) -> *mut EVP_PKEY;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the X509 functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509v3.h:33"]
pub mod x509v3_h {
    #[src_loc = "226:1"]
    pub type DIST_POINT_NAME = DIST_POINT_NAME_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "226:9"]
    pub struct DIST_POINT_NAME_st {
        pub type_0: libc::c_int,
        pub name: C2RustUnnamed_6,
        pub dpname: *mut X509_NAME,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "228:5"]
    pub union C2RustUnnamed_6 {
        pub fullname: *mut GENERAL_NAMES,
        pub relativename: *mut stack_st_X509_NAME_ENTRY,
    }
    #[src_loc = "209:1"]
    pub type GENERAL_NAMES = stack_st_GENERAL_NAME;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "330:1"]
    pub struct stack_st_GENERAL_SUBTREE {
        pub stack: _STACK,
    }
    use super::libc;
    use super::ossl_typ_h::{X509_NAME, X509};
    use super::x509_h::{stack_st_X509_NAME_ENTRY, stack_st_GENERAL_NAME};
    use super::stack_h::_STACK;
    use super::_size_t_h::size_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "701:1"]
        pub fn X509_check_issued(issuer: *mut X509, subject: *mut X509)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "741:1"]
        pub fn X509_check_host(x: *mut X509, chk: *const libc::c_char,
                               chklen: size_t, flags: libc::c_uint,
                               peername: *mut *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "745:1"]
        pub fn X509_check_ip(x: *mut X509, chk: *const libc::c_uchar,
                             chklen: size_t, flags: libc::c_uint)
         -> libc::c_int;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the X509V3 functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/evp.h:25"]
pub mod evp_h {
    /* crypto/evp/evp.h */
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
    /*-
#define EVP_RC2_KEY_SIZE                16
#define EVP_RC4_KEY_SIZE                16
#define EVP_BLOWFISH_KEY_SIZE           16
#define EVP_CAST5_KEY_SIZE              16
#define EVP_RC5_32_12_16_KEY_SIZE       16
*/
    /* longest known is SHA512 */
    /* Default PKCS#5 iteration count */
    /*
 * Type needs to be a bit field Sub-type needs to be for variations on the
 * method, as in, can it do arbitrary encryption....
 */
    /* RSA */
    /* DSA */
    /* DH */
    /* ECC */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "151:5"]
    pub struct stack_st_X509_ATTRIBUTE {
        pub stack: _STACK,
        /* [ 0 ] */
        /* EVP_PKEY */
        /* FIXME: prototype these some day */
        /* EVP_PKEY_xxx */
        /* how big does the ctx->md_data need to be */
        /* control function */
        /* EVP_MD */
        /* digest can only handle a single block */
        /*
 * digest is a "clone" digest used
 * which is a copy of an existing
 * one for a specific public key type.
 * EVP_dss1() etc
 */
        /* Digest uses EVP_PKEY_METHOD for signing instead of MD specific signing */
        /* DigestAlgorithmIdentifier flags... */
        /* NULL or absent parameter accepted. Use NULL */
        /* NULL or absent parameter accepted. Use NULL for PKCS#1 otherwise absent */
        /* Custom handling via ctrl */
        /* Note if suitable for use in FIPS mode */
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "135:5"]
    pub union C2RustUnnamed_7 {
        pub ptr: *mut libc::c_char,
        pub rsa: *mut rsa_st,
        pub dsa: *mut dsa_st,
        pub dh: *mut dh_st,
        pub ec: *mut ec_key_st,
    }
    use super::stack_h::_STACK;
    use super::libc;
    use super::ossl_typ_h::{rsa_st, dsa_st, dh_st, EVP_MD_CTX, EVP_MD, ENGINE,
                            EVP_PKEY};
    use super::_size_t_h::size_t;
    extern "C" {
        #[src_loc = "147:9"]
        pub type ec_key_st;
        /* possible final block */
        /* EVP_CIPHER_CTX */
        /* number saved in a partial encode/decode */
        /*
     * The length is either the output line length (in input bytes) or the
     * shortest input line length that is ok.  Once decoding begins, the
     * length is adjusted up each time a longer line is decoded
     */
        /* data to encode */
        /* number read on current line */
        /* Password based encryption function */
        /* Add some extra combinations */
        /* does nothing :-) */
        /*
 * This should now be supported through the dev_crypto ENGINE. But also, why
 * are rc4 and md5 declarations made here inside a "NO_DES" precompiler
 * branch?
 */
        #[no_mangle]
        #[src_loc = "905:1"]
        pub fn OPENSSL_add_all_algorithms_noconf();
        #[no_mangle]
        #[src_loc = "588:1"]
        pub fn EVP_MD_CTX_create() -> *mut EVP_MD_CTX;
        #[no_mangle]
        #[src_loc = "589:1"]
        pub fn EVP_MD_CTX_destroy(ctx: *mut EVP_MD_CTX);
        #[no_mangle]
        #[src_loc = "597:1"]
        pub fn EVP_Digest(data: *const libc::c_void, count: size_t,
                          md: *mut libc::c_uchar, size: *mut libc::c_uint,
                          type_0: *const EVP_MD, impl_0: *mut ENGINE)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "716:1"]
        pub fn EVP_md5() -> *const EVP_MD;
        #[no_mangle]
        #[src_loc = "720:1"]
        pub fn EVP_sha1() -> *const EVP_MD;
        #[no_mangle]
        #[src_loc = "727:1"]
        pub fn EVP_sha256() -> *const EVP_MD;
        #[no_mangle]
        #[src_loc = "731:1"]
        pub fn EVP_sha512() -> *const EVP_MD;
        #[no_mangle]
        #[src_loc = "981:1"]
        pub fn EVP_PKEY_free(pkey: *mut EVP_PKEY);
        #[no_mangle]
        #[src_loc = "1023:1"]
        pub fn PKCS5_PBKDF2_HMAC_SHA1(pass: *const libc::c_char,
                                      passlen: libc::c_int,
                                      salt: *const libc::c_uchar,
                                      saltlen: libc::c_int, iter: libc::c_int,
                                      keylen: libc::c_int,
                                      out: *mut libc::c_uchar) -> libc::c_int;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bn.h:25"]
pub mod bn_h {
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "352:5"]
    pub union C2RustUnnamed_8 {
        pub cb_1: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                              _: *mut libc::c_void) -> ()>,
        pub cb_2: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                              _: *mut BN_GENCB)
                             -> libc::c_int>,
    }
    use super::libc;
    use super::ossl_typ_h::BN_GENCB;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the BN functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dsa.h:25"]
pub mod dsa_h {
    #[src_loc = "124:1"]
    pub type DSA_SIG = DSA_SIG_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "124:9"]
    pub struct DSA_SIG_st {
        pub r: *mut BIGNUM,
        pub s: *mut BIGNUM,
    }
    use super::ossl_typ_h::BIGNUM;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the DSA functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pem.h:25"]
pub mod pem_h {
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
    #[src_loc = "389:1"]
    pub type pem_password_cb
        =
        unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int,
                             _: libc::c_int, _: *mut libc::c_void)
            -> libc::c_int;
    use super::libc;
    use super::bio_h::BIO;
    use super::ossl_typ_h::{X509, EVP_PKEY};
    extern "C" {
        #[no_mangle]
        #[src_loc = "452:22"]
        pub fn PEM_read_bio_X509(bp: *mut BIO, x: *mut *mut X509,
                                 cb:
                                     Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_char,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     *mut libc::c_void)
                                                -> libc::c_int>,
                                 u: *mut libc::c_void) -> *mut X509;
        #[no_mangle]
        #[src_loc = "453:26"]
        pub fn PEM_read_bio_X509_AUX(bp: *mut BIO, x: *mut *mut X509,
                                     cb:
                                         Option<unsafe extern "C" fn(_:
                                                                         *mut libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> libc::c_int>,
                                     u: *mut libc::c_void) -> *mut X509;
        #[no_mangle]
        #[src_loc = "481:31"]
        pub fn PEM_read_bio_PrivateKey(bp: *mut BIO, x: *mut *mut EVP_PKEY,
                                       cb:
                                           Option<unsafe extern "C" fn(_:
                                                                           *mut libc::c_char,
                                                                       _:
                                                                           libc::c_int,
                                                                       _:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut libc::c_void)
                                                      -> libc::c_int>,
                                       u: *mut libc::c_void) -> *mut EVP_PKEY;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/comp.h:25"]
pub mod comp_h {
    #[src_loc = "15:1"]
    pub type COMP_CTX = comp_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "15:9"]
    pub struct comp_ctx_st {
        pub meth: *mut COMP_METHOD,
        pub compress_in: libc::c_ulong,
        pub compress_out: libc::c_ulong,
        pub expand_in: libc::c_ulong,
        pub expand_out: libc::c_ulong,
        pub ex_data: CRYPTO_EX_DATA,
    }
    use super::ossl_typ_h::{COMP_METHOD, CRYPTO_EX_DATA};
    use super::libc;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the COMP functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dtls1.h:25"]
pub mod dtls1_h {
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
    #[src_loc = "162:1"]
    pub struct dtls1_timeout_st {
        pub read_timeouts: libc::c_uint,
        pub write_timeouts: libc::c_uint,
        pub num_alerts: libc::c_uint,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "147:1"]
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
    #[src_loc = "135:1"]
    pub struct dtls1_retransmit_state {
        pub enc_write_ctx: *mut EVP_CIPHER_CTX,
        pub write_hash: *mut EVP_MD_CTX,
        pub compress: *mut COMP_CTX,
        pub session: *mut SSL_SESSION,
        pub epoch: libc::c_ushort,
    }
    #[src_loc = "171:1"]
    pub type record_pqueue = record_pqueue_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "171:9"]
    pub struct record_pqueue_st {
        pub epoch: libc::c_ushort,
        pub q: pqueue,
    }
    #[src_loc = "128:1"]
    pub type DTLS1_BITMAP = dtls1_bitmap_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "128:9"]
    pub struct dtls1_bitmap_st {
        pub map: libc::c_ulong,
        pub max_seq_num: [libc::c_uchar; 8],
    }
    use super::libc;
    use super::ossl_typ_h::{EVP_CIPHER_CTX, EVP_MD_CTX};
    use super::comp_h::COMP_CTX;
    use super::ssl_h::SSL_SESSION;
    use super::pqueue_h::pqueue;
    /* Timeout multipliers (timeout slice is defined in apps/timeouts.h */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pqueue.h:25"]
pub mod pqueue_h {
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
    #[src_loc = "70:1"]
    pub type pqueue = *mut _pqueue;
    extern "C" {
        #[src_loc = "70:9"]
        pub type _pqueue;
    }
    /* ! HEADER_PQUEUE_H */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl3.h:25"]
pub mod ssl3_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "553:5"]
    pub struct C2RustUnnamed_10 {
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
    #[src_loc = "403:1"]
    pub type SSL3_RECORD = ssl3_record_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "403:9"]
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
    #[src_loc = "438:1"]
    pub type SSL3_BUFFER = ssl3_buffer_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "438:9"]
    pub struct ssl3_buffer_st {
        pub buf: *mut libc::c_uchar,
        pub len: size_t,
        pub offset: libc::c_int,
        pub left: libc::c_int,
    }
    use super::libc;
    use super::ssl_h::{SSL_CIPHER, SSL_COMP};
    use super::ossl_typ_h::{DH, EVP_CIPHER, EVP_MD};
    use super::ec_h::EC_KEY;
    use super::x509_h::stack_st_X509_NAME;
    use super::_size_t_h::size_t;
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
    /* at least SSL3_RT_MAX_PACKET_SIZE bytes, see ssl3_setup_buffers() */
    /* buffer size */
    /* where to 'copy from' */
    /* how many bytes left */
    /* These are used when changing over to a new cipher */
    /* write to client */
    /* read from client */
    /* write to client */
    /* read from client */
/* Do not change the number values, they do matter */
    /* server */
/* extra state */
    /* read from server */
    /* write to server */
    /* read from server */
    /* write to server */
    /* SSLv3 */
/*
 * client
 */
/* extra state */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ec.h:25"]
pub mod ec_h {
    /* crypto/ec/ec.h */
/*
 * Originally written by Bodo Moeller for the OpenSSL project.
 */
/* *
 * \file crypto/ec/ec.h Include file for the OpenSSL EC functions
 * \author Originally written by Bodo Moeller for the OpenSSL project
 */
/* ====================================================================
 * Copyright (c) 1998-2019 The OpenSSL Project.  All rights reserved.
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
 *
 * Portions of the attached software ("Contribution") are developed by
 * SUN MICROSYSTEMS, INC., and are contributed to the OpenSSL project.
 *
 * The Contribution is licensed pursuant to the OpenSSL open source
 * license provided above.
 *
 * The elliptic curve binary polynomial software is originally written by
 * Sheueling Chang Shantz and Douglas Stebila of Sun Microsystems Laboratories.
 *
 */
    /* * Enum for the point conversion form as defined in X9.62 (ECDSA)
 *  for the encoding of a elliptic curve point (x,y) */
    /* * the point is encoded as z||x, where the octet z specifies
         *  which solution of the quadratic equation y is  */
    /* * the point is encoded as z||x||y, where z is the octet 0x04  */
    /* * the point is encoded as z||x||y, where the octet z specifies
         *  which solution of the quadratic equation y is  */
    /*-
     EC_METHOD *meth;
     -- field definition
     -- curve coefficients
     -- optional generator with associated information (order, cofactor)
     -- optional extra data (precomputed table for fast computation of multiples of generator)
     -- ASN1 stuff
    */
    /* *******************************************************************/
/*               EC_METHODs for curves over GF(p)                   */
/* *******************************************************************/
    /* * Returns the basic GFp ec methods which provides the basis for the
 *  optimized methods.
 *  \return  EC_METHOD object
 */
    /* * Returns GFp methods using montgomery multiplication.
 *  \return  EC_METHOD object
 */
    /* * Returns GFp methods using optimized methods for NIST recommended curves
 *  \return  EC_METHOD object
 */
    /* * Returns 64-bit optimized methods for nistp224
 *  \return  EC_METHOD object
 */
    /* * Returns 64-bit optimized methods for nistp256
 *  \return  EC_METHOD object
 */
    /* * Returns 64-bit optimized methods for nistp521
 *  \return  EC_METHOD object
 */
    /* *******************************************************************/
/*           EC_METHOD for curves over GF(2^m)                      */
/* *******************************************************************/
    /* * Returns the basic GF2m ec method
 *  \return  EC_METHOD object
 */
    /* *******************************************************************/
/*                   EC_GROUP functions                             */
/* *******************************************************************/
    /* * Creates a new EC_GROUP object
 *  \param   meth  EC_METHOD to use
 *  \return  newly created EC_GROUP object or NULL in case of an error.
 */
    /* * Frees a EC_GROUP object
 *  \param  group  EC_GROUP object to be freed.
 */
    /* * Clears and frees a EC_GROUP object
 *  \param  group  EC_GROUP object to be cleared and freed.
 */
    /* * Copies EC_GROUP objects. Note: both EC_GROUPs must use the same EC_METHOD.
 *  \param  dst  destination EC_GROUP object
 *  \param  src  source EC_GROUP object
 *  \return 1 on success and 0 if an error occurred.
 */
    /* * Creates a new EC_GROUP object and copies the copies the content
 *  form src to the newly created EC_KEY object
 *  \param  src  source EC_GROUP object
 *  \return newly created EC_GROUP object or NULL in case of an error.
 */
    /* * Returns the EC_METHOD of the EC_GROUP object.
 *  \param  group  EC_GROUP object
 *  \return EC_METHOD used in this EC_GROUP object.
 */
    /* * Returns the field type of the EC_METHOD.
 *  \param  meth  EC_METHOD object
 *  \return NID of the underlying field type OID.
 */
    /* * Sets the generator and it's order/cofactor of a EC_GROUP object.
 *  \param  group      EC_GROUP object
 *  \param  generator  EC_POINT object with the generator.
 *  \param  order      the order of the group generated by the generator.
 *  \param  cofactor   the index of the sub-group generated by the generator
 *                     in the group of all points on the elliptic curve.
 *  \return 1 on success and 0 if an error occured
 */
    /* * Returns the generator of a EC_GROUP object.
 *  \param  group  EC_GROUP object
 *  \return the currently used generator (possibly NULL).
 */
    /* * Returns the montgomery data for order(Generator)
 *  \param  group  EC_GROUP object
 *  \return the currently used generator (possibly NULL).
*/
    /* * Gets the order of a EC_GROUP
 *  \param  group  EC_GROUP object
 *  \param  order  BIGNUM to which the order is copied
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the cofactor of a EC_GROUP
 *  \param  group     EC_GROUP object
 *  \param  cofactor  BIGNUM to which the cofactor is copied
 *  \param  ctx       BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the name of a EC_GROUP object
 *  \param  group  EC_GROUP object
 *  \param  nid    NID of the curve name OID
 */
    /* * Returns the curve name of a EC_GROUP object
 *  \param  group  EC_GROUP object
 *  \return NID of the curve name OID or 0 if not set.
 */
    /* * Sets the parameter of a ec over GFp defined by y^2 = x^3 + a*x + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM with the prime number
 *  \param  a      BIGNUM with parameter a of the equation
 *  \param  b      BIGNUM with parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the parameter of the ec over GFp defined by y^2 = x^3 + a*x + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM for the prime number
 *  \param  a      BIGNUM for parameter a of the equation
 *  \param  b      BIGNUM for parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the parameter of a ec over GF2m defined by y^2 + x*y = x^3 + a*x^2 + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM with the polynomial defining the underlying field
 *  \param  a      BIGNUM with parameter a of the equation
 *  \param  b      BIGNUM with parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the parameter of the ec over GF2m defined by y^2 + x*y = x^3 + a*x^2 + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM for the polynomial defining the underlying field
 *  \param  a      BIGNUM for parameter a of the equation
 *  \param  b      BIGNUM for parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Returns the number of bits needed to represent a field element
 *  \param  group  EC_GROUP object
 *  \return number of bits needed to represent a field element
 */
    /* * Checks whether the parameter in the EC_GROUP define a valid ec group
 *  \param  group  EC_GROUP object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 if group is a valid ec group and 0 otherwise
 */
    /* * Checks whether the discriminant of the elliptic curve is zero or not
 *  \param  group  EC_GROUP object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 if the discriminant is not zero and 0 otherwise
 */
    /* * Compares two EC_GROUP objects
 *  \param  a    first EC_GROUP object
 *  \param  b    second EC_GROUP object
 *  \param  ctx  BN_CTX object (optional)
 *  \return 0 if both groups are equal and 1 otherwise
 */
    /*
 * EC_GROUP_new_GF*() calls EC_GROUP_new() and EC_GROUP_set_GF*() after
 * choosing an appropriate EC_METHOD
 */
    /* * Creates a new EC_GROUP object with the specified parameters defined
 *  over GFp (defined by the equation y^2 = x^3 + a*x + b)
 *  \param  p    BIGNUM with the prime number
 *  \param  a    BIGNUM with the parameter a of the equation
 *  \param  b    BIGNUM with the parameter b of the equation
 *  \param  ctx  BN_CTX object (optional)
 *  \return newly created EC_GROUP object with the specified parameters
 */
    /* * Creates a new EC_GROUP object with the specified parameters defined
 *  over GF2m (defined by the equation y^2 + x*y = x^3 + a*x^2 + b)
 *  \param  p    BIGNUM with the polynomial defining the underlying field
 *  \param  a    BIGNUM with the parameter a of the equation
 *  \param  b    BIGNUM with the parameter b of the equation
 *  \param  ctx  BN_CTX object (optional)
 *  \return newly created EC_GROUP object with the specified parameters
 */
    /* * Creates a EC_GROUP object with a curve specified by a NID
 *  \param  nid  NID of the OID of the curve name
 *  \return newly created EC_GROUP object with specified curve or NULL
 *          if an error occurred
 */
    /* *******************************************************************/
/*               handling of internal curves                        */
/* *******************************************************************/
    /*
 * EC_builtin_curves(EC_builtin_curve *r, size_t size) returns number of all
 * available curves or zero if a error occurred. In case r ist not zero
 * nitems EC_builtin_curve structures are filled with the data of the first
 * nitems internal groups
 */
    /* *******************************************************************/
/*                    EC_POINT functions                            */
/* *******************************************************************/
    /* * Creates a new EC_POINT object for the specified EC_GROUP
 *  \param  group  EC_GROUP the underlying EC_GROUP object
 *  \return newly created EC_POINT object or NULL if an error occurred
 */
    /* * Frees a EC_POINT object
 *  \param  point  EC_POINT object to be freed
 */
    /* * Clears and frees a EC_POINT object
 *  \param  point  EC_POINT object to be cleared and freed
 */
    /* * Copies EC_POINT object
 *  \param  dst  destination EC_POINT object
 *  \param  src  source EC_POINT object
 *  \return 1 on success and 0 if an error occured
 */
    /* * Creates a new EC_POINT object and copies the content of the supplied
 *  EC_POINT
 *  \param  src    source EC_POINT object
 *  \param  group  underlying the EC_GROUP object
 *  \return newly created EC_POINT object or NULL if an error occurred
 */
    /* * Returns the EC_METHOD used in EC_POINT object
 *  \param  point  EC_POINT object
 *  \return the EC_METHOD used
 */
    /* * Sets a point to infinity (neutral element)
 *  \param  group  underlying EC_GROUP object
 *  \param  point  EC_POINT to set to infinity
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the jacobian projective coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with the x-coordinate
 *  \param  y      BIGNUM with the y-coordinate
 *  \param  z      BIGNUM with the z-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the jacobian projective coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM for the x-coordinate
 *  \param  y      BIGNUM for the y-coordinate
 *  \param  z      BIGNUM for the z-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the affine coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with the x-coordinate
 *  \param  y      BIGNUM with the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the affine coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM for the x-coordinate
 *  \param  y      BIGNUM for the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the x9.62 compressed coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with x-coordinate
 *  \param  y_bit  integer with the y-Bit (either 0 or 1)
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the affine coordinates of a EC_POINT over GF2m
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with the x-coordinate
 *  \param  y      BIGNUM with the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the affine coordinates of a EC_POINT over GF2m
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM for the x-coordinate
 *  \param  y      BIGNUM for the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the x9.62 compressed coordinates of a EC_POINT over GF2m
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with x-coordinate
 *  \param  y_bit  integer with the y-Bit (either 0 or 1)
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Encodes a EC_POINT object to a octet string
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  form   point conversion form
 *  \param  buf    memory buffer for the result. If NULL the function returns
 *                 required buffer size.
 *  \param  len    length of the memory buffer
 *  \param  ctx    BN_CTX object (optional)
 *  \return the length of the encoded octet string or 0 if an error occurred
 */
    /* * Decodes a EC_POINT from a octet string
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  buf    memory buffer with the encoded ec point
 *  \param  len    length of the encoded ec point
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* other interfaces to point2oct/oct2point: */
    /* *******************************************************************/
/*         functions for doing EC_POINT arithmetic                  */
/* *******************************************************************/
    /* * Computes the sum of two EC_POINT
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result (r = a + b)
 *  \param  a      EC_POINT object with the first summand
 *  \param  b      EC_POINT object with the second summand
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Computes the double of a EC_POINT
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result (r = 2 * a)
 *  \param  a      EC_POINT object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Computes the inverse of a EC_POINT
 *  \param  group  underlying EC_GROUP object
 *  \param  a      EC_POINT object to be inverted (it's used for the result as well)
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Checks whether the point is the neutral element of the group
 *  \param  group  the underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \return 1 if the point is the neutral element and 0 otherwise
 */
    /* * Checks whether the point is on the curve
 *  \param  group  underlying EC_GROUP object
 *  \param  point  EC_POINT object to check
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 if point if on the curve and 0 otherwise
 */
    /* * Compares two EC_POINTs
 *  \param  group  underlying EC_GROUP object
 *  \param  a      first EC_POINT object
 *  \param  b      second EC_POINT object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 0 if both points are equal and a value != 0 otherwise
 */
    /* * Computes r = generator * n sum_{i=0}^{num-1} p[i] * m[i]
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result
 *  \param  n      BIGNUM with the multiplier for the group generator (optional)
 *  \param  num    number futher summands
 *  \param  p      array of size num of EC_POINT objects
 *  \param  m      array of size num of BIGNUM objects
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Computes r = generator * n + q * m
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result
 *  \param  n      BIGNUM with the multiplier for the group generator (optional)
 *  \param  q      EC_POINT object with the first factor of the second summand
 *  \param  m      BIGNUM with the second factor of the second summand
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Stores multiples of generator for faster point multiplication
 *  \param  group  EC_GROUP object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Reports whether a precomputation has been done
 *  \param  group  EC_GROUP object
 *  \return 1 if a pre-computation has been done and 0 otherwise
 */
    /* *******************************************************************/
/*                       ASN1 stuff                                 */
/* *******************************************************************/
    /*
 * EC_GROUP_get_basis_type() returns the NID of the basis type used to
 * represent the field elements
 */
    /* *******************************************************************/
/*                      EC_KEY functions                            */
/* *******************************************************************/
    #[src_loc = "741:1"]
    pub type EC_KEY = ec_key_st;
    use super::evp_h::ec_key_st;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the EC functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bio.h:25"]
pub mod bio_h {
    /* crypto/bio/bio.h */
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
    /* These are the 'types' of BIOs */
    /* passive filter */
    /* filter */
    /* filter */
    /* filter */
    /* socket - connect */
    /* socket for accept */
    /* client proxy BIO */
    /* server proxy BIO */
    /* server proxy BIO */
    /* BER -> bin filter */
    /* (half a) BIO pair */
    /* filter */
    /* filter */
    /* filter */
    /* socket, fd, connect or accept */
    /*
 * BIO_FILENAME_READ|BIO_CLOSE to open or close on free.
 * BIO_set_fp(in,stdin,BIO_NOCLOSE);
 */
    /*
 * These are used in the following macros and are passed to BIO_ctrl()
 */
    /* opt - rewind/zero etc */
    /* opt - are we at the eof */
    /* opt - extra tit-bits */
    /* man - set the 'IO' type */
    /* man - get the 'IO' type */
    /* opt - internal, used to signify change */
    /* opt - internal, used to signify change */
    /* man - set the 'close' on free */
    /* man - set the 'close' on free */
    /* opt - is their more data buffered */
    /* opt - 'flush' buffered output */
    /* man - extra stuff for 'duped' BIO */
    /* opt - number of bytes still to write */
    /* callback is int cb(BIO *bio,state,ret); */
    /* opt - set callback function */
    /* opt - set callback function */
    /* BIO_s_file special */
    /* dgram BIO stuff */
    /* BIO dgram special */
    /* allow for an externally connected
                                         * socket to be passed in */
    /* setsockopt, essentially */
    /* getsockopt, essentially */
    /* setsockopt, essentially */
    /* getsockopt, essentially */
    /* flag whether the last */
    /* I/O operation tiemd out */
    /* #ifdef IP_MTU_DISCOVER */
    /* set DF bit on egress packets */
    /* #endif */
    /* as kernel for current MTU */
    /* get cached value for MTU */
    /* set cached value for MTU.
                                              * want to use this if asking
                                              * the kernel fails */
    /* check whether the MTU was
                                              * exceed in the previous write
                                              * operation */
    /* Destination for the data */
    /* Next DTLS handshake timeout
                                              * to adjust socket timeouts */
    /* modifiers */
    /*
 * "UPLINK" flag denotes file descriptors provided by application. It
 * defaults to 0, as most platforms don't require UPLINK interface.
 */
    /* Used in BIO_gethostbyname() */
    /* Mostly used in the SSL BIO */
/*-
 * Not used anymore
 * #define BIO_FLAGS_PROTOCOL_DELAYED_READ 0x10
 * #define BIO_FLAGS_PROTOCOL_DELAYED_WRITE 0x20
 * #define BIO_FLAGS_PROTOCOL_STARTUP   0x40
 */
    /*
 * This is used with memory BIOs: it means we shouldn't free up or change the
 * data in any way.
 */
    #[src_loc = "238:1"]
    pub type BIO = bio_st;
    #[src_loc = "312:1"]
    pub type BIO_METHOD = bio_method_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "312:9"]
    pub struct bio_method_st {
        pub type_0: libc::c_int,
        pub name: *const libc::c_char,
        pub bwrite: Option<unsafe extern "C" fn(_: *mut BIO,
                                                _: *const libc::c_char,
                                                _: libc::c_int)
                               -> libc::c_int>,
        pub bread: Option<unsafe extern "C" fn(_: *mut BIO,
                                               _: *mut libc::c_char,
                                               _: libc::c_int)
                              -> libc::c_int>,
        pub bputs: Option<unsafe extern "C" fn(_: *mut BIO,
                                               _: *const libc::c_char)
                              -> libc::c_int>,
        pub bgets: Option<unsafe extern "C" fn(_: *mut BIO,
                                               _: *mut libc::c_char,
                                               _: libc::c_int)
                              -> libc::c_int>,
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
    #[src_loc = "309:1"]
    pub type bio_info_cb
        =
        unsafe extern "C" fn(_: *mut bio_st, _: libc::c_int,
                             _: *const libc::c_char, _: libc::c_int,
                             _: libc::c_long, _: libc::c_long) -> ();
    use super::crypto_h::bio_st;
    use super::libc;
    use super::_stdio_h::FILE;
    extern "C" {
        /*-
     * Buffers are setup like this:
     *
     * <---------------------- size ----------------------->
     * +---------------------------------------------------+
     * | consumed | remaining          | free space        |
     * +---------------------------------------------------+
     * <-- off --><------- len ------->
     */
    /*- BIO *bio; */
        /*
     * this is now in the BIO struct
     */
        /* how big is the input buffer */
        /* how big is the output buffer */
        /* the char array */
        /* how many bytes are in it */
        /* write/read offset */
        /* the char array */
        /* how many bytes are in it */
        /* write/read offset */
        /* Prefix and suffix callback in ASN1 BIO */
        /* connect BIO stuff */
        /*
 * #define BIO_CONN_get_param_hostname BIO_ctrl
 */
        /* data to read first */
        /* return end of input
                                                     * value */
        /* for BIO_s_bio */
        /* BIO_s_connect() and BIO_s_socks4a_connect() */
        /* BIO_s_accept() */
        /* #define BIO_set_nbio(b,n)    BIO_ctrl(b,BIO_C_SET_NBIO,(n),NULL) */
        /* BIO_s_accept() and BIO_s_connect() */
        /* BIO_s_proxy_client() */
        /* BIO_set_nbio(b,n) */
        /* BIO *BIO_get_filter_bio(BIO *bio); */
        /* BIO_s_datagram(), BIO_s_fd(), BIO_s_socket(), BIO_s_accept() and BIO_s_connect() */
        /* BIO_s_file() */
        /* BIO_s_fd() and BIO_s_file() */
        /*
 * name is cast to lose const, but might be better to route through a
 * function so we can do it safely
 */
        /*
 * WARNING WARNING, this ups the reference count on the read bio of the SSL
 * structure.  This is because the ssl read BIO is now pointed to by the
 * next_bio field in the bio.  So when you free the BIO, make sure you are
 * doing a BIO_free_all() to catch the underlying BIO.
 */
        /* defined in evp.h */
/* #define BIO_set_md(b,md)     BIO_ctrl(b,BIO_C_SET_MD,1,(char *)md) */
        /* For the BIO_f_buffer() type */
        /* Don't use the next one unless you know what you are doing :-) */
        /* ...pending macros have inappropriate return type */
        /* For the BIO_f_buffer() type */
        /* For BIO_s_bio() */
        /* macros with inappropriate type -- but ...pending macros use int too: */
        /* ctrl macros for dgram */
        /* These two aren't currently implemented */
/* int BIO_get_ex_num(BIO *bio); */
/* void BIO_set_ex_free_func(BIO *bio,int idx,void (*cb)()); */
        /* For BIO_f_asn1() */
        #[no_mangle]
        #[src_loc = "691:1"]
        pub fn BIO_s_mem() -> *mut BIO_METHOD;
        #[no_mangle]
        #[src_loc = "659:1"]
        pub fn BIO_new(type_0: *mut BIO_METHOD) -> *mut BIO;
        #[no_mangle]
        #[src_loc = "661:1"]
        pub fn BIO_free(a: *mut BIO) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "668:1"]
        pub fn BIO_ctrl(bp: *mut BIO, cmd: libc::c_int, larg: libc::c_long,
                        parg: *mut libc::c_void) -> libc::c_long;
        #[no_mangle]
        #[src_loc = "665:1"]
        pub fn BIO_write(b: *mut BIO, data: *const libc::c_void,
                         len: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "656:1"]
        pub fn BIO_new_fp(stream: *mut FILE, close_flag: libc::c_int)
         -> *mut BIO;
        #[no_mangle]
        #[src_loc = "757:1"]
        pub fn BIO_new_socket(sock: libc::c_int, close_flag: libc::c_int)
         -> *mut BIO;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the BIO functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl2.h:25"]
pub mod ssl2_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "203:5"]
    pub struct C2RustUnnamed_11 {
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
    use super::libc;
    /* server */
    /* SSLv2 */
/* client */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zlib.h:25"]
pub mod zlib_h {
    /* zlib.h -- interface of the 'zlib' general purpose compression library
  version 1.2.11, January 15th, 2017

  Copyright (C) 1995-2017 Jean-loup Gailly and Mark Adler

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

  Jean-loup Gailly        Mark Adler
  jloup@gzip.org          madler@alumni.caltech.edu


  The data format used by the zlib library is described by RFCs (Request for
  Comments) 1950 to 1952 in the files http://tools.ietf.org/html/rfc1950
  (zlib format), rfc1951 (deflate format) and rfc1952 (gzip format).
*/
    /* !__APPLE__ */
    /* !__APPLE__ */
    /*
    The 'zlib' compression library provides in-memory compression and
  decompression functions, including integrity checks of the uncompressed data.
  This version of the library supports only one compression method (deflation)
  but other algorithms will be added later and will have the same stream
  interface.

    Compression can be done in a single step if the buffers are large enough,
  or can be done by repeated calls of the compression function.  In the latter
  case, the application must provide more input and/or consume the output
  (providing more output space) before each call.

    The compressed data format used by default by the in-memory functions is
  the zlib format, which is a zlib wrapper documented in RFC 1950, wrapped
  around a deflate stream, which is itself documented in RFC 1951.

    The library also supports reading and writing files in gzip (.gz) format
  with an interface similar to that of stdio using the functions that start
  with "gz".  The gzip format is different from the zlib format.  gzip is a
  gzip wrapper, documented in RFC 1952, wrapped around a deflate stream.

    This library can optionally read and write gzip and raw deflate streams in
  memory as well.

    The zlib format was designed to be compact and fast for use in memory
  and on communications channels.  The gzip format was designed for single-
  file compression on file systems, has a larger header than zlib to maintain
  directory information, and uses a different, slower check method than zlib.

    The library does not install any signal handler.  The decoder checks
  the consistency of the compressed data, so the library should never crash
  even in the case of corrupted input.
*/
    #[src_loc = "91:1"]
    pub type z_stream = z_stream_s;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "91:9"]
    pub struct z_stream_s {
        pub next_in: *mut Bytef,
        pub avail_in: uInt,
        pub total_in: uLong,
        pub next_out: *mut Bytef,
        pub avail_out: uInt,
        pub total_out: uLong,
        pub msg: *mut libc::c_char,
        pub state: *mut internal_state,
        pub zalloc: alloc_func,
        pub zfree: free_func,
        pub opaque: voidpf,
        pub data_type: libc::c_int,
        pub adler: uLong,
        pub reserved: uLong,
    }
    #[src_loc = "87:1"]
    pub type free_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
    #[src_loc = "86:1"]
    pub type alloc_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
    use super::zconf_h::{Bytef, uInt, uLong, voidpf};
    use super::libc;
    extern "C" {
        #[src_loc = "89:1"]
        pub type internal_state;
    }
    /* next input byte */
    /* number of bytes available at next_in */
    /* total number of input bytes read so far */
    /* next output byte will go here */
    /* remaining free space at next_out */
    /* total number of bytes output so far */
    /* last error message, NULL if no error */
    /* not visible by applications */
    /* used to allocate the internal state */
    /* used to free the internal state */
    /* private data object passed to zalloc and zfree */
    /* best guess about the data type: binary or text
                           for deflate, or the decoding state for inflate */
    /* Adler-32 or CRC-32 value of the uncompressed data */
    /* reserved for future use */
    /* ZLIB_H */
    /* !__APPLE__ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zconf.h:25"]
pub mod zconf_h {
    /* zconf.h -- configuration of the zlib compression library
 * Copyright (C) 1995-2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
    /* @(#) $Id$ */
    /*
 * If you *really* need a unique prefix for all types and library functions,
 * compile with -DZ_PREFIX. The "standard" zlib should be compiled without it.
 * Even better than compiling with -DZ_PREFIX would be to use configure to set
 * this permanently in zconf.h using "./configure --zprefix".
 */
    /* may be set to #if 1 by ./configure */
    /*
 * Compile with -DMAXSEG_64K if the alloc function cannot allocate more
 * than 64k bytes at a time (needed on systems with 16-bit int).
 */
    /* iSeries (formerly AS/400). */
    /* Maximum value for memLevel in deflateInit2 */
    /* Maximum value for windowBits in deflateInit2 and inflateInit2.
 * WARNING: reducing MAX_WBITS makes minigzip unable to extract .gz files
 * created by gzip. (Files created by minigzip can still be extracted by
 * gzip.)
 */
    /* 32K LZ77 window */
    /* The memory requirements for deflate are (in bytes):
            (1 << (windowBits+2)) +  (1 << (memLevel+9))
 that is: 128K for windowBits=15  +  128K for memLevel = 8  (default values)
 plus a few kilobytes for small objects. For example, if you want to reduce
 the default memory requirements from 256K to 128K, compile with
     make CFLAGS="-O -DMAX_WBITS=14 -DMAX_MEM_LEVEL=7"
 Of course this will generally degrade compression (there's no free lunch).

   The memory requirements for inflate are (in bytes) 1 << windowBits
 that is, 32K for windowBits=15 (default value) plus about 7 kilobytes
 for small objects.
*/
    /* Type declarations */
    /* function prototypes */
    /* function prototypes for stdarg */
    /* The following definitions for FAR are needed only for MSDOS mixed
 * model programming (small or medium model with some far allocations).
 * This was tested only with MSC; for other MSDOS compilers you may have
 * to define NO_MEMCPY in zutil.h.  If you don't need the mixed model,
 * just define FAR to be empty.
 */
    /* 8 bits */
    /* 16 bits or more */
    #[src_loc = "394:1"]
    pub type uLong = libc::c_ulong;
    /* 32 bits or more */
    #[src_loc = "409:4"]
    pub type voidpf = *mut libc::c_void;
    #[src_loc = "393:1"]
    pub type uInt = libc::c_uint;
    #[src_loc = "400:4"]
    pub type Bytef = Byte;
    #[src_loc = "391:1"]
    pub type Byte = libc::c_uchar;
    use super::libc;
    /* ZCONF_H */
    /* MVS linker does not support external names larger than 8 bytes */
    /* for SEEK_*, off_t, and _LFS64_LARGEFILE */
    /* a little trick to accommodate both "#define _LARGEFILE64_SOURCE" and
 * "#define _LARGEFILE64_SOURCE 1" as requesting 64-bit operations, (even
 * though the former does not conform to the LFS document), but considering
 * both "#undef _LARGEFILE64_SOURCE" and "#define _LARGEFILE64_SOURCE 0" as
 * equivalently requesting no 64-bit operations
 */
    /* for va_list */
    /* for off_t */
    /* was set to #if 1 by ./configure */
    /* defined(__APPLE__) */
    /* was set to #if 1 by ./configure */
    /* avoid unistd.h on Win32 */
}
#[header_src = "/usr/local/Cellar/stoken/0.92/include/stoken.h:25"]
pub mod stoken_h {
    extern "C" {
        /*
 * stoken.h - public libstoken library interface
 *
 * Copyright 2012 Kevin Cernekee <cernekee@gmail.com>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */
        /* Before API version 1.3 (stoken 0.8) this macro didn't exist.
 * Somewhat ironic, that the API version check itself needs to be
 * conditionally used depending on the API version. A very simple way
 * for users to handle this with an approximately correct answer is
 *   #include <stoken.h>
 *   #ifndef STOKEN_CHECK_VER
 *   #define STOKEN_CHECK_VER(x,y) 0
 *   #endif
 */
        #[src_loc = "59:1"]
        pub type stoken_ctx;
    }
    /* !__STOKEN_H__ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/gssapi/gssapi.h:25"]
pub mod gssapi_h {
    /*
 * This file is auto generated.  Please do not edit it.
 */
    /* This is the gssapi.h prologue. */
    /* no xom.h */
/* End of gssapi.h prologue. */
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
    /*
 * Determine platform-dependent configuration.
 */
    /* __cplusplus */
    /*
 * First, include stddef.h to get size_t defined.
 */
    /*
 * POSIX says that sys/types.h is where size_t is defined.
 */
    /*
 * $Id: gssapi.hin 20876 2008-10-15 21:58:43Z tlyu $
 */
    /*
 * First, define the three platform-dependent pointer types.
 */
    #[src_loc = "108:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[src_loc = "102:1"]
    pub type gss_name_t = *mut gss_name_struct;
    extern "C" {
        #[src_loc = "107:1"]
        pub type gss_ctx_id_struct;
        #[src_loc = "101:1"]
        pub type gss_name_struct;
    }
    /* __GSSAPI__ */
    /* _GSSAPI_H_ */
    /* XXXX This is a necessary evil until the spec is fixed */
    /* XXXX these are not part of the GSSAPI C bindings!  (but should be) */
    /* output_name */
}
#[header_src = "/usr/local/Cellar/libproxy/0.4.15_1/include/proxy.h:25"]
pub mod proxy_h {
    /* ******************************************************************************
 * libproxy - A library for proxy configuration
 * Copyright (C) 2006 Nathaniel McCallum <nathaniel@natemccallum.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301 USA
 ******************************************************************************/
    #[src_loc = "28:1"]
    pub type pxProxyFactory = pxProxyFactory_;
    extern "C" {
        #[src_loc = "28:9"]
        pub type pxProxyFactory_;
    }
    /*PROXY_H_*/
}
#[header_src =
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/tree.h:25"]
pub mod tree_h {
    /*
 * Summary: interfaces for tree manipulation
 * Description: this module describes the structures found in an tree resulting
 *              from an XML or HTML parsing, as well as the API provided for
 *              various processing on that tree
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /*
 * Some of the basic types pointer to structures:
 */
/* xmlIO.h */
    /* parser.h */
    /* entities.h */
    /* *
 * BASE_BUFFER_SIZE:
 *
 * default buffer size 4000.
 */
    /* *
 * LIBXML_NAMESPACE_DICT:
 *
 * Defines experimental behaviour:
 * 1) xmlNs gets an additional field @context (a xmlDoc)
 * 2) when creating a tree, xmlNs->href is stored in the dict of xmlDoc.
 */
/* #define LIBXML_NAMESPACE_DICT */
    /* *
 * xmlBufferAllocationScheme:
 *
 * A buffer allocation scheme can be defined to either match exactly the
 * need or double it's allocated size each time it is found too small.
 */
    /* double each time one need to grow */
    /* grow only to the minimal size */
    /* immutable buffer */
    /* special allocation scheme used for I/O */
    /* exact up to a threshold, and doubleit thereafter */
    /* limit the upper size of the buffer */
    /* *
 * xmlBuffer:
 *
 * A buffer structure, this old construct is limited to 2GB and
 * is being deprecated, use API with xmlBuf instead
 */
    /* The buffer content UTF8 */
    /* The buffer size used */
    /* The buffer size */
    /* The realloc method */
    /* in IO mode we may have a different base */
    /* *
 * xmlBuf:
 *
 * A buffer structure, new one, the actual structure internals are not public
 */
    /* *
 * xmlBufPtr:
 *
 * A pointer to a buffer structure, the actual structure internals are not
 * public
 */
    /*
 * A few public routines for xmlBuf. As those are expected to be used
 * mostly internally the bulk of the routines are internal in buf.h
 */
    /*
 * LIBXML2_NEW_BUFFER:
 *
 * Macro used to express that the API use the new buffers for
 * xmlParserInputBuffer and xmlOutputBuffer. The change was
 * introduced in 2.9.0.
 */
    /* *
 * XML_XML_NAMESPACE:
 *
 * This is the namespace for the special xml: prefix predefined in the
 * XML Namespace specification.
 */
    /* *
 * XML_XML_ID:
 *
 * This is the name for the special xml:id attribute
 */
    /*
 * The different element types carried by an XML tree.
 *
 * NOTE: This is synchronized with DOM Level1 values
 *       See http://www.w3.org/TR/REC-DOM-Level-1/
 *
 * Actually this had diverged a bit, and now XML_DOCUMENT_TYPE_NODE should
 * be deprecated to use an XML_DTD_NODE.
 */
    /* *
 * xmlNotation:
 *
 * A DTD Notation definition.
 */
    /* Notation name */
    /* Public identifier, if any */
    /* System identifier, if any */
    /* *
 * xmlAttributeType:
 *
 * A DTD Attribute type definition.
 */
    /* *
 * xmlAttributeDefault:
 *
 * A DTD Attribute default definition.
 */
    /* *
 * xmlEnumeration:
 *
 * List structure used when there is an enumeration in DTDs.
 */
    /* next one */
    /* Enumeration name */
    /* *
 * xmlAttribute:
 *
 * An Attribute declaration in a DTD.
 */
    /* application data */
    /* XML_ATTRIBUTE_DECL, must be second ! */
    /* Attribute name */
    /* NULL */
    /* NULL */
    /* -> DTD */
    /* next sibling link  */
    /* previous sibling link  */
    /* the containing document */
    /* next in hash table */
    /* The attribute type */
    /* the default */
    /* or the default value */
    /* or the enumeration tree if any */
    /* the namespace prefix if any */
    /* Element holding the attribute */
    /* *
 * xmlElementContentType:
 *
 * Possible definitions of element content types.
 */
    /* *
 * xmlElementContentOccur:
 *
 * Possible definitions of element content occurrences.
 */
    /* *
 * xmlElementContent:
 *
 * An XML Element content as stored after parsing an element definition
 * in a DTD.
 */
    /* PCDATA, ELEMENT, SEQ or OR */
    /* ONCE, OPT, MULT or PLUS */
    /* Element name */
    /* first child */
    /* second child */
    /* parent */
    /* Namespace prefix */
    /* *
 * xmlElementTypeVal:
 *
 * The different possibilities for an element content type.
 */
    /* *
 * xmlElement:
 *
 * An XML Element declaration from a DTD.
 */
    /* application data */
    /* XML_ELEMENT_DECL, must be second ! */
    /* Element name */
    /* NULL */
    /* NULL */
    /* -> DTD */
    /* next sibling link  */
    /* previous sibling link  */
    /* the containing document */
    /* The type */
    /* the allowed element content */
    /* List of the declared attributes */
    /* the namespace prefix if any */
    /* the validating regexp */
    /* *
 * XML_LOCAL_NAMESPACE:
 *
 * A namespace declaration node.
 */
    /* *
 * xmlNs:
 *
 * An XML namespace.
 * Note that prefix == NULL is valid, it defines the default namespace
 * within the subtree (until overridden).
 *
 * xmlNsType is unified with xmlElementType.
 */
    /* next Ns link for this node  */
    /* global or local */
    /* URL for the namespace */
    /* prefix for the namespace */
    /* application data */
    /* normally an xmlDoc */
    /* *
 * xmlDtd:
 *
 * An XML DTD, as defined by <!DOCTYPE ... There is actually one for
 * the internal subset and for the external subset.
 */
    /* application data */
    /* XML_DTD_NODE, must be second ! */
    /* Name of the DTD */
    /* the value of the property link */
    /* last child link */
    /* child->parent link */
    /* next sibling link  */
    /* previous sibling link  */
    /* the containing document */
    /* End of common part */
    /* Hash table for notations if any */
    /* Hash table for elements if any */
    /* Hash table for attributes if any */
    /* Hash table for entities if any */
    /* External identifier for PUBLIC DTD */
    /* URI for a SYSTEM or PUBLIC DTD */
    /* Hash table for param entities if any */
    /* *
 * xmlAttr:
 *
 * An attribute on an XML node.
 */
    /* application data */
    /* XML_ATTRIBUTE_NODE, must be second ! */
    /* the name of the property */
    /* the value of the property */
    /* NULL */
    /* child->parent link */
    /* next sibling link  */
    /* previous sibling link  */
    /* the containing document */
    /* pointer to the associated namespace */
    /* the attribute type if validating */
    /* for type/PSVI informations */
    /* *
 * xmlID:
 *
 * An XML ID instance.
 */
    /* next ID */
    /* The ID name */
    /* The attribute holding it */
    /* The attribute if attr is not available */
    /* The line number if attr is not available */
    /* The document holding the ID */
    /* *
 * xmlRef:
 *
 * An XML IDREF instance.
 */
    /* next Ref */
    /* The Ref name */
    /* The attribute holding it */
    /* The attribute if attr is not available */
    /* The line number if attr is not available */
    /* *
 * xmlNode:
 *
 * A node in an XML tree.
 */
    #[src_loc = "487:1"]
    pub type xmlNode = _xmlNode;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "257:5"]
    pub struct _xmlNode {
        pub _private: *mut libc::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlNode,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub ns: *mut xmlNs,
        pub content: *mut xmlChar,
        pub properties: *mut _xmlAttr,
        pub nsDef: *mut xmlNs,
        pub psvi: *mut libc::c_void,
        pub line: libc::c_ushort,
        pub extra: libc::c_ushort,
    }
    #[src_loc = "387:1"]
    pub type xmlNs = _xmlNs;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "387:9"]
    pub struct _xmlNs {
        pub next: *mut _xmlNs,
        pub type_0: xmlNsType,
        pub href: *const xmlChar,
        pub prefix: *const xmlChar,
        pub _private: *mut libc::c_void,
        pub context: *mut _xmlDoc,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "262:5"]
    pub struct _xmlDoc {
        pub _private: *mut libc::c_void,
        pub type_0: xmlElementType,
        pub name: *mut libc::c_char,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlNode,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub compression: libc::c_int,
        pub standalone: libc::c_int,
        pub intSubset: *mut _xmlDtd,
        pub extSubset: *mut _xmlDtd,
        pub oldNs: *mut _xmlNs,
        pub version: *const xmlChar,
        pub encoding: *const xmlChar,
        pub ids: *mut libc::c_void,
        pub refs: *mut libc::c_void,
        pub URL: *const xmlChar,
        pub charset: libc::c_int,
        pub dict: *mut _xmlDict,
        pub psvi: *mut libc::c_void,
        pub parseFlags: libc::c_int,
        pub properties: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "259:5"]
    pub struct _xmlDtd {
        pub _private: *mut libc::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlDoc,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub notations: *mut libc::c_void,
        pub elements: *mut libc::c_void,
        pub attributes: *mut libc::c_void,
        pub entities: *mut libc::c_void,
        pub ExternalID: *const xmlChar,
        pub SystemID: *const xmlChar,
        pub pentities: *mut libc::c_void,
    }
    #[src_loc = "159:9"]
    pub type xmlElementType = libc::c_uint;
    #[src_loc = "181:5"]
    pub const XML_DOCB_DOCUMENT_NODE: xmlElementType = 21;
    #[src_loc = "179:5"]
    pub const XML_XINCLUDE_END: xmlElementType = 20;
    #[src_loc = "178:5"]
    pub const XML_XINCLUDE_START: xmlElementType = 19;
    #[src_loc = "177:5"]
    pub const XML_NAMESPACE_DECL: xmlElementType = 18;
    #[src_loc = "176:5"]
    pub const XML_ENTITY_DECL: xmlElementType = 17;
    #[src_loc = "175:5"]
    pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
    #[src_loc = "174:5"]
    pub const XML_ELEMENT_DECL: xmlElementType = 15;
    #[src_loc = "173:5"]
    pub const XML_DTD_NODE: xmlElementType = 14;
    #[src_loc = "172:5"]
    pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
    #[src_loc = "171:5"]
    pub const XML_NOTATION_NODE: xmlElementType = 12;
    #[src_loc = "170:5"]
    pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
    #[src_loc = "169:5"]
    pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
    #[src_loc = "168:5"]
    pub const XML_DOCUMENT_NODE: xmlElementType = 9;
    #[src_loc = "167:5"]
    pub const XML_COMMENT_NODE: xmlElementType = 8;
    #[src_loc = "166:5"]
    pub const XML_PI_NODE: xmlElementType = 7;
    #[src_loc = "165:5"]
    pub const XML_ENTITY_NODE: xmlElementType = 6;
    #[src_loc = "164:5"]
    pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
    #[src_loc = "163:5"]
    pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
    #[src_loc = "162:5"]
    pub const XML_TEXT_NODE: xmlElementType = 3;
    #[src_loc = "161:5"]
    pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
    #[src_loc = "160:5"]
    pub const XML_ELEMENT_NODE: xmlElementType = 1;
    #[src_loc = "375:1"]
    pub type xmlNsType = xmlElementType;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "432:9"]
    pub struct _xmlAttr {
        pub _private: *mut libc::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlNode,
        pub next: *mut _xmlAttr,
        pub prev: *mut _xmlAttr,
        pub doc: *mut _xmlDoc,
        pub ns: *mut xmlNs,
        pub atype: xmlAttributeType,
        pub psvi: *mut libc::c_void,
    }
    #[src_loc = "206:9"]
    pub type xmlAttributeType = libc::c_uint;
    #[src_loc = "216:5"]
    pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
    #[src_loc = "215:5"]
    pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
    #[src_loc = "214:5"]
    pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
    #[src_loc = "213:5"]
    pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
    #[src_loc = "212:5"]
    pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
    #[src_loc = "211:5"]
    pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
    #[src_loc = "210:5"]
    pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
    #[src_loc = "209:5"]
    pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
    #[src_loc = "208:5"]
    pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
    #[src_loc = "207:5"]
    pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
    use super::libc;
    use super::xmlstring_h::xmlChar;
    use super::dict_h::_xmlDict;
    /* __XML_TREE_H__ */
}
#[header_src =
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/dict.h:25"]
pub mod dict_h {
    extern "C" {
        /*
 * Summary: string dictionary
 * Description: dictionary of reusable strings, just used to avoid allocation
 *         and freeing operations.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
        /*
 * The dictionary.
 */
        #[src_loc = "23:23"]
        pub type _xmlDict;
    }
    /* ! __XML_DICT_H__ */
}
#[header_src =
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/xmlstring.h:25"]
pub mod xmlstring_h {
    /*
 * Summary: set of routines to process strings
 * Description: type and interfaces needed for the internal string handling
 *              of the library, especially UTF8 processing.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /* *
 * xmlChar:
 *
 * This is a basic byte in an UTF-8 encoded string.
 * It's unsigned allowing to pinpoint case where char * are assigned
 * to xmlChar * (possibly making serialization back impossible).
 */
    #[src_loc = "28:1"]
    pub type xmlChar = libc::c_uchar;
    use super::libc;
    /* __XML_STRING_H__ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/iconv.h:25"]
pub mod iconv_h {
    /* Copyright (C) 1999-2003, 2005-2006 Free Software Foundation, Inc.
   This file is part of the GNU LIBICONV Library.

   The GNU LIBICONV Library is free software; you can redistribute it
   and/or modify it under the terms of the GNU Library General Public
   License as published by the Free Software Foundation; either version 2
   of the License, or (at your option) any later version.

   The GNU LIBICONV Library is distributed in the hope that it will be
   useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Library General Public License for more details.

   You should have received a copy of the GNU Library General Public
   License along with the GNU LIBICONV Library; see the file COPYING.LIB.
   If not, write to the Free Software Foundation, Inc., 51 Franklin Street,
   Fifth Floor, Boston, MA 02110-1301, USA.  */
    /* When installed, this file is called "iconv.h". */
    /* version number: (major<<8) + minor */
    /* Likewise */
    /* We would like to #include any system header file which could define
   iconv_t, 1. in order to eliminate the risk that the user gets compilation
   errors because some other system header file includes /usr/include/iconv.h
   which defines iconv_t or declares iconv after this file, 2. when compiling
   for LIBICONV_PLUG, we need the proper iconv_t type in order to produce
   binary compatible code.
   But gcc's #include_next is not portable. Thus, once libiconv's iconv.h
   has been installed in /usr/local/include, there is no way any more to
   include the original /usr/include/iconv.h. We simply have to get away
   without it.
   Ad 1. The risk that a system header file does
   #include "iconv.h"  or  #include_next "iconv.h"
   is small. They all do #include <iconv.h>.
   Ad 2. The iconv_t type is a pointer type in all cases I have seen. (It
   has to be a scalar type because (iconv_t)(-1) is a possible return value
   from iconv_open().) */
    /* Define iconv_t ourselves. */
    #[src_loc = "57:1"]
    pub type iconv_t = *mut libc::c_void;
    use super::libc;
    /* _LIBICONV_H */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:25"]
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
        /* current lseek offset (see WARNING) */
    }
    #[src_loc = "126:1"]
    pub type FILE = __sFILE;
    use super::sys__types_h::__darwin_off_t;
    use super::libc;
    extern "C" {
        /* hold a buncha junk that would grow the ABI */
        #[src_loc = "98:1"]
        pub type __sFILEX;
    }
    /* __STDIO_H_ */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pkcs7.h:25"]
pub mod pkcs7_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "103:1"]
    pub struct stack_st_PKCS7_SIGNER_INFO {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "114:1"]
    pub struct stack_st_PKCS7_RECIP_INFO {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "117:9"]
    pub struct pkcs7_signed_st {
        pub version: *mut ASN1_INTEGER,
        pub md_algs: *mut stack_st_X509_ALGOR,
        pub cert: *mut stack_st_X509,
        pub crl: *mut stack_st_X509_CRL,
        pub signer_info: *mut stack_st_PKCS7_SIGNER_INFO,
        pub contents: *mut pkcs7_st,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "123:5"]
    pub struct pkcs7_st {
        pub asn1: *mut libc::c_uchar,
        pub length: libc::c_long,
        pub state: libc::c_int,
        pub detached: libc::c_int,
        pub type_0: *mut ASN1_OBJECT,
        pub d: C2RustUnnamed_17,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "183:5"]
    pub union C2RustUnnamed_17 {
        pub ptr: *mut libc::c_char,
        pub data: *mut ASN1_OCTET_STRING,
        pub sign: *mut PKCS7_SIGNED,
        pub enveloped: *mut PKCS7_ENVELOPE,
        pub signed_and_enveloped: *mut PKCS7_SIGN_ENVELOPE,
        pub digest: *mut PKCS7_DIGEST,
        pub encrypted: *mut PKCS7_ENCRYPT,
        pub other: *mut ASN1_TYPE,
    }
    /*
 * The above structure is very very similar to PKCS7_SIGN_ENVELOPE. How about
 * merging the two
 */
    /* [ 0 ] */
    /* version 0 */
    /* version 1 */
    /* md used */
    /* [ 0 ] */
    /* [ 1 ] */
    /* version 0 */
    /* md used */
    #[src_loc = "160:1"]
    pub type PKCS7_ENCRYPT = pkcs7_encrypted_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "160:9"]
    pub struct pkcs7_encrypted_st {
        pub version: *mut ASN1_INTEGER,
        pub enc_data: *mut PKCS7_ENC_CONTENT,
    }
    #[src_loc = "130:1"]
    pub type PKCS7_ENC_CONTENT = pkcs7_enc_content_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "130:9"]
    pub struct pkcs7_enc_content_st {
        pub content_type: *mut ASN1_OBJECT,
        pub algorithm: *mut X509_ALGOR,
        pub enc_data: *mut ASN1_OCTET_STRING,
        pub cipher: *const EVP_CIPHER,
    }
    #[src_loc = "153:1"]
    pub type PKCS7_DIGEST = pkcs7_digest_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "153:9"]
    pub struct pkcs7_digest_st {
        pub version: *mut ASN1_INTEGER,
        pub md: *mut X509_ALGOR,
        pub contents: *mut pkcs7_st,
        pub digest: *mut ASN1_OCTET_STRING,
    }
    #[src_loc = "143:1"]
    pub type PKCS7_SIGN_ENVELOPE = pkcs7_signedandenveloped_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "143:9"]
    pub struct pkcs7_signedandenveloped_st {
        pub version: *mut ASN1_INTEGER,
        pub md_algs: *mut stack_st_X509_ALGOR,
        pub cert: *mut stack_st_X509,
        pub crl: *mut stack_st_X509_CRL,
        pub signer_info: *mut stack_st_PKCS7_SIGNER_INFO,
        pub enc_data: *mut PKCS7_ENC_CONTENT,
        pub recipientinfo: *mut stack_st_PKCS7_RECIP_INFO,
    }
    #[src_loc = "137:1"]
    pub type PKCS7_ENVELOPE = pkcs7_enveloped_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "137:9"]
    pub struct pkcs7_enveloped_st {
        pub version: *mut ASN1_INTEGER,
        pub recipientinfo: *mut stack_st_PKCS7_RECIP_INFO,
        pub enc_data: *mut PKCS7_ENC_CONTENT,
    }
    #[src_loc = "117:1"]
    pub type PKCS7_SIGNED = pkcs7_signed_st;
    #[src_loc = "165:1"]
    pub type PKCS7 = pkcs7_st;
    use super::stack_h::_STACK;
    use super::ossl_typ_h::{ASN1_INTEGER, ASN1_OBJECT, ASN1_OCTET_STRING,
                            X509_ALGOR, EVP_CIPHER};
    use super::asn1_h::{stack_st_X509_ALGOR, ASN1_TYPE};
    use super::x509_h::{stack_st_X509, stack_st_X509_CRL};
    use super::libc;
    /* version 0 */
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the PKCS7 functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pkcs12.h:32"]
pub mod pkcs12_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "108:9"]
    pub struct PKCS12 {
        pub version: *mut ASN1_INTEGER,
        pub mac: *mut PKCS12_MAC_DATA,
        pub authsafes: *mut PKCS7,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "102:9"]
    pub struct PKCS12_MAC_DATA {
        pub dinfo: *mut X509_SIG,
        pub salt: *mut ASN1_OCTET_STRING,
        pub iter: *mut ASN1_INTEGER,
    }
    use super::ossl_typ_h::{ASN1_INTEGER, ASN1_OCTET_STRING,
                            PKCS8_PRIV_KEY_INFO, EVP_PKEY, X509};
    use super::pkcs7_h::PKCS7;
    use super::x509_h::{X509_SIG, stack_st_X509};
    use super::libc;
    use super::_stdio_h::FILE;
    extern "C" {
        #[no_mangle]
        #[src_loc = "180:1"]
        pub fn PKCS8_decrypt(p8: *mut X509_SIG, pass: *const libc::c_char,
                             passlen: libc::c_int)
         -> *mut PKCS8_PRIV_KEY_INFO;
        #[no_mangle]
        #[src_loc = "246:1"]
        pub fn PKCS12_free(a: *mut PKCS12);
        #[no_mangle]
        #[src_loc = "255:1"]
        pub fn PKCS12_parse(p12: *mut PKCS12, pass: *const libc::c_char,
                            pkey: *mut *mut EVP_PKEY, cert: *mut *mut X509,
                            ca: *mut *mut stack_st_X509) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "272:1"]
        pub fn d2i_PKCS12_fp(fp: *mut FILE, p12: *mut *mut PKCS12)
         -> *mut PKCS12;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ui.h:29"]
pub mod ui_h {
    #[src_loc = "304:1"]
    pub type UI_STRING = ui_string_st;
    #[src_loc = "311:1"]
    pub type UI_string_types = libc::c_uint;
    #[src_loc = "317:5"]
    pub const UIT_ERROR: UI_string_types = 5;
    #[src_loc = "316:5"]
    pub const UIT_INFO: UI_string_types = 4;
    #[src_loc = "315:5"]
    pub const UIT_BOOLEAN: UI_string_types = 3;
    #[src_loc = "314:5"]
    pub const UIT_VERIFY: UI_string_types = 2;
    #[src_loc = "313:5"]
    pub const UIT_PROMPT: UI_string_types = 1;
    #[src_loc = "312:5"]
    pub const UIT_NONE: UI_string_types = 0;
    use super::libc;
    use super::ossl_typ_h::{UI_METHOD, UI};
    extern "C" {
        #[src_loc = "304:9"]
        pub type ui_string_st;
        #[no_mangle]
        #[src_loc = "329:1"]
        pub fn UI_method_set_closer(method: *mut UI_METHOD,
                                    closer:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut UI)
                                                   -> libc::c_int>)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "322:1"]
        pub fn UI_destroy_method(ui_method: *mut UI_METHOD);
        #[no_mangle]
        #[src_loc = "321:1"]
        pub fn UI_create_method(name: *mut libc::c_char) -> *mut UI_METHOD;
        #[no_mangle]
        #[src_loc = "326:1"]
        pub fn UI_method_set_flusher(method: *mut UI_METHOD,
                                     flusher:
                                         Option<unsafe extern "C" fn(_:
                                                                         *mut UI)
                                                    -> libc::c_int>)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "209:1"]
        pub fn UI_add_user_data(ui: *mut UI, user_data: *mut libc::c_void)
         -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "211:1"]
        pub fn UI_get0_user_data(ui: *mut UI) -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "323:1"]
        pub fn UI_method_set_opener(method: *mut UI_METHOD,
                                    opener:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut UI)
                                                   -> libc::c_int>)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "324:1"]
        pub fn UI_method_set_writer(method: *mut UI_METHOD,
                                    writer:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut UI,
                                                                    _:
                                                                        *mut UI_STRING)
                                                   -> libc::c_int>)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "355:1"]
        pub fn UI_get0_output_string(uis: *mut UI_STRING)
         -> *const libc::c_char;
        #[no_mangle]
        #[src_loc = "353:1"]
        pub fn UI_get_input_flags(uis: *mut UI_STRING) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "351:1"]
        pub fn UI_get_string_type(uis: *mut UI_STRING) -> UI_string_types;
        /* Set the result of a UI_STRING. */
        #[no_mangle]
        #[src_loc = "372:1"]
        pub fn UI_set_result(ui: *mut UI, uis: *mut UI_STRING,
                             result: *const libc::c_char) -> libc::c_int;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the UI functions. */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:20"]
pub mod errno_h {
    use super::libc;
    extern "C" {
        /*
 * Copyright (c) 2000-2012 Apple, Inc. All rights reserved.
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
/* Copyright (c) 1995 NeXT Computer, Inc. All Rights Reserved */
/*
 * Copyright (c) 1982, 1986, 1989, 1993
 *	The Regents of the University of California.  All rights reserved.
 * (c) UNIX System Laboratories, Inc.
 * All or some portions of this file are derived from material licensed
 * to the University of California by American Telephone and Telegraph
 * Co. or Unix System Laboratories, Inc. and are reproduced herein with
 * the permission of UNIX System Laboratories, Inc.
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
 *	@(#)errno.h	8.5 (Berkeley) 1/21/94
 */
        #[no_mangle]
        #[src_loc = "80:1"]
        pub fn __error() -> *mut libc::c_int;
    }
    /* _SYS_ERRNO_H_ */
    /* Must be equal largest errno */
    /* Interface output queue is full */
    /* Previous owner died */
    /* State not recoverable */
    /* No such policy registered */
    /* __DARWIN_UNIX03 || KERNEL */
    /* Operation not supported on socket */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:21"]
pub mod string_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "72:7"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "81:7"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[src_loc = "84:6"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "117:7"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/libkern/i386/_OSByteOrder.h:22"]
pub mod _OSByteOrder_h {
    #[inline]
    #[src_loc = "53:1"]
    pub unsafe extern "C" fn _OSSwapInt32(mut _data: __uint32_t)
     -> __uint32_t {
        return _data.swap_bytes();
    }
    use super::_types_h::__uint32_t;
    /* ! _OS__OSBYTEORDERI386_H */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arpa/inet.h:25"]
pub mod inet_h {
    use super::libc;
    use super::_socklen_t_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "77:1"]
        pub fn inet_ntop(_: libc::c_int, _: *const libc::c_void,
                         _: *mut libc::c_char, _: socklen_t)
         -> *const libc::c_char;
        #[no_mangle]
        #[src_loc = "78:1"]
        pub fn inet_pton(_: libc::c_int, _: *const libc::c_char,
                         _: *mut libc::c_void) -> libc::c_int;
    }
    /* !_ARPA_INET_H_ */
    /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:25"]
pub mod unistd_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "437:1"]
        pub fn close(_: libc::c_int) -> libc::c_int;
    }
    /* _UNISTD_H_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:25"]
pub mod _malloc_h {
    use super::libc;
    extern "C" {
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
        #[src_loc = "40:7"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "42:1"]
        pub fn free(_: *mut libc::c_void);
        #[no_mangle]
        #[src_loc = "41:7"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
    /* _MALLOC_UNDERSCORE_MALLOC_H_ */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/err.h:25"]
pub mod err_h {
    use super::libc;
    use super::_size_t_h::size_t;
    extern "C" {
        /* crypto/err/err.h */
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
 * Copyright (c) 1998-2019 The OpenSSL Project.  All rights reserved.
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
        /* library */
        /* #define ERR_LIB_METH         12 */
        /* #define ERR_LIB_SSL23        21 */
/* #define ERR_LIB_SSL2         22 */
/* #define ERR_LIB_SSL3         23 */
/* #define ERR_LIB_RSAREF       30 */
/* #define ERR_LIB_PROXY        31 */
        /*
 * Borland C seems too stupid to be able to shift and do longs in the
 * pre-processor :-(
 */
        /* OS functions */
        /* Winsock stuff */
        /* reasons */
        /* 2 */
        /* 3 */
        /* 4 */
        /* 5 */
        /* 6 */
        /* 7 */
        /* 8 */
        /* 9 */
        /* 10 */
        /* 11 */
        /* 13 */
        /* 14 */
        /* 15 */
        /* 16 */
        /* 20 */
        /* 32 */
        /* 33 */
        /* 34 */
        /* 35 */
        /* 36 */
        /* 37 */
        /* 38 */
        /* 39 */
        /* 40 */
        /* 41 */
        /* 42 */
        /* 43 */
        /* 44 */
        /* 45 */
        /* fatal error */
        /*
 * 99 is the maximum possible ERR_R_... code, higher values are reserved for
 * the individual libraries
 */
        #[no_mangle]
        #[src_loc = "335:1"]
        pub fn ERR_clear_error();
        #[no_mangle]
        #[src_loc = "327:1"]
        pub fn ERR_peek_error() -> libc::c_ulong;
        #[no_mangle]
        #[src_loc = "331:1"]
        pub fn ERR_peek_last_error() -> libc::c_ulong;
        #[no_mangle]
        #[src_loc = "341:1"]
        pub fn ERR_print_errors_cb(cb:
                                       Option<unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _: size_t,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> libc::c_int>,
                                   u: *mut libc::c_void);
    }
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:25"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "143:1"]
        pub fn fclose(_: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "149:1"]
        pub fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE)
         -> *mut libc::c_char;
        #[no_mangle]
        #[src_loc = "162:1"]
        pub fn fseek(_: *mut FILE, _: libc::c_long, _: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "180:6"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
         -> libc::c_int;
    }
    /* _STDIO_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/time.h:25"]
pub mod time_h {
    use super::_time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "118:1"]
        pub fn time(_: *mut time_t) -> time_t;
    }
    /* !_TIME_H_ */
    /* _USE_EXTENDED_LOCALES_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_select.h:25"]
pub mod _select_h {
    use super::libc;
    use super::_fd_def_h::fd_set;
    use super::_timeval_h::timeval;
    extern "C" {
        /*
 * Copyright (c) 2005, 2007 Apple Inc. All rights reserved.
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
        /*
 * This is called from sys/select.h and sys/time.h for the common prototype
 * of select().  Setting _DARWIN_C_SOURCE or _DARWIN_UNLIMITED_SELECT uses
 * the version of select() that does not place a limit on the first argument
 * (nfds).  In the UNIX conformance case, values of nfds greater than
 * FD_SETSIZE will return an error of EINVAL.
 */
        #[no_mangle]
        #[src_loc = "39:1"]
        pub fn select(_: libc::c_int, _: *mut fd_set, _: *mut fd_set,
                      _: *mut fd_set, _: *mut timeval) -> libc::c_int;
    }
    /* !_SYS__SELECT_H_ */
    /* _DARWIN_C_SOURCE || _DARWIN_UNLIMITED_SELECT */
    /* __LP64__ && !__DARWIN_NON_CANCELABLE */
    /* !__LP64__ || __DARWIN_NON_CANCELABLE */
}
#[header_src = "/usr/local/Cellar/gettext/0.20.1/include/libintl.h:25"]
pub mod libintl_h {
    use super::libc;
    extern "C" {
        /* Look up MSGID in the DOMAINNAME message catalog for the current
   LC_MESSAGES locale.  */
        #[no_mangle]
        #[src_loc = "156:1"]
        pub fn libintl_dgettext(__domainname: *const libc::c_char,
                                __msgid: *const libc::c_char)
         -> *mut libc::c_char;
    }
    /* libintl.h */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/engine.h:29"]
pub mod engine_h {
    use super::libc;
    use super::ossl_typ_h::{ENGINE, UI_METHOD, EVP_PKEY};
    extern "C" {
        /* Retrieve an engine from the list by its unique "id" value. */
        #[no_mangle]
        #[src_loc = "393:1"]
        pub fn ENGINE_by_id(id: *const libc::c_char) -> *mut ENGINE;
        #[no_mangle]
        #[src_loc = "417:1"]
        pub fn ENGINE_load_builtin_engines();
        /*
 * This function works like ENGINE_ctrl() with the exception of taking a
 * command name instead of a command number, and can handle optional
 * commands. See the comment on ENGINE_ctrl_cmd_string() for an explanation
 * on how to use the cmd_name and cmd_optional.
 */
        #[no_mangle]
        #[src_loc = "514:1"]
        pub fn ENGINE_ctrl_cmd(e: *mut ENGINE, cmd_name: *const libc::c_char,
                               i: libc::c_long, p: *mut libc::c_void,
                               f: Option<unsafe extern "C" fn() -> ()>,
                               cmd_optional: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "551:1"]
        pub fn ENGINE_free(e: *mut ENGINE) -> libc::c_int;
        /*
 * FUNCTIONAL functions. These functions deal with ENGINE structures that
 * have (or will) be initialised for use. Broadly speaking, the structural
 * functions are useful for iterating the list of available engine types,
 * creating new engine types, and other "list" operations. These functions
 * actually deal with ENGINEs that are to be used. As such these functions
 * can fail (if applicable) when particular engines are unavailable - eg. if
 * a hardware accelerator is not attached or not functioning correctly. Each
 * ENGINE has 2 reference counts; structural and functional. Every time a
 * functional reference is obtained or released, a corresponding structural
 * reference is automatically obtained or released too.
 */
        /*
 * Initialise a engine type for use (or up its reference count if it's
 * already in use). This will fail if the engine is not currently operational
 * and cannot initialise.
 */
        #[no_mangle]
        #[src_loc = "651:1"]
        pub fn ENGINE_init(e: *mut ENGINE) -> libc::c_int;
        /*
 * Free a functional reference to a engine type. This does not require a
 * corresponding call to ENGINE_free as it also releases a structural
 * reference.
 */
        #[no_mangle]
        #[src_loc = "657:1"]
        pub fn ENGINE_finish(e: *mut ENGINE) -> libc::c_int;
        /*
 * The following functions handle keys that are stored in some secondary
 * location, handled by the engine.  The storage may be on a card or
 * whatever.
 */
        #[no_mangle]
        #[src_loc = "664:1"]
        pub fn ENGINE_load_private_key(e: *mut ENGINE,
                                       key_id: *const libc::c_char,
                                       ui_method: *mut UI_METHOD,
                                       callback_data: *mut libc::c_void)
         -> *mut EVP_PKEY;
        /*
 * This sets a new default ENGINE structure for performing RSA operations. If
 * the result is non-zero (success) then the ENGINE structure will have had
 * its reference count up'd so the caller should still free their own
 * reference 'e'.
 */
        #[no_mangle]
        #[src_loc = "701:1"]
        pub fn ENGINE_set_default_RSA(e: *mut ENGINE) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "708:1"]
        pub fn ENGINE_set_default_RAND(e: *mut ENGINE) -> libc::c_int;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the ENGINE functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/rand.h:29"]
pub mod rand_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "101:1"]
        pub fn RAND_bytes(buf: *mut libc::c_uchar, num: libc::c_int)
         -> libc::c_int;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the RAND functions. */
}
pub use self::_types_h::{__uint8_t, __int32_t, __uint32_t, __int64_t,
                         __darwin_size_t, __darwin_socklen_t,
                         __darwin_time_t};
pub use self::sys__types_h::{__darwin_gid_t, __darwin_off_t,
                             __darwin_suseconds_t, __darwin_uid_t};
pub use self::_size_t_h::size_t;
pub use self::_gid_t_h::gid_t;
pub use self::_uid_t_h::uid_t;
pub use self::_time_t_h::time_t;
pub use self::_fd_def_h::fd_set;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::socket_h::sockaddr;
pub use self::_timeval_h::timeval;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::netdb_h::addrinfo;
pub use self::openconnect_h::{oc_form_opt, oc_choice, oc_form_opt_select,
                              oc_auth_form, oc_split_include, oc_ip_info,
                              oc_vpn_option, oc_stats, oc_cert,
                              openconnect_info, openconnect_reconnected_vfn,
                              openconnect_setup_tun_vfn,
                              openconnect_getaddrinfo_vfn,
                              openconnect_protect_socket_vfn,
                              openconnect_progress_vfn,
                              openconnect_process_auth_form_vfn,
                              openconnect_write_new_config_vfn,
                              openconnect_validate_peer_cert_vfn,
                              openconnect_stats_vfn,
                              openconnect_unlock_token_vfn,
                              openconnect_lock_token_vfn};
pub use self::openconnect_internal_h::{pkt_q, pkt, C2RustUnnamed,
                                       C2RustUnnamed_0, C2RustUnnamed_1,
                                       C2RustUnnamed_2, C2RustUnnamed_3,
                                       C2RustUnnamed_4, keepalive_info,
                                       pin_cache, oc_text_buf,
                                       C2RustUnnamed_12, HOTP_SECRET_PSKC,
                                       HOTP_SECRET_HEX, HOTP_SECRET_RAW,
                                       HOTP_SECRET_BASE32, C2RustUnnamed_13,
                                       OATH_ALG_HMAC_SHA512,
                                       OATH_ALG_HMAC_SHA256,
                                       OATH_ALG_HMAC_SHA1, http_auth_state,
                                       C2RustUnnamed_14, C2RustUnnamed_15,
                                       C2RustUnnamed_16, esp, vpn_proto,
                                       oc_packed_uint32_t, load_be32,
                                       oc_pcsc_ctx,
                                       openconnect_utf8_to_legacy,
                                       openconnect_print_err_cb,
                                       pulse_eap_ttls_recv,
                                       pulse_eap_ttls_send, is_cancel_pending,
                                       cmd_fd_set, string_is_hostname,
                                       connect_https_socket,
                                       request_passphrase,
                                       openconnect_fopen_utf8,
                                       load_pkcs11_key,
                                       load_pkcs11_certificate, free_pass,
                                       process_auth_form};
pub use self::ossl_typ_h::{SSL, ssl_st, BIGNUM, bignum_st, SSL_CTX,
                           ssl_ctx_st, EVP_MD_CTX, env_md_ctx_st,
                           EVP_PKEY_CTX, ENGINE, EVP_MD, env_md_st,
                           EVP_CIPHER_CTX, evp_cipher_ctx_st, EVP_CIPHER,
                           evp_cipher_st, ASN1_STRING, asn1_string_st,
                           ASN1_UTF8STRING, ASN1_VISIBLESTRING,
                           ASN1_GENERALIZEDTIME, ASN1_UTCTIME,
                           ASN1_UNIVERSALSTRING, ASN1_BMPSTRING,
                           ASN1_GENERALSTRING, ASN1_IA5STRING, ASN1_T61STRING,
                           ASN1_PRINTABLESTRING, ASN1_OCTET_STRING,
                           ASN1_BIT_STRING, ASN1_ENUMERATED, ASN1_INTEGER,
                           ASN1_OBJECT, asn1_object_st, ASN1_BOOLEAN,
                           X509_STORE_CTX, x509_store_ctx_st, CRYPTO_EX_DATA,
                           crypto_ex_data_st, X509_CRL, X509_crl_st,
                           X509_CRL_METHOD, ISSUING_DIST_POINT,
                           ISSUING_DIST_POINT_st, X509_NAME, X509_name_st,
                           BUF_MEM, buf_mem_st, AUTHORITY_KEYID,
                           AUTHORITY_KEYID_st, X509_ALGOR, X509_algor_st,
                           ASN1_TIME, X509, x509_st, NAME_CONSTRAINTS,
                           NAME_CONSTRAINTS_st, X509_POLICY_CACHE,
                           X509_PUBKEY, X509_pubkey_st, EVP_PKEY, evp_pkey_st,
                           dh_st, DH_METHOD, dh_method, BN_GENCB, bn_gencb_st,
                           DH, BN_MONT_CTX, bn_mont_ctx_st, BN_CTX, dsa_st,
                           DSA_METHOD, dsa_method, DSA, rsa_st, BN_BLINDING,
                           RSA_METHOD, rsa_meth_st, RSA, EVP_PKEY_ASN1_METHOD,
                           X509_POLICY_TREE, X509_STORE, x509_store_st,
                           COMP_METHOD, comp_method_st,
                           pkcs8_priv_key_info_st, PKCS8_PRIV_KEY_INFO, UI,
                           UI_METHOD, evp_pkey_ctx_st, engine_st,
                           x509_crl_method_st, X509_POLICY_CACHE_st,
                           bignum_ctx, bn_blinding_st,
                           evp_pkey_asn1_method_st, X509_POLICY_TREE_st,
                           ui_st, ui_method_st};
pub use self::ssl_h::{SRP_CTX, srp_ctx_st, SRTP_PROTECTION_PROFILE,
                      srtp_protection_profile_st,
                      stack_st_SRTP_PROTECTION_PROFILE, GEN_SESSION_CB,
                      stack_st_SSL_COMP, C2RustUnnamed_9, SSL_SESSION,
                      ssl_session_st, stack_st_SSL_CIPHER, SSL_CIPHER,
                      ssl_cipher_st, lhash_st_SSL_SESSION, SSL_METHOD,
                      ssl_method_st, tls_session_secret_cb_fn,
                      tls_session_ticket_ext_cb_fn, TLS_SESSION_TICKET_EXT,
                      tls_session_ticket_ext_st, dtls1_state_st,
                      ssl3_state_st, SSL_COMP, ssl_comp_st, ssl2_state_st,
                      ssl3_buf_freelist_st, cert_st, sess_cert_st,
                      ssl3_enc_method, stack_st_OCSP_RESPID,
                      SSL_load_error_strings, SSL_library_init,
                      SSL_CTX_set_cipher_list, SSL_CTX_new, SSL_CTX_free,
                      SSL_CTX_get_cert_store, SSL_get_current_cipher,
                      SSL_CIPHER_get_name, SSL_set_bio, SSL_set_verify,
                      SSL_CTX_set_cert_verify_callback,
                      SSL_CTX_use_PrivateKey, SSL_CTX_use_certificate,
                      SSL_CTX_check_private_key, SSL_new, SSL_CTX_set_purpose,
                      SSL_free, SSL_connect, SSL_read, SSL_write, SSL_ctrl,
                      SSL_CTX_ctrl, SSL_get_error, SSLv23_client_method,
                      SSL_get_certificate, SSL_CTX_set_default_verify_paths,
                      SSL_CTX_load_verify_locations};
pub use self::stack_h::{_STACK, stack_st, sk_value, sk_num, sk_new_null,
                        sk_pop_free, sk_push};
pub use self::hmac_h::{HMAC_CTX, hmac_ctx_st, HMAC};
pub use self::asn1_h::{ASN1_TYPE, asn1_type_st, C2RustUnnamed_5, ASN1_VALUE,
                       stack_st_ASN1_OBJECT, ASN1_ENCODING, ASN1_ENCODING_st,
                       stack_st_X509_ALGOR, ASN1_VALUE_st, ASN1_TIME_print};
pub use self::x509_vfy_h::{X509_VERIFY_PARAM, X509_VERIFY_PARAM_st,
                           X509_VERIFY_PARAM_ID, stack_st_X509_LOOKUP,
                           stack_st_X509_OBJECT, X509_VERIFY_PARAM_ID_st,
                           X509_STORE_CTX_new, X509_STORE_CTX_free,
                           X509_STORE_CTX_init, X509_STORE_CTX_get_error,
                           X509_STORE_CTX_get0_param,
                           X509_VERIFY_PARAM_set_flags};
pub use self::crypto_h::{stack_st_void, bio_st, CRYPTO_free, CRYPTO_add_lock};
pub use self::x509_h::{stack_st_X509_NAME_ENTRY, stack_st_GENERAL_NAME,
                       X509_CRL_INFO, X509_crl_info_st,
                       stack_st_X509_EXTENSION, stack_st_X509_REVOKED,
                       X509_CERT_AUX, x509_cert_aux_st, stack_st_DIST_POINT,
                       X509_CINF, x509_cinf_st, X509_VAL, X509_val_st,
                       stack_st_X509, stack_st_X509_CRL, stack_st_X509_NAME,
                       X509_EXTENSIONS, X509_sig_st, X509_SIG,
                       stack_st_GENERAL_NAMES, X509_verify_cert_error_string,
                       X509_print_ex, i2d_X509_bio, i2d_X509, X509_digest,
                       d2i_PKCS8_fp, d2i_PrivateKey_fp, i2d_PUBKEY_bio,
                       X509_cmp_time, X509_SIG_free, X509_free,
                       X509_NAME_oneline, X509_get_subject_name,
                       X509_get_pubkey, X509_cmp, X509_verify_cert,
                       PKCS8_PRIV_KEY_INFO_free, EVP_PKCS82PKEY};
pub use self::x509v3_h::{DIST_POINT_NAME, DIST_POINT_NAME_st, C2RustUnnamed_6,
                         GENERAL_NAMES, stack_st_GENERAL_SUBTREE,
                         X509_check_issued, X509_check_host, X509_check_ip};
pub use self::evp_h::{stack_st_X509_ATTRIBUTE, C2RustUnnamed_7, ec_key_st,
                      OPENSSL_add_all_algorithms_noconf, EVP_MD_CTX_create,
                      EVP_MD_CTX_destroy, EVP_Digest, EVP_md5, EVP_sha1,
                      EVP_sha256, EVP_sha512, EVP_PKEY_free,
                      PKCS5_PBKDF2_HMAC_SHA1};
pub use self::bn_h::C2RustUnnamed_8;
pub use self::dsa_h::{DSA_SIG, DSA_SIG_st};
pub use self::pem_h::{pem_password_cb, PEM_read_bio_X509,
                      PEM_read_bio_X509_AUX, PEM_read_bio_PrivateKey};
pub use self::comp_h::{COMP_CTX, comp_ctx_st};
pub use self::dtls1_h::{dtls1_timeout_st, hm_header_st,
                        dtls1_retransmit_state, record_pqueue,
                        record_pqueue_st, DTLS1_BITMAP, dtls1_bitmap_st};
pub use self::pqueue_h::{pqueue, _pqueue};
pub use self::ssl3_h::{C2RustUnnamed_10, SSL3_RECORD, ssl3_record_st,
                       SSL3_BUFFER, ssl3_buffer_st};
pub use self::ec_h::EC_KEY;
pub use self::bio_h::{BIO, BIO_METHOD, bio_method_st, bio_info_cb, BIO_s_mem,
                      BIO_new, BIO_free, BIO_ctrl, BIO_write, BIO_new_fp,
                      BIO_new_socket};
pub use self::ssl2_h::C2RustUnnamed_11;
pub use self::zlib_h::{z_stream, z_stream_s, free_func, alloc_func,
                       internal_state};
pub use self::zconf_h::{uLong, voidpf, uInt, Bytef, Byte};
use self::stoken_h::stoken_ctx;
pub use self::gssapi_h::{gss_ctx_id_t, gss_name_t, gss_ctx_id_struct,
                         gss_name_struct};
pub use self::proxy_h::{pxProxyFactory, pxProxyFactory_};
pub use self::tree_h::{xmlNode, _xmlNode, xmlNs, _xmlNs, _xmlDoc, _xmlDtd,
                       xmlElementType, XML_DOCB_DOCUMENT_NODE,
                       XML_XINCLUDE_END, XML_XINCLUDE_START,
                       XML_NAMESPACE_DECL, XML_ENTITY_DECL,
                       XML_ATTRIBUTE_DECL, XML_ELEMENT_DECL, XML_DTD_NODE,
                       XML_HTML_DOCUMENT_NODE, XML_NOTATION_NODE,
                       XML_DOCUMENT_FRAG_NODE, XML_DOCUMENT_TYPE_NODE,
                       XML_DOCUMENT_NODE, XML_COMMENT_NODE, XML_PI_NODE,
                       XML_ENTITY_NODE, XML_ENTITY_REF_NODE,
                       XML_CDATA_SECTION_NODE, XML_TEXT_NODE,
                       XML_ATTRIBUTE_NODE, XML_ELEMENT_NODE, xmlNsType,
                       _xmlAttr, xmlAttributeType, XML_ATTRIBUTE_NOTATION,
                       XML_ATTRIBUTE_ENUMERATION, XML_ATTRIBUTE_NMTOKENS,
                       XML_ATTRIBUTE_NMTOKEN, XML_ATTRIBUTE_ENTITIES,
                       XML_ATTRIBUTE_ENTITY, XML_ATTRIBUTE_IDREFS,
                       XML_ATTRIBUTE_IDREF, XML_ATTRIBUTE_ID,
                       XML_ATTRIBUTE_CDATA};
use self::dict_h::_xmlDict;
pub use self::xmlstring_h::xmlChar;
pub use self::iconv_h::iconv_t;
pub use self::_stdio_h::{fpos_t, __sbuf, __sFILE, FILE, __sFILEX};
pub use self::pkcs7_h::{stack_st_PKCS7_SIGNER_INFO, stack_st_PKCS7_RECIP_INFO,
                        pkcs7_signed_st, pkcs7_st, C2RustUnnamed_17,
                        PKCS7_ENCRYPT, pkcs7_encrypted_st, PKCS7_ENC_CONTENT,
                        pkcs7_enc_content_st, PKCS7_DIGEST, pkcs7_digest_st,
                        PKCS7_SIGN_ENVELOPE, pkcs7_signedandenveloped_st,
                        PKCS7_ENVELOPE, pkcs7_enveloped_st, PKCS7_SIGNED,
                        PKCS7};
pub use self::pkcs12_h::{PKCS12, PKCS12_MAC_DATA, PKCS8_decrypt, PKCS12_free,
                         PKCS12_parse, d2i_PKCS12_fp};
pub use self::ui_h::{UI_STRING, UI_string_types, UIT_ERROR, UIT_INFO,
                     UIT_BOOLEAN, UIT_VERIFY, UIT_PROMPT, UIT_NONE,
                     ui_string_st, UI_method_set_closer, UI_destroy_method,
                     UI_create_method, UI_method_set_flusher,
                     UI_add_user_data, UI_get0_user_data,
                     UI_method_set_opener, UI_method_set_writer,
                     UI_get0_output_string, UI_get_input_flags,
                     UI_get_string_type, UI_set_result};
use self::errno_h::__error;
use self::string_h::{memcpy, memset, strcmp, strerror, strlen, strncmp,
                     strdup};
pub use self::_OSByteOrder_h::_OSSwapInt32;
use self::inet_h::{inet_ntop, inet_pton};
use self::unistd_h::close;
use self::_malloc_h::{malloc, free, calloc};
use self::err_h::{ERR_clear_error, ERR_peek_error, ERR_peek_last_error,
                  ERR_print_errors_cb};
use self::stdio_h::{fclose, fgets, fseek, sprintf};
use self::time_h::time;
use self::_select_h::select;
use self::libintl_h::libintl_dgettext;
use self::engine_h::{ENGINE_by_id, ENGINE_load_builtin_engines,
                     ENGINE_ctrl_cmd, ENGINE_free, ENGINE_init, ENGINE_finish,
                     ENGINE_load_private_key, ENGINE_set_default_RSA,
                     ENGINE_set_default_RAND};
use self::rand_h::RAND_bytes;
#[src_loc = "47:1"]
pub type X509_STORE_CTX_get_issuer_fn
    =
    Option<unsafe extern "C" fn(_: *mut *mut X509, _: *mut X509_STORE_CTX,
                                _: *mut X509) -> libc::c_int>;
/* UI handling. All this just to handle the PIN callback from the TPM ENGINE,
   and turn it into a call to our ->process_auth_form function */
#[derive ( Copy, Clone )]
#[repr(C)]
#[src_loc = "326:1"]
pub struct ui_data {
    pub vpninfo: *mut openconnect_info,
    pub last_opt: *mut *mut oc_form_opt,
    pub form: oc_auth_form,
}
#[derive ( Copy, Clone )]
#[repr(C)]
#[src_loc = "332:1"]
pub struct ui_form_opt {
    pub opt: oc_form_opt,
    pub uis: *mut UI_STRING,
}
#[no_mangle]
#[src_loc = "52:1"]
pub unsafe extern "C" fn openconnect_sha1(mut result: *mut libc::c_uchar,
                                          mut data: *mut libc::c_void,
                                          mut len: libc::c_int)
 -> libc::c_int {
    let mut c: *mut EVP_MD_CTX = EVP_MD_CTX_create();
    if c.is_null() { return -12i32 }
    EVP_Digest(data, len as size_t, result, 0 as *mut libc::c_uint,
               EVP_sha1(), 0 as *mut ENGINE);
    EVP_MD_CTX_destroy(c);
    return 0i32;
}
#[no_mangle]
#[src_loc = "65:1"]
pub unsafe extern "C" fn openconnect_sha256(mut result: *mut libc::c_uchar,
                                            mut data: *mut libc::c_void,
                                            mut len: libc::c_int)
 -> libc::c_int {
    let mut c: *mut EVP_MD_CTX = EVP_MD_CTX_create();
    if c.is_null() { return -12i32 }
    EVP_Digest(data, len as size_t, result, 0 as *mut libc::c_uint,
               EVP_sha256(), 0 as *mut ENGINE);
    EVP_MD_CTX_destroy(c);
    return 0i32;
}
#[no_mangle]
#[src_loc = "78:1"]
pub unsafe extern "C" fn openconnect_md5(mut result: *mut libc::c_uchar,
                                         mut data: *mut libc::c_void,
                                         mut len: libc::c_int)
 -> libc::c_int {
    let mut c: *mut EVP_MD_CTX = EVP_MD_CTX_create();
    if c.is_null() { return -12i32 }
    EVP_Digest(data, len as size_t, result, 0 as *mut libc::c_uint, EVP_md5(),
               0 as *mut ENGINE);
    EVP_MD_CTX_destroy(c);
    return 0i32;
}
#[no_mangle]
#[src_loc = "91:1"]
pub unsafe extern "C" fn openconnect_get_peer_cert_DER(mut vpninfo:
                                                           *mut openconnect_info,
                                                       mut buf:
                                                           *mut *mut libc::c_uchar)
 -> libc::c_int {
    let mut bp: *mut BIO = BIO_new(BIO_s_mem());
    let mut certinfo: *mut BUF_MEM = 0 as *mut BUF_MEM;
    let mut l: size_t = 0;
    if i2d_X509_bio(bp, (*vpninfo).peer_cert as *mut X509) == 0 {
        BIO_free(bp);
        return -5i32
    }
    BIO_ctrl(bp, 115i32, 0i32 as libc::c_long,
             &mut certinfo as *mut *mut BUF_MEM as *mut libc::c_char as
                 *mut libc::c_void);
    l = (*certinfo).length;
    *buf = malloc(l) as *mut libc::c_uchar;
    if (*buf).is_null() { BIO_free(bp); return -12i32 }
    memcpy(*buf as *mut libc::c_void, (*certinfo).data as *const libc::c_void,
           l);
    BIO_free(bp);
    return l as libc::c_int;
}
#[no_mangle]
#[src_loc = "115:1"]
pub unsafe extern "C" fn openconnect_random(mut bytes: *mut libc::c_void,
                                            mut len: libc::c_int)
 -> libc::c_int {
    if RAND_bytes(bytes as *mut libc::c_uchar, len) != 1i32 { return -5i32 }
    return 0i32;
}
/* Helper functions for reading/writing lines over SSL.
   We could use cURL for the HTTP stuff, but it's overkill */
#[src_loc = "125:1"]
unsafe extern "C" fn _openconnect_openssl_write(mut ssl: *mut SSL,
                                                mut fd: libc::c_int,
                                                mut vpninfo:
                                                    *mut openconnect_info,
                                                mut buf: *mut libc::c_char,
                                                mut len: size_t)
 -> libc::c_int {
    let mut orig_len: size_t = len;
    while len != 0 {
        let mut done: libc::c_int =
            SSL_write(ssl, buf as *const libc::c_void, len as libc::c_int);
        if done > 0i32 {
            len =
                (len as libc::c_ulong).wrapping_sub(done as libc::c_ulong) as
                    size_t as size_t
        } else {
            let mut err: libc::c_int = SSL_get_error(ssl, done);
            let mut wr_set: fd_set = fd_set{fds_bits: [0; 32],};
            let mut rd_set: fd_set = fd_set{fds_bits: [0; 32],};
            let mut maxfd: libc::c_int = fd;
            if err == 2i32 {
                let mut __fd: libc::c_int = fd;
                rd_set.fds_bits[(__fd as
                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(8i32
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as usize] |=
                    ((1i32 as libc::c_ulong) <<
                         (__fd as
                              libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                               as
                                                                                               libc::c_ulong)))
                        as __int32_t
            } else if err == 3i32 {
                let mut __fd_0: libc::c_int = fd;
                wr_set.fds_bits[(__fd_0 as
                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(8i32
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as usize] |=
                    ((1i32 as libc::c_ulong) <<
                         (__fd_0 as
                              libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                               as
                                                                                               libc::c_ulong)))
                        as __int32_t
            } else {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to write to SSL socket\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _: size_t,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> libc::c_int),
                                    vpninfo as *mut libc::c_void);
                return -5i32
            }
            cmd_fd_set(vpninfo, &mut rd_set, &mut maxfd);
            select(maxfd + 1i32, &mut rd_set, &mut wr_set, 0 as *mut fd_set,
                   0 as *mut timeval);
            if is_cancel_pending(vpninfo, &mut rd_set) != 0 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"SSL write cancelled\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                return -4i32
            }
        }
    }
    return orig_len as libc::c_int;
}
#[src_loc = "162:1"]
unsafe extern "C" fn openconnect_openssl_write(mut vpninfo:
                                                   *mut openconnect_info,
                                               mut buf: *mut libc::c_char,
                                               mut len: size_t)
 -> libc::c_int {
    return _openconnect_openssl_write((*vpninfo).https_ssl, (*vpninfo).ssl_fd,
                                      vpninfo, buf, len);
}
#[no_mangle]
#[src_loc = "167:1"]
pub unsafe extern "C" fn openconnect_dtls_write(mut vpninfo:
                                                    *mut openconnect_info,
                                                mut buf: *mut libc::c_void,
                                                mut len: size_t)
 -> libc::c_int {
    return _openconnect_openssl_write((*vpninfo).dtls_ssl, (*vpninfo).dtls_fd,
                                      vpninfo, buf as *mut libc::c_char, len);
}
/* set ms to zero for no timeout */
#[src_loc = "173:1"]
unsafe extern "C" fn _openconnect_openssl_read(mut ssl: *mut SSL,
                                               mut fd: libc::c_int,
                                               mut vpninfo:
                                                   *mut openconnect_info,
                                               mut buf: *mut libc::c_char,
                                               mut len: size_t,
                                               mut ms: libc::c_uint)
 -> libc::c_int {
    let mut done: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut timeout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut tv: *mut timeval = 0 as *mut timeval;
    if ms != 0 {
        timeout.tv_sec =
            ms.wrapping_div(1000i32 as libc::c_uint) as __darwin_time_t;
        timeout.tv_usec =
            ms.wrapping_rem(1000i32 as
                                libc::c_uint).wrapping_mul(1000i32 as
                                                               libc::c_uint)
                as __darwin_suseconds_t;
        tv = &mut timeout
    }
    loop  {
        done = SSL_read(ssl, buf as *mut libc::c_void, len as libc::c_int);
        if !(done == -1i32) { break ; }
        let mut err: libc::c_int = SSL_get_error(ssl, done);
        let mut wr_set: fd_set = fd_set{fds_bits: [0; 32],};
        let mut rd_set: fd_set = fd_set{fds_bits: [0; 32],};
        let mut maxfd: libc::c_int = fd;
        if err == 2i32 {
            let mut __fd: libc::c_int = fd;
            rd_set.fds_bits[(__fd as
                                 libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(8i32
                                                                                                  as
                                                                                                  libc::c_ulong))
                                as usize] |=
                ((1i32 as libc::c_ulong) <<
                     (__fd as
                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8i32
                                                                                           as
                                                                                           libc::c_ulong)))
                    as __int32_t
        } else if err == 3i32 {
            let mut __fd_0: libc::c_int = fd;
            wr_set.fds_bits[(__fd_0 as
                                 libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(8i32
                                                                                                  as
                                                                                                  libc::c_ulong))
                                as usize] |=
                ((1i32 as libc::c_ulong) <<
                     (__fd_0 as
                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8i32
                                                                                           as
                                                                                           libc::c_ulong)))
                    as __int32_t
        } else {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Failed to read from SSL socket\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                         unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _: size_t,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> libc::c_int),
                                vpninfo as *mut libc::c_void);
            return -5i32
        }
        cmd_fd_set(vpninfo, &mut rd_set, &mut maxfd);
        ret =
            select(maxfd + 1i32, &mut rd_set, &mut wr_set, 0 as *mut fd_set,
                   tv);
        if is_cancel_pending(vpninfo, &mut rd_set) != 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"SSL read cancelled\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            return -4i32
        }
        if ret == 0i32 { return -60i32 }
    }
    return done;
}
#[src_loc = "215:1"]
unsafe extern "C" fn openconnect_openssl_read(mut vpninfo:
                                                  *mut openconnect_info,
                                              mut buf: *mut libc::c_char,
                                              mut len: size_t)
 -> libc::c_int {
    return _openconnect_openssl_read((*vpninfo).https_ssl, (*vpninfo).ssl_fd,
                                     vpninfo, buf, len, 0i32 as libc::c_uint);
}
#[no_mangle]
#[src_loc = "220:1"]
pub unsafe extern "C" fn openconnect_dtls_read(mut vpninfo:
                                                   *mut openconnect_info,
                                               mut buf: *mut libc::c_void,
                                               mut len: size_t,
                                               mut ms: libc::c_uint)
 -> libc::c_int {
    return _openconnect_openssl_read((*vpninfo).dtls_ssl, (*vpninfo).dtls_fd,
                                     vpninfo, buf as *mut libc::c_char, len,
                                     ms);
}
#[src_loc = "225:1"]
unsafe extern "C" fn openconnect_openssl_gets(mut vpninfo:
                                                  *mut openconnect_info,
                                              mut buf: *mut libc::c_char,
                                              mut len: size_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0i32;
    let mut ret: libc::c_int = 0;
    if len < 2i32 as libc::c_ulong { return -22i32 }
    loop  {
        ret =
            SSL_read((*vpninfo).https_ssl,
                     buf.offset(i as isize) as *mut libc::c_void, 1i32);
        if ret == 1i32 {
            if *buf.offset(i as isize) as libc::c_int == '\n' as i32 {
                *buf.offset(i as isize) = 0i32 as libc::c_char;
                if i != 0 &&
                       *buf.offset((i - 1i32) as isize) as libc::c_int ==
                           '\r' as i32 {
                    *buf.offset((i - 1i32) as isize) = 0i32 as libc::c_char;
                    i -= 1
                }
                return i
            }
            i += 1;
            if i as libc::c_ulong >= len.wrapping_sub(1i32 as libc::c_ulong) {
                *buf.offset(i as isize) = 0i32 as libc::c_char;
                return i
            }
        } else {
            let mut rd_set: fd_set = fd_set{fds_bits: [0; 32],};
            let mut wr_set: fd_set = fd_set{fds_bits: [0; 32],};
            let mut maxfd: libc::c_int = (*vpninfo).ssl_fd;
            ret = SSL_get_error((*vpninfo).https_ssl, ret);
            if ret == 2i32 {
                let mut __fd: libc::c_int = (*vpninfo).ssl_fd;
                rd_set.fds_bits[(__fd as
                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(8i32
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as usize] |=
                    ((1i32 as libc::c_ulong) <<
                         (__fd as
                              libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                               as
                                                                                               libc::c_ulong)))
                        as __int32_t
            } else if ret == 3i32 {
                let mut __fd_0: libc::c_int = (*vpninfo).ssl_fd;
                wr_set.fds_bits[(__fd_0 as
                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(8i32
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as usize] |=
                    ((1i32 as libc::c_ulong) <<
                         (__fd_0 as
                              libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                               as
                                                                                               libc::c_ulong)))
                        as __int32_t
            } else {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to read from SSL socket\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _: size_t,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> libc::c_int),
                                    vpninfo as *mut libc::c_void);
                ret = -5i32;
                break ;
            }
            cmd_fd_set(vpninfo, &mut rd_set, &mut maxfd);
            select(maxfd + 1i32, &mut rd_set, &mut wr_set, 0 as *mut fd_set,
                   0 as *mut timeval);
            if !(is_cancel_pending(vpninfo, &mut rd_set) != 0) { continue ; }
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"SSL read cancelled\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ret = -4i32;
            break ;
        }
    }
    *buf.offset(i as isize) = 0i32 as libc::c_char;
    return if i != 0 { i } else { ret };
}
#[no_mangle]
#[src_loc = "281:1"]
pub unsafe extern "C" fn ssl_nonblock_read(mut vpninfo: *mut openconnect_info,
                                           mut buf: *mut libc::c_void,
                                           mut maxlen: libc::c_int)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    len = SSL_read((*vpninfo).https_ssl, buf, maxlen);
    if len > 0i32 { return len }
    ret = SSL_get_error((*vpninfo).https_ssl, len);
    if ret == 5i32 || ret == 6i32 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"SSL read error %d (server probably closed connection); reconnecting.\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    ret);
        }
        return -5i32
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "299:1"]
pub unsafe extern "C" fn ssl_nonblock_write(mut vpninfo:
                                                *mut openconnect_info,
                                            mut buf: *mut libc::c_void,
                                            mut buflen: libc::c_int)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = SSL_write((*vpninfo).https_ssl, buf, buflen);
    if ret > 0i32 { return ret }
    ret = SSL_get_error((*vpninfo).https_ssl, ret);
    match ret {
        3 => {
            /* Waiting for the socket to become writable -- it's
		   probably stalled, and/or the buffers are full */
            let mut __fd: libc::c_int = (*vpninfo).ssl_fd;
            (*vpninfo)._select_wfds.fds_bits[(__fd as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8i32
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] |=
                ((1i32 as libc::c_ulong) <<
                     (__fd as
                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8i32
                                                                                           as
                                                                                           libc::c_ulong)))
                    as __int32_t
        }
        2 => { }
        _ => {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"SSL_write failed: %d\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        ret);
            }
            ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                         unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _: size_t,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> libc::c_int),
                                vpninfo as *mut libc::c_void);
            return -1i32
        }
    }
    return 0i32;
}
#[src_loc = "338:1"]
unsafe extern "C" fn ui_open(mut ui: *mut UI) -> libc::c_int {
    let mut vpninfo: *mut openconnect_info =
        UI_get0_user_data(ui) as *mut openconnect_info;
    let mut ui_data: *mut ui_data = 0 as *mut ui_data;
    if vpninfo.is_null() || (*vpninfo).process_auth_form.is_none() {
        return 0i32
    }
    ui_data =
        malloc(::std::mem::size_of::<ui_data>() as libc::c_ulong) as
            *mut ui_data;
    if ui_data.is_null() { return 0i32 }
    memset(ui_data as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ui_data>() as libc::c_ulong);
    (*ui_data).last_opt = &mut (*ui_data).form.opts;
    (*ui_data).vpninfo = vpninfo;
    (*ui_data).form.auth_id =
        b"openssl_ui\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    UI_add_user_data(ui, ui_data as *mut libc::c_void);
    return 1i32;
}
#[src_loc = "360:1"]
unsafe extern "C" fn ui_write(mut ui: *mut UI, mut uis: *mut UI_STRING)
 -> libc::c_int {
    let mut ui_data: *mut ui_data = UI_get0_user_data(ui) as *mut ui_data;
    let mut opt: *mut ui_form_opt = 0 as *mut ui_form_opt;
    match UI_get_string_type(uis) as libc::c_uint {
        5 => {
            (*ui_data).form.error =
                UI_get0_output_string(uis) as *mut libc::c_char
        }
        4 => {
            (*ui_data).form.message =
                UI_get0_output_string(uis) as *mut libc::c_char
        }
        1 => {
            opt =
                malloc(::std::mem::size_of::<ui_form_opt>() as libc::c_ulong)
                    as *mut ui_form_opt;
            if opt.is_null() { return 1i32 }
            memset(opt as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<ui_form_opt>() as libc::c_ulong);
            (*opt).uis = uis;
            (*opt).opt.name = UI_get0_output_string(uis) as *mut libc::c_char;
            (*opt).opt.label = (*opt).opt.name;
            if UI_get_input_flags(uis) & 0x1i32 != 0 {
                (*opt).opt.type_0 = 1i32
            } else { (*opt).opt.type_0 = 2i32 }
            *(*ui_data).last_opt = &mut (*opt).opt;
            (*ui_data).last_opt = &mut (*opt).opt.next
        }
        _ => {
            if (*(*ui_data).vpninfo).verbose >= 0i32 {
                (*(*ui_data).vpninfo).progress.expect("non-null function pointer")((*(*ui_data).vpninfo).cbdata,
                                                                                   0i32,
                                                                                   libintl_dgettext(b"openconnect\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char,
                                                                                                    b"Unhandled SSL UI request type %d\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char),
                                                                                   UI_get_string_type(uis)
                                                                                       as
                                                                                       libc::c_uint);
            }
            return 0i32
        }
    }
    return 1i32;
}
#[src_loc = "396:1"]
unsafe extern "C" fn ui_flush(mut ui: *mut UI) -> libc::c_int {
    let mut ui_data: *mut ui_data = UI_get0_user_data(ui) as *mut ui_data;
    let mut vpninfo: *mut openconnect_info = (*ui_data).vpninfo;
    let mut opt: *mut ui_form_opt = 0 as *mut ui_form_opt;
    let mut ret: libc::c_int = 0;
    ret = process_auth_form(vpninfo, &mut (*ui_data).form);
    if ret != 0 { return 0i32 }
    opt = (*ui_data).form.opts as *mut ui_form_opt;
    while !opt.is_null() {
        if !(*opt).opt._value.is_null() && !(*opt).uis.is_null() {
            UI_set_result(ui, (*opt).uis, (*opt).opt._value);
        }
        opt = (*opt).opt.next as *mut ui_form_opt
    }
    return 1i32;
}
#[src_loc = "415:1"]
unsafe extern "C" fn ui_close(mut ui: *mut UI) -> libc::c_int {
    let mut ui_data: *mut ui_data = UI_get0_user_data(ui) as *mut ui_data;
    let mut opt: *mut ui_form_opt = 0 as *mut ui_form_opt;
    let mut next_opt: *mut ui_form_opt = 0 as *mut ui_form_opt;
    opt = (*ui_data).form.opts as *mut ui_form_opt;
    while !opt.is_null() {
        next_opt = (*opt).opt.next as *mut ui_form_opt;
        if !(*opt).opt._value.is_null() {
            free((*opt).opt._value as *mut libc::c_void);
        }
        free(opt as *mut libc::c_void);
        opt = next_opt
    }
    free(ui_data as *mut libc::c_void);
    UI_add_user_data(ui, 0 as *mut libc::c_void);
    return 1i32;
}
#[src_loc = "434:1"]
unsafe extern "C" fn create_openssl_ui() -> *mut UI_METHOD {
    let mut ui_method: *mut UI_METHOD =
        UI_create_method(b"AnyConnect VPN UI\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char);
    /* Set up a UI method of our own for password/passphrase requests */
    UI_method_set_opener(ui_method,
                         Some(ui_open as
                                  unsafe extern "C" fn(_: *mut UI)
                                      -> libc::c_int));
    UI_method_set_writer(ui_method,
                         Some(ui_write as
                                  unsafe extern "C" fn(_: *mut UI,
                                                       _: *mut UI_STRING)
                                      -> libc::c_int));
    UI_method_set_flusher(ui_method,
                          Some(ui_flush as
                                   unsafe extern "C" fn(_: *mut UI)
                                       -> libc::c_int));
    UI_method_set_closer(ui_method,
                         Some(ui_close as
                                  unsafe extern "C" fn(_: *mut UI)
                                      -> libc::c_int));
    return ui_method;
}
#[src_loc = "448:1"]
unsafe extern "C" fn pem_pw_cb(mut buf: *mut libc::c_char,
                               mut len: libc::c_int, mut w: libc::c_int,
                               mut v: *mut libc::c_void) -> libc::c_int {
    let mut vpninfo: *mut openconnect_info = v as *mut openconnect_info;
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plen: libc::c_int = 0;
    if !(*vpninfo).cert_password.is_null() {
        pass = (*vpninfo).cert_password;
        (*vpninfo).cert_password = 0 as *mut libc::c_char
    } else if request_passphrase(vpninfo,
                                 b"openconnect_pem\x00" as *const u8 as
                                     *const libc::c_char,
                                 &mut pass as *mut *mut libc::c_char,
                                 libintl_dgettext(b"openconnect\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  b"Enter PEM pass phrase:\x00"
                                                      as *const u8 as
                                                      *const libc::c_char)) !=
                  0 {
        return -1i32
    }
    plen = strlen(pass) as libc::c_int;
    if len <= plen {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"PEM password too long (%d >= %d)\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    plen,
                                                                    len);
        }
        free_pass(&mut pass);
        return -1i32
    }
    memcpy(buf as *mut libc::c_void, pass as *const libc::c_void,
           (plen + 1i32) as libc::c_ulong);
    free_pass(&mut pass);
    return plen;
}
#[src_loc = "476:1"]
unsafe extern "C" fn install_extra_certs(mut vpninfo: *mut openconnect_info,
                                         mut source: *const libc::c_char,
                                         mut ca: *mut stack_st_X509)
 -> libc::c_int {
    let mut cert: *mut X509 = (*vpninfo).cert_x509;
    let mut i: libc::c_int = 0;
    if cert.is_null() { return -22i32 }
    's_16:
        loop  {
            i = 0i32;
            loop  {
                if !(i <
                         sk_num((if 1i32 != 0 {
                                     ca
                                 } else { 0 as *mut stack_st_X509 }) as
                                    *mut _STACK)) {
                    break 's_16 ;
                }
                let mut cert2: *mut X509 =
                    sk_value(if 1i32 != 0 {
                                 ca
                             } else { 0 as *mut stack_st_X509 } as
                                 *mut _STACK, i) as *mut X509;
                if X509_check_issued(cert2, cert) == 0i32 {
                    let mut buf: [libc::c_char; 200] = [0; 200];
                    if cert2 == cert { break 's_16 ; }
                    if X509_check_issued(cert2, cert2) == 0i32 {
                        break 's_16 ;
                    }
                    X509_NAME_oneline(X509_get_subject_name(cert2),
                                      buf.as_mut_ptr(),
                                      ::std::mem::size_of::<[libc::c_char; 200]>()
                                          as libc::c_ulong as libc::c_int);
                    if (*vpninfo).verbose >= 2i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                2i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Extra cert from %s: \'%s\'\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                source,
                                                                                buf.as_mut_ptr());
                    }
                    CRYPTO_add_lock(&mut (*cert2).references, 1i32, 3i32,
                                    b"openssl.c\x00" as *const u8 as
                                        *const libc::c_char, 499i32);
                    SSL_CTX_ctrl((*vpninfo).https_ctx, 14i32,
                                 0i32 as libc::c_long,
                                 cert2 as *mut libc::c_char as
                                     *mut libc::c_void);
                    cert = cert2;
                    break ;
                } else { i += 1 }
            }
        }
    sk_pop_free(if 1i32 != 0 { ca } else { 0 as *mut stack_st_X509 } as
                    *mut _STACK,
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut X509)
                                                   -> ()>,
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_void)
                                                   ->
                                                       ()>>(if 1i32 != 0 {
                                                                Some(X509_free
                                                                         as
                                                                         unsafe extern "C" fn(_:
                                                                                                  *mut X509)
                                                                             ->
                                                                                 ())
                                                            } else { None }));
    return 0i32;
}
#[src_loc = "510:1"]
unsafe extern "C" fn load_pkcs12_certificate(mut vpninfo:
                                                 *mut openconnect_info,
                                             mut p12: *mut PKCS12)
 -> libc::c_int {
    let mut pkey: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut cert: *mut X509 = 0 as *mut X509;
    let mut ca: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    let mut ret: libc::c_int = 0i32;
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    pass = (*vpninfo).cert_password;
    (*vpninfo).cert_password = 0 as *mut libc::c_char;
    loop  {
        /* We do this every time round the loop, to work around a bug in
	   OpenSSL < 1.0.0-beta2 -- where the stack at *ca will be freed
	   when PKCS12_parse() returns an error, but *ca is left pointing
	   to the freed memory. */
        ca = 0 as *mut stack_st_X509;
        if PKCS12_parse(p12, pass, &mut pkey, &mut cert, &mut ca) == 0 {
            let mut err: libc::c_ulong = ERR_peek_error();
            if (err >> 24i64 & 0xffi64 as libc::c_ulong) as libc::c_int ==
                   35i32 &&
                   (err >> 12i64 & 0xfffi64 as libc::c_ulong) as libc::c_int
                       == 118i32 &&
                   (err & 0xfffi64 as libc::c_ulong) as libc::c_int == 113i32
               {
                if !pass.is_null() {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Failed to decrypt PKCS#12 certificate file\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                }
                free_pass(&mut pass);
                if request_passphrase(vpninfo,
                                      b"openconnect_pkcs12\x00" as *const u8
                                          as *const libc::c_char,
                                      &mut pass as *mut *mut libc::c_char,
                                      libintl_dgettext(b"openconnect\x00" as
                                                           *const u8 as
                                                           *const libc::c_char,
                                                       b"Enter PKCS#12 pass phrase:\x00"
                                                           as *const u8 as
                                                           *const libc::c_char))
                       < 0i32 {
                    PKCS12_free(p12);
                    return -22i32
                }
            } else {
                ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _: size_t,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> libc::c_int),
                                    vpninfo as *mut libc::c_void);
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Parse PKCS#12 failed (see above errors)\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                PKCS12_free(p12);
                free_pass(&mut pass);
                return -22i32
            }
        } else {
            free_pass(&mut pass);
            if !cert.is_null() {
                let mut buf: [libc::c_char; 200] = [0; 200];
                (*vpninfo).cert_x509 = cert;
                SSL_CTX_use_certificate((*vpninfo).https_ctx, cert);
                X509_NAME_oneline(X509_get_subject_name(cert),
                                  buf.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 200]>()
                                      as libc::c_ulong as libc::c_int);
                if (*vpninfo).verbose >= 1i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            1i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Using client certificate \'%s\'\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
            } else {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"PKCS#12 contained no certificate!\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ret = -22i32
            }
            if !pkey.is_null() {
                SSL_CTX_use_PrivateKey((*vpninfo).https_ctx, pkey);
                EVP_PKEY_free(pkey);
            } else {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"PKCS#12 contained no private key!\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ret = -22i32
            }
            if !ca.is_null() {
                install_extra_certs(vpninfo,
                                    libintl_dgettext(b"openconnect\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"PKCS#12\x00" as
                                                         *const u8 as
                                                         *const libc::c_char),
                                    ca);
            }
            PKCS12_free(p12);
            return ret
        }
    };
}
#[src_loc = "584:1"]
unsafe extern "C" fn load_tpm_certificate(mut vpninfo: *mut openconnect_info,
                                          mut engine: *const libc::c_char)
 -> libc::c_int {
    let mut e: *mut ENGINE = 0 as *mut ENGINE;
    let mut key: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut meth: *mut UI_METHOD = 0 as *mut UI_METHOD;
    let mut ret: libc::c_int = 0i32;
    ENGINE_load_builtin_engines();
    e = ENGINE_by_id(engine);
    if e.is_null() &&
           strcmp(engine, b"tpm2\x00" as *const u8 as *const libc::c_char) ==
               0 {
        ERR_clear_error();
        e = ENGINE_by_id(b"tpm2tss\x00" as *const u8 as *const libc::c_char)
    }
    if e.is_null() {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Can\'t load TPM engine.\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                     unsafe extern "C" fn(_:
                                                              *const libc::c_char,
                                                          _: size_t,
                                                          _:
                                                              *mut libc::c_void)
                                         -> libc::c_int),
                            vpninfo as *mut libc::c_void);
        return -22i32
    }
    if ENGINE_init(e) == 0 || ENGINE_set_default_RSA(e) == 0 ||
           ENGINE_set_default_RAND(e) == 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Failed to init TPM engine\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                     unsafe extern "C" fn(_:
                                                              *const libc::c_char,
                                                          _: size_t,
                                                          _:
                                                              *mut libc::c_void)
                                         -> libc::c_int),
                            vpninfo as *mut libc::c_void);
        ENGINE_free(e);
        return -22i32
    }
    if !(*vpninfo).cert_password.is_null() {
        if ENGINE_ctrl_cmd(e, b"PIN\x00" as *const u8 as *const libc::c_char,
                           strlen((*vpninfo).cert_password) as libc::c_long,
                           (*vpninfo).cert_password as *mut libc::c_void,
                           None, 0i32) == 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Failed to set TPM SRK password\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                         unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _: size_t,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> libc::c_int),
                                vpninfo as *mut libc::c_void);
        }
        free_pass(&mut (*vpninfo).cert_password);
    }
    /* Provide our own UI method to handle the PIN callback. */
    meth = create_openssl_ui();
    key =
        ENGINE_load_private_key(e, (*vpninfo).sslkey, meth,
                                vpninfo as *mut libc::c_void);
    if !meth.is_null() { UI_destroy_method(meth); }
    if key.is_null() {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Failed to load TPM private key\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                     unsafe extern "C" fn(_:
                                                              *const libc::c_char,
                                                          _: size_t,
                                                          _:
                                                              *mut libc::c_void)
                                         -> libc::c_int),
                            vpninfo as *mut libc::c_void);
        ret = -22i32
    } else {
        if SSL_CTX_use_PrivateKey((*vpninfo).https_ctx, key) == 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Add key from TPM failed\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                         unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _: size_t,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> libc::c_int),
                                vpninfo as *mut libc::c_void);
            ret = -22i32
        }
        EVP_PKEY_free(key);
    }
    ENGINE_finish(e);
    ENGINE_free(e);
    return ret;
}
/* This is a reimplementation of SSL_CTX_use_certificate_chain_file().
 * We do this for three reasons:
 *
 * - Firstly, we have no way to obtain the primary X509 certificate
 *   after SSL_CTX_use_certificate_chain_file() has loaded it, and we
 *   need to inspect it to check for expiry and report its name etc.
 *   So in the past we've opened the cert file again and read the cert
 *   again in a reload_pem_cert() function which was a partial
 *   reimplementation anyway.
 *
 * - Secondly, on Windows, OpenSSL only partially handles UTF-8 filenames.
 *   Specifically, BIO_new_file() will convert UTF-8 to UTF-16 and attempt
 *   to use _wfopen() to open the file, but BIO_read_filename() will not.
 *   It is BIO_read_filename() which the SSL_CTX_*_file functions use, and
 *   thus they don't work with UTF-8 file names. This is filed as RT#3479:
 *   http://rt.openssl.org/Ticket/Display.html?id=3479
 *
 * - Finally, and least importantly, it does actually matter which supporting
 *   certs we offer on the wire because of RT#1942. Doing this for ourselves
 *   allows us to explicitly print the supporting certs that we're using,
 *   which may assist in diagnosing problems.
 */
#[src_loc = "678:1"]
unsafe extern "C" fn load_cert_chain_file(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut b: *mut BIO = 0 as *mut BIO;
    let mut f: *mut FILE =
        openconnect_fopen_utf8(vpninfo, (*vpninfo).cert,
                               b"rb\x00" as *const u8 as *const libc::c_char);
    let mut extra_certs: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    let mut buf: [libc::c_char; 200] = [0; 200];
    if f.is_null() {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Failed to open certificate file %s: %s\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*vpninfo).cert,
                                                                    strerror(*__error()));
        }
        return -2i32
    }
    b = BIO_new_fp(f, 1i32);
    if b.is_null() {
        fclose(f);
    } else {
        (*vpninfo).cert_x509 =
            PEM_read_bio_X509_AUX(b, 0 as *mut *mut X509, None,
                                  0 as *mut libc::c_void);
        if (*vpninfo).cert_x509.is_null() {
            BIO_free(b);
        } else {
            X509_NAME_oneline(X509_get_subject_name((*vpninfo).cert_x509),
                              buf.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 200]>() as
                                  libc::c_ulong as libc::c_int);
            if (*vpninfo).verbose >= 1i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        1i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Using client certificate \'%s\'\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        buf.as_mut_ptr());
            }
            if SSL_CTX_use_certificate((*vpninfo).https_ctx,
                                       (*vpninfo).cert_x509) == 0 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to install certificate in OpenSSL context\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _: size_t,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> libc::c_int),
                                    vpninfo as *mut libc::c_void);
                BIO_free(b);
                return -5i32
            }
            loop  {
                let mut x: *mut X509 =
                    PEM_read_bio_X509(b, 0 as *mut *mut X509, None,
                                      0 as *mut libc::c_void);
                if x.is_null() {
                    let mut err: libc::c_ulong = ERR_peek_last_error();
                    if (err >> 24i64 & 0xffi64 as libc::c_ulong) as
                           libc::c_int == 9i32 &&
                           (err & 0xfffi64 as libc::c_ulong) as libc::c_int ==
                               108i32 {
                        ERR_clear_error();
                        break ;
                    }
                } else {
                    if extra_certs.is_null() {
                        extra_certs = sk_new_null() as *mut stack_st_X509
                    }
                    if !extra_certs.is_null() {
                        if !(sk_push(if 1i32 != 0 {
                                         extra_certs
                                     } else { 0 as *mut stack_st_X509 } as
                                         *mut _STACK,
                                     if 1i32 != 0 {
                                         x
                                     } else { 0 as *mut X509 } as
                                         *mut libc::c_void) == 0) {
                            continue ;
                        }
                    }
                }
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to process all supporting certs. Trying anyway...\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _: size_t,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> libc::c_int),
                                    vpninfo as *mut libc::c_void);
                X509_free(x);
                break ;
            }
            /* It might work without... */
            BIO_free(b);
            if !extra_certs.is_null() {
                install_extra_certs(vpninfo,
                                    libintl_dgettext(b"openconnect\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"PEM file\x00" as
                                                         *const u8 as
                                                         *const libc::c_char),
                                    extra_certs);
            }
            return 0i32
        }
    }
    if (*vpninfo).verbose >= 0i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                0i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Loading certificate failed\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
    }
    ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                 unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: size_t,
                                                      _: *mut libc::c_void)
                                     -> libc::c_int),
                        vpninfo as *mut libc::c_void);
    return -5i32;
}
#[src_loc = "788:1"]
unsafe extern "C" fn is_pem_password_error(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut err: libc::c_ulong = ERR_peek_error();
    ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                 unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: size_t,
                                                      _: *mut libc::c_void)
                                     -> libc::c_int),
                        vpninfo as *mut libc::c_void);
    /* If the user fat-fingered the passphrase, try again */
    if (err >> 24i64 & 0xffi64 as libc::c_ulong) as libc::c_int == 6i32 &&
           (err >> 12i64 & 0xfffi64 as libc::c_ulong) as libc::c_int == 101i32
           && (err & 0xfffi64 as libc::c_ulong) as libc::c_int == 100i32 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Loading private key failed (wrong passphrase?)\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        ERR_clear_error();
        return 1i32
    }
    if (*vpninfo).verbose >= 0i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                0i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Loading private key failed (see above errors)\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
    }
    return 0i32;
}
#[src_loc = "812:1"]
unsafe extern "C" fn load_certificate(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut key: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut ret: libc::c_int = 0;
    if strncmp((*vpninfo).cert,
               b"pkcs11:\x00" as *const u8 as *const libc::c_char,
               7i32 as libc::c_ulong) == 0 {
        let mut ret_0: libc::c_int = load_pkcs11_certificate(vpninfo);
        if ret_0 != 0 { return ret_0 }
    } else {
        if (*vpninfo).verbose >= 2i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    2i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Using certificate file %s\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*vpninfo).cert);
        }
        if strncmp((*vpninfo).cert,
                   b"keystore:\x00" as *const u8 as *const libc::c_char,
                   9i32 as libc::c_ulong) != 0 {
            let mut p12: *mut PKCS12 = 0 as *mut PKCS12;
            f =
                openconnect_fopen_utf8(vpninfo, (*vpninfo).cert,
                                       b"rb\x00" as *const u8 as
                                           *const libc::c_char);
            if f.is_null() {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to open certificate file %s: %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            (*vpninfo).cert,
                                                                            strerror(*__error()));
                }
                return -2i32
            }
            p12 = d2i_PKCS12_fp(f, 0 as *mut *mut PKCS12);
            fclose(f);
            if !p12.is_null() { return load_pkcs12_certificate(vpninfo, p12) }
            /* Not PKCS#12. Clear error and fall through to see if it's a PEM file... */
            ERR_clear_error();
        }
        /* It's PEM or TPM now, and either way we need to load the plain cert: */
        /* ANDROID_KEYSTORE */
        let mut ret_1: libc::c_int = load_cert_chain_file(vpninfo);
        if ret_1 != 0 { return ret_1 }
    }
    /* ANDROID_KEYSTORE */
    if strncmp((*vpninfo).sslkey,
               b"pkcs11:\x00" as *const u8 as *const libc::c_char,
               7i32 as libc::c_ulong) == 0 {
        return load_pkcs11_key(vpninfo)
    }
    f =
        openconnect_fopen_utf8(vpninfo, (*vpninfo).sslkey,
                               b"rb\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Failed to open private key file %s: %s\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*vpninfo).sslkey,
                                                                    strerror(*__error()));
        }
        return -2i32
    }
    buf[255] = 0i32 as libc::c_char;
    while !fgets(buf.as_mut_ptr(), 255i32, f).is_null() {
        if strcmp(buf.as_mut_ptr(),
                  b"-----BEGIN TSS KEY BLOB-----\n\x00" as *const u8 as
                      *const libc::c_char) == 0 {
            fclose(f);
            return load_tpm_certificate(vpninfo,
                                        b"tpm\x00" as *const u8 as
                                            *const libc::c_char)
        } else {
            if strcmp(buf.as_mut_ptr(),
                      b"-----BEGIN TSS2 KEY BLOB-----\n\x00" as *const u8 as
                          *const libc::c_char) == 0 ||
                   strcmp(buf.as_mut_ptr(),
                          b"-----BEGIN TSS2 PRIVATE KEY-----\n\x00" as
                              *const u8 as *const libc::c_char) == 0 {
                fclose(f);
                return load_tpm_certificate(vpninfo,
                                            b"tpm2\x00" as *const u8 as
                                                *const libc::c_char)
            } else {
                if strcmp(buf.as_mut_ptr(),
                          b"-----BEGIN RSA PRIVATE KEY-----\n\x00" as
                              *const u8 as *const libc::c_char) == 0 ||
                       strcmp(buf.as_mut_ptr(),
                              b"-----BEGIN DSA PRIVATE KEY-----\n\x00" as
                                  *const u8 as *const libc::c_char) == 0 ||
                       strcmp(buf.as_mut_ptr(),
                              b"-----BEGIN EC PRIVATE KEY-----\n\x00" as
                                  *const u8 as *const libc::c_char) == 0 ||
                       strcmp(buf.as_mut_ptr(),
                              b"-----BEGIN ENCRYPTED PRIVATE KEY-----\n\x00"
                                  as *const u8 as *const libc::c_char) == 0 ||
                       strcmp(buf.as_mut_ptr(),
                              b"-----BEGIN PRIVATE KEY-----\n\x00" as
                                  *const u8 as *const libc::c_char) == 0 {
                    let mut b: *mut BIO = BIO_new_fp(f, 0x1i32);
                    if b.is_null() {
                        fclose(f);
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Loading private key failed\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                        ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                                     unsafe extern "C" fn(_:
                                                                              *const libc::c_char,
                                                                          _:
                                                                              size_t,
                                                                          _:
                                                                              *mut libc::c_void)
                                                         -> libc::c_int),
                                            vpninfo as *mut libc::c_void);
                    }
                    loop  {
                        fseek(f, 0i32 as libc::c_long, 0i32);
                        key =
                            PEM_read_bio_PrivateKey(b,
                                                    0 as *mut *mut EVP_PKEY,
                                                    Some(pem_pw_cb as
                                                             unsafe extern "C" fn(_:
                                                                                      *mut libc::c_char,
                                                                                  _:
                                                                                      libc::c_int,
                                                                                  _:
                                                                                      libc::c_int,
                                                                                  _:
                                                                                      *mut libc::c_void)
                                                                 ->
                                                                     libc::c_int),
                                                    vpninfo as
                                                        *mut libc::c_void);
                        if key.is_null() {
                            if is_pem_password_error(vpninfo) != 0 {
                                continue ;
                            }
                            BIO_free(b);
                            return -22i32
                        } else {
                            ret = 0i32;
                            if SSL_CTX_use_PrivateKey((*vpninfo).https_ctx,
                                                      key) == 0 {
                                if (*vpninfo).verbose >= 0i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            0i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"Loading private key failed\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char));
                                }
                                ERR_print_errors_cb(Some(openconnect_print_err_cb
                                                             as
                                                             unsafe extern "C" fn(_:
                                                                                      *const libc::c_char,
                                                                                  _:
                                                                                      size_t,
                                                                                  _:
                                                                                      *mut libc::c_void)
                                                                 ->
                                                                     libc::c_int),
                                                    vpninfo as
                                                        *mut libc::c_void);
                                ret = -22i32
                            }
                            EVP_PKEY_free(key);
                            BIO_free(b);
                            return ret
                        }
                    }
                }
            }
        }
    }
    /* Not PEM? Try DER... */
    fseek(f, 0i32 as libc::c_long, 0i32);
    /* This will catch PKCS#1 and unencrypted PKCS#8
	 * (except in OpenSSL 0.9.8 where it doesn't handle
	 * the latter but nobody cares about 0.9.8 any more. */
    key = d2i_PrivateKey_fp(f, 0 as *mut *mut EVP_PKEY);
    if !key.is_null() {
        ret = 0i32;
        if SSL_CTX_use_PrivateKey((*vpninfo).https_ctx, key) == 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Loading private key failed\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                         unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _: size_t,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> libc::c_int),
                                vpninfo as *mut libc::c_void);
            ret = -22i32
        }
        EVP_PKEY_free(key);
        fclose(f);
        return ret
    } else {
        /* Encrypted PKCS#8 DER */
        let mut p8: *mut X509_SIG = 0 as *mut X509_SIG;
        fseek(f, 0i32 as libc::c_long, 0i32);
        p8 = d2i_PKCS8_fp(f, 0 as *mut *mut X509_SIG);
        if !p8.is_null() {
            let mut p8inf: *mut PKCS8_PRIV_KEY_INFO =
                0 as *mut PKCS8_PRIV_KEY_INFO;
            let mut pass: *mut libc::c_char = (*vpninfo).cert_password;
            fclose(f);
            loop  {
                p8inf =
                    PKCS8_decrypt(p8,
                                  if !pass.is_null() {
                                      pass
                                  } else {
                                      b"\x00" as *const u8 as
                                          *const libc::c_char
                                  },
                                  if !pass.is_null() {
                                      strlen(pass)
                                  } else { 0i32 as libc::c_ulong } as
                                      libc::c_int);
                if !p8inf.is_null() { break ; }
                let mut err: libc::c_ulong = ERR_peek_error();
                if (err >> 24i64 & 0xffi64 as libc::c_ulong) as libc::c_int ==
                       6i32 &&
                       (err >> 12i64 & 0xfffi64 as libc::c_ulong) as
                           libc::c_int == 101i32 &&
                       (err & 0xfffi64 as libc::c_ulong) as libc::c_int ==
                           100i32 {
                    ERR_clear_error();
                    if !pass.is_null() {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Failed to decrypt PKCS#8 certificate file\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                        free_pass(&mut pass);
                        pass = 0 as *mut libc::c_char
                    }
                    if request_passphrase(vpninfo,
                                          b"openconnect_pkcs8\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          &mut pass as *mut *mut libc::c_char,
                                          libintl_dgettext(b"openconnect\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           b"Enter PKCS#8 pass phrase:\x00"
                                                               as *const u8 as
                                                               *const libc::c_char))
                           >= 0i32 {
                        continue ;
                    }
                } else {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Failed to decrypt PKCS#8 certificate file\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                    ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                                 unsafe extern "C" fn(_:
                                                                          *const libc::c_char,
                                                                      _:
                                                                          size_t,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> libc::c_int),
                                        vpninfo as *mut libc::c_void);
                }
                free_pass(&mut pass);
                (*vpninfo).cert_password = 0 as *mut libc::c_char;
                X509_SIG_free(p8);
                return -22i32
            }
            free_pass(&mut pass);
            (*vpninfo).cert_password = 0 as *mut libc::c_char;
            key = EVP_PKCS82PKEY(p8inf);
            PKCS8_PRIV_KEY_INFO_free(p8inf);
            X509_SIG_free(p8);
            if key.is_null() {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to convert PKCS#8 to OpenSSL EVP_PKEY\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                return -5i32
            }
            ret = 0i32;
            if SSL_CTX_use_PrivateKey((*vpninfo).https_ctx, key) == 0 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Loading private key failed\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _: size_t,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> libc::c_int),
                                    vpninfo as *mut libc::c_void);
                ret = -22i32
            }
            EVP_PKEY_free(key);
            return ret
        }
    }
    fclose(f);
    if (*vpninfo).verbose >= 0i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                0i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Failed to identify private key type in \'%s\'\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                (*vpninfo).sslkey);
    }
    return -22i32;
}
#[src_loc = "1054:1"]
unsafe extern "C" fn get_cert_fingerprint(mut vpninfo: *mut openconnect_info,
                                          mut cert: *mut X509,
                                          mut type_0: *const EVP_MD,
                                          mut buf: *mut libc::c_char)
 -> libc::c_int {
    let mut md: [libc::c_uchar; 64] = [0; 64];
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    if X509_digest(cert, type_0, md.as_mut_ptr(), &mut n) == 0 {
        return -12i32
    }
    i = 0i32 as libc::c_uint;
    while i < n {
        sprintf(&mut *buf.offset(i.wrapping_mul(2i32 as libc::c_uint) as
                                     isize) as *mut libc::c_char,
                b"%02X\x00" as *const u8 as *const libc::c_char,
                md[i as usize] as libc::c_int);
        i = i.wrapping_add(1)
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "1070:1"]
pub unsafe extern "C" fn get_cert_md5_fingerprint(mut vpninfo:
                                                      *mut openconnect_info,
                                                  mut cert: *mut libc::c_void,
                                                  mut buf: *mut libc::c_char)
 -> libc::c_int {
    return get_cert_fingerprint(vpninfo, cert as *mut X509, EVP_md5(), buf);
}
#[src_loc = "1076:1"]
unsafe extern "C" fn set_peer_cert_hash(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut pkey: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut bp: *mut BIO = BIO_new(BIO_s_mem());
    let mut keyinfo: *mut BUF_MEM = 0 as *mut BUF_MEM;
    /* We can't use X509_pubkey_digest() because it only hashes the
	   subjectPublicKey BIT STRING, and not the whole of the
	   SubjectPublicKeyInfo SEQUENCE. */
    pkey = X509_get_pubkey((*vpninfo).peer_cert as *mut X509);
    if i2d_PUBKEY_bio(bp, pkey) == 0 {
        EVP_PKEY_free(pkey);
        BIO_free(bp);
        return -12i32
    }
    EVP_PKEY_free(pkey);
    BIO_ctrl(bp, 115i32, 0i32 as libc::c_long,
             &mut keyinfo as *mut *mut BUF_MEM as *mut libc::c_char as
                 *mut libc::c_void);
    openconnect_sha256((*vpninfo).peer_cert_sha256_raw.as_mut_ptr(),
                       (*keyinfo).data as *mut libc::c_void,
                       (*keyinfo).length as libc::c_int);
    openconnect_sha1((*vpninfo).peer_cert_sha1_raw.as_mut_ptr(),
                     (*keyinfo).data as *mut libc::c_void,
                     (*keyinfo).length as libc::c_int);
    BIO_free(bp);
    return 0i32;
}
#[src_loc = "1363:1"]
unsafe extern "C" fn match_cert_hostname(mut vpninfo: *mut openconnect_info,
                                         mut peer_cert: *mut X509,
                                         mut ipaddr: *const libc::c_uchar,
                                         mut ipaddrlen: libc::c_int)
 -> libc::c_int {
    let mut matched: *mut libc::c_char = 0 as *mut libc::c_char;
    if ipaddrlen != 0 &&
           X509_check_ip(peer_cert, ipaddr, ipaddrlen as size_t,
                         0i32 as libc::c_uint) != 0 {
        if (*vpninfo).verbose >= 2i32 {
            let mut host: [libc::c_char; 80] = [0; 80];
            let mut family: libc::c_int = 0;
            if ipaddrlen == 4i32 { family = 2i32 } else { family = 30i32 }
            /* In Windows, the 'src' argument of inet_ntop() isn't const */
            inet_ntop(family, ipaddr as *mut libc::c_void, host.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 80]>() as
                          libc::c_ulong as socklen_t);
            if (*vpninfo).verbose >= 2i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        2i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Matched %s address \'%s\'\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        if family
                                                                               ==
                                                                               30i32
                                                                           {
                                                                            b"IPv6\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                        } else {
                                                                            b"IPv4\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                        },
                                                                        host.as_mut_ptr());
            }
        }
        return 0i32
    }
    if X509_check_host(peer_cert, (*vpninfo).hostname, 0i32 as size_t,
                       0i32 as libc::c_uint, &mut matched) != 0 {
        if (*vpninfo).verbose >= 2i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    2i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Matched peer certificate subject name \'%s\'\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    matched);
        }
        CRYPTO_free(matched as *mut libc::c_void);
        return 0i32
    }
    /* We do it like this because these two strings are already
	 * translated in gnutls.c */
    if (*vpninfo).verbose >= 1i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                1i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Server certificate verify failed: %s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"certificate does not match hostname\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
    }
    return -22i32;
}
/* OpenSSL < 1.0.2 */
/* Before OpenSSL 1.1 we could do this directly. And needed to. */
#[src_loc = "1411:1"]
unsafe extern "C" fn workaround_openssl_certchain_bug(mut vpninfo:
                                                          *mut openconnect_info,
                                                      mut ssl: *mut SSL) {
    /* OpenSSL has problems with certificate chains -- if there are
	   multiple certs with the same name, it doesn't necessarily
	   choose the _right_ one. (RT#1942)
	   Pick the right ones for ourselves and add them manually. */
    let mut cert: *mut X509 = SSL_get_certificate(ssl);
    let mut cert2: *mut X509 = 0 as *mut X509;
    let mut store: *mut X509_STORE =
        SSL_CTX_get_cert_store((*vpninfo).https_ctx);
    let mut ctx: *mut X509_STORE_CTX = 0 as *mut X509_STORE_CTX;
    let mut extra_certs: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut issuer_fn: X509_STORE_CTX_get_issuer_fn = None;
    if cert.is_null() || store.is_null() { return }
    /* If we already have 'supporting' certs, don't add them again */
    SSL_CTX_ctrl((*vpninfo).https_ctx, 82i32, 1i32 as libc::c_long,
                 &mut extra_certs as *mut *mut libc::c_void as
                     *mut libc::c_void);
    if !extra_certs.is_null() { return }
    ctx = X509_STORE_CTX_new();
    if ctx.is_null() { return }
    if !(X509_STORE_CTX_init(ctx, store, 0 as *mut X509,
                             0 as *mut stack_st_X509) != 0) {
        issuer_fn = (*ctx).get_issuer;
        while issuer_fn.expect("non-null function pointer")(&mut cert2, ctx,
                                                            cert) == 1i32 {
            let mut buf: [libc::c_char; 200] = [0; 200];
            if cert2 == cert { break ; }
            if X509_check_issued(cert2, cert2) == 0i32 { break ; }
            cert = cert2;
            X509_NAME_oneline(X509_get_subject_name(cert), buf.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 200]>() as
                                  libc::c_ulong as libc::c_int);
            if (*vpninfo).verbose >= 2i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        2i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Extra cert from cafile: \'%s\'\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        buf.as_mut_ptr());
            }
            SSL_CTX_ctrl((*vpninfo).https_ctx, 14i32, 0i32 as libc::c_long,
                         cert as *mut libc::c_char as *mut libc::c_void);
        }
    }
    X509_STORE_CTX_free(ctx);
}
#[no_mangle]
#[src_loc = "1458:1"]
pub unsafe extern "C" fn openconnect_get_peer_cert_chain(mut vpninfo:
                                                             *mut openconnect_info,
                                                         mut chainp:
                                                             *mut *mut oc_cert)
 -> libc::c_int {
    let mut chain: *mut oc_cert = 0 as *mut oc_cert;
    let mut p: *mut oc_cert = 0 as *mut oc_cert;
    let mut ctx: *mut X509_STORE_CTX =
        (*vpninfo).cert_list_handle as *mut X509_STORE_CTX;
    let mut untrusted: *mut stack_st_X509 = (*ctx).untrusted;
    let mut i: libc::c_int = 0;
    let mut cert_list_size: libc::c_int = 0;
    if ctx.is_null() { return -22i32 }
    cert_list_size =
        sk_num(if 1i32 != 0 { untrusted } else { 0 as *mut stack_st_X509 } as
                   *mut _STACK);
    if cert_list_size == 0 { return -5i32 }
    chain =
        calloc(cert_list_size as libc::c_ulong,
               ::std::mem::size_of::<oc_cert>() as libc::c_ulong) as
            *mut oc_cert;
    p = chain;
    if chain.is_null() { return -12i32 }
    i = 0i32;
    while i < cert_list_size {
        let mut cert: *mut X509 =
            sk_value(if 1i32 != 0 {
                         untrusted
                     } else { 0 as *mut stack_st_X509 } as *mut _STACK, i) as
                *mut X509;
        (*p).der_len = i2d_X509(cert, &mut (*p).der_data);
        if (*p).der_len < 0i32 {
            openconnect_free_peer_cert_chain(vpninfo, chain);
            return -12i32
        }
        i += 1;
        p = p.offset(1)
    }
    *chainp = chain;
    return cert_list_size;
}
#[no_mangle]
#[src_loc = "1491:1"]
pub unsafe extern "C" fn openconnect_free_peer_cert_chain(mut vpninfo:
                                                              *mut openconnect_info,
                                                          mut chain:
                                                              *mut oc_cert) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*vpninfo).cert_list_size {
        CRYPTO_free((*chain.offset(i as isize)).der_data as
                        *mut libc::c_void);
        i += 1
    }
    free(chain as *mut libc::c_void);
}
#[src_loc = "1501:1"]
unsafe extern "C" fn ssl_app_verify_callback(mut ctx: *mut X509_STORE_CTX,
                                             mut arg: *mut libc::c_void)
 -> libc::c_int {
    let mut vpninfo: *mut openconnect_info = arg as *mut openconnect_info;
    let mut err_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut cert: *mut X509 = (*ctx).cert;
    let mut param: *mut X509_VERIFY_PARAM = 0 as *mut X509_VERIFY_PARAM;
    if !(*vpninfo).peer_cert.is_null() {
        /* This is a *rehandshake*. Require that the server
		 * presents exactly the same certificate as the
		 * first time. */
        if X509_cmp(cert, (*vpninfo).peer_cert as *const X509) != 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Server presented different cert on rehandshake\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            return 0i32
        }
        if (*vpninfo).verbose >= 3i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    3i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Server presented identical cert on rehandshake\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        return 1i32
    }
    (*vpninfo).peer_cert = cert as *mut libc::c_void;
    CRYPTO_add_lock(&mut (*cert).references, 1i32, 3i32,
                    b"openssl.c\x00" as *const u8 as *const libc::c_char,
                    1522i32);
    set_peer_cert_hash(vpninfo);
    param = X509_STORE_CTX_get0_param(ctx);
    if !param.is_null() {
        X509_VERIFY_PARAM_set_flags(param, 0x80000i32 as libc::c_ulong);
    }
    if X509_verify_cert(ctx) == 0 {
        err_string =
            X509_verify_cert_error_string(X509_STORE_CTX_get_error(ctx) as
                                              libc::c_long)
    } else {
        let mut addrbuf: [libc::c_uchar; 16] = [0; 16];
        let mut addrlen: libc::c_int = 0i32;
        if inet_pton(2i32, (*vpninfo).hostname,
                     addrbuf.as_mut_ptr() as *mut libc::c_void) > 0i32 {
            addrlen = 4i32
        } else if inet_pton(30i32, (*vpninfo).hostname,
                            addrbuf.as_mut_ptr() as *mut libc::c_void) > 0i32
         {
            addrlen = 16i32
        } else if *(*vpninfo).hostname.offset(0) as libc::c_int == '[' as i32
                      &&
                      *(*vpninfo).hostname.offset(strlen((*vpninfo).hostname).wrapping_sub(1i32
                                                                                               as
                                                                                               libc::c_ulong)
                                                      as isize) as libc::c_int
                          == ']' as i32 {
            let mut p: *mut libc::c_char =
                &mut *(*vpninfo).hostname.offset((strlen as
                                                      unsafe extern "C" fn(_:
                                                                               *const libc::c_char)
                                                          ->
                                                              libc::c_ulong)((*vpninfo).hostname).wrapping_sub(1i32
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                     as isize) as
                    *mut libc::c_char;
            *p = 0i32 as libc::c_char;
            if inet_pton(30i32, (*vpninfo).hostname.offset(1),
                         addrbuf.as_mut_ptr() as *mut libc::c_void) > 0i32 {
                addrlen = 16i32
            }
            *p = ']' as i32 as libc::c_char
        }
        if match_cert_hostname(vpninfo, (*vpninfo).peer_cert as *mut X509,
                               addrbuf.as_mut_ptr(), addrlen) != 0 {
            err_string =
                libintl_dgettext(b"openconnect\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"certificate does not match hostname\x00" as
                                     *const u8 as *const libc::c_char)
        } else { return 1i32 }
    }
    if (*vpninfo).verbose >= 1i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                1i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Server certificate verify failed: %s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                err_string);
    }
    if (*vpninfo).validate_peer_cert.is_some() {
        let mut ret: libc::c_int = 0;
        (*vpninfo).cert_list_handle = ctx as *mut libc::c_void;
        ret =
            (*vpninfo).validate_peer_cert.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                              err_string);
        (*vpninfo).cert_list_handle = 0 as *mut libc::c_void;
        if ret == 0 { return 1i32 }
    }
    return 0i32;
}
#[src_loc = "1574:1"]
unsafe extern "C" fn check_certificate_expiry(mut vpninfo:
                                                  *mut openconnect_info)
 -> libc::c_int {
    let mut notAfter: *const ASN1_TIME = 0 as *const ASN1_TIME;
    let mut reason: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: time_t = 0;
    let mut i: libc::c_int = 0;
    if (*vpninfo).cert_x509.is_null() { return 0i32 }
    t = time(0 as *mut time_t);
    notAfter = (*(*(*(*vpninfo).cert_x509).cert_info).validity).notAfter;
    i = X509_cmp_time(notAfter, &mut t);
    if i == 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Error in client cert notAfter field\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        return -22i32
    } else {
        if i < 0i32 {
            reason =
                libintl_dgettext(b"openconnect\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Client certificate has expired at\x00" as
                                     *const u8 as *const libc::c_char)
        } else {
            t += (*vpninfo).cert_expire_warning as libc::c_long;
            i = X509_cmp_time(notAfter, &mut t);
            if i < 0i32 {
                reason =
                    libintl_dgettext(b"openconnect\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Client certificate expires soon at\x00"
                                         as *const u8 as *const libc::c_char)
            }
        }
    }
    if !reason.is_null() {
        let mut bp: *mut BIO = BIO_new(BIO_s_mem());
        let mut bm: *mut BUF_MEM = 0 as *mut BUF_MEM;
        let mut expiry: *const libc::c_char =
            libintl_dgettext(b"openconnect\x00" as *const u8 as
                                 *const libc::c_char,
                             b"<error>\x00" as *const u8 as
                                 *const libc::c_char);
        let mut zero: libc::c_char = 0i32 as libc::c_char;
        if !bp.is_null() {
            ASN1_TIME_print(bp, notAfter);
            BIO_write(bp,
                      &mut zero as *mut libc::c_char as *const libc::c_void,
                      1i32);
            BIO_ctrl(bp, 115i32, 0i32 as libc::c_long,
                     &mut bm as *mut *mut BUF_MEM as *mut libc::c_char as
                         *mut libc::c_void);
            expiry = (*bm).data
        }
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    b"%s: %s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    reason,
                                                                    expiry);
        }
        if !bp.is_null() { BIO_free(bp); }
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "1618:1"]
pub unsafe extern "C" fn openconnect_open_https(mut vpninfo:
                                                    *mut openconnect_info)
 -> libc::c_int {
    let mut https_ssl: *mut SSL = 0 as *mut SSL;
    let mut https_bio: *mut BIO = 0 as *mut BIO;
    let mut ssl_sock: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    if !(*vpninfo).https_ssl.is_null() { return 0i32 }
    if !(*vpninfo).peer_cert.is_null() {
        X509_free((*vpninfo).peer_cert as *mut X509);
        (*vpninfo).peer_cert = 0 as *mut libc::c_void
    }
    free((*vpninfo).peer_cert_hash as *mut libc::c_void);
    (*vpninfo).peer_cert_hash = 0 as *mut libc::c_char;
    (*vpninfo).cstp_cipher = 0 as *mut libc::c_char;
    ssl_sock = connect_https_socket(vpninfo);
    if ssl_sock < 0i32 { return ssl_sock }
    if (*vpninfo).https_ctx.is_null() {
        (*vpninfo).https_ctx = SSL_CTX_new(SSLv23_client_method());
        if !(*vpninfo).https_ctx.is_null() {
            SSL_CTX_ctrl((*vpninfo).https_ctx, 32i32,
                         0x1000000i64 | 0x2000000i64, 0 as *mut libc::c_void);
        }
        if (*vpninfo).https_ctx.is_null() {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Create TLSv1 CTX failed\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                         unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _: size_t,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> libc::c_int),
                                vpninfo as *mut libc::c_void);
            return -22i32
        }
        /* Try to work around the broken firewalls which reject ClientHello
		 * packets in certain size ranges. If we have SSL_OP_TLSEXT_PADDING
		 * use it, else fall back to SSL_OP_NO_TICKET which mostly worked for
		 * a long time. */
        SSL_CTX_ctrl((*vpninfo).https_ctx, 32i32, 0x10i64,
                     0 as *mut libc::c_void);
        if !(*vpninfo).cert.is_null() {
            err = load_certificate(vpninfo);
            if err == 0 &&
                   SSL_CTX_check_private_key((*vpninfo).https_ctx) == 0 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"SSL certificate and key do not match\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                err = -22i32
            }
            if err != 0 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Loading certificate failed. Aborting.\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                SSL_CTX_free((*vpninfo).https_ctx);
                (*vpninfo).https_ctx = 0 as *mut SSL_CTX;
                close(ssl_sock);
                return err
            }
            check_certificate_expiry(vpninfo);
        }
        /* We've seen certificates in the wild which don't have the
		   purpose fields filled in correctly */
        SSL_CTX_set_purpose((*vpninfo).https_ctx, 7i32);
        SSL_CTX_set_cert_verify_callback((*vpninfo).https_ctx,
                                         Some(ssl_app_verify_callback as
                                                  unsafe extern "C" fn(_:
                                                                           *mut X509_STORE_CTX,
                                                                       _:
                                                                           *mut libc::c_void)
                                                      -> libc::c_int),
                                         vpninfo as *mut libc::c_void);
        if (*vpninfo).no_system_trust == 0 {
            SSL_CTX_set_default_verify_paths((*vpninfo).https_ctx);
        }
        if (*vpninfo).pfs != 0 {
            SSL_CTX_set_cipher_list((*vpninfo).https_ctx,
                                    b"HIGH:!aNULL:!eNULL:-RSA\x00" as
                                        *const u8 as *const libc::c_char);
        }
        if !(*vpninfo).cafile.is_null() {
            /* OpenSSL does actually manage to cope with UTF-8 for
			   this one, under Windows. So only convert for legacy
			   UNIX. */
            let mut cafile: *mut libc::c_char =
                openconnect_utf8_to_legacy(vpninfo, (*vpninfo).cafile);
            err =
                SSL_CTX_load_verify_locations((*vpninfo).https_ctx, cafile,
                                              0 as *const libc::c_char);
            if cafile != (*vpninfo).cafile {
                free(cafile as *mut libc::c_void);
            }
            if err == 0 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to open CA file \'%s\'\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            (*vpninfo).cafile);
                }
                ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                             unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _: size_t,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> libc::c_int),
                                    vpninfo as *mut libc::c_void);
                SSL_CTX_free((*vpninfo).https_ctx);
                (*vpninfo).https_ctx = 0 as *mut SSL_CTX;
                close(ssl_sock);
                return -22i32
            }
        }
    }
    https_ssl = SSL_new((*vpninfo).https_ctx);
    workaround_openssl_certchain_bug(vpninfo, https_ssl);
    https_bio = BIO_new_socket(ssl_sock, 0i32);
    BIO_ctrl(https_bio, 102i32, 1i32 as libc::c_long, 0 as *mut libc::c_void);
    SSL_set_bio(https_ssl, https_bio, https_bio);
    /*
	 * If a ClientHello is between 256 and 511 bytes, the
	 * server cannot distinguish between a SSLv2 formatted
	 * packet and a SSLv3 formatted packet.
	 *
	 * F5 BIG-IP reverse proxies in particular will
	 * silently drop an ambiguous ClientHello.
	 *
	 * OpenSSL fixes this in v1.0.1g+ by padding ClientHello
	 * packets to at least 512 bytes.
	 *
	 * For older versions of OpenSSL, we try to avoid long
	 * packets by silently disabling extensions such as SNI.
	 *
	 * Discussion:
	 * http://www.ietf.org/mail-archive/web/tls/current/msg10423.html
	 *
	 * OpenSSL commits:
	 * 4fcdd66fff5fea0cfa1055c6680a76a4303f28a2
	 * cd6bd5ffda616822b52104fee0c4c7d623fd4f53
	 */
    if string_is_hostname((*vpninfo).hostname) != 0 {
        SSL_ctrl(https_ssl, 55i32, 0i32 as libc::c_long,
                 (*vpninfo).hostname as *mut libc::c_void);
    }
    SSL_set_verify(https_ssl, 0x1i32, None);
    if (*vpninfo).verbose >= 1i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                1i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"SSL negotiation with %s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                (*vpninfo).hostname);
    }
    loop  {
        err = SSL_connect(https_ssl);
        if !(err <= 0i32) { break ; }
        let mut wr_set: fd_set = fd_set{fds_bits: [0; 32],};
        let mut rd_set: fd_set = fd_set{fds_bits: [0; 32],};
        let mut maxfd: libc::c_int = ssl_sock;
        err = SSL_get_error(https_ssl, err);
        if err == 2i32 {
            let mut __fd: libc::c_int = ssl_sock;
            rd_set.fds_bits[(__fd as
                                 libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(8i32
                                                                                                  as
                                                                                                  libc::c_ulong))
                                as usize] |=
                ((1i32 as libc::c_ulong) <<
                     (__fd as
                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8i32
                                                                                           as
                                                                                           libc::c_ulong)))
                    as __int32_t
        } else if err == 3i32 {
            let mut __fd_0: libc::c_int = ssl_sock;
            wr_set.fds_bits[(__fd_0 as
                                 libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(8i32
                                                                                                  as
                                                                                                  libc::c_ulong))
                                as usize] |=
                ((1i32 as libc::c_ulong) <<
                     (__fd_0 as
                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8i32
                                                                                           as
                                                                                           libc::c_ulong)))
                    as __int32_t
        } else {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"SSL connection failure\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                         unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _: size_t,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> libc::c_int),
                                vpninfo as *mut libc::c_void);
            SSL_free(https_ssl);
            close(ssl_sock);
            return -22i32
        }
        cmd_fd_set(vpninfo, &mut rd_set, &mut maxfd);
        select(maxfd + 1i32, &mut rd_set, &mut wr_set, 0 as *mut fd_set,
               0 as *mut timeval);
        if is_cancel_pending(vpninfo, &mut rd_set) != 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"SSL connection cancelled\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            SSL_free(https_ssl);
            close(ssl_sock);
            return -22i32
        }
    }
    (*vpninfo).cstp_cipher =
        SSL_CIPHER_get_name(SSL_get_current_cipher(https_ssl)) as
            *mut libc::c_char;
    (*vpninfo).ssl_fd = ssl_sock;
    (*vpninfo).https_ssl = https_ssl;
    (*vpninfo).ssl_read =
        Some(openconnect_openssl_read as
                 unsafe extern "C" fn(_: *mut openconnect_info,
                                      _: *mut libc::c_char, _: size_t)
                     -> libc::c_int);
    (*vpninfo).ssl_write =
        Some(openconnect_openssl_write as
                 unsafe extern "C" fn(_: *mut openconnect_info,
                                      _: *mut libc::c_char, _: size_t)
                     -> libc::c_int);
    (*vpninfo).ssl_gets =
        Some(openconnect_openssl_gets as
                 unsafe extern "C" fn(_: *mut openconnect_info,
                                      _: *mut libc::c_char, _: size_t)
                     -> libc::c_int);
    if (*vpninfo).verbose >= 1i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                1i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Connected to HTTPS on %s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                (*vpninfo).hostname);
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "1845:1"]
pub unsafe extern "C" fn cstp_handshake(mut vpninfo: *mut openconnect_info,
                                        mut init: libc::c_uint)
 -> libc::c_int {
    return -102i32;
}
#[no_mangle]
#[src_loc = "1850:1"]
pub unsafe extern "C" fn openconnect_close_https(mut vpninfo:
                                                     *mut openconnect_info,
                                                 mut final_0: libc::c_int) {
    if !(*vpninfo).https_ssl.is_null() {
        SSL_free((*vpninfo).https_ssl);
        (*vpninfo).https_ssl = 0 as *mut SSL
    }
    if (*vpninfo).ssl_fd != -1i32 {
        close((*vpninfo).ssl_fd);
        let mut __fd: libc::c_int = (*vpninfo).ssl_fd;
        (*vpninfo)._select_rfds.fds_bits[(__fd as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &=
            !(((1i32 as libc::c_ulong) <<
                   (__fd as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8i32
                                                                                         as
                                                                                         libc::c_ulong)))
                  as __int32_t);
        let mut __fd_0: libc::c_int = (*vpninfo).ssl_fd;
        (*vpninfo)._select_wfds.fds_bits[(__fd_0 as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &=
            !(((1i32 as libc::c_ulong) <<
                   (__fd_0 as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8i32
                                                                                         as
                                                                                         libc::c_ulong)))
                  as __int32_t);
        let mut __fd_1: libc::c_int = (*vpninfo).ssl_fd;
        (*vpninfo)._select_efds.fds_bits[(__fd_1 as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &=
            !(((1i32 as libc::c_ulong) <<
                   (__fd_1 as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8i32
                                                                                         as
                                                                                         libc::c_ulong)))
                  as __int32_t);
        (*vpninfo).ssl_fd = -1i32
    }
    if final_0 != 0 {
        if !(*vpninfo).https_ctx.is_null() {
            SSL_CTX_free((*vpninfo).https_ctx);
            (*vpninfo).https_ctx = 0 as *mut SSL_CTX
        }
        if !(*vpninfo).cert_x509.is_null() {
            X509_free((*vpninfo).cert_x509);
            (*vpninfo).cert_x509 = 0 as *mut X509
        }
    };
}
#[no_mangle]
#[src_loc = "1875:1"]
pub unsafe extern "C" fn openconnect_init_ssl() -> libc::c_int {
    SSL_library_init();
    ERR_clear_error();
    SSL_load_error_strings();
    OPENSSL_add_all_algorithms_noconf();
    return 0i32;
}
#[no_mangle]
#[src_loc = "1891:1"]
pub unsafe extern "C" fn openconnect_get_peer_cert_details(mut vpninfo:
                                                               *mut openconnect_info)
 -> *mut libc::c_char {
    let mut bp: *mut BIO = BIO_new(BIO_s_mem());
    let mut certinfo: *mut BUF_MEM = 0 as *mut BUF_MEM;
    let mut zero: libc::c_char = 0i32 as libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    X509_print_ex(bp, (*vpninfo).peer_cert as *mut X509,
                  0i32 as libc::c_ulong, 0i32 as libc::c_ulong);
    BIO_write(bp, &mut zero as *mut libc::c_char as *const libc::c_void,
              1i32);
    BIO_ctrl(bp, 115i32, 0i32 as libc::c_long,
             &mut certinfo as *mut *mut BUF_MEM as *mut libc::c_char as
                 *mut libc::c_void);
    ret = strdup((*certinfo).data);
    BIO_free(bp);
    return ret;
}
#[no_mangle]
#[src_loc = "1907:1"]
pub unsafe extern "C" fn openconnect_free_cert_info(mut vpninfo:
                                                        *mut openconnect_info,
                                                    mut buf:
                                                        *mut libc::c_void) {
    free(buf);
}
#[no_mangle]
#[src_loc = "1913:1"]
pub unsafe extern "C" fn openconnect_local_cert_md5(mut vpninfo:
                                                        *mut openconnect_info,
                                                    mut buf:
                                                        *mut libc::c_char)
 -> libc::c_int {
    *buf.offset(0) = 0i32 as libc::c_char;
    if (*vpninfo).cert_x509.is_null() { return -5i32 }
    if get_cert_md5_fingerprint(vpninfo,
                                (*vpninfo).cert_x509 as *mut libc::c_void,
                                buf) != 0 {
        return -5i32
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "1928:1"]
pub unsafe extern "C" fn openconnect_hash_yubikey_password(mut vpninfo:
                                                               *mut openconnect_info,
                                                           mut password:
                                                               *const libc::c_char,
                                                           mut pwlen:
                                                               libc::c_int,
                                                           mut ident:
                                                               *const libc::c_void,
                                                           mut id_len:
                                                               libc::c_int)
 -> libc::c_int {
    if PKCS5_PBKDF2_HMAC_SHA1(password, pwlen, ident as *const libc::c_uchar,
                              id_len, 1000i32, 16i32,
                              (*vpninfo).yubikey_pwhash.as_mut_ptr()) == 0 {
        return -5i32
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "1939:1"]
pub unsafe extern "C" fn openconnect_yubikey_chalresp(mut vpninfo:
                                                          *mut openconnect_info,
                                                      mut challenge:
                                                          *const libc::c_void,
                                                      mut chall_len:
                                                          libc::c_int,
                                                      mut result:
                                                          *mut libc::c_void)
 -> libc::c_int {
    let mut mdlen: libc::c_uint = 20i32 as libc::c_uint;
    if HMAC(EVP_sha1(),
            (*vpninfo).yubikey_pwhash.as_mut_ptr() as *const libc::c_void,
            16i32, challenge as *const libc::c_uchar, chall_len as size_t,
            result as *mut libc::c_uchar, &mut mdlen).is_null() {
        return -5i32
    }
    return 0i32;
}
/* dtls.c */
/* cstp.c */
/* auth-juniper.c */
/* oncp.c */
/* pulse.c */
/* auth-globalprotect.c */
/* gpst.c */
/* lzs.c */
/* ssl.c */
/* openssl-pkcs11.c */
/* esp.c */
/* {gnutls,openssl}-esp.c */
/* {gnutls,openssl}.c */
#[no_mangle]
#[src_loc = "1951:1"]
pub unsafe extern "C" fn hotp_hmac(mut vpninfo: *mut openconnect_info,
                                   mut challenge: *const libc::c_void)
 -> libc::c_int {
    let mut hash: [libc::c_uchar; 64] = [0; 64]; /* Enough for a SHA256 */
    let mut hashlen: libc::c_uint =
        ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong as
            libc::c_uint;
    let mut alg: *const EVP_MD = 0 as *const EVP_MD;
    match (*vpninfo).oath_hmac_alg as libc::c_uint {
        0 => { alg = EVP_sha1() }
        1 => { alg = EVP_sha256() }
        2 => { alg = EVP_sha512() }
        _ => {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Unsupported OATH HMAC algorithm\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            return -22i32
        }
    }
    if HMAC(alg, (*vpninfo).oath_secret as *const libc::c_void,
            (*vpninfo).oath_secret_len as libc::c_int,
            challenge as *const libc::c_uchar, 8i32 as size_t,
            hash.as_mut_ptr(), &mut hashlen).is_null() {
        (*vpninfo).progress.expect("non-null function pointer")(vpninfo as
                                                                    *mut libc::c_void,
                                                                0i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Failed to calculate OATH HMAC\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
        ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                     unsafe extern "C" fn(_:
                                                              *const libc::c_char,
                                                          _: size_t,
                                                          _:
                                                              *mut libc::c_void)
                                         -> libc::c_int),
                            vpninfo as *mut libc::c_void);
        return -22i32
    }
    hashlen =
        (hash[hashlen.wrapping_sub(1i32 as libc::c_uint) as usize] as
             libc::c_int & 15i32) as libc::c_uint;
    return (load_be32(&mut *hash.as_mut_ptr().offset(hashlen as isize) as
                          *mut libc::c_uchar as *const libc::c_void) &
                0x7fffffffi32 as libc::c_uint) as libc::c_int;
}
#[src_loc = "2001:19"]
static mut ttls_bio_meth: BIO_METHOD =
    unsafe {
        {
            let mut init =
                bio_method_st{type_0: 0x80i32,
                              name:
                                  b"EAP-TTLS\x00" as *const u8 as
                                      *const libc::c_char,
                              bwrite:
                                  Some(ttls_push_func as
                                           unsafe extern "C" fn(_: *mut BIO,
                                                                _:
                                                                    *const libc::c_char,
                                                                _:
                                                                    libc::c_int)
                                               -> libc::c_int),
                              bread:
                                  Some(ttls_pull_func as
                                           unsafe extern "C" fn(_: *mut BIO,
                                                                _:
                                                                    *mut libc::c_char,
                                                                _:
                                                                    libc::c_int)
                                               -> libc::c_int),
                              bputs: None,
                              bgets: None,
                              ctrl:
                                  Some(ttls_ctrl_func as
                                           unsafe extern "C" fn(_: *mut BIO,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_long,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> libc::c_long),
                              create: None,
                              destroy: None,
                              callback_ctrl: None,};
            init
        }
    };
#[src_loc = "2008:1"]
unsafe extern "C" fn eap_ttls_method() -> *mut BIO_METHOD {
    return &mut ttls_bio_meth;
}
#[inline]
#[src_loc = "2013:1"]
unsafe extern "C" fn BIO_set_data(mut b: *mut BIO, mut p: *mut libc::c_void) {
    (*b).ptr = p;
}
#[inline]
#[src_loc = "2018:1"]
unsafe extern "C" fn BIO_get_data(mut b: *mut BIO) -> *mut libc::c_void {
    return (*b).ptr;
}
#[src_loc = "2023:1"]
unsafe extern "C" fn BIO_set_init(mut b: *mut BIO, mut i: libc::c_int) {
    (*b).init = i;
}
/* !HAVE_BIO_METH_FREE */
#[src_loc = "2029:1"]
unsafe extern "C" fn ttls_push_func(mut b: *mut BIO,
                                    mut buf: *const libc::c_char,
                                    mut len: libc::c_int) -> libc::c_int {
    let mut vpninfo: *mut openconnect_info =
        BIO_get_data(b) as *mut openconnect_info;
    let mut ret: libc::c_int =
        pulse_eap_ttls_send(vpninfo, buf as *const libc::c_void, len);
    if ret >= 0i32 { return ret }
    return 0i32;
}
#[src_loc = "2039:1"]
unsafe extern "C" fn ttls_pull_func(mut b: *mut BIO,
                                    mut buf: *mut libc::c_char,
                                    mut len: libc::c_int) -> libc::c_int {
    let mut vpninfo: *mut openconnect_info =
        BIO_get_data(b) as *mut openconnect_info;
    let mut ret: libc::c_int =
        pulse_eap_ttls_recv(vpninfo, buf as *mut libc::c_void, len);
    if ret >= 0i32 { return ret }
    return 0i32;
}
#[src_loc = "2049:1"]
unsafe extern "C" fn ttls_ctrl_func(mut b: *mut BIO, mut cmd: libc::c_int,
                                    mut larg: libc::c_long,
                                    mut iarg: *mut libc::c_void)
 -> libc::c_long {
    match cmd {
        11 => { return 1i32 as libc::c_long }
        _ => { return 0i32 as libc::c_long }
    };
}
#[no_mangle]
#[src_loc = "2059:1"]
pub unsafe extern "C" fn establish_eap_ttls(mut vpninfo:
                                                *mut openconnect_info)
 -> *mut libc::c_void {
    let mut ttls_ssl: *mut SSL = 0 as *mut SSL;
    let mut bio: *mut BIO = 0 as *mut BIO;
    let mut err: libc::c_int = 0;
    if (*vpninfo).ttls_bio_meth.is_null() {
        (*vpninfo).ttls_bio_meth = eap_ttls_method()
    }
    bio = BIO_new((*vpninfo).ttls_bio_meth);
    BIO_set_data(bio, vpninfo as *mut libc::c_void);
    BIO_set_init(bio, 1i32);
    ttls_ssl = SSL_new((*vpninfo).https_ctx);
    workaround_openssl_certchain_bug(vpninfo, ttls_ssl);
    SSL_set_bio(ttls_ssl, bio, bio);
    SSL_set_verify(ttls_ssl, 0x1i32, None);
    if (*vpninfo).verbose >= 1i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                1i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"EAP-TTLS negotiation with %s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                (*vpninfo).hostname);
    }
    err = SSL_connect(ttls_ssl);
    if err == 1i32 {
        if (*vpninfo).verbose >= 3i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    3i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Established EAP-TTLS session\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        return ttls_ssl as *mut libc::c_void
    }
    err = SSL_get_error(ttls_ssl, err);
    if (*vpninfo).verbose >= 0i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                0i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"EAP-TTLS connection failure %d\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                err);
    }
    ERR_print_errors_cb(Some(openconnect_print_err_cb as
                                 unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: size_t,
                                                      _: *mut libc::c_void)
                                     -> libc::c_int),
                        vpninfo as *mut libc::c_void);
    SSL_free(ttls_ssl);
    return 0 as *mut libc::c_void;
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
#[no_mangle]
#[src_loc = "2095:1"]
pub unsafe extern "C" fn destroy_eap_ttls(mut vpninfo: *mut openconnect_info,
                                          mut ttls: *mut libc::c_void) {
    SSL_free(ttls as *mut SSL);
    /* Leave the BIO_METH for now. It may get reused and we don't want to
	 * have to call BIO_get_new_index() more times than is necessary */
}
