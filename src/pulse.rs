use libc;
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:20"]
pub mod _types_h {
    #[src_loc = "41:1"]
    pub type __uint8_t = libc::c_uchar;
    #[src_loc = "43:1"]
    pub type __uint16_t = libc::c_ushort;
    #[src_loc = "44:1"]
    pub type __int32_t = libc::c_int;
    #[src_loc = "45:1"]
    pub type __uint32_t = libc::c_uint;
    #[src_loc = "70:1"]
    pub type __darwin_ct_rune_t = libc::c_int;
    #[src_loc = "92:1"]
    pub type __darwin_size_t = libc::c_ulong;
    #[src_loc = "104:1"]
    pub type __darwin_wchar_t = libc::c_int;
    #[src_loc = "109:1"]
    pub type __darwin_rune_t = __darwin_wchar_t;
    #[src_loc = "118:1"]
    pub type __darwin_socklen_t = __uint32_t;
    /* byte count or error */
    #[src_loc = "120:1"]
    pub type __darwin_time_t = libc::c_long;
    /* _BSD_I386__TYPES_H_ */
    /* time() */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:20"]
pub mod sys__types_h {
    /* Used by statvfs and fstatvfs */
    #[src_loc = "60:1"]
    pub type __darwin_gid_t = __uint32_t;
    /* [???] signal set */
    #[src_loc = "74:1"]
    pub type __darwin_suseconds_t = __int32_t;
    /* [???] microseconds */
    #[src_loc = "75:1"]
    pub type __darwin_uid_t = __uint32_t;
    use super::_types_h::{__uint32_t, __int32_t};
    /* _SYS__TYPES_H_ */
    /* (gcc >= 3.5) */
    /* !(gcc >= 3.5) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:20"]
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
    /* _UINT64_T */
}
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
    /* _UINT32_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:20"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_gid_t.h:20"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_fd_def.h:20"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timeval.h:20"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_time_t.h:20"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/time.h:22"]
pub mod time_h {
    /*
 * Copyright (c) 2000 Apple Computer, Inc. All rights reserved.
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
 * Copyright (c) 1989, 1993
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
 *	@(#)time.h	8.3 (Berkeley) 1/21/94
 */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "75:1"]
    pub struct tm {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
        pub tm_gmtoff: libc::c_long,
        pub tm_zone: *mut libc::c_char,
        /* timezone abbreviation */
    }
    use super::_size_t_h::size_t;
    use super::_time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "116:1"]
        pub fn strftime(_: *mut libc::c_char, _: size_t,
                        _: *const libc::c_char, _: *const tm) -> size_t;
        #[no_mangle]
        #[src_loc = "118:1"]
        pub fn time(_: *mut time_t) -> time_t;
        #[no_mangle]
        #[src_loc = "128:1"]
        pub fn localtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
    }
    /* !_TIME_H_ */
    /* _USE_EXTENDED_LOCALES_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/runetype.h:24"]
pub mod runetype_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "60:9"]
    pub struct _RuneEntry {
        pub __min: __darwin_rune_t,
        pub __max: __darwin_rune_t,
        pub __map: __darwin_rune_t,
        pub __types: *mut __uint32_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "67:9"]
    pub struct _RuneRange {
        pub __nranges: libc::c_int,
        pub __ranges: *mut _RuneEntry,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "72:9"]
    pub struct _RuneCharClass {
        pub __name: [libc::c_char; 14],
        pub __mask: __uint32_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "77:9"]
    pub struct _RuneLocale {
        pub __magic: [libc::c_char; 8],
        pub __encoding: [libc::c_char; 32],
        pub __sgetrune: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: __darwin_size_t,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> __darwin_rune_t>,
        pub __sputrune: Option<unsafe extern "C" fn(_: __darwin_rune_t,
                                                    _: *mut libc::c_char,
                                                    _: __darwin_size_t,
                                                    _: *mut *mut libc::c_char)
                                   -> libc::c_int>,
        pub __invalid_rune: __darwin_rune_t,
        pub __runetype: [__uint32_t; 256],
        pub __maplower: [__darwin_rune_t; 256],
        pub __mapupper: [__darwin_rune_t; 256],
        pub __runetype_ext: _RuneRange,
        pub __maplower_ext: _RuneRange,
        pub __mapupper_ext: _RuneRange,
        pub __variable: *mut libc::c_void,
        pub __variable_len: libc::c_int,
        pub __ncharclasses: libc::c_int,
        pub __charclasses: *mut _RuneCharClass,
    }
    use super::_types_h::{__darwin_rune_t, __uint32_t, __darwin_size_t};
    extern "C" {
        /* Indicates version A of RuneLocale */
        #[no_mangle]
        #[src_loc = "111:20"]
        pub static mut _DefaultRuneLocale: _RuneLocale;
    }
    /* !_RUNETYPE_H_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:26"]
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
    /* _UINT8_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint16_t.h:26"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:28"]
pub mod _in_addr_t_h {
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
    /* __uint32_t */
    #[src_loc = "31:1"]
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
    /* _IN_ADDR_T */
    /* base type for internet address */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_port_t.h:28"]
pub mod _in_port_t_h {
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
    /* __uint16_t */
    #[src_loc = "31:1"]
    pub type in_port_t = __uint16_t;
    use super::_types_h::__uint16_t;
    /* _IN_PORT_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:32"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:32"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:32"]
pub mod socket_h {
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
    use super::_socklen_t_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "697:1"]
        pub fn getsockname(_: libc::c_int, _: *mut sockaddr,
                           _: *mut socklen_t) -> libc::c_int;
    }
    /* !_SYS_SOCKET_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:32"]
pub mod in_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "301:1"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "375:1"]
    pub struct sockaddr_in {
        pub sin_len: __uint8_t,
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_char; 8],
    }
    use super::_in_addr_t_h::in_addr_t;
    use super::_types_h::__uint8_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_in_port_t_h::in_port_t;
    /* _NETINET_IN_H_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:32"]
pub mod in6_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "151:9"]
    pub struct in6_addr {
        pub __u6_addr: C2RustUnnamed,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "152:2"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [__uint8_t; 16],
        pub __u6_addr16: [__uint16_t; 8],
        pub __u6_addr32: [__uint32_t; 4],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "169:1"]
    pub struct sockaddr_in6 {
        pub sin6_len: __uint8_t,
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: __uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: __uint32_t,
    }
    use super::_types_h::{__uint8_t, __uint16_t, __uint32_t};
    use super::_sa_family_t_h::sa_family_t;
    use super::_in_port_t_h::in_port_t;
    /* !_NETINET6_IN6_H_ */
    /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netdb.h:32"]
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
    }
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
    /* !_NETDB_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src = "/Users/cameron/github/openconnect/openconnect.h:32"]
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
    }
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
    /*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2008-2016 Intel Corporation.
 * Copyright © 2008 Nick Andrew <nick@nick-andrew.net>
 * Copyright © 2013 John Morrissey <jwm@horde.net>
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
    /*
 * API version 5.5 (v8.00; 2019-01-05):
 *  - add openconnect_set_version_string()
 *  - add openconnect_set_key_password()
 *  - Add openconnect_has_tss2_blob_support()
 *  - Add openconnect_get_supported_protocols()
 *  - Add openconnect_free_supported_protocols()
 *  - Add openconnect_get_protocol()
 *  - Add openconnect_get_idle_timeout()
 *
 * API version 5.4 (v7.08; 2016-12-13):
 *  - Add openconnect_set_pass_tos()
 *
 * API version 5.3 (v7.07; 2016-07-11):
 *  - Add openconnect_set_localname().
 *  - Add openconnect_override_getaddrinfo().
 *  - Add openconnect_get_cstp_compression().
 *  - Add openconnect_get_dtls_compression().
 *  - Add openconnect_disable_ipv6().
 *  - Add ip_info->gateway_addr.
 *  - Add openconnect_set_setup_tun_handler().
 *  - Add openconnect_set_reconnected_handler().
 *  - Add openconnect_get_dnsname().
 *  - Add openconnect_get_peer_cert_chain() and
 *        openconnect_free_peer_cert_chain().
 *
 * API version 5.2 (v7.05; 2015-03-10):
 *  - Add openconnect_set_http_auth(), openconnect_set_protocol().
 *
 * API version 5.1 (v7.05; 2015-03-10):
 *  - Add openconnect_set_compression_mode(), openconnect_set_loglevel()
 *
 *   (Note: API 5.1 and openconnect_set_compression_mode() were present in
 *    this file in the v7.04 release on 2015-01-25, but the symbol versioning
 *    for the new function was OPENCONNECT_5_0, and openconnect_set_loglevel()
 *    was not yet present.)
 *
 * API version 5.0 (v7.00; 2014-11-27):
 *  - Remove OPENCONNECT_X509 and openconnect_get_peer_cert().
 *  - Change openconnect_get_cert_der() to openconnect_get_peer_cert_DER() etc.
 *  - Add openconnect_check_peer_cert_hash().
 *  - Remove openconnect_set_server_cert_sha1().
 *  - Add openconnect_has_yubioath_support() and OC_TOKEN_MODE_YUBIOATH.
 *  - Add openconnect_has_system_key_support().
 *
 * API version 4.1 (v7.00; 2014-11-27):
 *  - Add openconnect_get_cstp_cipher(), openconnect_get_dtls_cipher(),
 *    openconnect_set_system_trust(), openconnect_set_csd_environ().
 *  - Change openconnect_init_ssl() to return int.
 *
 * API version 4.0 (v7.00; 2014-11-27):
 *  - Change string handling to never transfer ownership of allocations.
 *  - Add openconnect_set_option_value(), openconnect_free_cert_info().
 *
 * API version 3.4 (v7.00; 2014-11-27):
 *  - Add openconnect_set_token_callbacks()
 *
 * API version 3.3 (v6.00; 2014-07-08):
 *  - Add openconnect_set_pfs(), openconnect_set_dpd(),
 *    openconnect_set_proxy_auth()
 *
 * API version 3.2 (v5.99; 2014-03-05):
 *  - Add OC_TOKEN_MODE_HOTP and allow openconnect_has_oath_support() to
 *    return 2 to indicate that it is present.
 *
 * API version 3.1 (v5.99; 2014-03-05):
 *  - Add openconnect_setup_cmd_pipe(), openconnect_mainloop(),
 *    openconnect_setup_tun_device(), openconnect_setup_tun_script(),
 *    openconnect_setup_tun_fd(), openconnect_setup_dtls(),
 *    openconnect_make_cstp_connection(), openconnect_set_server_cert_sha1(),
 *    openconnect_get_ifname(), openconnect_set_reqmtu(),
 *    openconnect_get_ip_info(), openconnect_set_protect_socket_handler(),
 *    openconnect_set_mobile_info(), openconnect_set_xmlpost(),
 *    openconnect_set_stats_handler()
 *
 * API version 3.0 (v5.99; 2014-03-05):
 *  - Change oc_form_opt_select->choices to an array of pointers
 *  - Add oc_form_opt->flags
 *  - Add OC_FORM_RESULT_* and oc_auth_form->authgroup_*
 *
 * API version 2.2 (v5.00; 2013-05-16):
 *  - Add openconnect_set_token_mode(), openconnect_has_oath_support()
 *  - Deprecate openconnect_set_stoken_mode()
 *
 * API version 2.1 (v4.99; 2013-02-07):
 *  - Add openconnect_set_reported_os()
 *  - Add openconnect_set_stoken_mode(), openconnect_has_stoken_support()
 *
 * API version 2.0 (v3.99; 2012-06-13):
 *  - OPENCONNECT_X509 is now an opaque type.
 *  - Add openconnect_has_pkcs11_support(), openconnect_has_tss_blob_support()
 *  - Rename openconnect_init_openssl() -> openconnect_init_ssl()
 *  - Rename openconnect_vpninfo_new_with_cbdata() -> openconnect_vpninfo_new()
 *    and kill the old openconnect_vpninfo_new() and its callback types.
 *
 * API version 1.5 (v3.99; 2012-06-13):
 *  - Add openconnect_get_cert_details(), openconnect_get_cert_DER().
 *
 * API version 1.4 (v3.19; 2012-05-17):
 *  - Add openconnect_set_cancel_fd()
 *
 * API version 1.3 (v3.13; 2011-09-30):
 *  - Add openconnect_set_cert_expiry_warning() to change from default 60 days
 *
 * API version 1.2 (v3.10; 2011-06-30)
 *  - Add openconnect_vpninfo_new_with_cbdata()
 *
 * API version 1.1 (v3.02; 2011-04-19):
 *  - Add openconnect_vpninfo_free()
 *
 * API version 1.0 (v3.00; 2011-03-09):
 *  - Initial version
 *
 * NEW LIBRARY FUNCTION CHECKLIST:
 *
 * 1) Bump the API version if the current API version has already appeared
 *    in a release
 * 2) Add function to the above changelog
 * 3) Add function to libopenconnect.map.in
 * 4) Add declaration + comments in the latter part of this file
 * 5) Add function to jni.c, then test with ./configure --with-java && make
 * 6) Add declaration to LibOpenConnect.java, then run "cd java && ant" to test
 */
    /* Before API version 1.4 (OpenConnect 3.19) this macro didn't exist.
 * Somewhat ironic, that the API version check itself needs to be
 * conditionally used depending on the API version. A very simple way
 * for users to handle this with an approximately correct answer is
 *   #include <openconnect.h>
 *   #ifndef OPENCONNECT_CHECK_VER
 *   #define OPENCONNECT_CHECK_VER(x,y) 0
 *   #endif
 */
    /* ***************************************************************************/
    /* Enumeration of supported VPN protocols */
    /* ***************************************************************************/
    /* Authentication form processing */
    /* char * fields are static (owned by XML parser) and don't need to be
   freed by the form handling code — except for value, which for TEXT
   and PASSWORD options is allocated by openconnect_set_option_value()
   when process_form() interacts with the user and must be freed. */
    /* Use openconnect_set_option_value() to set this */
    /* To set the value to a form use the following function */
    /* All fields are static, owned by the XML parser */
    /* All char * fields are static, owned by the XML parser */
    /* The elements above this line come from server-provided CSTP headers,
	 * so they should be handled with caution.  gateway_addr is generated
	 * locally from getnameinfo(). */
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
    #[src_loc = "478:1"]
    pub type openconnect_unlock_token_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char) -> libc::c_int>;
    #[src_loc = "477:1"]
    pub type openconnect_lock_token_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
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
#[header_src = "/Users/cameron/github/openconnect/openconnect-internal.h:32"]
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
        pub c2rust_unnamed: C2RustUnnamed_0,
        pub data: [libc::c_uchar; 0],
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "126:2"]
    pub union C2RustUnnamed_0 {
        pub esp: C2RustUnnamed_5,
        pub oncp: C2RustUnnamed_4,
        pub cstp: C2RustUnnamed_3,
        pub gpst: C2RustUnnamed_2,
        pub pulse: C2RustUnnamed_1,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "146:3"]
    pub struct C2RustUnnamed_1 {
        pub pad: [libc::c_uchar; 8],
        pub vendor: uint32_t,
        pub type_0: uint32_t,
        pub len: uint32_t,
        pub ident: uint32_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "142:3"]
    pub struct C2RustUnnamed_2 {
        pub pad: [libc::c_uchar; 8],
        pub hdr: [libc::c_uchar; 16],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "138:3"]
    pub struct C2RustUnnamed_3 {
        pub pad: [libc::c_uchar; 16],
        pub hdr: [libc::c_uchar; 8],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "133:3"]
    pub struct C2RustUnnamed_4 {
        pub pad: [libc::c_uchar; 2],
        pub rec: [libc::c_uchar; 2],
        pub kmp: [libc::c_uchar; 20],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "127:3"]
    pub struct C2RustUnnamed_5 {
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
    /*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2008-2015 Intel Corporation.
 * Copyright © 2008 Nick Andrew <nick@nick-andrew.net>
 * Copyright © 2013 John Morrissey <jwm@horde.net>
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
    /* Ick */
    /* OPENSSL */
    /* FreeBSD provides this in <sys/param.h>  */
    /* At least MinGW headers seem not to provide IPPROTO_IPIP */
    /* ***************************************************************************/
    /* Random secret has not been generated yet */
    /* Secret is present, ready to attempt DTLS */
    /* DTLS was disabled on the *client* side */
    /* For ESP, sometimes sending probes */
    /* ESP probe received; must tell server */
    /* Server informed and should be sending ESP */
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
    #[src_loc = "1189:1"]
    pub struct oc_packed_uint16_t {
        pub d: uint16_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C, packed)]
    #[src_loc = "1186:1"]
    pub struct oc_packed_uint32_t {
        pub d: uint32_t,
    }
    #[inline]
    #[src_loc = "310:1"]
    pub unsafe extern "C" fn dequeue_packet(mut q: *mut pkt_q) -> *mut pkt {
        let mut ret: *mut pkt = (*q).head;
        if !ret.is_null() {
            (*q).head = (*ret).next;
            (*q).count -= 1;
            if (*q).count == 0 { (*q).tail = &mut (*q).head }
        }
        return ret;
    }
    #[inline]
    #[src_loc = "330:1"]
    pub unsafe extern "C" fn queue_packet(mut q: *mut pkt_q, mut p: *mut pkt)
     -> libc::c_int {
        *(*q).tail = p;
        (*p).next = 0 as *mut pkt;
        (*q).tail = &mut (*p).next;
        (*q).count += 1;
        return (*q).count;
    }
    #[inline]
    #[src_loc = "1199:1"]
    pub unsafe extern "C" fn load_be16(mut _p: *const libc::c_void)
     -> uint16_t {
        let mut p: *const oc_packed_uint16_t =
            _p as *const oc_packed_uint16_t;
        return if 0 != 0 {
                   (((*p).d as libc::c_int & 0xff00i32) >> 8i32 |
                        ((*p).d as libc::c_int & 0xffi32) << 8i32) as
                       __uint16_t as libc::c_int
               } else { _OSSwapInt16((*p).d) as libc::c_int } as __uint16_t;
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
    #[inline]
    #[src_loc = "1205:1"]
    pub unsafe extern "C" fn store_be32(mut _p: *mut libc::c_void,
                                        mut d: uint32_t) {
        let mut p: *mut oc_packed_uint32_t = _p as *mut oc_packed_uint32_t;
        (*p).d =
            if 0 != 0 {
                ((d & 0xff000000u32) >> 24i32 |
                     (d & 0xff0000i32 as libc::c_uint) >> 8i32 |
                     (d & 0xff00i32 as libc::c_uint) << 8i32) |
                    (d & 0xffi32 as libc::c_uint) << 24i32
            } else { _OSSwapInt32(d) };
    }
    #[inline]
    #[src_loc = "1211:1"]
    pub unsafe extern "C" fn store_be16(mut _p: *mut libc::c_void,
                                        mut d: uint16_t) {
        let mut p: *mut oc_packed_uint16_t = _p as *mut oc_packed_uint16_t;
        (*p).d =
            if 0 != 0 {
                ((d as libc::c_int & 0xff00i32) >> 8i32 |
                     (d as libc::c_int & 0xffi32) << 8i32) as __uint16_t as
                    libc::c_int
            } else { _OSSwapInt16(d) as libc::c_int } as __uint16_t;
    }
    #[inline]
    #[src_loc = "1243:1"]
    pub unsafe extern "C" fn store_le32(mut _p: *mut libc::c_void,
                                        mut d: uint32_t) {
        let mut p: *mut oc_packed_uint32_t = _p as *mut oc_packed_uint32_t;
        (*p).d = d;
    }
    #[inline]
    #[src_loc = "1231:1"]
    pub unsafe extern "C" fn load_le32(mut _p: *const libc::c_void)
     -> uint32_t {
        let mut p: *const oc_packed_uint32_t =
            _p as *const oc_packed_uint32_t;
        return (*p).d;
    }
    use super::_uint32_t_h::uint32_t;
    use super::_time_t_h::time_t;
    use super::gssapi_h::{gss_name_t, gss_ctx_id_t};
    use super::hmac_h::HMAC_CTX;
    use super::ossl_typ_h::EVP_CIPHER_CTX;
    use super::_uint64_t_h::uint64_t;
    use super::openconnect_h::{openconnect_info, oc_auth_form, oc_form_opt};
    use super::_uint16_t_h::uint16_t;
    use super::_types_h::__uint16_t;
    use super::_OSByteOrder_h::{_OSSwapInt16, _OSSwapInt32};
    extern "C" {
        /* Failed */
        /* Server has not offered it */
        /* Server has offered it, we have not tried it */
        /* Individual auth types may use 2 onwards for their own state */
        /* In-progress attempt */
        /* This does the full authentication, calling back as appropriate */
        /* Establish the TCP connection (and obtain configuration) */
        /* Add headers common to each HTTP request */
        /* Set up the UDP (DTLS) connection. Doesn't actually *start* it. */
        /* This will actually complete the UDP connection setup/handshake on the wire,
	   as well as transporting packets */
        /* Close the connection but leave the session setup so it restarts */
        /* Close and destroy the (UDP) session */
        /* Send probe packets to start or maintain the (UDP) session */
        /* Catch probe packet confirming the (UDP) session */
        /* packet + header */
        /* DTLS header */
        /* biggest supported MAC (SHA1) */
        /* biggest supported IV (AES-256) */
        /* max padding */
        /* Stored network-endian */
        /* Encryption key */
        /* HMAC key */
        #[src_loc = "363:1"]
        pub type oc_pcsc_ctx;
        #[no_mangle]
        #[src_loc = "863:1"]
        pub fn establish_eap_ttls(vpninfo: *mut openconnect_info)
         -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "864:1"]
        pub fn destroy_eap_ttls(vpninfo: *mut openconnect_info,
                                sess: *mut libc::c_void);
        #[no_mangle]
        #[src_loc = "1097:1"]
        pub fn buf_free(buf: *mut oc_text_buf) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "989:1"]
        pub fn openconnect_close_https(vpninfo: *mut openconnect_info,
                                       final_0: libc::c_int);
        #[no_mangle]
        #[src_loc = "1084:1"]
        pub fn dump_buf_hex(vpninfo: *mut openconnect_info,
                            loglevel: libc::c_int, prefix: libc::c_char,
                            buf: *mut libc::c_uchar, len: libc::c_int);
        #[no_mangle]
        #[src_loc = "1093:1"]
        pub fn buf_truncate(buf: *mut oc_text_buf);
        #[no_mangle]
        #[src_loc = "1096:1"]
        pub fn buf_error(buf: *mut oc_text_buf) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1088:1"]
        pub fn buf_append_bytes(buf: *mut oc_text_buf,
                                bytes: *const libc::c_void, len: libc::c_int);
        #[no_mangle]
        #[src_loc = "1082:1"]
        pub fn buf_alloc() -> *mut oc_text_buf;
        #[no_mangle]
        #[src_loc = "1137:1"]
        pub fn process_auth_form(vpninfo: *mut openconnect_info,
                                 form: *mut oc_auth_form) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1086:1"]
        pub fn buf_append(buf: *mut oc_text_buf, fmt: *const libc::c_char,
                          _: ...);
        #[no_mangle]
        #[src_loc = "1072:1"]
        pub fn free_pass(p: *mut *mut libc::c_char);
        #[no_mangle]
        #[src_loc = "1075:1"]
        pub fn do_gen_tokencode(vpninfo: *mut openconnect_info,
                                form: *mut oc_auth_form) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1077:1"]
        pub fn can_gen_tokencode(vpninfo: *mut openconnect_info,
                                 form: *mut oc_auth_form,
                                 opt: *mut oc_form_opt) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "991:1"]
        pub fn get_cert_md5_fingerprint(vpninfo: *mut openconnect_info,
                                        cert: *mut libc::c_void,
                                        buf: *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1107:1"]
        pub fn process_http_response(vpninfo: *mut openconnect_info,
                                     connect: libc::c_int,
                                     header_cb:
                                         Option<unsafe extern "C" fn(_:
                                                                         *mut openconnect_info,
                                                                     _:
                                                                         *mut libc::c_char,
                                                                     _:
                                                                         *mut libc::c_char)
                                                    -> libc::c_int>,
                                     body: *mut oc_text_buf) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1083:1"]
        pub fn dump_buf(vpninfo: *mut openconnect_info, prefix: libc::c_char,
                        buf: *mut libc::c_char);
        #[no_mangle]
        #[src_loc = "1111:1"]
        pub fn http_common_headers(vpninfo: *mut openconnect_info,
                                   buf: *mut oc_text_buf);
        #[no_mangle]
        #[src_loc = "988:1"]
        pub fn openconnect_open_https(vpninfo: *mut openconnect_info)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "976:1"]
        pub fn openconnect_setup_esp_keys(vpninfo: *mut openconnect_info,
                                          new_keys: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "953:1"]
        pub fn udp_sockaddr(vpninfo: *mut openconnect_info, port: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "955:1"]
        pub fn ssl_reconnect(vpninfo: *mut openconnect_info) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "974:1"]
        pub fn esp_shutdown(vpninfo: *mut openconnect_info);
        #[no_mangle]
        #[src_loc = "987:1"]
        pub fn ssl_nonblock_write(vpninfo: *mut openconnect_info,
                                  buf: *mut libc::c_void, buflen: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "975:1"]
        pub fn print_esp_keys(vpninfo: *mut openconnect_info,
                              name: *const libc::c_char, esp: *mut esp)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "986:1"]
        pub fn ssl_nonblock_read(vpninfo: *mut openconnect_info,
                                 buf: *mut libc::c_void, maxlen: libc::c_int)
         -> libc::c_int;
    }
    /* __OPENCONNECT_INTERNAL_H__ */
    /* !Not known to be little-endian */
}
#[header_src =
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ossl_typ.h:32"]
pub mod ossl_typ_h {
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
    /* Digest ctrls */
    /* Minimum Algorithm specific ctrl value */
    /* !EVP_MD */
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
    /* functional reference if 'digest' is
                                 * ENGINE-provided */
    /* Public key context for sign/verify */
    /* Update function: usually copied from EVP_MD */
    /* EVP_MD_CTX */
    /* values for EVP_MD_CTX flags */
    /* digest update will be
                                                * called once only */
    /* context has already been
                                                * cleaned */
    /* Don't free up ctx->md_data
                                                * in EVP_MD_CTX_cleanup */
    /*
 * FIPS and pad options are ignored in 1.0.0, definitions are here so we
 * don't accidentally reuse the values for other purposes.
 */
    /* Allow use of non FIPS
                                                * digest in FIPS mode */
    /*
 * The following PAD options are also currently ignored in 1.0.0, digest
 * parameters are handled through EVP_DigestSign*() and EVP_DigestVerify*()
 * instead.
 */
    /* RSA mode to use */
    /* PKCS#1 v1.5 mode */
    /* X9.31 mode */
    /* PSS mode */
    /* Don't initialize md_data */
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
        /* If placed in pkcs12.h, we end up with a circular depency with pkcs7.h */
        /* Nothing */
        /* Nothing */
        /* Callback types for crypto.h */
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
        /* def HEADER_OPENSSL_TYPES_H */
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
    #[src_loc = "188:1"]
    pub type AUTHORITY_KEYID = AUTHORITY_KEYID_st;
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
        #[src_loc = "190:9"]
        pub type ISSUING_DIST_POINT_st;
        #[src_loc = "188:9"]
        pub type AUTHORITY_KEYID_st;
        #[src_loc = "191:9"]
        pub type NAME_CONSTRAINTS_st;
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
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl.h:32"]
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
    /* ssl/ssl.h */
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
 * Copyright (c) 1998-2018 The OpenSSL Project.  All rights reserved.
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
/* ====================================================================
 * Copyright 2005 Nokia. All rights reserved.
 *
 * The portions of the attached software ("Contribution") is developed by
 * Nokia Corporation and is licensed pursuant to the OpenSSL open source
 * license.
 *
 * The Contribution, originally written by Mika Kousa and Pasi Eronen of
 * Nokia Corporation, consists of the "PSK" (Pre-Shared Key) ciphersuites
 * support (see RFC 4279) to OpenSSL.
 *
 * No patent licenses or other rights except those expressly stated in
 * the OpenSSL open source license shall be deemed granted or received
 * expressly, by implication, estoppel, or otherwise.
 *
 * No assurances are provided by Nokia that the Contribution does not
 * infringe the patent or other intellectual property rights of any third
 * party or that the license provides you with all the necessary rights
 * to make use of the Contribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND. IN
 * ADDITION TO THE DISCLAIMERS INCLUDED IN THE LICENSE, NOKIA
 * SPECIFICALLY DISCLAIMS ANY LIABILITY FOR CLAIMS BROUGHT BY YOU OR ANY
 * OTHER ENTITY BASED ON INFRINGEMENT OF INTELLECTUAL PROPERTY RIGHTS OR
 * OTHERWISE.
 */
    /* SSLeay version number for ASN.1 encoding of the session information */
/*-
 * Version 0 - initial version
 * Version 1 - added the optional peer certificate
 */
    /* text strings for the ciphers */
    /*
 * VRS Additional Kerberos5 entries
 */
    /* These are used to specify which ciphers to use and not to use */
    /* unused! */
    /* unused! */
    /* unused! */
    /* unused! */
    /* alias for kEDH */
    /* alias for kEECDH */
    /* same as "kEDH:-ADH" */
    /* alias for EDH */
    /* same as "kEECDH:-AECDH" */
    /* alias for ECDHE" */
    /* same as "SHA1" */
    /*-
 * COMPLEMENTOF* definitions. These identifiers are used to (de-select)
 * ciphers normally not being used.
 * Example: "RC4" will activate all ciphers using RC4 including ciphers
 * without authentication, which would normally disabled by DEFAULT (due
 * the "!ADH" being part of default). Therefore "RC4:!COMPLEMENTOFDEFAULT"
 * will make sure that it is also disabled in the specific selection.
 * COMPLEMENTOF* identifiers are portable between version, as adjustments
 * to the default cipher setup will also be included here.
 *
 * COMPLEMENTOFDEFAULT does not experience the same special treatment that
 * DEFAULT gets, as only selection is being done and no sorting as needed
 * for DEFAULT.
 */
    /*
 * The following cipher list is used by default. It also is substituted when
 * an application-defined cipher list string starts with 'DEFAULT'.
 */
    /*
 * As of OpenSSL 1.0.0, ssl_create_cipher_list() in ssl/ssl_ciph.c always
 * starts with a reasonable order, and all we have to do for DEFAULT is
 * throwing out anonymous and unencrypted ciphersuites! (The latter are not
 * actually enabled by ALL, but "ALL:RSA" would enable some of them.)
 */
    /* Used in SSL_set_shutdown()/SSL_get_shutdown(); */
    /*
 * This is needed to stop compilers complaining about the 'struct ssl_st *'
 * function parameters used to prototype callbacks in SSL_CTX.
 */
    /* SRTP protection profiles for use with the use_srtp extension (RFC 5764)*/
    /* Typedefs for handling custom extensions */
    /* used to hold info on the particular ciphers used */
    /* text name */
    /* id, 4 bytes, first is version */
    /*
     * changed in 0.9.9: these four used to be portions of a single value
     * 'algorithms'
     */
    /* key exchange algorithm */
    /* server authentication */
    /* symmetric encryption */
    /* symmetric authentication */
    /* (major) protocol version */
    /* strength and export flags */
    /* Extra flags */
    /* Number of bits really used */
    /* Number of bits for algorithm */
    /* Used to hold functions for SSLv2 or SSLv3/TLSv1 functions */
    /* Extra SSLv3/TLS stuff */
    /*-
 * Lets make this into an ASN.1 type structure as follows
 * SSL_SESSION_ID ::= SEQUENCE {
 *      version                 INTEGER,        -- structure version number
 *      SSLversion              INTEGER,        -- SSL version number
 *      Cipher                  OCTET STRING,   -- the 3 byte cipher ID
 *      Session_ID              OCTET STRING,   -- the Session ID
 *      Master_key              OCTET STRING,   -- the master key
 *      KRB5_principal          OCTET STRING    -- optional Kerberos principal
 *      Key_Arg [ 0 ] IMPLICIT  OCTET STRING,   -- the optional Key argument
 *      Time [ 1 ] EXPLICIT     INTEGER,        -- optional Start Time
 *      Timeout [ 2 ] EXPLICIT  INTEGER,        -- optional Timeout ins seconds
 *      Peer [ 3 ] EXPLICIT     X509,           -- optional Peer Certificate
 *      Session_ID_context [ 4 ] EXPLICIT OCTET STRING,   -- the Session ID context
 *      Verify_result [ 5 ] EXPLICIT INTEGER,   -- X509_V_... code for `Peer'
 *      HostName [ 6 ] EXPLICIT OCTET STRING,   -- optional HostName from servername TLS extension
 *      PSK_identity_hint [ 7 ] EXPLICIT OCTET STRING, -- optional PSK identity hint
 *      PSK_identity [ 8 ] EXPLICIT OCTET STRING,  -- optional PSK identity
 *      Ticket_lifetime_hint [9] EXPLICIT INTEGER, -- server's lifetime hint for session ticket
 *      Ticket [10]             EXPLICIT OCTET STRING, -- session ticket (clients only)
 *      Compression_meth [11]   EXPLICIT OCTET STRING, -- optional compression method
 *      SRP_username [ 12 ] EXPLICIT OCTET STRING -- optional SRP username
 *      }
 * Look in ssl/ssl_asn1.c for more details
 * I'm using EXPLICIT tags so I can read the damn things using asn1parse :-).
 */
    /* what ssl version session info is being
                                 * kept in here? */
    /* only really used in SSLv2 */
    /* session_id - valid? */
    /*
     * this is used to determine whether the session is being reused in the
     * appropriate context. It is up to the application to set this, via
     * SSL_new
     */
    /* OPENSSL_NO_KRB5 */
    /*
     * Used to indicate that session resumption is not allowed. Applications
     * can also set this bit for a new session via not_resumable_session_cb
     * to disable session caching and tickets.
     */
    /* The cert is the certificate used to establish this connection */
    /* SESS_CERT */
    /*
     * This is the cert for the other end. On clients, it will be the same as
     * sess_cert->peer_key->x509 (the latter is not enough as sess_cert is
     * not retained in the external representation of sessions, see
     * ssl_asn1.c).
     */
    /*
     * when app_verify_callback accepts a session where the peer's
     * certificate is not ok, we must remember the error for session reuse:
     */
    /* only for servers */
    /* Need to lookup the method */
    /* when ASN.1 loaded, this needs to be used
                                 * to load the 'cipher' structure */
    /* ciphers offered by the client */
    /* application specific data */
    /*
     * These are used to make removal of session-ids more efficient and to
     * implement a maximum cache size.
     */
    /* peer's list */
    /* peer's list */
    /* OPENSSL_NO_EC */
    /* RFC4507 info */
    /* Session ticket */
    /* Session ticket length */
    /* Session lifetime hint in seconds */
    /* Allow initial connection to servers that don't support RI */
    /* Hasn't done anything since OpenSSL 0.9.7h, retained for compatibility */
    /* Refers to ancient SSLREF and SSLv2, retained for compatibility */
    /*
 * Disable SSL 3.0/TLS 1.0 CBC vulnerability workaround that was added in
 * OpenSSL 0.9.6d.  Usually (depending on the application protocol) the
 * workaround is not needed.  Unfortunately some broken SSL/TLS
 * implementations cannot handle it at all, which is why we include it in
 * SSL_OP_ALL.
 */
/* added in 0.9.6e */
    /*
 * SSL_OP_ALL: various bug workarounds that should be rather harmless.  This
 * used to be 0x000FFFFFL before 0.9.7.
 */
    /* DTLS options */
    /* Turn on Cookie Exchange (on relevant for servers) */
    /* Don't use RFC4507 ticket extension */
    /* Use Cisco's "speshul" version of DTLS_BAD_VER (as client)  */
    /* As server, disallow session resumption on renegotiation */
    /* Don't use compression even if supported */
    /* Permit unsafe legacy renegotiation */
    /* If set, always create a new key when using tmp_ecdh parameters */
    /* Does nothing: retained for compatibility */
    /* Does nothing: retained for compatibiity */
    /*
 * Set on servers to choose the cipher according to the server's preferences
 */
    /*
 * If set, a server will allow a client to issue a SSLv3.0 version number as
 * latest version supported in the premaster secret, even when TLSv1.0
 * (version 3.1) was announced in the client hello. Normally this is
 * forbidden to prevent version rollback attacks.
 */
    /*
 * These next two were never actually used for anything since SSLeay zap so
 * we have some more flags.
 */
/*
 * The next flag deliberately changes the ciphertest, this is a check for the
 * PKCS#1 attack
 */
    /*
 * Make server add server-hello extension from early version of cryptopro
 * draft, when GOST ciphersuite is negotiated. Required for interoperability
 * with CryptoPro CSP 3.x
 */
    /*
 * Allow SSL_write(..., n) to return r with 0 < r < n (i.e. report success
 * when just a single record has been written):
 */
    /*
 * Make it possible to retry SSL_write() with changed buffer location (buffer
 * contents must stay the same!); this is not the default to avoid the
 * misconception that non-blocking SSL_write() behaves like non-blocking
 * write():
 */
    /*
 * Never bother the application with retries if the transport is blocking:
 */
    /* Don't attempt to automatically build certificate chain */
    /*
 * Save RAM by releasing read and write buffers when they're empty. (SSL3 and
 * TLS only.) "Released" buffers are put onto a free-list in the context or
 * just freed (depending on the context's setting for freelist_max_len).
 */
    /*
 * Send the current time in the Random fields of the ClientHello and
 * ServerHello records for compatibility with hypothetical implementations
 * that require it.
 */
    /*
 * Send TLS_FALLBACK_SCSV in the ClientHello. To be set only by applications
 * that reconnect with a downgraded protocol version; see
 * draft-ietf-tls-downgrade-scsv-00 for details. DO NOT ENABLE THIS if your
 * application attempts a normal handshake. Only use this in explicit
 * fallback retries, following the guidance in
 * draft-ietf-tls-downgrade-scsv-00.
 */
    /* Cert related flags */
/*
 * Many implementations ignore some aspects of the TLS standards such as
 * enforcing certifcate chain algorithms. When this is set we enforce them.
 */
    /* Suite B modes, takes same values as certificate verify flags */
    /* Suite B 192 bit only mode */
    /* Suite B 128 bit mode allowing 192 bit algorithms */
    /* Perform all sorts of protocol violations for testing purposes */
    /* Flags for building certificate chains */
/* Treat any existing certificates as untrusted CAs */
    /* Don't include root CA in chain */
    /* Just check certificates already there */
    /* Ignore verification errors */
    /* Clear verification errors from queue */
    /* Flags returned by SSL_check_chain */
/* Certificate can be used with this session */
    /* Certificate can also be used for signing */
    /* EE certificate signing algorithm OK */
    /* CA signature algorithms OK */
    /* EE certificate parameters OK */
    /* CA certificate parameters OK */
    /* Signing explicitly allowed as opposed to SHA1 fallback */
    /* Client CA issuer names match (always set for server cert) */
    /* Cert type matches client types (always set for server cert) */
    /* Cert chain suitable to Suite B */
    /* Configuration value types */
    /*
 * Note: SSL[_CTX]_set_{options,mode} use |= op on the previous value, they
 * cannot be used to clear bits.
 */
    /* param for all the callbacks */
    /* set client Hello login callback */
    /* set SRP N/g param callback for verification */
    /* set SRP client passwd callback */
    /* see tls_srp.c */
    /* 100k max cert list :-) */
    /*
 * This callback type is used inside SSL_CTX, SSL, and in the functions that
 * set them. It is used to override the generation of SSL/TLS session IDs in
 * a server. Return value should be zero on an error, non-zero to proceed.
 * Also, callbacks should themselves check if the id they generate is unique
 * otherwise the SSL handshake will fail with an error - callbacks can do
 * this using the 'ssl' value they're passed by;
 * SSL_has_matching_session_id(ssl, id, *id_len) The length value passed in
 * is set at the maximum size the session ID can be. In SSLv2 this is 16
 * bytes, whereas SSLv3/TLSv1 it is 32 bytes. The callback can alter this
 * length to be less if desired, but under SSLv2 session IDs are supposed to
 * be fixed at 16 bytes so the id will be padded after the callback returns
 * in this case. It is also an error for the callback to set the size to
 * zero.
 */
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
    use super::ossl_typ_h::{SSL, BIGNUM, X509, CRYPTO_EX_DATA, SSL_CTX,
                            EVP_MD_CTX, COMP_METHOD};
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
        #[no_mangle]
        #[src_loc = "2334:1"]
        pub fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void,
                        num: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "2336:1"]
        pub fn SSL_write(ssl: *mut SSL, buf: *const libc::c_void,
                         num: libc::c_int) -> libc::c_int;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the SSL functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/stack.h:32"]
pub mod stack_h {
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
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/hmac.h:32"]
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
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/asn1.h:32"]
pub mod asn1_h {
    #[src_loc = "524:1"]
    pub type ASN1_TYPE = asn1_type_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "524:9"]
    pub struct asn1_type_st {
        pub type_0: libc::c_int,
        pub value: C2RustUnnamed_6,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "526:5"]
    pub union C2RustUnnamed_6 {
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
    /* Should we free this one */
    /* Set if 0x07 has bits left value */
    /*
 * This indicates that the ASN1_STRING is not a real value but just a place
 * holder for the location where indefinite length constructed data should be
 * inserted in the memory buffer
 */
    /*
 * This flag is used by the CMS code to indicate that a string is not
 * complete and is a place holder for content when it had all been accessed.
 * The flag will be reset when content has been written to it.
 */
    /*
 * This flag is used by ASN1 code to indicate an ASN1_STRING is an MSTRING
 * type.
 */
    /* This is the base type that holds just about everything :-) */
    /*
     * The value of the following field depends on the type being held.  It
     * is mostly being used for BIT_STRING so if the input data has a
     * non-zero 'unused bits' value, it will be handled correctly
     */
    /*
 * ASN1_ENCODING structure: this is used to save the received encoding of an
 * ASN1 type. This is useful to get round problems with invalid encodings
 * which can break signatures.
 */
    /* DER encoding */
    /* Length of encoding */
    /* set to 1 if 'enc' is invalid */
    /* Used with ASN1 LONG type: if a long is set to this it is omitted */
    /* size limits: this stuff is taken straight from RFC2459 */
    /*
 * Declarations for template structures: for full definitions see asn1t.h
 */
    /* This is just an opaque pointer */
    /* Declare ASN1 functions: the implement macro in in asn1t.h */
    /*-
 * The following macros and typedefs allow an ASN1_ITEM
 * to be embedded in a structure and referenced. Since
 * the ASN1_ITEM pointers need to be globally accessible
 * (possibly from shared libraries) they may exist in
 * different forms. On platforms that support it the
 * ASN1_ITEM structure itself will be globally exported.
 * Other platforms will export a function that returns
 * an ASN1_ITEM pointer.
 *
 * To handle both cases transparently the macros below
 * should be used instead of hard coding an ASN1_ITEM
 * pointer in a structure.
 *
 * The structure will look like this:
 *
 * typedef struct SOMETHING_st {
 *      ...
 *      ASN1_ITEM_EXP *iptr;
 *      ...
 * } SOMETHING;
 *
 * It would be initialised as e.g.:
 *
 * SOMETHING somevar = {...,ASN1_ITEM_ref(X509),...};
 *
 * and the actual pointer extracted with:
 *
 * const ASN1_ITEM *it = ASN1_ITEM_ptr(somevar.iptr);
 *
 * Finally an ASN1_ITEM pointer can be extracted from an
 * appropriate reference with: ASN1_ITEM_rptr(X509). This
 * would be used when a function takes an ASN1_ITEM * argument.
 *
 */
    /* ASN1_ITEM pointer exported type */
    /* Macro to obtain ASN1_ITEM pointer from exported type */
    /* Macro to include ASN1_ITEM pointer from base type */
    /* Parameters used by ASN1_STRING_print_ex() */
    /*
 * These determine which characters to escape: RFC2253 special characters,
 * control characters and MSB set characters
 */
    /*
 * This flag determines how we do escaping: normally RC2253 backslash only,
 * set this to use backslash and quote.
 */
    /* These three flags are internal use only. */
    /* Character is a valid PrintableString character */
    /* Character needs escaping if it is the first character */
    /* Character needs escaping if it is the last character */
    /*
 * NB the internal flags are safely reused below by flags handled at the top
 * level.
 */
    /*
 * If this is set we convert all character strings to UTF8 first
 */
    /*
 * If this is set we don't attempt to interpret content: just assume all
 * strings are 1 byte per character. This will produce some pretty odd
 * looking output!
 */
    /* If this is set we include the string type in the output */
    /*
 * This determines which strings to display and which to 'dump' (hex dump of
 * content octets or DER encoding). We can only dump non character strings or
 * everything. If we don't dump 'unknown' they are interpreted as character
 * strings with 1 octet per character and are subject to the usual escaping
 * options.
 */
    /*
 * These determine what 'dumping' does, we can dump the content octets or the
 * DER encoding: both use the RFC2253 #XXXXX notation.
 */
    /*
 * All the string flags consistent with RFC2253, escaping control characters
 * isn't essential in RFC2253 but it is advisable anyway.
 */
    /*
         * set and sequence are left complete and still contain the set or
         * sequence bytes
         */
    /* This is used to contain a list of bit names */
    /* Macros for string operations */
    /* for the is_set parameter to i2d_ASN1_SET */
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
    use super::ossl_typ_h::{ASN1_BOOLEAN, ASN1_STRING, ASN1_OBJECT,
                            ASN1_INTEGER, ASN1_ENUMERATED, ASN1_BIT_STRING,
                            ASN1_OCTET_STRING, ASN1_PRINTABLESTRING,
                            ASN1_T61STRING, ASN1_IA5STRING,
                            ASN1_GENERALSTRING, ASN1_BMPSTRING,
                            ASN1_UNIVERSALSTRING, ASN1_UTCTIME,
                            ASN1_GENERALIZEDTIME, ASN1_VISIBLESTRING,
                            ASN1_UTF8STRING};
    use super::stack_h::_STACK;
    extern "C" {
        #[src_loc = "299:9"]
        pub type ASN1_VALUE_st;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the ASN1 functions. */
}
#[header_src =
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509_vfy.h:32"]
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
    use super::_time_t_h::time_t;
    use super::asn1_h::stack_st_ASN1_OBJECT;
    use super::stack_h::_STACK;
    extern "C" {
        #[src_loc = "159:9"]
        pub type X509_VERIFY_PARAM_ID_st;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/crypto.h:32"]
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
    use super::ossl_typ_h::CRYPTO_EX_DATA;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the CRYPTO functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509.h:32"]
pub mod x509_h {
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
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "175:1"]
    pub struct stack_st_X509_NAME_ENTRY {
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
    use super::ossl_typ_h::{ASN1_INTEGER, X509_ALGOR, X509_NAME, ASN1_TIME,
                            ASN1_UTF8STRING, ASN1_OCTET_STRING, X509_PUBKEY,
                            ASN1_BIT_STRING};
    use super::asn1_h::{ASN1_ENCODING, stack_st_ASN1_OBJECT,
                        stack_st_X509_ALGOR};
    use super::stack_h::_STACK;
    extern "C" {
        #[src_loc = "471:5"]
        pub type stack_st_GENERAL_NAMES;
        #[src_loc = "289:5"]
        pub type stack_st_GENERAL_NAME;
        #[src_loc = "288:5"]
        pub type stack_st_DIST_POINT;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the X509 functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/evp.h:32"]
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
    use super::ossl_typ_h::{rsa_st, dsa_st, dh_st};
    extern "C" {
        #[src_loc = "147:9"]
        pub type ec_key_st;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the EVP functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bn.h:32"]
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
    use super::ossl_typ_h::BN_GENCB;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the BN functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dsa.h:32"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pem.h:32"]
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
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the PEM functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/comp.h:32"]
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
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the COMP functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dtls1.h:32"]
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
    use super::ossl_typ_h::{EVP_CIPHER_CTX, EVP_MD_CTX};
    use super::comp_h::COMP_CTX;
    use super::ssl_h::SSL_SESSION;
    use super::pqueue_h::pqueue;
    /* Timeout multipliers (timeout slice is defined in apps/timeouts.h */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pqueue.h:32"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl3.h:32"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ec.h:32"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bio.h:32"]
pub mod bio_h {
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
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the BIO functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl2.h:32"]
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
    /* server */
    /* SSLv2 */
/* client */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zlib.h:32"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zconf.h:32"]
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
#[header_src = "/usr/local/Cellar/stoken/0.92/include/stoken.h:32"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/gssapi/gssapi.h:32"]
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
#[header_src = "/usr/local/Cellar/libproxy/0.4.15_1/include/proxy.h:32"]
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
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/tree.h:32"]
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
    use super::xmlstring_h::xmlChar;
    use super::dict_h::_xmlDict;
    /* __XML_TREE_H__ */
}
#[header_src =
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/dict.h:32"]
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
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/xmlstring.h:32"]
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
    /* __XML_STRING_H__ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/iconv.h:32"]
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
    /* _LIBICONV_H */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:23"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[src_loc = "72:7"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "73:7"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "76:7"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[src_loc = "117:7"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[src_loc = "132:7"]
        pub fn strndup(_: *const libc::c_char, _: libc::c_ulong)
         -> *mut libc::c_char;
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/strings.h:23"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[src_loc = "79:6"]
        pub fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                           _: libc::c_ulong) -> libc::c_int;
    }
    /* _STRINGS_H_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:24"]
pub mod _ctype_h {
    #[inline]
    #[src_loc = "134:1"]
    pub unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
        return (_c & !0x7fi32 == 0i32) as libc::c_int;
    }
    /* USE_ASCII */
    #[inline]
    #[src_loc = "152:1"]
    pub unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t,
                                      mut _f: libc::c_ulong) -> libc::c_int {
        /* USE_ASCII */
        return if isascii(_c) != 0 {
                   (_DefaultRuneLocale.__runetype[_c as usize] as
                        libc::c_ulong & _f != 0) as libc::c_int
               } else { (__maskrune(_c, _f) != 0) as libc::c_int };
        /* USE_ASCII */
    }
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[src_loc = "254:1"]
    pub unsafe extern "C" fn isprint(mut _c: libc::c_int) -> libc::c_int {
        return __istype(_c, 0x40000i64 as libc::c_ulong);
    }
    use super::_types_h::{__darwin_ct_rune_t, __uint32_t};
    use super::runetype_h::_DefaultRuneLocale;
    extern "C" {
        /* !USE_ASCII */
        #[no_mangle]
        #[src_loc = "148:1"]
        pub fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong)
         -> libc::c_int;
    }
    /* !_CTYPE_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* using inlines */
    /* not using inlines */
    /* _EXTERNALIZE_CTYPE_INLINES_ */
    /* !_ANSI_SOURCE && (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/libkern/i386/_OSByteOrder.h:26"]
pub mod _OSByteOrder_h {
    /*
 * Copyright (c) 2006-2012 Apple Inc. All rights reserved.
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
    /* Generic byte swapping functions. */
    #[inline]
    #[src_loc = "44:1"]
    pub unsafe extern "C" fn _OSSwapInt16(mut _data: __uint16_t)
     -> __uint16_t {
        return ((_data as libc::c_int) << 8i32 | _data as libc::c_int >> 8i32)
                   as __uint16_t;
    }
    #[inline]
    #[src_loc = "53:1"]
    pub unsafe extern "C" fn _OSSwapInt32(mut _data: __uint32_t)
     -> __uint32_t {
        return _data.swap_bytes();
    }
    use super::_types_h::{__uint16_t, __uint32_t};
    /* ! _OS__OSBYTEORDERI386_H */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:26"]
pub mod _malloc_h {
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
        #[src_loc = "41:7"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "42:1"]
        pub fn free(_: *mut libc::c_void);
    }
    /* _MALLOC_UNDERSCORE_MALLOC_H_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:27"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[src_loc = "180:6"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        /* __DARWIN_C_LEVEL >= 200112L */
        #[no_mangle]
        #[src_loc = "334:6"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
    /* _STDIO_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arpa/inet.h:32"]
pub mod inet_h {
    use super::_socklen_t_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "77:1"]
        pub fn inet_ntop(_: libc::c_int, _: *const libc::c_void,
                         _: *mut libc::c_char, _: socklen_t)
         -> *const libc::c_char;
    }
    /* !_ARPA_INET_H_ */
    /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
}
#[header_src = "/usr/local/Cellar/gettext/0.20.1/include/libintl.h:32"]
pub mod libintl_h {
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
pub use self::_types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                         __darwin_ct_rune_t, __darwin_size_t,
                         __darwin_wchar_t, __darwin_rune_t,
                         __darwin_socklen_t, __darwin_time_t};
pub use self::sys__types_h::{__darwin_gid_t, __darwin_suseconds_t,
                             __darwin_uid_t};
pub use self::_size_t_h::size_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uid_t_h::uid_t;
pub use self::_gid_t_h::gid_t;
pub use self::_fd_def_h::fd_set;
pub use self::_timeval_h::timeval;
pub use self::_time_t_h::time_t;
pub use self::time_h::{tm, strftime, time, localtime_r};
pub use self::runetype_h::{_RuneEntry, _RuneRange, _RuneCharClass,
                           _RuneLocale, _DefaultRuneLocale};
pub use self::_uint8_t_h::uint8_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::socket_h::{sockaddr, getsockname};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::in6_h::{in6_addr, C2RustUnnamed, sockaddr_in6};
pub use self::netdb_h::addrinfo;
pub use self::openconnect_h::{oc_form_opt, oc_choice, oc_form_opt_select,
                              oc_auth_form, oc_split_include, oc_ip_info,
                              oc_vpn_option, oc_stats, openconnect_info,
                              openconnect_reconnected_vfn,
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
pub use self::openconnect_internal_h::{pkt_q, pkt, C2RustUnnamed_0,
                                       C2RustUnnamed_1, C2RustUnnamed_2,
                                       C2RustUnnamed_3, C2RustUnnamed_4,
                                       C2RustUnnamed_5, keepalive_info,
                                       pin_cache, oc_text_buf,
                                       C2RustUnnamed_12, HOTP_SECRET_PSKC,
                                       HOTP_SECRET_HEX, HOTP_SECRET_RAW,
                                       HOTP_SECRET_BASE32, C2RustUnnamed_13,
                                       OATH_ALG_HMAC_SHA512,
                                       OATH_ALG_HMAC_SHA256,
                                       OATH_ALG_HMAC_SHA1, http_auth_state,
                                       C2RustUnnamed_14, C2RustUnnamed_15,
                                       C2RustUnnamed_16, esp, vpn_proto,
                                       oc_packed_uint16_t, oc_packed_uint32_t,
                                       dequeue_packet, queue_packet,
                                       load_be16, load_be32, store_be32,
                                       store_be16, store_le32, load_le32,
                                       oc_pcsc_ctx, establish_eap_ttls,
                                       destroy_eap_ttls, buf_free,
                                       openconnect_close_https, dump_buf_hex,
                                       buf_truncate, buf_error,
                                       buf_append_bytes, buf_alloc,
                                       process_auth_form, buf_append,
                                       free_pass, do_gen_tokencode,
                                       can_gen_tokencode,
                                       get_cert_md5_fingerprint,
                                       process_http_response, dump_buf,
                                       http_common_headers,
                                       openconnect_open_https,
                                       openconnect_setup_esp_keys,
                                       udp_sockaddr, ssl_reconnect,
                                       esp_shutdown, ssl_nonblock_write,
                                       print_esp_keys, ssl_nonblock_read};
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
                           AUTHORITY_KEYID, X509_ALGOR, X509_algor_st,
                           ASN1_TIME, X509_NAME, X509_name_st, BUF_MEM,
                           buf_mem_st, X509, x509_st, NAME_CONSTRAINTS,
                           X509_POLICY_CACHE, X509_PUBKEY, X509_pubkey_st,
                           EVP_PKEY, evp_pkey_st, dh_st, DH_METHOD, dh_method,
                           BN_GENCB, bn_gencb_st, DH, BN_MONT_CTX,
                           bn_mont_ctx_st, BN_CTX, dsa_st, DSA_METHOD,
                           dsa_method, DSA, rsa_st, BN_BLINDING, RSA_METHOD,
                           rsa_meth_st, RSA, EVP_PKEY_ASN1_METHOD,
                           X509_POLICY_TREE, X509_STORE, x509_store_st,
                           COMP_METHOD, comp_method_st, evp_pkey_ctx_st,
                           engine_st, x509_crl_method_st,
                           ISSUING_DIST_POINT_st, AUTHORITY_KEYID_st,
                           NAME_CONSTRAINTS_st, X509_POLICY_CACHE_st,
                           bignum_ctx, bn_blinding_st,
                           evp_pkey_asn1_method_st, X509_POLICY_TREE_st};
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
                      ssl3_enc_method, stack_st_OCSP_RESPID, SSL_read,
                      SSL_write};
pub use self::stack_h::{_STACK, stack_st};
pub use self::hmac_h::{HMAC_CTX, hmac_ctx_st};
pub use self::asn1_h::{ASN1_TYPE, asn1_type_st, C2RustUnnamed_6, ASN1_VALUE,
                       stack_st_ASN1_OBJECT, ASN1_ENCODING, ASN1_ENCODING_st,
                       stack_st_X509_ALGOR, ASN1_VALUE_st};
pub use self::x509_vfy_h::{X509_VERIFY_PARAM, X509_VERIFY_PARAM_st,
                           X509_VERIFY_PARAM_ID, stack_st_X509_LOOKUP,
                           stack_st_X509_OBJECT, X509_VERIFY_PARAM_ID_st};
pub use self::crypto_h::{stack_st_void, bio_st};
pub use self::x509_h::{X509_CRL_INFO, X509_crl_info_st,
                       stack_st_X509_EXTENSION, stack_st_X509_REVOKED,
                       stack_st_X509_NAME_ENTRY, X509_CERT_AUX,
                       x509_cert_aux_st, X509_CINF, x509_cinf_st, X509_VAL,
                       X509_val_st, stack_st_X509, stack_st_X509_CRL,
                       stack_st_X509_NAME, X509_EXTENSIONS,
                       stack_st_GENERAL_NAMES, stack_st_GENERAL_NAME,
                       stack_st_DIST_POINT};
pub use self::evp_h::{stack_st_X509_ATTRIBUTE, C2RustUnnamed_7, ec_key_st};
pub use self::bn_h::C2RustUnnamed_8;
pub use self::dsa_h::{DSA_SIG, DSA_SIG_st};
pub use self::pem_h::pem_password_cb;
pub use self::comp_h::{COMP_CTX, comp_ctx_st};
pub use self::dtls1_h::{dtls1_timeout_st, hm_header_st,
                        dtls1_retransmit_state, record_pqueue,
                        record_pqueue_st, DTLS1_BITMAP, dtls1_bitmap_st};
pub use self::pqueue_h::{pqueue, _pqueue};
pub use self::ssl3_h::{C2RustUnnamed_10, SSL3_RECORD, ssl3_record_st,
                       SSL3_BUFFER, ssl3_buffer_st};
pub use self::ec_h::EC_KEY;
pub use self::bio_h::{BIO, BIO_METHOD, bio_method_st, bio_info_cb};
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
use self::string_h::{memcpy, memmove, memset, strchr, strlen, strdup,
                     strndup};
use self::strings_h::strncasecmp;
pub use self::_ctype_h::{isascii, __istype, isprint, __maskrune};
pub use self::_OSByteOrder_h::{_OSSwapInt16, _OSSwapInt32};
use self::_malloc_h::{malloc, calloc, free};
use self::stdio_h::{sprintf, snprintf};
use self::inet_h::inet_ntop;
use self::libintl_h::libintl_dgettext;
#[src_loc = "82:1"]
unsafe extern "C" fn buf_append_be16(mut buf: *mut oc_text_buf,
                                     mut val: uint16_t) {
    let mut b: [libc::c_uchar; 2] =
        [0; 2]; /* Length will be filled in later. */
    store_be16(b.as_mut_ptr() as *mut libc::c_void, val);
    buf_append_bytes(buf, b.as_mut_ptr() as *const libc::c_void, 2i32);
}
#[src_loc = "91:1"]
unsafe extern "C" fn buf_append_be32(mut buf: *mut oc_text_buf,
                                     mut val: uint32_t) {
    let mut b: [libc::c_uchar; 4] = [0; 4];
    store_be32(b.as_mut_ptr() as *mut libc::c_void, val);
    buf_append_bytes(buf, b.as_mut_ptr() as *const libc::c_void, 4i32);
}
#[src_loc = "100:1"]
unsafe extern "C" fn buf_append_ift_hdr(mut buf: *mut oc_text_buf,
                                        mut vendor: uint32_t,
                                        mut type_0: uint32_t) {
    let mut b: [uint32_t; 4] = [0; 4];
    store_be32(&mut *b.as_mut_ptr().offset(0) as *mut uint32_t as
                   *mut libc::c_void, vendor);
    store_be32(&mut *b.as_mut_ptr().offset(1) as *mut uint32_t as
                   *mut libc::c_void, type_0);
    b[2] = 0i32 as uint32_t;
    b[3] = 0i32 as uint32_t;
    buf_append_bytes(buf, b.as_mut_ptr() as *const libc::c_void, 16i32);
}
/* Append EAP header, using VENDOR_JUNIPER and the given subtype if
 * the main type is EAP_TYPE_EXPANDED */
#[src_loc = "113:1"]
unsafe extern "C" fn buf_append_eap_hdr(mut buf: *mut oc_text_buf,
                                        mut code: uint8_t, mut ident: uint8_t,
                                        mut type_0: uint8_t,
                                        mut subtype: uint32_t)
 -> libc::c_int {
    let mut b: [libc::c_uchar; 24] = [0; 24]; /* Length is filled in later. */
    let mut len_ofs: libc::c_int = -1i32;
    if buf_error(buf) == 0 { len_ofs = (*buf).pos }
    b[0] = code;
    b[1] = ident;
    b[3] = 0i32 as libc::c_uchar;
    b[2] = b[3];
    if type_0 as libc::c_int == 0xfei32 {
        store_be32(b.as_mut_ptr().offset(4) as *mut libc::c_void,
                   (0xfei32 << 24i32 | 0xa4ci32) as uint32_t);
        store_be32(b.as_mut_ptr().offset(8) as *mut libc::c_void, subtype);
        buf_append_bytes(buf, b.as_mut_ptr() as *const libc::c_void, 12i32);
    } else {
        b[4] = type_0;
        buf_append_bytes(buf, b.as_mut_ptr() as *const libc::c_void, 5i32);
    }
    return len_ofs;
}
/* For an IF-T/TLS auth frame containing the Juniper/1 Auth Type,
 * the EAP header is at offset 0x14. Fill in the length field,
 * based on the current length of the buf */
#[src_loc = "139:1"]
unsafe extern "C" fn buf_fill_eap_len(mut buf: *mut oc_text_buf,
                                      mut ofs: libc::c_int) {
    /* EAP length word is always at 0x16, and counts bytes from 0x14 */
    if ofs >= 0i32 && buf_error(buf) == 0 && (*buf).pos > ofs + 8i32 {
        store_be16((*buf).data.offset(ofs as isize).offset(2) as
                       *mut libc::c_void, ((*buf).pos - ofs) as uint16_t);
    };
}
#[src_loc = "146:1"]
unsafe extern "C" fn buf_append_avp(mut buf: *mut oc_text_buf,
                                    mut type_0: uint32_t,
                                    mut bytes: *const libc::c_void,
                                    mut len: libc::c_int) {
    buf_append_be32(buf, type_0);
    buf_append_be16(buf, 0x8000i32 as uint16_t);
    buf_append_be16(buf, (len + 12i32) as uint16_t);
    buf_append_be32(buf, 0x583i32 as uint32_t);
    buf_append_bytes(buf, bytes, len);
    if len & 3i32 != 0 {
        let mut pad: uint32_t = 0i32 as uint32_t;
        buf_append_bytes(buf,
                         &mut pad as *mut uint32_t as *const libc::c_void,
                         4i32 - (len & 3i32));
    };
}
#[src_loc = "159:1"]
unsafe extern "C" fn buf_append_avp_string(mut buf: *mut oc_text_buf,
                                           mut type_0: uint32_t,
                                           mut str: *const libc::c_char) {
    buf_append_avp(buf, type_0, str as *const libc::c_void,
                   strlen(str) as libc::c_int);
}
#[src_loc = "164:1"]
unsafe extern "C" fn buf_append_avp_be32(mut buf: *mut oc_text_buf,
                                         mut type_0: uint32_t,
                                         mut val: uint32_t) {
    let mut val_be: uint32_t = 0;
    store_be32(&mut val_be as *mut uint32_t as *mut libc::c_void, val);
    buf_append_avp(buf, type_0,
                   &mut val_be as *mut uint32_t as *const libc::c_void,
                   ::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       libc::c_int);
}
#[src_loc = "172:1"]
unsafe extern "C" fn valid_ift_success(mut bytes: *mut libc::c_uchar,
                                       mut len: libc::c_int) -> libc::c_int {
    if len != 0x18i32 ||
           load_be32(bytes as *const libc::c_void) &
               0xffffffi32 as libc::c_uint != 0x5597i32 as libc::c_uint ||
           load_be32(bytes.offset(4) as *const libc::c_void) !=
               7i32 as libc::c_uint ||
           load_be32(bytes.offset(8) as *const libc::c_void) !=
               len as libc::c_uint ||
           load_be32(bytes.offset(0x10i32 as isize) as *const libc::c_void) !=
               (0xa4ci32 << 8i32 | 1i32) as libc::c_uint ||
           *bytes.offset(0x14i32 as isize) as libc::c_int != 3i32 ||
           load_be16(bytes.offset(0x16i32 as isize) as *const libc::c_void) as
               libc::c_int != len - 0x14i32 {
        return 0i32
    }
    return 1i32;
}
/* Check for a valid IF-T/TLS auth challenge of the Juniper/1 Auth Type */
#[src_loc = "186:1"]
unsafe extern "C" fn valid_ift_auth(mut bytes: *mut libc::c_uchar,
                                    mut len: libc::c_int) -> libc::c_int {
    if len < 0x14i32 ||
           load_be32(bytes as *const libc::c_void) &
               0xffffffi32 as libc::c_uint != 0x5597i32 as libc::c_uint ||
           load_be32(bytes.offset(4) as *const libc::c_void) !=
               5i32 as libc::c_uint ||
           load_be32(bytes.offset(8) as *const libc::c_void) !=
               len as libc::c_uint ||
           load_be32(bytes.offset(0x10i32 as isize) as *const libc::c_void) !=
               (0xa4ci32 << 8i32 | 1i32) as libc::c_uint {
        return 0i32
    }
    return 1i32;
}
#[src_loc = "198:1"]
unsafe extern "C" fn valid_ift_auth_eap(mut bytes: *mut libc::c_uchar,
                                        mut len: libc::c_int) -> libc::c_int {
    /* Needs to be a valid IF-T/TLS auth challenge with the
	 * expect Auth Type, *and* the payload has to be a valid
	 * EAP request with correct length field. */
    if valid_ift_auth(bytes, len) == 0 || len < 0x19i32 ||
           *bytes.offset(0x14i32 as isize) as libc::c_int != 1i32 ||
           load_be16(bytes.offset(0x16i32 as isize) as *const libc::c_void) as
               libc::c_int != len - 0x14i32 {
        return 0i32
    }
    return 1i32;
}
#[src_loc = "211:1"]
unsafe extern "C" fn valid_ift_auth_eap_exj1(mut bytes: *mut libc::c_uchar,
                                             mut len: libc::c_int)
 -> libc::c_int {
    /* Also needs to be the Expanded Juniper/1 EAP Type */
    if valid_ift_auth_eap(bytes, len) == 0 || len < 0x20i32 ||
           load_be32(bytes.offset(0x18i32 as isize) as *const libc::c_void) !=
               (0xfei32 << 24i32 | 0xa4ci32) as libc::c_uint ||
           load_be32(bytes.offset(0x1ci32 as isize) as *const libc::c_void) !=
               1i32 as libc::c_uint {
        return 0i32
    }
    return 1i32;
}
/* We behave like CSTP — create a linked list in vpninfo->cstp_options
 * with the strings containing the information we got from the server,
 * and oc_ip_info contains const copies of those pointers. */
#[src_loc = "226:1"]
unsafe extern "C" fn add_option(mut vpninfo: *mut openconnect_info,
                                mut opt: *const libc::c_char,
                                mut val: *const libc::c_char,
                                mut val_len: libc::c_int)
 -> *const libc::c_char {
    let mut new: *mut oc_vpn_option =
        malloc(::std::mem::size_of::<oc_vpn_option>() as libc::c_ulong) as
            *mut oc_vpn_option;
    if new.is_null() { return 0 as *const libc::c_char }
    (*new).option = strdup(opt);
    if (*new).option.is_null() {
        free(new as *mut libc::c_void);
        return 0 as *const libc::c_char
    }
    if val_len >= 0i32 {
        (*new).value = strndup(val, val_len as libc::c_ulong)
    } else { (*new).value = strdup(val) }
    if (*new).value.is_null() {
        free((*new).option as *mut libc::c_void);
        free(new as *mut libc::c_void);
        return 0 as *const libc::c_char
    }
    (*new).next = (*vpninfo).cstp_options;
    (*vpninfo).cstp_options = new;
    return (*new).value;
}
#[src_loc = "253:1"]
unsafe extern "C" fn process_attr(mut vpninfo: *mut openconnect_info,
                                  mut type_0: uint16_t,
                                  mut data: *mut libc::c_uchar,
                                  mut attrlen: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut xc: *mut oc_split_include = 0 as *mut oc_split_include;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut i: libc::c_int = 0;
    match type_0 as libc::c_int {
        1 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 80]>() as
                             libc::c_ulong,
                         b"%d.%d.%d.%d\x00" as *const u8 as
                             *const libc::c_char,
                         *data.offset(0) as libc::c_int,
                         *data.offset(1) as libc::c_int,
                         *data.offset(2) as libc::c_int,
                         *data.offset(3) as libc::c_int);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received internal Legacy IP address %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                (*vpninfo).ip_info.addr =
                    add_option(vpninfo,
                               b"ipaddr\x00" as *const u8 as
                                   *const libc::c_char, buf.as_mut_ptr(),
                               -1i32);
                current_block = 1131722263116153860;
            }
        }
        2 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 80]>() as
                             libc::c_ulong,
                         b"%d.%d.%d.%d\x00" as *const u8 as
                             *const libc::c_char,
                         *data.offset(0) as libc::c_int,
                         *data.offset(1) as libc::c_int,
                         *data.offset(2) as libc::c_int,
                         *data.offset(3) as libc::c_int);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received netmask %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                (*vpninfo).ip_info.netmask =
                    add_option(vpninfo,
                               b"netmask\x00" as *const u8 as
                                   *const libc::c_char, buf.as_mut_ptr(),
                               -1i32);
                current_block = 1131722263116153860;
            }
        }
        3 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 80]>() as
                             libc::c_ulong,
                         b"%d.%d.%d.%d\x00" as *const u8 as
                             *const libc::c_char,
                         *data.offset(0) as libc::c_int,
                         *data.offset(1) as libc::c_int,
                         *data.offset(2) as libc::c_int,
                         *data.offset(3) as libc::c_int);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received DNS server %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                i = 0i32;
                while i < 3i32 {
                    if (*vpninfo).ip_info.dns[i as usize].is_null() {
                        (*vpninfo).ip_info.dns[i as usize] =
                            add_option(vpninfo,
                                       b"DNS\x00" as *const u8 as
                                           *const libc::c_char,
                                       buf.as_mut_ptr(), -1i32);
                        break ;
                    } else { i += 1 }
                }
                current_block = 1131722263116153860;
            }
        }
        4 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 80]>() as
                             libc::c_ulong,
                         b"%d.%d.%d.%d\x00" as *const u8 as
                             *const libc::c_char,
                         *data.offset(0) as libc::c_int,
                         *data.offset(1) as libc::c_int,
                         *data.offset(2) as libc::c_int,
                         *data.offset(3) as libc::c_int);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received WINS server %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                i = 0i32;
                while i < 3i32 {
                    if (*vpninfo).ip_info.nbns[i as usize].is_null() {
                        (*vpninfo).ip_info.nbns[i as usize] =
                            add_option(vpninfo,
                                       b"WINS\x00" as *const u8 as
                                           *const libc::c_char,
                                       buf.as_mut_ptr(), -1i32);
                        break ;
                    } else { i += 1 }
                }
                current_block = 1131722263116153860;
            }
        }
        8 => {
            if attrlen != 17i32 {
                current_block = 16406841518920604577;
            } else {
                if inet_ntop(30i32, data as *const libc::c_void,
                             buf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 80]>() as
                                 libc::c_ulong as socklen_t).is_null() {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Failed to handle IPv6 address\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                    return -22i32
                }
                (*vpninfo).ip_info.addr6 =
                    add_option(vpninfo,
                               b"ip6addr\x00" as *const u8 as
                                   *const libc::c_char, buf.as_mut_ptr(),
                               -1i32);
                i = strlen(buf.as_mut_ptr()) as libc::c_int;
                snprintf(buf.as_mut_ptr().offset(i as isize),
                         (::std::mem::size_of::<[libc::c_char; 80]>() as
                              libc::c_ulong).wrapping_sub(i as libc::c_ulong),
                         b"/%d\x00" as *const u8 as *const libc::c_char,
                         *data.offset(16) as libc::c_int);
                (*vpninfo).ip_info.netmask6 =
                    add_option(vpninfo,
                               b"ip6netmask\x00" as *const u8 as
                                   *const libc::c_char, buf.as_mut_ptr(),
                               -1i32);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received internal IPv6 address %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                current_block = 1131722263116153860;
            }
        }
        10 => {
            if attrlen != 16i32 {
                current_block = 16406841518920604577;
            } else {
                if inet_ntop(30i32, data as *const libc::c_void,
                             buf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 80]>() as
                                 libc::c_ulong as socklen_t).is_null() {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Failed to handle IPv6 address\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                    return -22i32
                }
                i = 0i32;
                while i < 3i32 {
                    if (*vpninfo).ip_info.dns[i as usize].is_null() {
                        (*vpninfo).ip_info.dns[i as usize] =
                            add_option(vpninfo,
                                       b"DNS\x00" as *const u8 as
                                           *const libc::c_char,
                                       buf.as_mut_ptr(), -1i32);
                        break ;
                    } else { i += 1 }
                }
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received DNS server %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                current_block = 1131722263116153860;
            }
        }
        15 => {
            if attrlen != 17i32 {
                current_block = 16406841518920604577;
            } else {
                if inet_ntop(30i32, data as *const libc::c_void,
                             buf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 80]>() as
                                 libc::c_ulong as socklen_t).is_null() {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Failed to handle IPv6 address\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                    return -22i32
                }
                i = strlen(buf.as_mut_ptr()) as libc::c_int;
                snprintf(buf.as_mut_ptr().offset(i as isize),
                         (::std::mem::size_of::<[libc::c_char; 80]>() as
                              libc::c_ulong).wrapping_sub(i as libc::c_ulong),
                         b"/%d\x00" as *const u8 as *const libc::c_char,
                         *data.offset(16) as libc::c_int);
                xc =
                    malloc(::std::mem::size_of::<oc_split_include>() as
                               libc::c_ulong) as *mut oc_split_include;
                if !xc.is_null() {
                    (*xc).route =
                        add_option(vpninfo,
                                   b"split-include6\x00" as *const u8 as
                                       *const libc::c_char, buf.as_mut_ptr(),
                                   -1i32);
                    if !(*xc).route.is_null() {
                        (*xc).next = (*vpninfo).ip_info.split_includes;
                        (*vpninfo).ip_info.split_includes = xc
                    } else { free(xc as *mut libc::c_void); }
                }
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received IPv6 split include %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                current_block = 1131722263116153860;
            }
        }
        16 => {
            if attrlen != 17i32 {
                current_block = 16406841518920604577;
            } else {
                if inet_ntop(30i32, data as *const libc::c_void,
                             buf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 80]>() as
                                 libc::c_ulong as socklen_t).is_null() {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Failed to handle IPv6 address\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                    return -22i32
                }
                i = strlen(buf.as_mut_ptr()) as libc::c_int;
                snprintf(buf.as_mut_ptr().offset(i as isize),
                         (::std::mem::size_of::<[libc::c_char; 80]>() as
                              libc::c_ulong).wrapping_sub(i as libc::c_ulong),
                         b"/%d\x00" as *const u8 as *const libc::c_char,
                         *data.offset(16) as libc::c_int);
                xc =
                    malloc(::std::mem::size_of::<oc_split_include>() as
                               libc::c_ulong) as *mut oc_split_include;
                if !xc.is_null() {
                    (*xc).route =
                        add_option(vpninfo,
                                   b"split-exclude6\x00" as *const u8 as
                                       *const libc::c_char, buf.as_mut_ptr(),
                                   -1i32);
                    if !(*xc).route.is_null() {
                        (*xc).next = (*vpninfo).ip_info.split_excludes;
                        (*vpninfo).ip_info.split_excludes = xc
                    } else { free(xc as *mut libc::c_void); }
                }
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received IPv6 split exclude %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                current_block = 1131722263116153860;
            }
        }
        16389 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                (*vpninfo).ip_info.mtu =
                    load_be32(data as *const libc::c_void) as libc::c_int;
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received MTU %d from server\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            (*vpninfo).ip_info.mtu);
                }
                current_block = 1131722263116153860;
            }
        }
        16390 => {
            if attrlen == 0 {
                current_block = 16406841518920604577;
            } else {
                if *data.offset((attrlen - 1i32) as isize) == 0 {
                    attrlen -= 1
                }
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received DNS search domain %.*s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            attrlen,
                                                                            data
                                                                                as
                                                                                *mut libc::c_char);
                }
                (*vpninfo).ip_info.domain =
                    add_option(vpninfo,
                               b"search\x00" as *const u8 as
                                   *const libc::c_char,
                               data as *mut libc::c_char, attrlen);
                if !(*vpninfo).ip_info.domain.is_null() {
                    let mut p: *mut libc::c_char =
                        (*vpninfo).ip_info.domain as *mut libc::c_char;
                    loop  {
                        p = strchr(p, ',' as i32);
                        if p.is_null() { break ; }
                        *p = ' ' as i32 as libc::c_char
                    }
                }
                current_block = 1131722263116153860;
            }
        }
        16395 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 80]>() as
                             libc::c_ulong,
                         b"%d.%d.%d.%d\x00" as *const u8 as
                             *const libc::c_char,
                         *data.offset(0) as libc::c_int,
                         *data.offset(1) as libc::c_int,
                         *data.offset(2) as libc::c_int,
                         *data.offset(3) as libc::c_int);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received internal gateway address %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                /* Hm, what are we supposed to do with this? It's a tunnel;
		   having a gateway is meaningless. */
                add_option(vpninfo,
                           b"ipaddr\x00" as *const u8 as *const libc::c_char,
                           buf.as_mut_ptr(), -1i32);
                current_block = 1131722263116153860;
            }
        }
        16400 => {
            let mut enctype: *const libc::c_char = 0 as *const libc::c_char;
            let mut val: uint16_t = 0;
            if attrlen != 2i32 {
                current_block = 16406841518920604577;
            } else {
                val = load_be16(data as *const libc::c_void);
                if val as libc::c_int == 2i32 {
                    enctype =
                        b"AES-128\x00" as *const u8 as *const libc::c_char;
                    (*vpninfo).enc_key_len = 16i32
                } else if val as libc::c_int == 5i32 {
                    enctype =
                        b"AES-256\x00" as *const u8 as *const libc::c_char;
                    (*vpninfo).enc_key_len = 32i32
                } else {
                    enctype =
                        b"unknown\x00" as *const u8 as *const libc::c_char
                }
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"ESP encryption: 0x%04x (%s)\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            val
                                                                                as
                                                                                libc::c_int,
                                                                            enctype);
                }
                (*vpninfo).esp_enc = val as libc::c_uchar;
                current_block = 1131722263116153860;
            }
        }
        16401 => {
            let mut mactype: *const libc::c_char = 0 as *const libc::c_char;
            let mut val_0: uint16_t = 0;
            if attrlen != 2i32 {
                current_block = 16406841518920604577;
            } else {
                val_0 = load_be16(data as *const libc::c_void);
                if val_0 as libc::c_int == 1i32 {
                    mactype = b"MD5\x00" as *const u8 as *const libc::c_char;
                    (*vpninfo).hmac_key_len = 16i32
                } else if val_0 as libc::c_int == 2i32 {
                    mactype = b"SHA1\x00" as *const u8 as *const libc::c_char;
                    (*vpninfo).hmac_key_len = 20i32
                } else if val_0 as libc::c_int == 3i32 {
                    mactype =
                        b"SHA256\x00" as *const u8 as *const libc::c_char;
                    (*vpninfo).hmac_key_len = 32i32
                } else {
                    mactype =
                        b"unknown\x00" as *const u8 as *const libc::c_char
                }
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"ESP HMAC: 0x%04x (%s)\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            val_0
                                                                                as
                                                                                libc::c_int,
                                                                            mactype);
                }
                (*vpninfo).esp_hmac = val_0 as libc::c_uchar;
                current_block = 1131722263116153860;
            }
        }
        16402 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                (*vpninfo).esp_lifetime_seconds =
                    load_be32(data as *const libc::c_void);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"ESP key lifetime: %u seconds\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            (*vpninfo).esp_lifetime_seconds);
                }
                current_block = 1131722263116153860;
            }
        }
        16403 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                (*vpninfo).esp_lifetime_bytes =
                    load_be32(data as *const libc::c_void);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"ESP key lifetime: %u bytes\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            (*vpninfo).esp_lifetime_bytes);
                }
                current_block = 1131722263116153860;
            }
        }
        16404 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                (*vpninfo).esp_replay_protect =
                    load_be32(data as *const libc::c_void);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"ESP replay protection: %d\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            load_be32(data
                                                                                          as
                                                                                          *const libc::c_void));
                }
                current_block = 1131722263116153860;
            }
        }
        16406 => {
            if attrlen != 2i32 {
                current_block = 16406841518920604577;
            } else {
                i = load_be16(data as *const libc::c_void) as libc::c_int;
                udp_sockaddr(vpninfo, i);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"ESP port: %d\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            i);
                }
                current_block = 1131722263116153860;
            }
        }
        16407 => {
            if attrlen != 4i32 {
                current_block = 16406841518920604577;
            } else {
                (*vpninfo).esp_ssl_fallback =
                    load_be32(data as *const libc::c_void);
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"ESP to SSL fallback: %u seconds\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            (*vpninfo).esp_ssl_fallback);
                }
                current_block = 1131722263116153860;
            }
        }
        16410 => {
            if attrlen != 1i32 {
                current_block = 16406841518920604577;
            } else {
                /* Amusingly, this isn't enforced. It's client-only */
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"ESP only: %d\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            *data.offset(0)
                                                                                as
                                                                                libc::c_int);
                }
                current_block = 1131722263116153860;
            }
        }
        _ => {
            /* 0x4022: disable proxy
	   0x400a: preserve proxy
	   0x4008: proxy (string)
	   0x4000: disconnect when routes changed
	   0x4015: tos copy
	   0x4001:  tunnel routes take precedence
	   0x401f:  tunnel routes with subnet access (also 4001 set)
	   0x4020: Enforce IPv4
	   0x4021: Enforce IPv6
	   0x401e: Server IPv6 address
	   0x000f: IPv6 netmask?
	*/
            buf[0] = 0i32 as libc::c_char;
            i = 0i32;
            while i < 16i32 && i < attrlen {
                sprintf(buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as
                                                    isize),
                        b" %02x\x00" as *const u8 as *const libc::c_char,
                        *data.offset(i as isize) as libc::c_int);
                i += 1
            }
            if attrlen > 16i32 {
                sprintf(buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as
                                                    isize),
                        b"...\x00" as *const u8 as *const libc::c_char);
            }
            if (*vpninfo).verbose >= 2i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        2i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Unknown attr 0x%x len %d:%s\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        type_0
                                                                            as
                                                                            libc::c_int,
                                                                        attrlen,
                                                                        buf.as_mut_ptr());
            }
            current_block = 1131722263116153860;
        }
    }
    match current_block {
        1131722263116153860 => { return 0i32 }
        _ => {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Unexpected length %d for attr 0x%x\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        attrlen,
                                                                        type_0
                                                                            as
                                                                            libc::c_int);
            }
            return -22i32
        }
    };
}
#[src_loc = "569:1"]
unsafe extern "C" fn recv_ift_packet(mut vpninfo: *mut openconnect_info,
                                     mut buf: *mut libc::c_void,
                                     mut len: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int =
        (*vpninfo).ssl_read.expect("non-null function pointer")(vpninfo,
                                                                buf as
                                                                    *mut libc::c_char,
                                                                len as
                                                                    size_t);
    if ret > 0i32 && (*vpninfo).dump_http_traffic != 0 {
        if (*vpninfo).verbose >= 3i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    3i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Read %d bytes of IF-T/TLS record\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    ret);
        }
        dump_buf_hex(vpninfo, 3i32, '<' as i32 as libc::c_char,
                     buf as *mut libc::c_uchar, ret);
    }
    return ret;
}
#[src_loc = "580:1"]
unsafe extern "C" fn send_ift_bytes(mut vpninfo: *mut openconnect_info,
                                    mut bytes: *mut libc::c_void,
                                    mut len: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let fresh0 = (*vpninfo).ift_seq;
    (*vpninfo).ift_seq = (*vpninfo).ift_seq.wrapping_add(1);
    store_be32((bytes as *mut libc::c_char).offset(12) as *mut libc::c_void,
               fresh0);
    dump_buf_hex(vpninfo, 2i32, '>' as i32 as libc::c_char,
                 bytes as *mut libc::c_uchar, len);
    ret =
        (*vpninfo).ssl_write.expect("non-null function pointer")(vpninfo,
                                                                 bytes as
                                                                     *mut libc::c_char,
                                                                 len as
                                                                     size_t);
    if ret != len {
        if ret >= 0i32 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Short write to IF-T/TLS\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ret = -5i32
        }
        return ret
    }
    return 0i32;
}
#[src_loc = "600:1"]
unsafe extern "C" fn send_ift_packet(mut vpninfo: *mut openconnect_info,
                                     mut buf: *mut oc_text_buf)
 -> libc::c_int {
    if buf_error(buf) != 0 || (*buf).pos < 16i32 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Error creating IF-T packet\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        return buf_error(buf)
    }
    /* Fill in the length word in the header with the full length of the buffer.
	 * Also populate the sequence number. */
    store_be32((*buf).data.offset(8) as *mut libc::c_void,
               (*buf).pos as uint32_t);
    return send_ift_bytes(vpninfo, (*buf).data as *mut libc::c_void,
                          (*buf).pos);
}
/* We create packets with IF-T/TLS headers prepended because that's the
 * larger header. In the case where they need to be sent over EAP-TTLS,
 * convert the header to the EAP-Message AVP instead. */
#[src_loc = "618:1"]
unsafe extern "C" fn send_eap_packet(mut vpninfo: *mut openconnect_info,
                                     mut ttls: *mut libc::c_void,
                                     mut buf: *mut oc_text_buf)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if buf_error(buf) != 0 || (*buf).pos < 16i32 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Error creating EAP packet\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        return buf_error(buf)
    }
    if ttls.is_null() { return send_ift_packet(vpninfo, buf) }
    /* AVP EAP-Message header */
    store_be32((*buf).data.offset(0xci32 as isize) as *mut libc::c_void,
               79i32 as uint32_t);
    store_be32((*buf).data.offset(0x10i32 as isize) as *mut libc::c_void,
               ((*buf).pos - 0xci32) as uint32_t);
    dump_buf_hex(vpninfo, 2i32, '.' as i32 as libc::c_char,
                 (*buf).data.offset(0xci32 as isize) as *mut libc::c_void as
                     *mut libc::c_uchar, (*buf).pos - 0xci32);
    ret =
        SSL_write(ttls as *mut SSL,
                  (*buf).data.offset(0xci32 as isize) as *const libc::c_void,
                  (*buf).pos - 0xci32);
    if ret != (*buf).pos - 0xci32 { return -5i32 }
    return 0i32;
}
/*
 * Using the given buffer, receive and validate an EAP request of the
 * Expanded Juniper/1 type, either natively over IF-T/TLS or by EAP-TTLS
 * over IF-T/TLS. Return a pointer to the EAP header, with its length and
 * type already validated.
 */
#[src_loc = "648:1"]
unsafe extern "C" fn recv_eap_packet(mut vpninfo: *mut openconnect_info,
                                     mut ttls: *mut libc::c_void,
                                     mut buf: *mut libc::c_void,
                                     mut len: libc::c_int)
 -> *mut libc::c_void {
    let mut cbuf: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    let mut ret: libc::c_int = 0;
    if ttls.is_null() {
        ret = recv_ift_packet(vpninfo, buf, len);
        if ret < 0i32 { return 0 as *mut libc::c_void }
        if valid_ift_auth_eap_exj1(buf as *mut libc::c_uchar, ret) == 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Unexpected IF-T/TLS authentication challenge:\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            dump_buf_hex(vpninfo, 0i32, '<' as i32 as libc::c_char,
                         buf as *mut libc::c_uchar, ret);
            return 0 as *mut libc::c_void
        }
        return cbuf.offset(0x14i32 as isize) as *mut libc::c_void
    } else {
        ret = SSL_read(ttls as *mut SSL, buf, len);
        if ret <= 8i32 { return 0 as *mut libc::c_void }
        if load_be32(cbuf as *const libc::c_void) != 79i32 as libc::c_uint ||
               load_be32(cbuf.offset(0x4i32 as isize) as *const libc::c_void)
                   & !0x40000000i32 as libc::c_uint != ret as libc::c_uint ||
               *cbuf.offset(0x8i32 as isize) as libc::c_int != 1i32 ||
               load_be16(cbuf.offset(0xai32 as isize) as *const libc::c_void)
                   as libc::c_int != ret - 8i32 ||
               load_be32(cbuf.offset(0xci32 as isize) as *const libc::c_void)
                   != (0xfei32 << 24i32 | 0xa4ci32) as libc::c_uint ||
               load_be32(cbuf.offset(0x10i32 as isize) as *const libc::c_void)
                   != 1i32 as libc::c_uint {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Unexpected EAP-TTLS payload:\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            dump_buf_hex(vpninfo, 0i32, '<' as i32 as libc::c_char,
                         buf as *mut libc::c_uchar, ret);
            return 0 as *mut libc::c_void
        }
        return cbuf.offset(0x8i32 as isize) as *mut libc::c_void
    };
}
#[src_loc = "685:1"]
unsafe extern "C" fn dump_avp(mut vpninfo: *mut openconnect_info,
                              mut flags: uint8_t, mut vendor: uint32_t,
                              mut code: uint32_t, mut p: *mut libc::c_void,
                              mut len: libc::c_int) {
    let mut buf: *mut oc_text_buf = buf_alloc();
    let mut pretty: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < len {
        if isprint(*(p as *mut libc::c_char).offset(i as isize) as
                       libc::c_int) == 0 {
            break ;
        }
        i += 1
    }
    if i == len {
        buf_append(buf, b" \'\x00" as *const u8 as *const libc::c_char);
        buf_append_bytes(buf, p, len);
        buf_append(buf, b"\'\x00" as *const u8 as *const libc::c_char);
    } else {
        i = 0i32;
        while i < len {
            buf_append(buf, b" %02x\x00" as *const u8 as *const libc::c_char,
                       *(p as *mut libc::c_uchar).offset(i as isize) as
                           libc::c_int);
            i += 1
        }
    }
    if buf_error(buf) != 0 {
        pretty = b" <error>\x00" as *const u8 as *const libc::c_char
    } else { pretty = (*buf).data }
    if flags as libc::c_int & 0x80i32 != 0 {
        if (*vpninfo).verbose >= 3i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    3i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"AVP 0x%x/0x%x:%s\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    vendor,
                                                                    code,
                                                                    pretty);
        }
    } else if (*vpninfo).verbose >= 3i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                3i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"AVP %d:%s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                code, pretty);
    }
    buf_free(buf);
}
/* RFC5281 §10 */
#[src_loc = "717:1"]
unsafe extern "C" fn parse_avp(mut vpninfo: *mut openconnect_info,
                               mut pkt: *mut *mut libc::c_void,
                               mut pkt_len: *mut libc::c_int,
                               mut avp_out: *mut *mut libc::c_void,
                               mut avp_len: *mut libc::c_int,
                               mut avp_flags: *mut uint8_t,
                               mut avp_vendor: *mut uint32_t,
                               mut avp_code: *mut uint32_t) -> libc::c_int {
    let mut p: *mut libc::c_uchar = *pkt as *mut libc::c_uchar;
    let mut l: libc::c_int = *pkt_len;
    let mut code: uint32_t = 0;
    let mut len: uint32_t = 0;
    let mut vendor: uint32_t = 0i32 as uint32_t;
    let mut flags: uint8_t = 0;
    if l < 8i32 { return -22i32 }
    code = load_be32(p as *const libc::c_void);
    len =
        load_be32(p.offset(4) as *const libc::c_void) &
            0xffffffi32 as libc::c_uint;
    flags = *p.offset(4);
    if len > l as libc::c_uint || len < 8i32 as libc::c_uint { return -22i32 }
    p = p.offset(8);
    l -= 8i32;
    len =
        (len as libc::c_uint).wrapping_sub(8i32 as libc::c_uint) as uint32_t
            as uint32_t;
    /* Vendor field is optional. */
    if flags as libc::c_int & 0x80i32 != 0 {
        if l < 4i32 { return -22i32 }
        vendor = load_be32(p as *const libc::c_void);
        p = p.offset(4);
        l -= 4i32;
        len =
            (len as libc::c_uint).wrapping_sub(4i32 as libc::c_uint) as
                uint32_t as uint32_t
    }
    *avp_vendor = vendor;
    *avp_flags = flags;
    *avp_code = code;
    *avp_out = p as *mut libc::c_void;
    *avp_len = len as libc::c_int;
    /* Now set up packet pointer and length for next AVP,
	 * aligned to 4 octets (if they exist in the packet) */
    len =
        len.wrapping_add(3i32 as libc::c_uint) &
            !3i32 as libc::c_uint; /* Already validated */
    if len > l as libc::c_uint { len = l as uint32_t }
    *pkt = p.offset(len as isize) as *mut libc::c_void;
    *pkt_len = (l as libc::c_uint).wrapping_sub(len) as libc::c_int;
    return 0i32;
}
#[src_loc = "768:1"]
unsafe extern "C" fn pulse_request_realm_entry(mut vpninfo:
                                                   *mut openconnect_info,
                                               mut reqbuf: *mut oc_text_buf)
 -> libc::c_int {
    let mut f: oc_auth_form =
        oc_auth_form{banner: 0 as *mut libc::c_char,
                     message: 0 as *mut libc::c_char,
                     error: 0 as *mut libc::c_char,
                     auth_id: 0 as *mut libc::c_char,
                     method: 0 as *mut libc::c_char,
                     action: 0 as *mut libc::c_char,
                     opts: 0 as *mut oc_form_opt,
                     authgroup_opt: 0 as *mut oc_form_opt_select,
                     authgroup_selection: 0,};
    let mut o: oc_form_opt =
        oc_form_opt{next: 0 as *mut oc_form_opt,
                    type_0: 0,
                    name: 0 as *mut libc::c_char,
                    label: 0 as *mut libc::c_char,
                    _value: 0 as *mut libc::c_char,
                    flags: 0,
                    reserved: 0 as *mut libc::c_void,};
    let mut ret: libc::c_int = 0;
    memset(&mut f as *mut oc_auth_form as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<oc_auth_form>() as libc::c_ulong);
    memset(&mut o as *mut oc_form_opt as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<oc_form_opt>() as libc::c_ulong);
    f.auth_id =
        b"pulse_realm_entry\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    f.opts = &mut o;
    f.message =
        libintl_dgettext(b"openconnect\x00" as *const u8 as
                             *const libc::c_char,
                         b"Enter Pulse user realm:\x00" as *const u8 as
                             *const libc::c_char);
    o.next = 0 as *mut oc_form_opt;
    o.type_0 = 1i32;
    o.name =
        b"realm\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    o.label =
        libintl_dgettext(b"openconnect\x00" as *const u8 as
                             *const libc::c_char,
                         b"Realm:\x00" as *const u8 as *const libc::c_char);
    ret = process_auth_form(vpninfo, &mut f);
    if ret != 0 { return ret }
    if !o._value.is_null() {
        buf_append_avp_string(reqbuf, 0xd50i32 as uint32_t, o._value);
        free_pass(&mut o._value);
        return 0i32
    }
    return -22i32;
}
#[src_loc = "799:1"]
unsafe extern "C" fn pulse_request_realm_choice(mut vpninfo:
                                                    *mut openconnect_info,
                                                mut reqbuf: *mut oc_text_buf,
                                                mut realms: libc::c_int,
                                                mut eap: *mut libc::c_uchar)
 -> libc::c_int {
    let mut current_block: u64;
    let mut avp_flags: uint8_t = 0;
    let mut avp_code: uint32_t = 0;
    let mut avp_vendor: uint32_t = 0;
    let mut avp_len: libc::c_int = 0;
    let mut avp_p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut f: oc_auth_form =
        oc_auth_form{banner: 0 as *mut libc::c_char,
                     message: 0 as *mut libc::c_char,
                     error: 0 as *mut libc::c_char,
                     auth_id: 0 as *mut libc::c_char,
                     method: 0 as *mut libc::c_char,
                     action: 0 as *mut libc::c_char,
                     opts: 0 as *mut oc_form_opt,
                     authgroup_opt: 0 as *mut oc_form_opt_select,
                     authgroup_selection: 0,};
    let mut o: oc_form_opt_select =
        oc_form_opt_select{form:
                               oc_form_opt{next: 0 as *mut oc_form_opt,
                                           type_0: 0,
                                           name: 0 as *mut libc::c_char,
                                           label: 0 as *mut libc::c_char,
                                           _value: 0 as *mut libc::c_char,
                                           flags: 0,
                                           reserved: 0 as *mut libc::c_void,},
                           nr_choices: 0,
                           choices: 0 as *mut *mut oc_choice,};
    let mut i: libc::c_int = 0i32;
    let mut ret: libc::c_int = 0;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut l: libc::c_int = 0;
    l =
        load_be16(eap.offset(2) as *const libc::c_void) as libc::c_int -
            0xci32;
    p = eap.offset(0xci32 as isize) as *mut libc::c_void;
    memset(&mut f as *mut oc_auth_form as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<oc_auth_form>() as libc::c_ulong);
    memset(&mut o as *mut oc_form_opt_select as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<oc_form_opt_select>() as libc::c_ulong);
    f.auth_id =
        b"pulse_realm_choice\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    f.opts = &mut o.form;
    f.authgroup_opt = &mut o;
    f.authgroup_selection = 1i32;
    f.message =
        libintl_dgettext(b"openconnect\x00" as *const u8 as
                             *const libc::c_char,
                         b"Choose Pulse user realm:\x00" as *const u8 as
                             *const libc::c_char);
    o.form.next = 0 as *mut oc_form_opt;
    o.form.type_0 = 3i32;
    o.form.name =
        b"realm_choice\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    o.form.label =
        libintl_dgettext(b"openconnect\x00" as *const u8 as
                             *const libc::c_char,
                         b"Realm:\x00" as *const u8 as *const libc::c_char);
    o.nr_choices = realms;
    o.choices =
        calloc(realms as libc::c_ulong,
               ::std::mem::size_of::<*mut oc_choice>() as libc::c_ulong) as
            *mut *mut oc_choice;
    if o.choices.is_null() { return -12i32 }
    loop  {
        if !(l != 0) { current_block = 6450636197030046351; break ; }
        if parse_avp(vpninfo, &mut p, &mut l, &mut avp_p, &mut avp_len,
                     &mut avp_flags, &mut avp_vendor, &mut avp_code) != 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Failed to parse AVP\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ret = -22i32;
            current_block = 9314030793429475878;
            break ;
        } else {
            if avp_vendor != 0x583i32 as libc::c_uint ||
                   avp_code != 0xd4ei32 as libc::c_uint {
                continue ;
            }
            let ref mut fresh1 = *o.choices.offset(i as isize);
            *fresh1 =
                malloc(::std::mem::size_of::<oc_choice>() as libc::c_ulong) as
                    *mut oc_choice;
            if (*o.choices.offset(i as isize)).is_null() {
                ret = -12i32;
                current_block = 9314030793429475878;
                break ;
            } else {
                let ref mut fresh2 = (**o.choices.offset(i as isize)).label;
                *fresh2 =
                    strndup(avp_p as *const libc::c_char,
                            avp_len as libc::c_ulong);
                let ref mut fresh3 = (**o.choices.offset(i as isize)).name;
                *fresh3 = *fresh2;
                if (**o.choices.offset(i as isize)).name.is_null() {
                    ret = -12i32;
                    current_block = 9314030793429475878;
                    break ;
                } else { i += 1 }
            }
        }
    }
    match current_block {
        6450636197030046351 => {
            loop 
                 /* We don't need to do anything on group changes. */
                 {
                ret =
                    process_auth_form(vpninfo,
                                      &mut f); /* Already validated */
                if !(ret == 2i32) {
                    break
                        ; /* Point to password prompt in case that's all we use */
                }
            } /* Again, for now */
            if ret == 0 {
                buf_append_avp_string(reqbuf, 0xd50i32 as uint32_t,
                                      o.form._value);
            }
        }
        _ => { }
    }
    if !o.choices.is_null() {
        i = 0i32;
        while i < realms {
            if !(*o.choices.offset(i as isize)).is_null() {
                free((**o.choices.offset(i as isize)).name as
                         *mut libc::c_void);
                free(*o.choices.offset(i as isize) as *mut libc::c_void);
            }
            i += 1
        }
        free(o.choices as *mut libc::c_void);
    }
    return ret;
}
#[src_loc = "880:1"]
unsafe extern "C" fn pulse_request_session_kill(mut vpninfo:
                                                    *mut openconnect_info,
                                                mut reqbuf: *mut oc_text_buf,
                                                mut sessions: libc::c_int,
                                                mut eap: *mut libc::c_uchar)
 -> libc::c_int {
    let mut current_block: u64;
    let mut avp_flags: uint8_t = 0;
    let mut avp_code: uint32_t = 0;
    let mut avp_vendor: uint32_t = 0;
    let mut avp_len: libc::c_int = 0;
    let mut avp_len2: libc::c_int = 0;
    let mut avp_p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut avp_p2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut f: oc_auth_form =
        oc_auth_form{banner: 0 as *mut libc::c_char,
                     message: 0 as *mut libc::c_char,
                     error: 0 as *mut libc::c_char,
                     auth_id: 0 as *mut libc::c_char,
                     method: 0 as *mut libc::c_char,
                     action: 0 as *mut libc::c_char,
                     opts: 0 as *mut oc_form_opt,
                     authgroup_opt: 0 as *mut oc_form_opt_select,
                     authgroup_selection: 0,};
    let mut o: oc_form_opt_select =
        oc_form_opt_select{form:
                               oc_form_opt{next: 0 as *mut oc_form_opt,
                                           type_0: 0,
                                           name: 0 as *mut libc::c_char,
                                           label: 0 as *mut libc::c_char,
                                           _value: 0 as *mut libc::c_char,
                                           flags: 0,
                                           reserved: 0 as *mut libc::c_void,},
                           nr_choices: 0,
                           choices: 0 as *mut *mut oc_choice,};
    let mut i: libc::c_int = 0i32;
    let mut ret: libc::c_int = 0;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut l: libc::c_int = 0;
    let mut form_msg: *mut oc_text_buf = buf_alloc();
    let mut tmbuf: [libc::c_char; 80] = [0; 80];
    let mut tm: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *mut libc::c_char,};
    l =
        load_be16(eap.offset(2) as *const libc::c_void) as libc::c_int -
            0xci32;
    p = eap.offset(0xci32 as isize) as *mut libc::c_void;
    memset(&mut f as *mut oc_auth_form as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<oc_auth_form>() as libc::c_ulong);
    memset(&mut o as *mut oc_form_opt_select as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<oc_form_opt_select>() as libc::c_ulong);
    f.auth_id =
        b"pulse_session_kill\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    f.opts = &mut o.form;
    buf_append(form_msg,
               libintl_dgettext(b"openconnect\x00" as *const u8 as
                                    *const libc::c_char,
                                b"Session limit reached. Choose session to kill:\n\x00"
                                    as *const u8 as *const libc::c_char));
    o.form.next = 0 as *mut oc_form_opt;
    o.form.type_0 = 3i32;
    o.form.name =
        b"session_choice\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    o.form.label =
        libintl_dgettext(b"openconnect\x00" as *const u8 as
                             *const libc::c_char,
                         b"Session:\x00" as *const u8 as *const libc::c_char);
    o.nr_choices = sessions;
    o.choices =
        calloc(sessions as libc::c_ulong,
               ::std::mem::size_of::<*mut oc_choice>() as libc::c_ulong) as
            *mut *mut oc_choice;
    if o.choices.is_null() { return -12i32 }
    loop  {
        if !(l != 0) { current_block = 4741994311446740739; break ; }
        let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut when: time_t = 0i32 as time_t;
        let mut sessid: *mut libc::c_char = 0 as *mut libc::c_char;
        if !(parse_avp(vpninfo, &mut p, &mut l, &mut avp_p, &mut avp_len,
                       &mut avp_flags, &mut avp_vendor, &mut avp_code) != 0) {
            if avp_vendor != 0x583i32 as libc::c_uint ||
                   avp_code != 0xd65i32 as libc::c_uint {
                continue ;
            }
            loop  {
                if !(avp_len != 0) {
                    current_block = 1622411330066726685;
                    break ;
                }
                if parse_avp(vpninfo, &mut avp_p, &mut avp_len, &mut avp_p2,
                             &mut avp_len2, &mut avp_flags, &mut avp_vendor,
                             &mut avp_code) != 0 {
                    current_block = 14767322590804963321;
                    break ;
                }
                dump_avp(vpninfo, avp_flags, avp_vendor, avp_code, avp_p2,
                         avp_len2);
                if avp_vendor == 0x583i32 as libc::c_uint &&
                       avp_code == 0xd66i32 as libc::c_uint {
                    sessid =
                        strndup(avp_p2 as *const libc::c_char,
                                avp_len2 as libc::c_ulong)
                } else if avp_vendor == 0x583i32 as libc::c_uint &&
                              avp_code == 0xd67i32 as libc::c_uint {
                    from =
                        strndup(avp_p2 as *const libc::c_char,
                                avp_len2 as libc::c_ulong)
                } else if avp_vendor == 0x583i32 as libc::c_uint &&
                              avp_code == 0xd68i32 as libc::c_uint &&
                              avp_len2 == 8i32 {
                    when =
                        load_be32((avp_p2 as *mut libc::c_char).offset(4) as
                                      *const libc::c_void) as time_t;
                    if ::std::mem::size_of::<time_t>() as libc::c_ulong >
                           4i32 as libc::c_ulong {
                        when =
                            (when as libc::c_ulonglong |
                                 (load_be32(avp_p2) as uint64_t) << 32i32) as
                                time_t
                    }
                }
            }
            match current_block {
                14767322590804963321 => { }
                _ => {
                    if from.is_null() || sessid.is_null() || when == 0 {
                        free(from as *mut libc::c_void);
                        free(sessid as *mut libc::c_void);
                    } else {
                        localtime_r(&mut when, &mut tm);
                        strftime(tmbuf.as_mut_ptr(), 80i32 as size_t,
                                 b"%a, %d %b %Y %H:%M:%S %Z\x00" as *const u8
                                     as *const libc::c_char, &mut tm);
                        buf_append(form_msg,
                                   b" - %s from %s at %s\n\x00" as *const u8
                                       as *const libc::c_char, sessid, from,
                                   tmbuf.as_mut_ptr());
                        free(from as *mut libc::c_void);
                        let ref mut fresh4 = *o.choices.offset(i as isize);
                        *fresh4 =
                            malloc(::std::mem::size_of::<oc_choice>() as
                                       libc::c_ulong) as *mut oc_choice;
                        if (*o.choices.offset(i as isize)).is_null() {
                            ret = -12i32;
                            current_block = 3115813050751586513;
                            break ;
                        } else {
                            let ref mut fresh5 =
                                (**o.choices.offset(i as isize)).label;
                            *fresh5 = sessid;
                            let ref mut fresh6 =
                                (**o.choices.offset(i as isize)).name;
                            *fresh6 = *fresh5;
                            if (**o.choices.offset(i as isize)).name.is_null()
                               {
                                ret = -12i32;
                                current_block = 3115813050751586513;
                                break ;
                            } else { i += 1; continue ; }
                        }
                    }
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
                                                                                     b"Failed to parse session list\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        ret = -22i32;
        current_block = 3115813050751586513;
        break ;
    }
    match current_block {
        4741994311446740739 => {
            ret = buf_error(form_msg);
            if !(ret != 0) {
                f.message = (*form_msg).data;
                ret = process_auth_form(vpninfo, &mut f);
                if ret == 0 {
                    buf_append_avp_string(reqbuf, 0xd69i32 as uint32_t,
                                          o.form._value);
                }
            }
        }
        _ => { }
    }
    if !o.choices.is_null() {
        i = 0i32;
        while i < sessions {
            if !(*o.choices.offset(i as isize)).is_null() {
                free((**o.choices.offset(i as isize)).name as
                         *mut libc::c_void);
                free(*o.choices.offset(i as isize) as *mut libc::c_void);
            }
            i += 1
        }
        free(o.choices as *mut libc::c_void);
    }
    buf_free(form_msg);
    return ret;
}
#[src_loc = "998:1"]
unsafe extern "C" fn pulse_request_user_auth(mut vpninfo:
                                                 *mut openconnect_info,
                                             mut reqbuf: *mut oc_text_buf,
                                             mut eap_ident: uint8_t,
                                             mut prompt_flags: libc::c_int,
                                             mut user_prompt:
                                                 *mut libc::c_char,
                                             mut pass_prompt:
                                                 *mut libc::c_char)
 -> libc::c_int {
    let mut f: oc_auth_form =
        oc_auth_form{banner: 0 as *mut libc::c_char,
                     message: 0 as *mut libc::c_char,
                     error: 0 as *mut libc::c_char,
                     auth_id: 0 as *mut libc::c_char,
                     method: 0 as *mut libc::c_char,
                     action: 0 as *mut libc::c_char,
                     opts: 0 as *mut oc_form_opt,
                     authgroup_opt: 0 as *mut oc_form_opt_select,
                     authgroup_selection: 0,};
    let mut o: [oc_form_opt; 2] =
        [oc_form_opt{next: 0 as *mut oc_form_opt,
                     type_0: 0,
                     name: 0 as *mut libc::c_char,
                     label: 0 as *mut libc::c_char,
                     _value: 0 as *mut libc::c_char,
                     flags: 0,
                     reserved: 0 as *mut libc::c_void,}; 2];
    let mut eap_avp: [libc::c_uchar; 23] = [0; 23];
    let mut l: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    memset(&mut f as *mut oc_auth_form as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<oc_auth_form>() as libc::c_ulong);
    memset(o.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[oc_form_opt; 2]>() as libc::c_ulong);
    f.auth_id =
        if prompt_flags & 1i32 != 0 {
            b"pulse_user\x00" as *const u8 as *const libc::c_char
        } else { b"pulse_secondary\x00" as *const u8 as *const libc::c_char }
            as *mut libc::c_char;
    f.opts = &mut *o.as_mut_ptr().offset(1) as *mut oc_form_opt;
    f.message =
        if prompt_flags & 1i32 != 0 {
            libintl_dgettext(b"openconnect\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Enter user credentials:\x00" as *const u8 as
                                 *const libc::c_char)
        } else {
            libintl_dgettext(b"openconnect\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Enter secondary credentials:\x00" as *const u8
                                 as *const libc::c_char)
        };
    if prompt_flags & 2i32 != 0 {
        f.opts = &mut *o.as_mut_ptr().offset(0) as *mut oc_form_opt;
        o[0].next = 0 as *mut oc_form_opt;
        o[0].type_0 = 1i32;
        o[0].name =
            b"username\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        if !user_prompt.is_null() {
            o[0].label = user_prompt
        } else {
            o[0].label =
                if prompt_flags & 1i32 != 0 {
                    libintl_dgettext(b"openconnect\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Username:\x00" as *const u8 as
                                         *const libc::c_char)
                } else {
                    libintl_dgettext(b"openconnect\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Secondary username:\x00" as *const u8
                                         as *const libc::c_char)
                }
        }
    }
    if prompt_flags & 4i32 != 0 {
        /* Might be referenced from o[0] or directly from f.opts */
        o[0].next = &mut *o.as_mut_ptr().offset(1) as *mut oc_form_opt;
        o[1].type_0 = 2i32;
        o[1].name =
            b"password\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        if !pass_prompt.is_null() {
            o[1].label = pass_prompt
        } else {
            o[1].label =
                if prompt_flags & 1i32 != 0 {
                    libintl_dgettext(b"openconnect\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Password:\x00" as *const u8 as
                                         *const libc::c_char)
                } else {
                    libintl_dgettext(b"openconnect\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Secondary password:\x00" as *const u8
                                         as *const libc::c_char)
                }
        }
    }
    ret = process_auth_form(vpninfo, &mut f);
    if !(ret != 0) {
        if !o[0]._value.is_null() {
            buf_append_avp_string(reqbuf, 0xd6di32 as uint32_t, o[0]._value);
            free_pass(&mut (*o.as_mut_ptr().offset(0))._value);
        }
        if !o[1]._value.is_null() {
            l = strlen(o[1]._value) as libc::c_int;
            if l > 253i32 {
                free_pass(&mut (*o.as_mut_ptr().offset(1))._value);
                return -22i32
            }
        } else {
            /* Their client actually resubmits the primary password when
		 * a secondary password is requested. But it doesn't seem to
		 * be necessary; might even just be a bug. */
            l = 0i32
        }
        /* AVP flags+mandatory+length */
        store_be32(eap_avp.as_mut_ptr() as *mut libc::c_void,
                   79i32 as uint32_t);
        store_be32(eap_avp.as_mut_ptr().offset(4) as *mut libc::c_void,
                   ((0x40i32 << 24i32) as
                        libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_uchar; 23]>()
                                                        as
                                                        libc::c_ulong).wrapping_add(l
                                                                                        as
                                                                                        libc::c_ulong)
                       as uint32_t);
        /* EAP header: code/ident/len */
        eap_avp[8] = 2i32 as libc::c_uchar; /* EAP length */
        eap_avp[9] = eap_ident;
        store_be16(eap_avp.as_mut_ptr().offset(10) as *mut libc::c_void,
                   (l + 15i32) as uint16_t);
        store_be32(eap_avp.as_mut_ptr().offset(12) as *mut libc::c_void,
                   (0xfei32 << 24i32 | 0xa4ci32) as uint32_t);
        store_be32(eap_avp.as_mut_ptr().offset(16) as *mut libc::c_void,
                   2i32 as uint32_t);
        /* EAP Juniper/2 payload: 02 02 <len> <password> */
        eap_avp[21] = 0x2i32 as libc::c_uchar; /* Why 2? */
        eap_avp[20] = eap_avp[21];
        eap_avp[22] = (l + 2i32) as libc::c_uchar;
        buf_append_bytes(reqbuf, eap_avp.as_mut_ptr() as *const libc::c_void,
                         ::std::mem::size_of::<[libc::c_uchar; 23]>() as
                             libc::c_ulong as libc::c_int);
        if !o[1]._value.is_null() {
            buf_append_bytes(reqbuf, o[1]._value as *const libc::c_void, l);
            free_pass(&mut (*o.as_mut_ptr().offset(1))._value);
        }
        /* Padding */
        if (::std::mem::size_of::<[libc::c_uchar; 23]>() as
                libc::c_ulong).wrapping_add(l as libc::c_ulong) &
               3i32 as libc::c_ulong != 0 {
            let mut pad: uint32_t = 0i32 as uint32_t;
            buf_append_bytes(reqbuf,
                             &mut pad as *mut uint32_t as *const libc::c_void,
                             (4i32 as
                                  libc::c_ulong).wrapping_sub((::std::mem::size_of::<[libc::c_uchar; 23]>()
                                                                   as
                                                                   libc::c_ulong).wrapping_add(l
                                                                                                   as
                                                                                                   libc::c_ulong)
                                                                  &
                                                                  3i32 as
                                                                      libc::c_ulong)
                                 as libc::c_int);
        }
        ret = 0i32
    }
    return ret;
}
#[src_loc = "1089:1"]
unsafe extern "C" fn pulse_request_gtc(mut vpninfo: *mut openconnect_info,
                                       mut reqbuf: *mut oc_text_buf,
                                       mut eap_ident: uint8_t,
                                       mut prompt_flags: libc::c_int,
                                       mut user_prompt: *mut libc::c_char,
                                       mut pass_prompt: *mut libc::c_char,
                                       mut gtc_prompt: *mut libc::c_char)
 -> libc::c_int {
    let mut f: oc_auth_form =
        oc_auth_form{banner: 0 as *mut libc::c_char,
                     message: 0 as *mut libc::c_char,
                     error: 0 as *mut libc::c_char,
                     auth_id: 0 as *mut libc::c_char,
                     method: 0 as *mut libc::c_char,
                     action: 0 as *mut libc::c_char,
                     opts: 0 as *mut oc_form_opt,
                     authgroup_opt: 0 as *mut oc_form_opt_select,
                     authgroup_selection: 0,};
    let mut o: [oc_form_opt; 2] =
        [oc_form_opt{next: 0 as *mut oc_form_opt,
                     type_0: 0,
                     name: 0 as *mut libc::c_char,
                     label: 0 as *mut libc::c_char,
                     _value: 0 as *mut libc::c_char,
                     flags: 0,
                     reserved: 0 as *mut libc::c_void,}; 2];
    let mut ret: libc::c_int = 0;
    memset(&mut f as *mut oc_auth_form as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<oc_auth_form>() as libc::c_ulong);
    memset(o.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[oc_form_opt; 2]>() as libc::c_ulong);
    f.auth_id =
        b"pulse_gtc\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    /* The first prompt always seems to be 'Enter SecurID PASSCODE:' and is ignored. */
    if !gtc_prompt.is_null() && prompt_flags & 0x10000i32 != 0 {
        f.message = gtc_prompt
    } else {
        f.message =
            libintl_dgettext(b"openconnect\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Token code request:\x00" as *const u8 as
                                 *const libc::c_char)
    }
    if prompt_flags & 2i32 != 0 {
        f.opts = &mut *o.as_mut_ptr().offset(0) as *mut oc_form_opt;
        o[0].next = &mut *o.as_mut_ptr().offset(1) as *mut oc_form_opt;
        o[0].type_0 = 1i32;
        o[0].name =
            b"username\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        if !user_prompt.is_null() {
            o[0].label = user_prompt
        } else {
            o[0].label =
                if prompt_flags & 1i32 != 0 {
                    libintl_dgettext(b"openconnect\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Username:\x00" as *const u8 as
                                         *const libc::c_char)
                } else {
                    libintl_dgettext(b"openconnect\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Secondary username:\x00" as *const u8
                                         as *const libc::c_char)
                }
        }
    } else { f.opts = &mut *o.as_mut_ptr().offset(1) as *mut oc_form_opt }
    o[1].type_0 = 2i32;
    o[1].name =
        b"tokencode\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    /*
	 * For retries, we have a gtc_prompt and we just say 'Please enter response:'.
	 * Otherwise, use the pass_prompt if it exists, or create our own based
	 * on whether it's primary authentication or not.
	 */
    if prompt_flags & 0x10000i32 != 0 {
        o[1].label =
            libintl_dgettext(b"openconnect\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Please enter response:\x00" as *const u8 as
                                 *const libc::c_char)
    } else if !pass_prompt.is_null() {
        o[1].label = pass_prompt
    } else if prompt_flags & 1i32 != 0 {
        o[1].label =
            libintl_dgettext(b"openconnect\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Please enter your passcode:\x00" as *const u8
                                 as *const libc::c_char)
    } else {
        o[1].label =
            libintl_dgettext(b"openconnect\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Please enter your secondary token information:\x00"
                                 as *const u8 as *const libc::c_char)
    }
    if can_gen_tokencode(vpninfo, &mut f, &mut *o.as_mut_ptr().offset(1)) == 0
       {
        o[1].type_0 = 5i32
    }
    ret = process_auth_form(vpninfo, &mut f);
    if !(ret != 0) {
        ret = do_gen_tokencode(vpninfo, &mut f);
        if !(ret != 0) {
            if !o[0]._value.is_null() {
                buf_append_avp_string(reqbuf, 0xd6di32 as uint32_t,
                                      o[0]._value);
                free_pass(&mut (*o.as_mut_ptr().offset(0))._value);
            }
            if !o[1]._value.is_null() {
                let mut eap_avp: [libc::c_uchar; 13] = [0; 13];
                let mut l: libc::c_int = strlen(o[1]._value) as libc::c_int;
                if l > 253i32 {
                    free_pass(&mut (*o.as_mut_ptr().offset(1))._value);
                    ret = -22i32
                } else {
                    /* AVP flags+mandatory+length */
                    store_be32(eap_avp.as_mut_ptr() as *mut libc::c_void,
                               79i32 as uint32_t);
                    store_be32(eap_avp.as_mut_ptr().offset(4) as
                                   *mut libc::c_void,
                               ((0x40i32 << 24i32) as
                                    libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_uchar; 13]>()
                                                                    as
                                                                    libc::c_ulong).wrapping_add(l
                                                                                                    as
                                                                                                    libc::c_ulong)
                                   as uint32_t);
                    /* EAP header: code/ident/len */
                    eap_avp[8] = 2i32 as libc::c_uchar; /* EAP length */
                    eap_avp[9] = eap_ident;
                    store_be16(eap_avp.as_mut_ptr().offset(10) as
                                   *mut libc::c_void, (l + 5i32) as uint16_t);
                    eap_avp[12] = 6i32 as libc::c_uchar;
                    buf_append_bytes(reqbuf,
                                     eap_avp.as_mut_ptr() as
                                         *const libc::c_void,
                                     ::std::mem::size_of::<[libc::c_uchar; 13]>()
                                         as libc::c_ulong as libc::c_int);
                    buf_append_bytes(reqbuf,
                                     o[1]._value as *const libc::c_void, l);
                    /* Padding */
                    if (::std::mem::size_of::<[libc::c_uchar; 13]>() as
                            libc::c_ulong).wrapping_add(l as libc::c_ulong) &
                           3i32 as libc::c_ulong != 0 {
                        let mut pad: uint32_t = 0i32 as uint32_t;
                        buf_append_bytes(reqbuf,
                                         &mut pad as *mut uint32_t as
                                             *const libc::c_void,
                                         (4i32 as
                                              libc::c_ulong).wrapping_sub((::std::mem::size_of::<[libc::c_uchar; 13]>()
                                                                               as
                                                                               libc::c_ulong).wrapping_add(l
                                                                                                               as
                                                                                                               libc::c_ulong)
                                                                              &
                                                                              3i32
                                                                                  as
                                                                                  libc::c_ulong)
                                             as libc::c_int);
                    }
                    free_pass(&mut (*o.as_mut_ptr().offset(1))._value);
                    ret = 0i32
                }
            } else { ret = -22i32 }
        }
    }
    return ret;
}
#[src_loc = "1191:1"]
unsafe extern "C" fn dup_prompt(mut p: *mut *mut libc::c_char,
                                mut avp_p: *mut uint8_t,
                                mut avp_len: libc::c_int) -> libc::c_int {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    free(*p as *mut libc::c_void);
    *p = 0 as *mut libc::c_char;
    if avp_len == 0 {
        return 0i32
    } else {
        if *avp_p.offset((avp_len - 1i32) as isize) as libc::c_int ==
               ':' as i32 {
            ret =
                strndup(avp_p as *mut libc::c_char, avp_len as libc::c_ulong)
        } else {
            ret =
                calloc((avp_len + 2i32) as libc::c_ulong,
                       1i32 as libc::c_ulong) as *mut libc::c_char;
            if !ret.is_null() {
                memcpy(ret as *mut libc::c_void, avp_p as *const libc::c_void,
                       avp_len as libc::c_ulong);
                *ret.offset(avp_len as isize) = ':' as i32 as libc::c_char;
                *ret.offset((avp_len + 1i32) as isize) = 0i32 as libc::c_char
            }
        }
    }
    if !ret.is_null() { *p = ret; return 0i32 } else { return -12i32 };
}
/*
 * There is complex client-side logic around when to (re)prompt for a password.
 * The first prompt always needs it, whether it's a TokenCode request (EAP-06)
 * or a normal password request (EAP-Expanded-Juniper/2). If a password request
 * fails (0x81) then we prompt for username again in case that's what was wrong.
 *
 * If there's a secondary password request, it might need a *secondary* username.
 * The first request comes with a 0xd73 AVP which has a single integer:
 *   1: prompt for both username and password.
 *   3: Prompt for password only.
 *   5: Prompt for username only.
 *
 */
/* IF-T/TLS session establishment is the same for both pulse_obtain_cookie() and
 * pulse_connect(). We have to go through the EAP phase of the connection either
 * way; it's just that we might do it with just the cookie, or we might need to
 * use the password/cert etc. */
#[src_loc = "1236:1"]
unsafe extern "C" fn pulse_authenticate(mut vpninfo: *mut openconnect_info,
                                        mut connecting: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut reqbuf: *mut oc_text_buf = 0 as *mut oc_text_buf;
    let mut bytes: [libc::c_uchar; 16384] = [0; 16384];
    let mut eap_ofs: libc::c_int = 0;
    let mut eap_ident: uint8_t = 0;
    let mut eap2_ident: uint8_t = 0i32 as uint8_t;
    let mut avp_flags: uint8_t = 0;
    let mut avp_code: uint32_t = 0;
    let mut avp_vendor: uint32_t = 0;
    let mut avp_len: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut avp_p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut eap: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cookie_found: libc::c_int = 0i32;
    let mut j2_found: libc::c_int = 0i32;
    let mut realms_found: libc::c_int = 0i32;
    let mut realm_entry: libc::c_int = 0i32;
    let mut old_sessions: libc::c_int = 0i32;
    let mut gtc_found: libc::c_int = 0i32;
    let mut j2_code: uint8_t = 0i32 as uint8_t;
    let mut ttls: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut user_prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pass_prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gtc_prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut signin_prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user2_prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pass2_prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prompt_flags: libc::c_int = 1i32 | 2i32 | 4i32;
    /* XXX: We should do what cstp_connect() does to check that configuration
	   hasn't changed on a reconnect. */
    ret = openconnect_open_https(vpninfo);
    if ret != 0 { return ret }
    reqbuf = buf_alloc();
    buf_append(reqbuf,
               b"GET /%s HTTP/1.1\r\n\x00" as *const u8 as
                   *const libc::c_char,
               if !(*vpninfo).urlpath.is_null() {
                   (*vpninfo).urlpath
               } else { b"\x00" as *const u8 as *const libc::c_char });
    http_common_headers(vpninfo, reqbuf);
    buf_append(reqbuf,
               b"Content-Type: EAP\r\n\x00" as *const u8 as
                   *const libc::c_char);
    buf_append(reqbuf,
               b"Upgrade: IF-T/TLS 1.0\r\n\x00" as *const u8 as
                   *const libc::c_char);
    buf_append(reqbuf,
               b"Content-Length: 0\r\n\x00" as *const u8 as
                   *const libc::c_char);
    buf_append(reqbuf, b"\r\n\x00" as *const u8 as *const libc::c_char);
    if buf_error(reqbuf) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Error creating Pulse connection request\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        ret = buf_error(reqbuf)
    } else {
        if (*vpninfo).dump_http_traffic != 0 {
            dump_buf(vpninfo, '>' as i32 as libc::c_char, (*reqbuf).data);
        }
        ret =
            (*vpninfo).ssl_write.expect("non-null function pointer")(vpninfo,
                                                                     (*reqbuf).data,
                                                                     (*reqbuf).pos
                                                                         as
                                                                         size_t);
        if !(ret < 0i32) {
            ret = process_http_response(vpninfo, 1i32, None, reqbuf);
            if !(ret < 0i32) {
                if ret != 101i32 {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Unexpected %d result from server\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                ret);
                    }
                    ret = -22i32
                } else {
                    (*vpninfo).ift_seq = 0i32 as uint32_t;
                    /* IF-T version request. */
                    buf_truncate(reqbuf);
                    buf_append_ift_hdr(reqbuf, 0x5597i32 as uint32_t,
                                       1i32 as uint32_t);
                    /* Min version 1, max 2, preferred 2. Not that we actually do v2; the auth is
	 * still all IF-T/TLS v1. But the server won't offer us HMAC-SHA256 unless we
	 * advertise v2 */
                    buf_append_be32(reqbuf, 0x10202i32 as uint32_t);
                    ret = send_ift_packet(vpninfo, reqbuf);
                    if !(ret != 0) {
                        ret =
                            recv_ift_packet(vpninfo,
                                            bytes.as_mut_ptr() as
                                                *mut libc::c_void,
                                            ::std::mem::size_of::<[libc::c_uchar; 16384]>()
                                                as libc::c_ulong as
                                                libc::c_int);
                        if !(ret < 0i32) {
                            if ret != 0x14i32 ||
                                   load_be32(bytes.as_mut_ptr() as
                                                 *const libc::c_void) &
                                       0xffffffi32 as libc::c_uint !=
                                       0x5597i32 as libc::c_uint ||
                                   load_be32(bytes.as_mut_ptr().offset(4) as
                                                 *const libc::c_void) !=
                                       2i32 as libc::c_uint ||
                                   load_be32(bytes.as_mut_ptr().offset(8) as
                                                 *const libc::c_void) !=
                                       0x14i32 as libc::c_uint {
                                if (*vpninfo).verbose >= 0i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            0i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"Unexpected response to IF-T/TLS version negotiation:\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char));
                                }
                                dump_buf_hex(vpninfo, 0i32,
                                             '<' as i32 as libc::c_char,
                                             bytes.as_mut_ptr() as
                                                 *mut libc::c_void as
                                                 *mut libc::c_uchar, ret);
                                ret = -22i32
                            } else {
                                if (*vpninfo).verbose >= 3i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            3i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"IF-T/TLS version from server: %d\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char),
                                                                                            bytes[0x13i32
                                                                                                      as
                                                                                                      usize]
                                                                                                as
                                                                                                libc::c_int);
                                }
                                /* Client information packet over IF-T/TLS */
                                buf_truncate(reqbuf);
                                buf_append_ift_hdr(reqbuf,
                                                   0xa4ci32 as uint32_t,
                                                   0x88i32 as uint32_t);
                                buf_append(reqbuf,
                                           b"clientHostName=%s\x00" as
                                               *const u8 as
                                               *const libc::c_char,
                                           (*vpninfo).localname);
                                bytes[0] = 0i32 as libc::c_uchar;
                                if !(*vpninfo).peer_addr.is_null() &&
                                       (*(*vpninfo).peer_addr).sa_family as
                                           libc::c_int == 30i32 {
                                    let mut a: sockaddr_in6 =
                                        sockaddr_in6{sin6_len: 0,
                                                     sin6_family: 0,
                                                     sin6_port: 0,
                                                     sin6_flowinfo: 0,
                                                     sin6_addr:
                                                         in6_addr{__u6_addr:
                                                                      C2RustUnnamed{__u6_addr8:
                                                                                        [0;
                                                                                            16],},},
                                                     sin6_scope_id: 0,};
                                    let mut l_0: socklen_t =
                                        ::std::mem::size_of::<sockaddr_in6>()
                                            as libc::c_ulong as socklen_t;
                                    if getsockname((*vpninfo).ssl_fd,
                                                   &mut a as *mut sockaddr_in6
                                                       as *mut libc::c_void as
                                                       *mut sockaddr,
                                                   &mut l_0) == 0 {
                                        inet_ntop(30i32,
                                                  &mut a.sin6_addr as
                                                      *mut in6_addr as
                                                      *const libc::c_void,
                                                  bytes.as_mut_ptr() as
                                                      *mut libc::c_void as
                                                      *mut libc::c_char,
                                                  ::std::mem::size_of::<[libc::c_uchar; 16384]>()
                                                      as libc::c_ulong as
                                                      socklen_t);
                                    }
                                } else if !(*vpninfo).peer_addr.is_null() &&
                                              (*(*vpninfo).peer_addr).sa_family
                                                  as libc::c_int == 2i32 {
                                    let mut a_0: sockaddr_in =
                                        sockaddr_in{sin_len: 0,
                                                    sin_family: 0,
                                                    sin_port: 0,
                                                    sin_addr:
                                                        in_addr{s_addr: 0,},
                                                    sin_zero: [0; 8],};
                                    let mut l_1: socklen_t =
                                        ::std::mem::size_of::<sockaddr_in>()
                                            as libc::c_ulong as socklen_t;
                                    if getsockname((*vpninfo).ssl_fd,
                                                   &mut a_0 as
                                                       *mut sockaddr_in as
                                                       *mut libc::c_void as
                                                       *mut sockaddr,
                                                   &mut l_1) == 0 {
                                        inet_ntop(2i32,
                                                  &mut a_0.sin_addr as
                                                      *mut in_addr as
                                                      *const libc::c_void,
                                                  bytes.as_mut_ptr() as
                                                      *mut libc::c_void as
                                                      *mut libc::c_char,
                                                  ::std::mem::size_of::<[libc::c_uchar; 16384]>()
                                                      as libc::c_ulong as
                                                      socklen_t);
                                    }
                                }
                                if bytes[0] != 0 {
                                    buf_append(reqbuf,
                                               b" clientIp=%s\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               bytes.as_mut_ptr());
                                }
                                buf_append(reqbuf,
                                           b"\n%c\x00" as *const u8 as
                                               *const libc::c_char, 0i32);
                                ret = send_ift_packet(vpninfo, reqbuf);
                                if !(ret != 0) {
                                    /* Await start of auth negotiations */
                                    ret =
                                        recv_ift_packet(vpninfo,
                                                        bytes.as_mut_ptr() as
                                                            *mut libc::c_void,
                                                        ::std::mem::size_of::<[libc::c_uchar; 16384]>()
                                                            as libc::c_ulong
                                                            as libc::c_int);
                                    if !(ret < 0i32) {
                                        /* Basically an empty IF-T/TLS auth challenge packet of type Juniper/1,
	 * without even an EAP header in the payload. */
                                        if valid_ift_auth(bytes.as_mut_ptr(),
                                                          ret) == 0 ||
                                               ret != 0x14i32 {
                                            if (*vpninfo).verbose >= 0i32 {
                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                        0i32,
                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char,
                                                                                                                         b"Unexpected IF-T/TLS authentication challenge:\n\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char));
                                            }
                                            dump_buf_hex(vpninfo, 0i32,
                                                         '<' as i32 as
                                                             libc::c_char,
                                                         bytes.as_mut_ptr() as
                                                             *mut libc::c_void
                                                             as
                                                             *mut libc::c_uchar,
                                                         ret);
                                            ret = -22i32
                                        } else {
                                            /* Start by sending an EAP Identity of 'anonymous'. At this point we
	 * aren't yet very far down the rabbithole...
	 *
	 *     --------------------------------------
	 *     |                TCP/IP              |
	 *     |------------------------------------|
	 *     |                 TLS                |
	 *     |------------------------------------|
	 *     |               IF-T/TLS             |
	 *     |------------------------------------|
	 *     | EAP (IF-T/TLS Auth Type Juniper/1) |
	 *     |------------------------------------|
	 *     |             EAP-Identity           |
	 *     --------------------------------------
	 */
                                            buf_truncate(reqbuf); /* IF-T/TLS Auth Type */
                                            buf_append_ift_hdr(reqbuf,
                                                               0x5597i32 as
                                                                   uint32_t,
                                                               6i32 as
                                                                   uint32_t);
                                            buf_append_be32(reqbuf,
                                                            (0xa4ci32 << 8i32
                                                                 | 1i32) as
                                                                uint32_t);
                                            eap_ofs =
                                                buf_append_eap_hdr(reqbuf,
                                                                   2i32 as
                                                                       uint8_t,
                                                                   1i32 as
                                                                       uint8_t,
                                                                   1i32 as
                                                                       uint8_t,
                                                                   0i32 as
                                                                       uint32_t);
                                            buf_append(reqbuf,
                                                       b"anonymous\x00" as
                                                           *const u8 as
                                                           *const libc::c_char);
                                            buf_fill_eap_len(reqbuf, eap_ofs);
                                            ret =
                                                send_ift_packet(vpninfo,
                                                                reqbuf);
                                            if !(ret != 0) {
                                                /*
	 * Phase 2 may continue directly with EAP within IF-T/TLS, or if certificate
	 * auth is enabled, the server may use EAP-TTLS. In that case, we end up
	 * with EAP within EAP-Message AVPs within EAP-TTLS within IF-T/TLS.
	 * The send_eap_packet() and recv_eap_packet() functions cope with both
	 * formats. The buffers have 0x14 bytes of header space, to allow for
	 * the IF-T/TLS header which is the larger of the two.
	 *
	 *     --------------------------------------
	 *     |                TCP/IP              |
	 *     |------------------------------------|
	 *     |                 TLS                |
	 *     |------------------------------------|
	 *     |               IF-T/TLS             |
	 *     |------------------------------------|
	 *     | EAP (IF-T/TLS Auth Type Juniper/1) |
	 *     |------------------                  |
	 *     |     EAP-TTLS    |                  |
	 *     |-----------------|  (or directly)   |
	 *     | EAP-Message AVP |                  |
	 *     |-----------------|------------------|
	 *     |            EAP-Juniper-1           |
	 *     --------------------------------------
	 */
                                                ret =
                                                    recv_ift_packet(vpninfo,
                                                                    bytes.as_mut_ptr()
                                                                        as
                                                                        *mut libc::c_void,
                                                                    ::std::mem::size_of::<[libc::c_uchar; 16384]>()
                                                                        as
                                                                        libc::c_ulong
                                                                        as
                                                                        libc::c_int);
                                                if !(ret < 0i32) {
                                                    /* Check EAP header and length */
                                                    if valid_ift_auth_eap(bytes.as_mut_ptr(),
                                                                          ret)
                                                           == 0 {
                                                        current_block =
                                                            4578160380713824469;
                                                    } else {
                                                        /*
	 * We know the packet is valid at least down to the first layer of
	 * EAP in the diagram above, directly within the IF-T/TLS Auth Type
	 * of Juniper/1. Now, disambiguate between the two cases where the
	 * diagram diverges. Is it EAP-TTLS or is it EAP-Juniper-1 directly?
	 */
                                                        if valid_ift_auth_eap_exj1(bytes.as_mut_ptr(),
                                                                                   ret)
                                                               != 0 {
                                                            eap =
                                                                bytes.as_mut_ptr().offset(0x14i32
                                                                                              as
                                                                                              isize);
                                                            current_block =
                                                                5636883459695696059;
                                                        } else if bytes[0x18i32
                                                                            as
                                                                            usize]
                                                                      as
                                                                      libc::c_int
                                                                      !=
                                                                      0x15i32
                                                         {
                                                            current_block =
                                                                4578160380713824469;
                                                        } else {
                                                            (*vpninfo).ttls_eap_ident
                                                                =
                                                                bytes[0x15i32
                                                                          as
                                                                          usize];
                                                            (*vpninfo).ttls_recvbuf
                                                                =
                                                                malloc(16384i32
                                                                           as
                                                                           libc::c_ulong)
                                                                    as
                                                                    *mut libc::c_uchar;
                                                            if (*vpninfo).ttls_recvbuf.is_null()
                                                               {
                                                                return -12i32
                                                            }
                                                            (*vpninfo).ttls_recvlen
                                                                = 0i32;
                                                            (*vpninfo).ttls_recvpos
                                                                = 0i32;
                                                            ttls =
                                                                establish_eap_ttls(vpninfo);
                                                            if ttls.is_null()
                                                               {
                                                                if (*vpninfo).verbose
                                                                       >= 0i32
                                                                   {
                                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                            0i32,
                                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                                 as
                                                                                                                                                 *const u8
                                                                                                                                                 as
                                                                                                                                                 *const libc::c_char,
                                                                                                                                             b"Failed to establish EAP-TTLS session\n\x00"
                                                                                                                                                 as
                                                                                                                                                 *const u8
                                                                                                                                                 as
                                                                                                                                                 *const libc::c_char));
                                                                }
                                                                ret = -22i32;
                                                                current_block
                                                                    =
                                                                    805098095739922543;
                                                            } else {
                                                                /* If it isn't that, it'd better be EAP-TTLS... */
                                                                /* Resend the EAP Identity 'anonymous' packet within EAP-TTLS */
                                                                ret =
                                                                    send_eap_packet(vpninfo,
                                                                                    ttls,
                                                                                    reqbuf);
                                                                if ret != 0 {
                                                                    current_block
                                                                        =
                                                                        805098095739922543;
                                                                } else {
                                                                    /*
		 * The recv_eap_packet() function receives and validates the EAP
		 * packet of type Extended Juniper-1, either natively or within
		 * EAP-TTLS according to whether 'ttls' is set.
		 */
                                                                    eap =
                                                                        recv_eap_packet(vpninfo,
                                                                                        ttls,
                                                                                        bytes.as_mut_ptr()
                                                                                            as
                                                                                            *mut libc::c_void,
                                                                                        ::std::mem::size_of::<[libc::c_uchar; 16384]>()
                                                                                            as
                                                                                            libc::c_ulong
                                                                                            as
                                                                                            libc::c_int)
                                                                            as
                                                                            *mut libc::c_uchar;
                                                                    if eap.is_null()
                                                                       {
                                                                        ret =
                                                                            -5i32;
                                                                        current_block
                                                                            =
                                                                            805098095739922543;
                                                                    } else {
                                                                        current_block
                                                                            =
                                                                            5636883459695696059;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        match current_block {
                                                            4578160380713824469
                                                            => {
                                                            }
                                                            805098095739922543
                                                            => {
                                                            }
                                                            _ => {
                                                                /* Now we (hopefully) have the server information packet, in an EAP request
	 * from the server. Either it was received directly in IF-T/TLS, or within
	 * an EAP-Message within EAP-TTLS. Either way, the EAP message we're
	 * interested in will be at offset 0x14 in the packet, its header will
	 * have been checked, and is Expanded Juniper/1, and its payload thus
	 * starts at 0x20. And its length is sufficient that we won't underflow */
                                                                eap_ident =
                                                                    *eap.offset(1); /* Already validated */
                                                                l =
                                                                    load_be16(eap.offset(2)
                                                                                  as
                                                                                  *const libc::c_void)
                                                                        as
                                                                        libc::c_int
                                                                        -
                                                                        0xci32;
                                                                p =
                                                                    eap.offset(0xci32
                                                                                   as
                                                                                   isize)
                                                                        as
                                                                        *mut libc::c_void;
                                                                loop 
                                                                     /* We don't actually use anything we get here. Typically it
	 * contains Juniper/0xd49 and Juniper/0xd4a word AVPs, and
	 * a Juniper/0xd56 AVP with server licensing information. */
                                                                     {
                                                                    if l != 0
                                                                       {
                                                                        if parse_avp(vpninfo,
                                                                                     &mut p,
                                                                                     &mut l,
                                                                                     &mut avp_p,
                                                                                     &mut avp_len,
                                                                                     &mut avp_flags,
                                                                                     &mut avp_vendor,
                                                                                     &mut avp_code)
                                                                               !=
                                                                               0
                                                                           {
                                                                            if (*vpninfo).verbose
                                                                                   >=
                                                                                   0i32
                                                                               {
                                                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                        0i32,
                                                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                                                             as
                                                                                                                                                             *const u8
                                                                                                                                                             as
                                                                                                                                                             *const libc::c_char,
                                                                                                                                                         b"Failed to parse AVP\n\x00"
                                                                                                                                                             as
                                                                                                                                                             *const u8
                                                                                                                                                             as
                                                                                                                                                             *const libc::c_char));
                                                                            }
                                                                            current_block
                                                                                =
                                                                                7591608735147044776;
                                                                            break
                                                                                ;
                                                                        } else {
                                                                            dump_avp(vpninfo,
                                                                                     avp_flags,
                                                                                     avp_vendor,
                                                                                     avp_code,
                                                                                     avp_p,
                                                                                     avp_len);
                                                                        }
                                                                    } else {
                                                                        /* Present the client information and auth cookie */
                                                                        buf_truncate(reqbuf); /* IF-T/TLS Auth Type */
                                                                        buf_append_ift_hdr(reqbuf,
                                                                                           0x5597i32
                                                                                               as
                                                                                               uint32_t,
                                                                                           6i32
                                                                                               as
                                                                                               uint32_t);
                                                                        buf_append_be32(reqbuf,
                                                                                        (0xa4ci32
                                                                                             <<
                                                                                             8i32
                                                                                             |
                                                                                             1i32)
                                                                                            as
                                                                                            uint32_t);
                                                                        eap_ofs
                                                                            =
                                                                            buf_append_eap_hdr(reqbuf,
                                                                                               2i32
                                                                                                   as
                                                                                                   uint8_t,
                                                                                               eap_ident,
                                                                                               0xfei32
                                                                                                   as
                                                                                                   uint8_t,
                                                                                               1i32
                                                                                                   as
                                                                                                   uint32_t);
                                                                        buf_append_avp_string(reqbuf,
                                                                                              0xd70i32
                                                                                                  as
                                                                                                  uint32_t,
                                                                                              (*vpninfo).useragent);
                                                                        if !(*vpninfo).cookie.is_null()
                                                                           {
                                                                            buf_append_avp_string(reqbuf,
                                                                                                  0xd53i32
                                                                                                      as
                                                                                                      uint32_t,
                                                                                                  (*vpninfo).cookie);
                                                                        }
                                                                        buf_fill_eap_len(reqbuf,
                                                                                         eap_ofs);
                                                                        ret =
                                                                            send_eap_packet(vpninfo,
                                                                                            ttls,
                                                                                            reqbuf);
                                                                        if ret
                                                                               !=
                                                                               0
                                                                           {
                                                                            current_block
                                                                                =
                                                                                805098095739922543;
                                                                            break
                                                                                ;
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                3260529072242652013;
                                                                            break
                                                                                ;
                                                                        }
                                                                    }
                                                                }
                                                                match current_block
                                                                    {
                                                                    805098095739922543
                                                                    => {
                                                                    }
                                                                    _ => {
                                                                        'c_25650:
                                                                            loop 
                                                                                 {
                                                                                match current_block
                                                                                    {
                                                                                    7591608735147044776
                                                                                    =>
                                                                                    {
                                                                                        dump_buf_hex(vpninfo,
                                                                                                     0i32,
                                                                                                     'E'
                                                                                                         as
                                                                                                         i32
                                                                                                         as
                                                                                                         libc::c_char,
                                                                                                     eap,
                                                                                                     load_be16(eap.offset(2)
                                                                                                                   as
                                                                                                                   *const libc::c_void)
                                                                                                         as
                                                                                                         libc::c_int);
                                                                                        ret
                                                                                            =
                                                                                            -22i32;
                                                                                        current_block
                                                                                            =
                                                                                            805098095739922543;
                                                                                        break
                                                                                            ;
                                                                                    }
                                                                                    _
                                                                                    =>

                                                                                    /* Await start of auth negotiations */
                                                                                    {
                                                                                        free(signin_prompt
                                                                                                 as
                                                                                                 *mut libc::c_void);
                                                                                        signin_prompt
                                                                                            =
                                                                                            0
                                                                                                as
                                                                                                *mut libc::c_char;
                                                                                        /* If there's a follow-on GTC prompt, remember it's not the first */
                                                                                        if gtc_found
                                                                                               !=
                                                                                               0
                                                                                           {
                                                                                            prompt_flags
                                                                                                |=
                                                                                                0x10000i32
                                                                                        } else {
                                                                                            prompt_flags
                                                                                                &=
                                                                                                !0x10000i32
                                                                                        } /* Already validated */
                                                                                        old_sessions
                                                                                            =
                                                                                            0i32;
                                                                                        j2_found
                                                                                            =
                                                                                            old_sessions;
                                                                                        realms_found
                                                                                            =
                                                                                            j2_found;
                                                                                        realm_entry
                                                                                            =
                                                                                            realms_found;
                                                                                        gtc_found
                                                                                            =
                                                                                            0i32;
                                                                                        eap
                                                                                            =
                                                                                            recv_eap_packet(vpninfo,
                                                                                                            ttls,
                                                                                                            bytes.as_mut_ptr()
                                                                                                                as
                                                                                                                *mut libc::c_void,
                                                                                                            ::std::mem::size_of::<[libc::c_uchar; 16384]>()
                                                                                                                as
                                                                                                                libc::c_ulong
                                                                                                                as
                                                                                                                libc::c_int)
                                                                                                as
                                                                                                *mut libc::c_uchar;
                                                                                        if eap.is_null()
                                                                                           {
                                                                                            ret
                                                                                                =
                                                                                                -5i32;
                                                                                            current_block
                                                                                                =
                                                                                                805098095739922543;
                                                                                            break
                                                                                                ;
                                                                                        } else {
                                                                                            eap_ident
                                                                                                =
                                                                                                *eap.offset(1);
                                                                                            l
                                                                                                =
                                                                                                load_be16(eap.offset(2)
                                                                                                              as
                                                                                                              *const libc::c_void)
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    -
                                                                                                    0xci32;
                                                                                            p
                                                                                                =
                                                                                                eap.offset(0xci32
                                                                                                               as
                                                                                                               isize)
                                                                                                    as
                                                                                                    *mut libc::c_void;
                                                                                            loop 
                                                                                                 {
                                                                                                if !(l
                                                                                                         !=
                                                                                                         0)
                                                                                                   {
                                                                                                    current_block
                                                                                                        =
                                                                                                        1131722263116153860;
                                                                                                    break
                                                                                                        ;
                                                                                                }
                                                                                                if parse_avp(vpninfo,
                                                                                                             &mut p,
                                                                                                             &mut l,
                                                                                                             &mut avp_p,
                                                                                                             &mut avp_len,
                                                                                                             &mut avp_flags,
                                                                                                             &mut avp_vendor,
                                                                                                             &mut avp_code)
                                                                                                       !=
                                                                                                       0
                                                                                                   {
                                                                                                    if (*vpninfo).verbose
                                                                                                           >=
                                                                                                           0i32
                                                                                                       {
                                                                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                0i32,
                                                                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                     as
                                                                                                                                                                                     *const u8
                                                                                                                                                                                     as
                                                                                                                                                                                     *const libc::c_char,
                                                                                                                                                                                 b"Failed to parse AVP\n\x00"
                                                                                                                                                                                     as
                                                                                                                                                                                     *const u8
                                                                                                                                                                                     as
                                                                                                                                                                                     *const libc::c_char));
                                                                                                    }
                                                                                                    current_block
                                                                                                        =
                                                                                                        7591608735147044776;
                                                                                                    continue
                                                                                                        'c_25650
                                                                                                        ;
                                                                                                } else {
                                                                                                    dump_avp(vpninfo,
                                                                                                             avp_flags,
                                                                                                             avp_vendor,
                                                                                                             avp_code,
                                                                                                             avp_p,
                                                                                                             avp_len);
                                                                                                    /* It's a bit late for this given that we don't get it until after
		 * we provide the password. */
                                                                                                    if avp_vendor
                                                                                                           ==
                                                                                                           0x583i32
                                                                                                               as
                                                                                                               libc::c_uint
                                                                                                           &&
                                                                                                           avp_code
                                                                                                               ==
                                                                                                               0xd55i32
                                                                                                                   as
                                                                                                                   libc::c_uint
                                                                                                       {
                                                                                                        let mut md5buf:
                                                                                                                [libc::c_char; 33] =
                                                                                                            [0;
                                                                                                                33];
                                                                                                        get_cert_md5_fingerprint(vpninfo,
                                                                                                                                 (*vpninfo).peer_cert,
                                                                                                                                 md5buf.as_mut_ptr());
                                                                                                        if avp_len
                                                                                                               !=
                                                                                                               16i32
                                                                                                                   *
                                                                                                                   2i32
                                                                                                               ||
                                                                                                               strncasecmp(avp_p
                                                                                                                               as
                                                                                                                               *const libc::c_char,
                                                                                                                           md5buf.as_mut_ptr(),
                                                                                                                           (16i32
                                                                                                                                *
                                                                                                                                2i32)
                                                                                                                               as
                                                                                                                               libc::c_ulong)
                                                                                                                   !=
                                                                                                                   0
                                                                                                           {
                                                                                                            if (*vpninfo).verbose
                                                                                                                   >=
                                                                                                                   0i32
                                                                                                               {
                                                                                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                        0i32,
                                                                                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                             as
                                                                                                                                                                                             *const u8
                                                                                                                                                                                             as
                                                                                                                                                                                             *const libc::c_char,
                                                                                                                                                                                         b"Server certificate mismatch. Aborting due to suspected MITM attack\n\x00"
                                                                                                                                                                                             as
                                                                                                                                                                                             *const u8
                                                                                                                                                                                             as
                                                                                                                                                                                             *const libc::c_char));
                                                                                                            }
                                                                                                            ret
                                                                                                                =
                                                                                                                -1i32;
                                                                                                            current_block
                                                                                                                =
                                                                                                                805098095739922543;
                                                                                                            break
                                                                                                                'c_25650
                                                                                                                ;
                                                                                                        }
                                                                                                    }
                                                                                                    if avp_vendor
                                                                                                           ==
                                                                                                           0x583i32
                                                                                                               as
                                                                                                               libc::c_uint
                                                                                                           &&
                                                                                                           avp_code
                                                                                                               ==
                                                                                                               0xd65i32
                                                                                                                   as
                                                                                                                   libc::c_uint
                                                                                                       {
                                                                                                        old_sessions
                                                                                                            +=
                                                                                                            1
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd60i32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        let mut failcode:
                                                                                                                uint32_t =
                                                                                                            0;
                                                                                                        if avp_len
                                                                                                               !=
                                                                                                               4i32
                                                                                                           {
                                                                                                            current_block
                                                                                                                =
                                                                                                                16887951004113897137;
                                                                                                            break
                                                                                                                ;
                                                                                                        }
                                                                                                        failcode
                                                                                                            =
                                                                                                            load_be32(avp_p);
                                                                                                        if failcode
                                                                                                               ==
                                                                                                               0xdi32
                                                                                                                   as
                                                                                                                   libc::c_uint
                                                                                                           {
                                                                                                            if (*vpninfo).verbose
                                                                                                                   >=
                                                                                                                   0i32
                                                                                                               {
                                                                                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                        0i32,
                                                                                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                             as
                                                                                                                                                                                             *const u8
                                                                                                                                                                                             as
                                                                                                                                                                                             *const libc::c_char,
                                                                                                                                                                                         b"Authentication failure: Account locked out\n\x00"
                                                                                                                                                                                             as
                                                                                                                                                                                             *const u8
                                                                                                                                                                                             as
                                                                                                                                                                                             *const libc::c_char));
                                                                                                            }
                                                                                                        } else if (*vpninfo).verbose
                                                                                                                      >=
                                                                                                                      0i32
                                                                                                         {
                                                                                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                    0i32,
                                                                                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                         as
                                                                                                                                                                                         *const u8
                                                                                                                                                                                         as
                                                                                                                                                                                         *const libc::c_char,
                                                                                                                                                                                     b"Authentication failure: Code 0x%02x\n\x00"
                                                                                                                                                                                         as
                                                                                                                                                                                         *const u8
                                                                                                                                                                                         as
                                                                                                                                                                                         *const libc::c_char),
                                                                                                                                                                    failcode);
                                                                                                        }
                                                                                                        ret
                                                                                                            =
                                                                                                            -1i32;
                                                                                                        current_block
                                                                                                            =
                                                                                                            805098095739922543;
                                                                                                        break
                                                                                                            'c_25650
                                                                                                            ;
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd80i32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        dup_prompt(&mut user_prompt,
                                                                                                                   avp_p
                                                                                                                       as
                                                                                                                       *mut uint8_t,
                                                                                                                   avp_len);
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd81i32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        dup_prompt(&mut pass_prompt,
                                                                                                                   avp_p
                                                                                                                       as
                                                                                                                       *mut uint8_t,
                                                                                                                   avp_len);
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd82i32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        dup_prompt(&mut user2_prompt,
                                                                                                                   avp_p
                                                                                                                       as
                                                                                                                       *mut uint8_t,
                                                                                                                   avp_len);
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd83i32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        dup_prompt(&mut pass2_prompt,
                                                                                                                   avp_p
                                                                                                                       as
                                                                                                                       *mut uint8_t,
                                                                                                                   avp_len);
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd73i32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        let mut val:
                                                                                                                uint32_t =
                                                                                                            0;
                                                                                                        if avp_len
                                                                                                               !=
                                                                                                               4i32
                                                                                                           {
                                                                                                            current_block
                                                                                                                =
                                                                                                                16887951004113897137;
                                                                                                            break
                                                                                                                ;
                                                                                                        }
                                                                                                        val
                                                                                                            =
                                                                                                            load_be32(avp_p);
                                                                                                        match val
                                                                                                            {
                                                                                                            1
                                                                                                            =>
                                                                                                            {
                                                                                                                /* Prompt for both username and password. */
                                                                                                                prompt_flags
                                                                                                                    =
                                                                                                                    4i32
                                                                                                                        |
                                                                                                                        2i32
                                                                                                            }
                                                                                                            3
                                                                                                            =>
                                                                                                            {
                                                                                                                /* Prompt for password.*/
                                                                                                                prompt_flags
                                                                                                                    =
                                                                                                                    4i32
                                                                                                            }
                                                                                                            5
                                                                                                            =>
                                                                                                            {
                                                                                                                /* Prompt for username.*/
                                                                                                                prompt_flags
                                                                                                                    =
                                                                                                                    2i32
                                                                                                            }
                                                                                                            _
                                                                                                            =>
                                                                                                            {
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    16887951004113897137;
                                                                                                                break
                                                                                                                    ;
                                                                                                            }
                                                                                                        }
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd7bi32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        free(signin_prompt
                                                                                                                 as
                                                                                                                 *mut libc::c_void);
                                                                                                        signin_prompt
                                                                                                            =
                                                                                                            strndup(avp_p
                                                                                                                        as
                                                                                                                        *const libc::c_char,
                                                                                                                    avp_len
                                                                                                                        as
                                                                                                                        libc::c_ulong)
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd4ei32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        realms_found
                                                                                                            +=
                                                                                                            1
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd4fi32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        realm_entry
                                                                                                            +=
                                                                                                            1
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0x583i32
                                                                                                                      as
                                                                                                                      libc::c_uint
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      0xd53i32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        free((*vpninfo).cookie
                                                                                                                 as
                                                                                                                 *mut libc::c_void);
                                                                                                        (*vpninfo).cookie
                                                                                                            =
                                                                                                            strndup(avp_p
                                                                                                                        as
                                                                                                                        *const libc::c_char,
                                                                                                                    avp_len
                                                                                                                        as
                                                                                                                        libc::c_ulong);
                                                                                                        cookie_found
                                                                                                            =
                                                                                                            1i32
                                                                                                    } else if avp_vendor
                                                                                                                  ==
                                                                                                                  0
                                                                                                                  &&
                                                                                                                  avp_code
                                                                                                                      ==
                                                                                                                      79i32
                                                                                                                          as
                                                                                                                          libc::c_uint
                                                                                                     {
                                                                                                        let mut avp_c:
                                                                                                                *mut libc::c_char =
                                                                                                            avp_p
                                                                                                                as
                                                                                                                *mut libc::c_char;
                                                                                                        /* EAP within AVP within EAP within IF-T/TLS. Chewck EAP header. */
                                                                                                        if avp_len
                                                                                                               <
                                                                                                               5i32
                                                                                                               ||
                                                                                                               *avp_c.offset(0)
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   !=
                                                                                                                   1i32
                                                                                                               ||
                                                                                                               load_be16(avp_c.offset(2)
                                                                                                                             as
                                                                                                                             *const libc::c_void)
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   !=
                                                                                                                   avp_len
                                                                                                           {
                                                                                                            current_block
                                                                                                                =
                                                                                                                16887951004113897137;
                                                                                                            break
                                                                                                                ;
                                                                                                        }
                                                                                                        eap2_ident
                                                                                                            =
                                                                                                            *avp_c.offset(1)
                                                                                                                as
                                                                                                                uint8_t;
                                                                                                        if *avp_c.offset(4)
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               ==
                                                                                                               6i32
                                                                                                           {
                                                                                                            gtc_found
                                                                                                                =
                                                                                                                1i32;
                                                                                                            free(gtc_prompt
                                                                                                                     as
                                                                                                                     *mut libc::c_void);
                                                                                                            gtc_prompt
                                                                                                                =
                                                                                                                strndup(avp_c.offset(5),
                                                                                                                        (avp_len
                                                                                                                             -
                                                                                                                             5i32)
                                                                                                                            as
                                                                                                                            libc::c_ulong)
                                                                                                        } else {
                                                                                                            if !(avp_len
                                                                                                                     ==
                                                                                                                     13i32
                                                                                                                     &&
                                                                                                                     load_be32(avp_c.offset(4)
                                                                                                                                   as
                                                                                                                                   *const libc::c_void)
                                                                                                                         ==
                                                                                                                         (0xfei32
                                                                                                                              <<
                                                                                                                              24i32
                                                                                                                              |
                                                                                                                              0xa4ci32)
                                                                                                                             as
                                                                                                                             libc::c_uint)
                                                                                                               {
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    16887951004113897137;
                                                                                                                break
                                                                                                                    ;
                                                                                                            }
                                                                                                            match load_be32(avp_c.offset(8)
                                                                                                                                as
                                                                                                                                *const libc::c_void)
                                                                                                                {
                                                                                                                2
                                                                                                                =>
                                                                                                                {
                                                                                                                }
                                                                                                                _
                                                                                                                =>
                                                                                                                {
                                                                                                                    current_block
                                                                                                                        =
                                                                                                                        16887951004113897137;
                                                                                                                    break
                                                                                                                        ;
                                                                                                                }
                                                                                                            }
                                                                                                            /*  Expanded Juniper/2: password */
                                                                                                            j2_found
                                                                                                                =
                                                                                                                1i32;
                                                                                                            j2_code
                                                                                                                =
                                                                                                                *avp_c.offset(12)
                                                                                                                    as
                                                                                                                    uint8_t
                                                                                                        }
                                                                                                    } else if avp_flags
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  &
                                                                                                                  0x40i32
                                                                                                                  !=
                                                                                                                  0
                                                                                                     {
                                                                                                        current_block
                                                                                                            =
                                                                                                            16887951004113897137;
                                                                                                        break
                                                                                                            ;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            match current_block
                                                                                                {
                                                                                                1131722263116153860
                                                                                                =>

                                                                                                /* We want it to be precisely one type of request, not a mixture. */
                                                                                                {
                                                                                                    if !(realm_entry
                                                                                                             +
                                                                                                             (realms_found
                                                                                                                  !=
                                                                                                                  0)
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                             +
                                                                                                             j2_found
                                                                                                             +
                                                                                                             gtc_found
                                                                                                             +
                                                                                                             cookie_found
                                                                                                             +
                                                                                                             (old_sessions
                                                                                                                  !=
                                                                                                                  0)
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                             !=
                                                                                                             1i32
                                                                                                             &&
                                                                                                             signin_prompt.is_null())
                                                                                                       {
                                                                                                        /* Prepare next response packet */
                                                                                                        buf_truncate(reqbuf); /* IF-T/TLS Auth Type */
                                                                                                        buf_append_ift_hdr(reqbuf,
                                                                                                                           0x5597i32
                                                                                                                               as
                                                                                                                               uint32_t,
                                                                                                                           6i32
                                                                                                                               as
                                                                                                                               uint32_t);
                                                                                                        buf_append_be32(reqbuf,
                                                                                                                        (0xa4ci32
                                                                                                                             <<
                                                                                                                             8i32
                                                                                                                             |
                                                                                                                             1i32)
                                                                                                                            as
                                                                                                                            uint32_t);
                                                                                                        eap_ofs
                                                                                                            =
                                                                                                            buf_append_eap_hdr(reqbuf,
                                                                                                                               2i32
                                                                                                                                   as
                                                                                                                                   uint8_t,
                                                                                                                               eap_ident,
                                                                                                                               0xfei32
                                                                                                                                   as
                                                                                                                                   uint8_t,
                                                                                                                               1i32
                                                                                                                                   as
                                                                                                                                   uint32_t);
                                                                                                        if cookie_found
                                                                                                               ==
                                                                                                               0
                                                                                                           {
                                                                                                            /* No user interaction when called from pulse_connect().
		 * We expect the cookie to work. */
                                                                                                            if connecting
                                                                                                                   !=
                                                                                                                   0
                                                                                                               {
                                                                                                                if (*vpninfo).verbose
                                                                                                                       >=
                                                                                                                       0i32
                                                                                                                   {
                                                                                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                            0i32,
                                                                                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                                 as
                                                                                                                                                                                                 *const u8
                                                                                                                                                                                                 as
                                                                                                                                                                                                 *const libc::c_char,
                                                                                                                                                                                             b"Pulse authentication cookie not accepted\n\x00"
                                                                                                                                                                                                 as
                                                                                                                                                                                                 *const u8
                                                                                                                                                                                                 as
                                                                                                                                                                                                 *const libc::c_char));
                                                                                                                }
                                                                                                                ret
                                                                                                                    =
                                                                                                                    -1i32;
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    805098095739922543;
                                                                                                                break
                                                                                                                    ;
                                                                                                            } else {
                                                                                                                if realm_entry
                                                                                                                       !=
                                                                                                                       0
                                                                                                                   {
                                                                                                                    if (*vpninfo).verbose
                                                                                                                           >=
                                                                                                                           3i32
                                                                                                                       {
                                                                                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                                3i32,
                                                                                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char,
                                                                                                                                                                                                 b"Pulse realm entry\n\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char));
                                                                                                                    }
                                                                                                                    ret
                                                                                                                        =
                                                                                                                        pulse_request_realm_entry(vpninfo,
                                                                                                                                                  reqbuf);
                                                                                                                    if ret
                                                                                                                           !=
                                                                                                                           0
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            805098095739922543;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                } else if realms_found
                                                                                                                              !=
                                                                                                                              0
                                                                                                                 {
                                                                                                                    if (*vpninfo).verbose
                                                                                                                           >=
                                                                                                                           3i32
                                                                                                                       {
                                                                                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                                3i32,
                                                                                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char,
                                                                                                                                                                                                 b"Pulse realm choice\n\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char));
                                                                                                                    }
                                                                                                                    ret
                                                                                                                        =
                                                                                                                        pulse_request_realm_choice(vpninfo,
                                                                                                                                                   reqbuf,
                                                                                                                                                   realms_found,
                                                                                                                                                   eap);
                                                                                                                    if ret
                                                                                                                           !=
                                                                                                                           0
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            805098095739922543;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                } else if j2_found
                                                                                                                              !=
                                                                                                                              0
                                                                                                                 {
                                                                                                                    if (*vpninfo).verbose
                                                                                                                           >=
                                                                                                                           3i32
                                                                                                                       {
                                                                                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                                3i32,
                                                                                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char,
                                                                                                                                                                                                 b"Pulse password auth request, code 0x%02x\n\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char),
                                                                                                                                                                                j2_code
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int);
                                                                                                                    }
                                                                                                                    /* Present user/password form to user */
                                                                                                                    ret
                                                                                                                        =
                                                                                                                        pulse_request_user_auth(vpninfo,
                                                                                                                                                reqbuf,
                                                                                                                                                eap2_ident,
                                                                                                                                                prompt_flags,
                                                                                                                                                if prompt_flags
                                                                                                                                                       &
                                                                                                                                                       1i32
                                                                                                                                                       !=
                                                                                                                                                       0
                                                                                                                                                   {
                                                                                                                                                    user_prompt
                                                                                                                                                } else {
                                                                                                                                                    user2_prompt
                                                                                                                                                },
                                                                                                                                                if prompt_flags
                                                                                                                                                       &
                                                                                                                                                       1i32
                                                                                                                                                       !=
                                                                                                                                                       0
                                                                                                                                                   {
                                                                                                                                                    pass_prompt
                                                                                                                                                } else {
                                                                                                                                                    pass2_prompt
                                                                                                                                                });
                                                                                                                    if ret
                                                                                                                           !=
                                                                                                                           0
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            805098095739922543;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                } else if gtc_found
                                                                                                                              !=
                                                                                                                              0
                                                                                                                 {
                                                                                                                    if (*vpninfo).verbose
                                                                                                                           >=
                                                                                                                           3i32
                                                                                                                       {
                                                                                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                                3i32,
                                                                                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char,
                                                                                                                                                                                                 b"Pulse password general token code request\n\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char));
                                                                                                                    }
                                                                                                                    /* Present user/password form to user */
                                                                                                                    ret
                                                                                                                        =
                                                                                                                        pulse_request_gtc(vpninfo,
                                                                                                                                          reqbuf,
                                                                                                                                          eap2_ident,
                                                                                                                                          prompt_flags,
                                                                                                                                          if prompt_flags
                                                                                                                                                 &
                                                                                                                                                 1i32
                                                                                                                                                 !=
                                                                                                                                                 0
                                                                                                                                             {
                                                                                                                                              user_prompt
                                                                                                                                          } else {
                                                                                                                                              user2_prompt
                                                                                                                                          },
                                                                                                                                          if prompt_flags
                                                                                                                                                 &
                                                                                                                                                 1i32
                                                                                                                                                 !=
                                                                                                                                                 0
                                                                                                                                             {
                                                                                                                                              pass_prompt
                                                                                                                                          } else {
                                                                                                                                              pass2_prompt
                                                                                                                                          },
                                                                                                                                          gtc_prompt);
                                                                                                                    if ret
                                                                                                                           !=
                                                                                                                           0
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            805098095739922543;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                } else if old_sessions
                                                                                                                              !=
                                                                                                                              0
                                                                                                                 {
                                                                                                                    if (*vpninfo).verbose
                                                                                                                           >=
                                                                                                                           3i32
                                                                                                                       {
                                                                                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                                3i32,
                                                                                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char,
                                                                                                                                                                                                 b"Pulse session limit, %d sessions\n\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char),
                                                                                                                                                                                old_sessions);
                                                                                                                    }
                                                                                                                    ret
                                                                                                                        =
                                                                                                                        pulse_request_session_kill(vpninfo,
                                                                                                                                                   reqbuf,
                                                                                                                                                   old_sessions,
                                                                                                                                                   eap);
                                                                                                                    if ret
                                                                                                                           !=
                                                                                                                           0
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            805098095739922543;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                } else if !signin_prompt.is_null()
                                                                                                                 {
                                                                                                                    buf_append_avp_be32(reqbuf,
                                                                                                                                        0xd7ci32
                                                                                                                                            as
                                                                                                                                            uint32_t,
                                                                                                                                        1i32
                                                                                                                                            as
                                                                                                                                            uint32_t);
                                                                                                                } else {
                                                                                                                    if (*vpninfo).verbose
                                                                                                                           >=
                                                                                                                           0i32
                                                                                                                       {
                                                                                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                                                0i32,
                                                                                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char,
                                                                                                                                                                                                 b"Unhandled Pulse auth request\n\x00"
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const u8
                                                                                                                                                                                                     as
                                                                                                                                                                                                     *const libc::c_char));
                                                                                                                    }
                                                                                                                    current_block
                                                                                                                        =
                                                                                                                        7591608735147044776;
                                                                                                                    continue
                                                                                                                        ;
                                                                                                                }
                                                                                                                /* If we get here, something has filled in the next response */
                                                                                                                buf_fill_eap_len(reqbuf,
                                                                                                                                 eap_ofs);
                                                                                                                ret
                                                                                                                    =
                                                                                                                    send_eap_packet(vpninfo,
                                                                                                                                    ttls,
                                                                                                                                    reqbuf);
                                                                                                                if ret
                                                                                                                       !=
                                                                                                                       0
                                                                                                                   {
                                                                                                                    current_block
                                                                                                                        =
                                                                                                                        805098095739922543;
                                                                                                                    break
                                                                                                                        ;
                                                                                                                } else {
                                                                                                                    current_block
                                                                                                                        =
                                                                                                                        3260529072242652013;
                                                                                                                    continue
                                                                                                                        ;
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            /* We're done, but need to send an empty response to the above information
	 * in order that the EAP session can complete with 'success'. Not quite
	 * sure why they didn't send it as payload on the success frame, mind you. */
                                                                                                            buf_fill_eap_len(reqbuf,
                                                                                                                             eap_ofs);
                                                                                                            ret
                                                                                                                =
                                                                                                                send_eap_packet(vpninfo,
                                                                                                                                ttls,
                                                                                                                                reqbuf);
                                                                                                            if ret
                                                                                                                   !=
                                                                                                                   0
                                                                                                               {
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    805098095739922543;
                                                                                                                break
                                                                                                                    ;
                                                                                                            } else {
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    11231108819225936288;
                                                                                                                break
                                                                                                                    ;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                _
                                                                                                =>
                                                                                                {
                                                                                                }
                                                                                            }
                                                                                            if (*vpninfo).verbose
                                                                                                   >=
                                                                                                   0i32
                                                                                               {
                                                                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                        0i32,
                                                                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                             as
                                                                                                                                                                             *const u8
                                                                                                                                                                             as
                                                                                                                                                                             *const libc::c_char,
                                                                                                                                                                         b"Unhandled Pulse authentication packet, or authentication failure\n\x00"
                                                                                                                                                                             as
                                                                                                                                                                             *const u8
                                                                                                                                                                             as
                                                                                                                                                                             *const libc::c_char));
                                                                                            }
                                                                                            current_block
                                                                                                =
                                                                                                7591608735147044776;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        match current_block
                                                                            {
                                                                            805098095739922543
                                                                            =>
                                                                            {
                                                                            }
                                                                            _
                                                                            =>
                                                                            {
                                                                                if !ttls.is_null()
                                                                                   {
                                                                                    /* Normally we don't actually send the EAP-TTLS frame until
		 * we're waiting for a response, which allows us to coalesce.
		 * This time, we need to flush the outbound frames. The empty
		 * EAP response (within EAP-TTLS) causes the server to close
		 * the EAP-TTLS session and the next response is plain IF-T/TLS
		 * IFT_CLIENT_AUTH_SUCCESS just like the non-certificate mode. */
                                                                                    pulse_eap_ttls_recv(vpninfo,
                                                                                                        0
                                                                                                            as
                                                                                                            *mut libc::c_void,
                                                                                                        0i32);
                                                                                }
                                                                                ret
                                                                                    =
                                                                                    recv_ift_packet(vpninfo,
                                                                                                    bytes.as_mut_ptr()
                                                                                                        as
                                                                                                        *mut libc::c_void,
                                                                                                    ::std::mem::size_of::<[libc::c_uchar; 16384]>()
                                                                                                        as
                                                                                                        libc::c_ulong
                                                                                                        as
                                                                                                        libc::c_int);
                                                                                if ret
                                                                                       <
                                                                                       0i32
                                                                                   {
                                                                                    current_block
                                                                                        =
                                                                                        805098095739922543;
                                                                                } else {
                                                                                    if valid_ift_success(bytes.as_mut_ptr(),
                                                                                                         ret)
                                                                                           ==
                                                                                           0
                                                                                       {
                                                                                        if (*vpninfo).verbose
                                                                                               >=
                                                                                               0i32
                                                                                           {
                                                                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                                                    0i32,
                                                                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                                                                         as
                                                                                                                                                                         *const u8
                                                                                                                                                                         as
                                                                                                                                                                         *const libc::c_char,
                                                                                                                                                                     b"Unexpected response instead of IF-T/TLS auth success:\n\x00"
                                                                                                                                                                         as
                                                                                                                                                                         *const u8
                                                                                                                                                                         as
                                                                                                                                                                         *const libc::c_char));
                                                                                        }
                                                                                        dump_buf_hex(vpninfo,
                                                                                                     0i32,
                                                                                                     '<'
                                                                                                         as
                                                                                                         i32
                                                                                                         as
                                                                                                         libc::c_char,
                                                                                                     bytes.as_mut_ptr()
                                                                                                         as
                                                                                                         *mut libc::c_void
                                                                                                         as
                                                                                                         *mut libc::c_uchar,
                                                                                                     ret);
                                                                                        ret
                                                                                            =
                                                                                            -22i32
                                                                                    } else {
                                                                                        ret
                                                                                            =
                                                                                            0i32
                                                                                    }
                                                                                    current_block
                                                                                        =
                                                                                        805098095739922543;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    match current_block {
                                                        805098095739922543 =>
                                                        {
                                                        }
                                                        _ => {
                                                            if (*vpninfo).verbose
                                                                   >= 0i32 {
                                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                        0i32,
                                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                                             as
                                                                                                                                             *const u8
                                                                                                                                             as
                                                                                                                                             *const libc::c_char,
                                                                                                                                         b"Unexpected IF-T/TLS authentication challenge:\n\x00"
                                                                                                                                             as
                                                                                                                                             *const u8
                                                                                                                                             as
                                                                                                                                             *const libc::c_char));
                                                            }
                                                            dump_buf_hex(vpninfo,
                                                                         0i32,
                                                                         '<'
                                                                             as
                                                                             i32
                                                                             as
                                                                             libc::c_char,
                                                                         bytes.as_mut_ptr()
                                                                             as
                                                                             *mut libc::c_void
                                                                             as
                                                                             *mut libc::c_uchar,
                                                                         ret);
                                                            ret = -22i32
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ret != 0 { openconnect_close_https(vpninfo, 0i32); }
    buf_free(reqbuf);
    if !ttls.is_null() { destroy_eap_ttls(vpninfo, ttls); }
    buf_free((*vpninfo).ttls_pushbuf);
    (*vpninfo).ttls_pushbuf = 0 as *mut oc_text_buf;
    free((*vpninfo).ttls_recvbuf as *mut libc::c_void);
    (*vpninfo).ttls_recvbuf = 0 as *mut libc::c_uchar;
    free(user_prompt as *mut libc::c_void);
    free(pass_prompt as *mut libc::c_void);
    free(user2_prompt as *mut libc::c_void);
    free(pass2_prompt as *mut libc::c_void);
    free(gtc_prompt as *mut libc::c_void);
    free(signin_prompt as *mut libc::c_void);
    return ret;
}
#[no_mangle]
#[src_loc = "1798:1"]
pub unsafe extern "C" fn pulse_eap_ttls_send(mut vpninfo:
                                                 *mut openconnect_info,
                                             mut data: *const libc::c_void,
                                             mut len: libc::c_int)
 -> libc::c_int {
    let mut buf: *mut oc_text_buf = (*vpninfo).ttls_pushbuf;
    if buf.is_null() {
        (*vpninfo).ttls_pushbuf = buf_alloc();
        buf = (*vpninfo).ttls_pushbuf;
        if buf.is_null() { return -12i32 }
    }
    /* We concatenate sent data into a single EAP-TTLS frame which is
	 * sent just before we actually need to read something. */
    if (*buf).pos == 0 {
        buf_append_ift_hdr(buf, 0x5597i32 as uint32_t,
                           6i32 as uint32_t); /* IF-T/TLS Auth Type */
        buf_append_be32(buf, (0xa4ci32 << 8i32 | 1i32) as uint32_t);
        buf_append_eap_hdr(buf, 2i32 as uint8_t, (*vpninfo).ttls_eap_ident,
                           0x15i32 as uint8_t, 0i32 as uint32_t);
        /* Flags byte for EAP-TTLS */
        buf_append_bytes(buf,
                         b"\x00\x00" as *const u8 as *const libc::c_char as
                             *const libc::c_void,
                         1i32); /* else send a continue? */
    }
    buf_append_bytes(buf, data, len);
    return len;
}
#[no_mangle]
#[src_loc = "1822:1"]
pub unsafe extern "C" fn pulse_eap_ttls_recv(mut vpninfo:
                                                 *mut openconnect_info,
                                             mut data: *mut libc::c_void,
                                             mut len: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut pushbuf: *mut oc_text_buf = (*vpninfo).ttls_pushbuf;
    let mut ret: libc::c_int = 0;
    if (*vpninfo).ttls_recvlen == 0 {
        let mut flags: uint8_t = 0;
        if !pushbuf.is_null() && buf_error(pushbuf) == 0 &&
               (*pushbuf).pos != 0 {
            buf_fill_eap_len(pushbuf, 0x14i32);
            ret = send_ift_packet(vpninfo, pushbuf);
            if ret != 0 { return ret }
            buf_truncate(pushbuf);
        }
        if len == 0 { return 0i32 }
        (*vpninfo).ttls_recvlen =
            (*vpninfo).ssl_read.expect("non-null function pointer")(vpninfo,
                                                                    (*vpninfo).ttls_recvbuf
                                                                        as
                                                                        *mut libc::c_void
                                                                        as
                                                                        *mut libc::c_char,
                                                                    16384i32
                                                                        as
                                                                        size_t);
        if (*vpninfo).ttls_recvlen > 0i32 && (*vpninfo).dump_http_traffic != 0
           {
            if (*vpninfo).verbose >= 3i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        3i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Read %d bytes of IF-T/TLS EAP-TTLS record\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        (*vpninfo).ttls_recvlen);
            }
            dump_buf_hex(vpninfo, 3i32, '<' as i32 as libc::c_char,
                         (*vpninfo).ttls_recvbuf as *mut libc::c_void as
                             *mut libc::c_uchar, (*vpninfo).ttls_recvlen);
        }
        if valid_ift_auth_eap((*vpninfo).ttls_recvbuf,
                              (*vpninfo).ttls_recvlen) == 0 ||
               (*vpninfo).ttls_recvlen < 0x1ai32 ||
               *(*vpninfo).ttls_recvbuf.offset(0x18i32 as isize) as
                   libc::c_int != 0x15i32 {
            current_block = 711171034806511900;
        } else {
            (*vpninfo).ttls_eap_ident =
                *(*vpninfo).ttls_recvbuf.offset(0x15i32 as isize);
            flags = *(*vpninfo).ttls_recvbuf.offset(0x19i32 as isize);
            if flags as libc::c_int & 0x7fi32 != 0 {
                current_block = 711171034806511900;
            } else if flags as libc::c_int & 0x80i32 != 0 {
                /* Length bit. */
                if (*vpninfo).ttls_recvlen < 0x1ei32 ||
                       load_be32((*vpninfo).ttls_recvbuf.offset(0x1ai32 as
                                                                    isize) as
                                     *const libc::c_void) !=
                           ((*vpninfo).ttls_recvlen - 0x1ei32) as libc::c_uint
                   {
                    current_block = 711171034806511900;
                } else {
                    (*vpninfo).ttls_recvpos = 0x1ei32;
                    (*vpninfo).ttls_recvlen -= 0x1ei32;
                    current_block = 11385396242402735691;
                }
            } else {
                (*vpninfo).ttls_recvpos = 0x1ai32;
                (*vpninfo).ttls_recvlen -= 0x1ai32;
                current_block = 11385396242402735691;
            }
        }
        match current_block {
            11385396242402735691 => { }
            _ => {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Bad EAP-TTLS packet\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                return -5i32
            }
        }
    }
    if len > (*vpninfo).ttls_recvlen {
        memcpy(data,
               (*vpninfo).ttls_recvbuf.offset((*vpninfo).ttls_recvpos as
                                                  isize) as
                   *const libc::c_void,
               (*vpninfo).ttls_recvlen as libc::c_ulong);
        len = (*vpninfo).ttls_recvlen;
        (*vpninfo).ttls_recvlen = 0i32;
        return len
    }
    memcpy(data,
           (*vpninfo).ttls_recvbuf.offset((*vpninfo).ttls_recvpos as isize) as
               *const libc::c_void, len as libc::c_ulong);
    (*vpninfo).ttls_recvpos += len;
    (*vpninfo).ttls_recvlen -= len;
    return len;
}
#[no_mangle]
#[src_loc = "1889:1"]
pub unsafe extern "C" fn pulse_obtain_cookie(mut vpninfo:
                                                 *mut openconnect_info)
 -> libc::c_int {
    return pulse_authenticate(vpninfo, 0i32);
}
/* Example config packet:
   < 0000: 00 00 0a 4c 00 00 00 01  00 00 01 80 00 00 01 fb  |...L............|
   < 0010: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
   < 0020: 2c 20 f0 00 00 00 00 00  00 00 01 70 2e 00 00 78  |, .........p...x|
   < 0030: 07 00 00 00 07 00 00 10  00 00 ff ff 05 05 00 00  |................|
   < 0040: 05 05 ff ff 07 00 00 10  00 00 ff ff 07 00 00 00  |................|
   < 0050: 07 00 00 ff 07 00 00 10  00 00 ff ff 08 08 08 08  |................|
   < 0060: 08 08 08 08 f1 00 00 10  00 00 ff ff 06 06 06 06  |................|
   < 0070: 06 06 06 07 f1 00 00 10  00 00 ff ff 09 09 09 09  |................|
   < 0080: 09 09 09 09 f1 00 00 10  00 00 ff ff 0a 0a 0a 0a  |................|
   < 0090: 0a 0a 0a 0a f1 00 00 10  00 00 ff ff 0b 0b 0b 0b  |................|
   < 00a0: 0b 0b 0b 0b 00 00 00 dc  03 00 00 00 40 00 00 01  |............@...|
   < 00b0: 00 40 01 00 01 00 40 1f  00 01 00 40 20 00 01 00  |.@....@....@ ...|
   < 00c0: 40 21 00 01 00 40 05 00  04 00 00 05 78 00 03 00  |@!...@......x...|
   < 00d0: 04 08 08 08 08 00 03 00  04 08 08 04 04 40 06 00  |.............@..|
   < 00e0: 0c 70 73 65 63 75 72 65  2e 6e 65 74 00 40 07 00  |.psecure.net.@..|
   < 00f0: 04 00 00 00 00 00 04 00  04 01 01 01 01 40 19 00  |.............@..|
   < 0100: 01 01 40 1a 00 01 00 40  0f 00 02 00 00 40 10 00  |..@....@.....@..|
   < 0110: 02 00 05 40 11 00 02 00  02 40 12 00 04 00 00 04  |...@.....@......|
   < 0120: b0 40 13 00 04 00 00 00  00 40 14 00 04 00 00 00  |.@.......@......|
   < 0130: 01 40 15 00 04 00 00 00  00 40 16 00 02 11 94 40  |.@.......@.....@|
   < 0140: 17 00 04 00 00 00 0f 40  18 00 04 00 00 00 3c 00  |.......@......<.|
   < 0150: 01 00 04 0a 14 03 01 00  02 00 04 ff ff ff ff 40  |...............@|
   < 0160: 0b 00 04 0a c8 c8 c8 40  0c 00 01 00 40 0d 00 01  |.......@....@...|
   < 0170: 00 40 0e 00 01 00 40 1b  00 01 00 40 1c 00 01 00  |.@....@....@....|

   It starts as an IF-T/TLS packet of type Juniper/1.

   Lots of zeroes at the start, and at 0x20 there is a distinctive 0x2c20f000
   signature which appears to be in all config packets.

   At 0x28 it has the payload length (0x10 less than the full IF-T length).
   0x2c is the start of the routing information. The 0x2e byte always
   seems to be there, and in this example 0x78 is the length of the
   routing information block. The number of entries is in byte 0x30.
   In the absence of IPv6 perhaps, the length at 0x2c seems always to be
   the number of entries (in 0x30) * 0x10 + 8.

   Routing entries are 0x10 bytes each, starting at 0x34. The ones starting
   with 0x07 are include, with 0xf1 are exclude. No idea what the following 7
   bytes 0f 00 00 10 00 00 ff ff mean; perhaps the 0010 is a length? The IP
   address range is in bytes 8-11 (starting address) and the highest address
   of the range (traditionally a broadcast address) is in bytes 12-15.

   After the routing inforamation (in this example at 0xa4) comes another
   length field, this time for the information elements which comprise
   the rest of the packet. Not sure what the 03 00 00 00 at 0xa8 means;
   it *could* be an element type 0x3000 with payload length zero but if it
   is, we don't know what it means. Following that, the elements all have
   two bytes of type followed by two bytes length, then their payload.

   There follows an attempt to parse the packet based on the above
   understanding. Having more examples, especially with IPv6 split includes
   and excludes, would be useful...
*/
#[src_loc = "1949:1"]
unsafe extern "C" fn handle_main_config_packet(mut vpninfo:
                                                   *mut openconnect_info,
                                               mut bytes: *mut libc::c_uchar,
                                               mut len: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut routes_len: libc::c_int = 0i32;
    let mut l: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* First part of header, similar to ESP, has already been checked */
    if !(len < 0x31i32 ||
             load_be16(bytes.offset(0x2ci32 as isize) as *const libc::c_void)
                 as libc::c_int != 0x2e00i32 ||
             {
                 routes_len =
                     load_be16(bytes.offset(0x2ei32 as isize) as
                                   *const libc::c_void) as
                         libc::c_int; /* The header including length and number of routes */
                 (routes_len) !=
                     *bytes.offset(0x30i32 as isize) as libc::c_int * 0x10i32
                         + 8i32
             } || len < 0x2ci32 + routes_len + 4i32 ||
             load_be32(bytes.offset(0x2ci32 as
                                        isize).offset(routes_len as isize) as
                           *const libc::c_void).wrapping_add(routes_len as
                                                                 libc::c_uint).wrapping_add(0x2ci32
                                                                                                as
                                                                                                libc::c_uint)
                 != len as libc::c_uint) {
        p = bytes.offset(0x34i32 as isize);
        routes_len -= 8i32;
        loop 
             /* We know it's a multiple of 0x10 now. We checked. */
             {
            if !(routes_len != 0) {
                current_block = 7990025728955927862;
                break ;
            }
            let mut buf: [libc::c_char; 80] = [0; 80];
            /* Probably not a whole be32 but let's see if anything ever changes */
            let mut type_0: uint32_t = load_be32(p as *const libc::c_void);
            let mut ffff: uint32_t =
                load_be32(p.offset(4) as *const libc::c_void);
            if ffff != 0xffffi32 as libc::c_uint {
                current_block = 15560111600559154999;
                break ;
            }
            /* Convert the range end into a netmask by xor. Mask out the
		 * bits in the network address, leaving only the low bits set,
		 * then invert what's left so that only the high bits are set
		 * as in a normal netmask.
		 *
		 * e.g.
		 * 10.0.0.0-10.0.63.255 becomes 0.0.63.255 becomes 255.255.192.0
		*/
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 80]>() as
                         libc::c_ulong,
                     b"%d.%d.%d.%d/%d.%d.%d.%d\x00" as *const u8 as
                         *const libc::c_char, *p.offset(8) as libc::c_int,
                     *p.offset(9) as libc::c_int,
                     *p.offset(10) as libc::c_int,
                     *p.offset(11) as libc::c_int,
                     255i32 ^
                         (*p.offset(8) as libc::c_int ^
                              *p.offset(12) as libc::c_int),
                     255i32 ^
                         (*p.offset(9) as libc::c_int ^
                              *p.offset(13) as libc::c_int),
                     255i32 ^
                         (*p.offset(10) as libc::c_int ^
                              *p.offset(14) as libc::c_int),
                     255i32 ^
                         (*p.offset(11) as libc::c_int ^
                              *p.offset(15) as libc::c_int));
            if type_0 == 0x7000010i32 as libc::c_uint {
                let mut inc: *mut oc_split_include =
                    0 as *mut oc_split_include;
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received split include route %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                inc =
                    malloc(::std::mem::size_of::<oc_split_include>() as
                               libc::c_ulong) as *mut oc_split_include;
                if !inc.is_null() {
                    (*inc).route =
                        add_option(vpninfo,
                                   b"split-include\x00" as *const u8 as
                                       *const libc::c_char, buf.as_mut_ptr(),
                                   -1i32);
                    if !(*inc).route.is_null() {
                        (*inc).next = (*vpninfo).ip_info.split_includes;
                        (*vpninfo).ip_info.split_includes = inc
                    } else { free(inc as *mut libc::c_void); }
                }
            } else if type_0 == 0xf1000010u32 {
                let mut exc: *mut oc_split_include =
                    0 as *mut oc_split_include;
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Received split exclude route %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                exc =
                    malloc(::std::mem::size_of::<oc_split_include>() as
                               libc::c_ulong) as *mut oc_split_include;
                if !exc.is_null() {
                    (*exc).route =
                        add_option(vpninfo,
                                   b"split-exclude\x00" as *const u8 as
                                       *const libc::c_char, buf.as_mut_ptr(),
                                   -1i32);
                    if !(*exc).route.is_null() {
                        (*exc).next = (*vpninfo).ip_info.split_excludes;
                        (*vpninfo).ip_info.split_excludes = exc
                    } else { free(exc as *mut libc::c_void); }
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
                                                                                             b"Receive route of unknown type 0x%08x\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            type_0);
                }
                current_block = 15560111600559154999;
                break ;
            }
            p = p.offset(0x10i32 as isize);
            routes_len -= 0x10i32
        }
        match current_block {
            15560111600559154999 => { }
            _ => {
                /* p now points at the length field of the final elements, which
	   was already checked. */
                l = load_be32(p as *const libc::c_void) as libc::c_int;
                /* No idea what this is */
                if !(l < 8i32 ||
                         load_be32(p.offset(4) as *const libc::c_void) !=
                             0x3000000i32 as libc::c_uint) {
                    p = p.offset(8);
                    l -= 8i32;
                    loop  {
                        if !(l != 0) {
                            current_block = 5181772461570869434;
                            break ;
                        }
                        let mut type_1: uint16_t =
                            load_be16(p as *const libc::c_void);
                        let mut attrlen: uint16_t =
                            load_be16(p.offset(2) as *const libc::c_void);
                        if attrlen as libc::c_int + 4i32 > l {
                            current_block = 15560111600559154999;
                            break ;
                        }
                        p = p.offset(4);
                        l -= 4i32;
                        process_attr(vpninfo, type_1, p,
                                     attrlen as libc::c_int);
                        p = p.offset(attrlen as libc::c_int as isize);
                        l -= attrlen as libc::c_int;
                        if l != 0 && l < 4i32 {
                            current_block = 15560111600559154999;
                            break ;
                        }
                    }
                    match current_block {
                        15560111600559154999 => { }
                        _ => { return 0i32 }
                    }
                }
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
                                                                                 b"Unexpected Pulse config packet:\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
    }
    dump_buf_hex(vpninfo, 0i32, '<' as i32 as libc::c_char,
                 bytes as *mut libc::c_void as *mut libc::c_uchar, len);
    return -22i32;
}
/* Example ESP config packet:
   < 0000:  00 00 0a 4c 00 00 00 01  00 00 00 80 00 00 01 fc  |...L............|
   < 0010:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
   < 0020:  21 20 24 00 00 00 00 00  00 00 00 70 00 00 00 54  |! $........p...T|
   < 0030:  01 00 00 00 ec 52 1b 6c  00 40 11 9d c5 f6 85 f3  |.....R.l.@......|
   < 0040:  26 7d 70 75 44 45 63 eb  64 00 fb ba 89 4f 24 b2  |&}puDEc.d....O$.|
   < 0050:  81 42 ce 24 b8 0a f8 b6  71 39 78 f8 5e 6f 5f d6  |.B.$....q9x.^o_.|
   < 0060:  9e 5c 06 47 8d 1e f3 0e  5a 51 ae b2 3d 09 8d 27  |.\.G....ZQ..=..'|
   < 0070:  e0 50 76 6a 22 9a d1 20  86 78 00 00 00 00 00 00  |.Pvj".. .x......|

   First 0x2c bytes are like the main config packet header.

   At 0x2c there is another length field, covering the whole of the
   rest of this packet.  Then an unknown 0x01000000 at 0x30, followed
   by the server->client SPI in little-endian(!) form at 0x34.

   Then follows the secrets, with a 2-byte length field at 0x38 (which
   is always 0x40), followed by the secrets themselves. As with
   Juniper Network Connect, the HMAC secret immediately follows the
   encryption key, however large the latter is.
*/
#[src_loc = "2082:1"]
unsafe extern "C" fn handle_esp_config_packet(mut vpninfo:
                                                  *mut openconnect_info,
                                              mut bytes: *mut libc::c_uchar,
                                              mut len: libc::c_int)
 -> libc::c_int {
    let mut esp: *mut esp = 0 as *mut esp;
    let mut secretslen: libc::c_int = 0;
    let mut spi: uint32_t = 0;
    let mut ret: libc::c_int = 0;
    if len < 0x6ai32 ||
           load_be32(bytes.offset(0x2ci32 as isize) as *const libc::c_void) !=
               (len - 0x2ci32) as libc::c_uint ||
           load_be32(bytes.offset(0x30i32 as isize) as *const libc::c_void) !=
               0x1000000i32 as libc::c_uint ||
           load_be16(bytes.offset(0x38i32 as isize) as *const libc::c_void) as
               libc::c_int != 0x40i32 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Invalid ESP config packet:\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        dump_buf_hex(vpninfo, 0i32, '<' as i32 as libc::c_char, bytes, len);
        return -22i32
    }
    /* We insist on this being 0x40 for now. But just in case it later changes... */
    secretslen =
        load_be16(bytes.offset(0x38i32 as isize) as *const libc::c_void) as
            libc::c_int;
    if (*vpninfo).verbose >= 2i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                2i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"%d bytes of ESP secrets\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                secretslen);
    }
    if (*vpninfo).enc_key_len == 0 || (*vpninfo).hmac_key_len == 0 ||
           (*vpninfo).enc_key_len + (*vpninfo).hmac_key_len > secretslen {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Invalid ESP setup\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        return -22i32
    }
    /* Yes, bizarrely this is little-endian on the wire. I have no idea
	 * what made them do this. */
    spi = load_le32(bytes.offset(0x34i32 as isize) as *const libc::c_void);
    if (*vpninfo).verbose >= 2i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                2i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"ESP SPI (outbound): %x\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                spi);
    }
    /* But we store it internally as big-endian because we never do any
	 * calculations on it; it's only set into outbound packets and matched
	 * on incoming ones... and we've NEVER had to see it in little-endian
	 * form ever before because that's insane! */
    store_be32(&mut (*vpninfo).esp_out.spi as *mut uint32_t as
                   *mut libc::c_void, spi);
    memcpy((*vpninfo).esp_out.enc_key.as_mut_ptr() as *mut libc::c_void,
           bytes.offset(0x3ai32 as isize) as *const libc::c_void,
           (*vpninfo).enc_key_len as libc::c_ulong);
    memcpy((*vpninfo).esp_out.hmac_key.as_mut_ptr() as *mut libc::c_void,
           bytes.offset(0x3ai32 as
                            isize).offset((*vpninfo).enc_key_len as isize) as
               *const libc::c_void, (*vpninfo).hmac_key_len as libc::c_ulong);
    ret = openconnect_setup_esp_keys(vpninfo, 1i32);
    if ret != 0 { return ret }
    esp =
        &mut *(*vpninfo).esp_in.as_mut_ptr().offset((*vpninfo).current_esp_in
                                                        as isize) as *mut esp;
    /* Now, using the buffer in which we received the original packet (which
	 * we trust our caller made large enough), create an appropriate reply.
	 * A reply packet contains two sets of ESP information, as we are expected
	 * to send our own followed by a copy of what the server sent to us. */
    /* Adjust the length in the IF-T/TLS header */
    store_be32(bytes.offset(8) as *mut libc::c_void,
               (0x40i32 + 2i32 * secretslen) as uint32_t);
    /* Copy the server's own ESP information into place */
    memmove(bytes.offset(secretslen as isize).offset(0x3ai32 as isize) as
                *mut libc::c_void,
            bytes.offset(0x34i32 as isize) as *const libc::c_void,
            (secretslen + 0x6i32) as libc::c_ulong);
    /* Adjust other length fields. */
    store_be32(bytes.offset(0x28i32 as isize) as *mut libc::c_void,
               (0x30i32 + 2i32 * secretslen) as uint32_t);
    store_be32(bytes.offset(0x2ci32 as isize) as *mut libc::c_void,
               (0x14i32 + 2i32 * secretslen) as uint32_t);
    /* Store the SPI. Bizarrely little-endian again. */
    store_le32(bytes.offset(0x34i32 as isize) as *mut libc::c_void,
               load_be32(&mut (*esp).spi as *mut uint32_t as
                             *const libc::c_void));
    memcpy(bytes.offset(0x3ai32 as isize) as *mut libc::c_void,
           (*esp).enc_key.as_mut_ptr() as *const libc::c_void,
           (*vpninfo).enc_key_len as libc::c_ulong);
    memcpy(bytes.offset(0x3ai32 as
                            isize).offset((*vpninfo).enc_key_len as isize) as
               *mut libc::c_void,
           (*esp).hmac_key.as_mut_ptr() as *const libc::c_void,
           (*vpninfo).hmac_key_len as libc::c_ulong);
    memset(bytes.offset(0x3ai32 as
                            isize).offset((*vpninfo).enc_key_len as
                                              isize).offset((*vpninfo).hmac_key_len
                                                                as isize) as
               *mut libc::c_void, 0i32,
           (0x40i32 - (*vpninfo).enc_key_len - (*vpninfo).hmac_key_len) as
               libc::c_ulong);
    return 0i32;
}
#[no_mangle]
#[src_loc = "2160:1"]
pub unsafe extern "C" fn pulse_connect(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut current_block: u64;
    let mut reqbuf: *mut oc_text_buf = 0 as *mut oc_text_buf;
    let mut bytes: [libc::c_uchar; 16384] = [0; 16384];
    let mut ret: libc::c_int = 0;
    /* If we already have a channel open, it's because we have just
	 * successfully authenticated on it from pulse_obtain_cookie(). */
    if (*vpninfo).ssl_fd == -1i32 {
        ret = pulse_authenticate(vpninfo, 1i32);
        if ret != 0 { return ret }
    }
    loop  {
        let mut pkt_type: uint32_t = 0;
        ret =
            recv_ift_packet(vpninfo, bytes.as_mut_ptr() as *mut libc::c_void,
                            ::std::mem::size_of::<[libc::c_uchar; 16384]>() as
                                libc::c_ulong as libc::c_int);
        if ret < 0i32 { return ret }
        if ret < 16i32 ||
               load_be32(bytes.as_mut_ptr().offset(8) as *const libc::c_void)
                   != ret as libc::c_uint {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Bad IF-T/TLS packet when expecting configuration:\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            dump_buf_hex(vpninfo, 0i32, '<' as i32 as libc::c_char,
                         bytes.as_mut_ptr(), ret);
            return -22i32
        }
        if !(load_be32(bytes.as_mut_ptr() as *const libc::c_void) !=
                 0xa4ci32 as libc::c_uint) {
            pkt_type =
                load_be32(bytes.as_mut_ptr().offset(4) as
                              *const libc::c_void);
            /* End of configuration? Seems to have a 4-byte payload of zeroes. */
            if pkt_type == 0x8fi32 as libc::c_uint { break ; }
            /* The main and ESP config packets both start like this. The word at
		 * 0x20 is 0x2c20f000 for config and 0x0x21202400 for ESP, and the word
		 * at 0x2c is the length of the payload (0x10 less than the overall
		 * length including (and in) the IF-T/TLS header. e.g 0x170 here:
		 *
		 * < 0000: 00 00 0a 4c 00 00 00 01  00 00 01 80 00 00 01 fb  |...L............|
		 * < 0010: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
		 * < 0020: 2c 20 f0 00 00 00 00 00  00 00 01 70 ...          |, .........|
		 */
            if !(pkt_type != 1i32 as libc::c_uint || ret < 0x2ci32 ||
                     load_be32(bytes.as_mut_ptr().offset(0x10i32 as isize) as
                                   *const libc::c_void) != 0 ||
                     load_be32(bytes.as_mut_ptr().offset(0x14i32 as isize) as
                                   *const libc::c_void) != 0 ||
                     load_be32(bytes.as_mut_ptr().offset(0x18i32 as isize) as
                                   *const libc::c_void) != 0 ||
                     load_be32(bytes.as_mut_ptr().offset(0x1ci32 as isize) as
                                   *const libc::c_void) != 0 ||
                     load_be32(bytes.as_mut_ptr().offset(0x24i32 as isize) as
                                   *const libc::c_void) != 0 ||
                     load_be32(bytes.as_mut_ptr().offset(0x28i32 as isize) as
                                   *const libc::c_void) !=
                         (ret - 0x10i32) as libc::c_uint) {
                match load_be32(bytes.as_mut_ptr().offset(0x20i32 as isize) as
                                    *const libc::c_void) {
                    740356096 => {
                        current_block = 3837793900499220879;
                        match current_block {
                            3837793900499220879 => {
                                ret =
                                    handle_main_config_packet(vpninfo,
                                                              bytes.as_mut_ptr(),
                                                              ret);
                                if ret != 0 { return ret }
                                continue ;
                            }
                            _ => {
                                ret =
                                    handle_esp_config_packet(vpninfo,
                                                             bytes.as_mut_ptr(),
                                                             ret);
                                if ret != 0 {
                                    (*vpninfo).dtls_state = 2i32;
                                    continue ;
                                } else {
                                    /* It has created a response packet to send. */
                                    ret =
                                        send_ift_bytes(vpninfo,
                                                       bytes.as_mut_ptr() as
                                                           *mut libc::c_void,
                                                       load_be32(bytes.as_mut_ptr().offset(8)
                                                                     as
                                                                     *const libc::c_void)
                                                           as libc::c_int);
                                    if ret != 0 { return ret }
                                    /* Tell server to enable ESP handling */
                                    reqbuf = buf_alloc();
                                    buf_append_ift_hdr(reqbuf,
                                                       0xa4ci32 as uint32_t,
                                                       5i32 as uint32_t);
                                    buf_append(reqbuf,
                                               b"ncmo=1\n%c\x00" as *const u8
                                                   as *const libc::c_char,
                                               0i32);
                                    ret = send_ift_packet(vpninfo, reqbuf);
                                    buf_free(reqbuf);
                                    if ret != 0 { return ret }
                                    continue ;
                                }
                            }
                        }
                    }
                    555754496 => {
                        current_block = 10706535541072142590;
                        match current_block {
                            3837793900499220879 => {
                                ret =
                                    handle_main_config_packet(vpninfo,
                                                              bytes.as_mut_ptr(),
                                                              ret);
                                if ret != 0 { return ret }
                                continue ;
                            }
                            _ => {
                                ret =
                                    handle_esp_config_packet(vpninfo,
                                                             bytes.as_mut_ptr(),
                                                             ret);
                                if ret != 0 {
                                    (*vpninfo).dtls_state = 2i32;
                                    continue ;
                                } else {
                                    ret =
                                        send_ift_bytes(vpninfo,
                                                       bytes.as_mut_ptr() as
                                                           *mut libc::c_void,
                                                       load_be32(bytes.as_mut_ptr().offset(8)
                                                                     as
                                                                     *const libc::c_void)
                                                           as libc::c_int);
                                    if ret != 0 { return ret }
                                    reqbuf = buf_alloc();
                                    buf_append_ift_hdr(reqbuf,
                                                       0xa4ci32 as uint32_t,
                                                       5i32 as uint32_t);
                                    buf_append(reqbuf,
                                               b"ncmo=1\n%c\x00" as *const u8
                                                   as *const libc::c_char,
                                               0i32);
                                    ret = send_ift_packet(vpninfo, reqbuf);
                                    buf_free(reqbuf);
                                    if ret != 0 { return ret }
                                    continue ;
                                }
                            }
                        }
                    }
                    _ => { }
                }
            }
        }
        if (*vpninfo).verbose >= 1i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    1i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Unexpected IF-T/TLS packet when expecting configuration.\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        dump_buf_hex(vpninfo, 2i32, '<' as i32 as libc::c_char,
                     bytes.as_mut_ptr(), ret);
    }
    if (*vpninfo).ip_info.mtu == 0 ||
           (*vpninfo).ip_info.addr.is_null() &&
               (*vpninfo).ip_info.addr6.is_null() {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    b"Insufficient configuration found\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -22i32
    }
    ret = 0i32;
    if (*vpninfo)._select_nfds <= (*vpninfo).ssl_fd {
        (*vpninfo)._select_nfds = (*vpninfo).ssl_fd + 1i32
    }
    let mut __fd: libc::c_int = (*vpninfo).ssl_fd;
    (*vpninfo)._select_rfds.fds_bits[(__fd as
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
            as __int32_t;
    let mut __fd_0: libc::c_int = (*vpninfo).ssl_fd;
    (*vpninfo)._select_efds.fds_bits[(__fd_0 as
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
            as __int32_t;
    free((*vpninfo).cstp_pkt as *mut libc::c_void);
    (*vpninfo).cstp_pkt = 0 as *mut pkt;
    return ret;
}
#[no_mangle]
#[src_loc = "2272:1"]
pub unsafe extern "C" fn pulse_mainloop(mut vpninfo: *mut openconnect_info,
                                        mut timeout: *mut libc::c_int,
                                        mut readable: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut work_done: libc::c_int = 0i32;
    if (*vpninfo).ssl_fd == -1i32 {
        current_block = 14901311691614577181;
    } else { current_block = 14916268686031723178; }
    'c_38118:
        loop  {
            match current_block {
                14916268686031723178 =>
                /* FIXME: The poll() handling here is fairly simplistic. Actually,
	   if the SSL connection stalls it could return a WANT_WRITE error
	   on _either_ of the SSL_read() or SSL_write() calls. In that case,
	   we should probably remove POLLIN from the events we're looking for,
	   and add POLLOUT. As it is, though, it'll just chew CPU time in that
	   fairly unlikely situation, until the write backlog clears. */
                {
                    if readable != 0 {
                        /* Some servers send us packets that are larger than
		   negotiated MTU. We reserve some extra space to
		   handle that */
                        let mut receive_mtu: libc::c_int =
                            if 16384i32 >
                                   if (*vpninfo).deflate_pkt_size != 0 {
                                       (*vpninfo).deflate_pkt_size
                                   } else { (*vpninfo).ip_info.mtu } {
                                16384i32
                            } else if (*vpninfo).deflate_pkt_size != 0 {
                                (*vpninfo).deflate_pkt_size
                            } else { (*vpninfo).ip_info.mtu };
                        let mut pkt: *mut pkt = (*vpninfo).cstp_pkt;
                        let mut len: libc::c_int = 0;
                        let mut payload_len: libc::c_int = 0;
                        if pkt.is_null() {
                            (*vpninfo).cstp_pkt =
                                malloc((::std::mem::size_of::<pkt>() as
                                            libc::c_ulong).wrapping_add(receive_mtu
                                                                            as
                                                                            libc::c_ulong))
                                    as *mut pkt;
                            pkt = (*vpninfo).cstp_pkt;
                            if pkt.is_null() {
                                if (*vpninfo).verbose >= 0i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            0i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"Allocation failed\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char));
                                }
                                current_block = 10853015579903106591;
                            } else { current_block = 13586036798005543211; }
                        } else { current_block = 13586036798005543211; }
                        match current_block {
                            10853015579903106591 => { }
                            _ => {
                                /* Receive packet header, if there's anything there... */
                                len =
                                    ssl_nonblock_read(vpninfo,
                                                      &mut (*pkt).c2rust_unnamed.pulse.vendor
                                                          as *mut uint32_t as
                                                          *mut libc::c_void,
                                                      16i32);
                                if !(len == 0) {
                                    if len < 0i32 {
                                        current_block = 14901311691614577181;
                                        continue ;
                                    }
                                    if len < 16i32 {
                                        if (*vpninfo).verbose >= 0i32 {
                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                    0i32,
                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char,
                                                                                                                     b"Short packet received (%d bytes)\n\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char),
                                                                                                    len);
                                        }
                                        (*vpninfo).quit_reason =
                                            b"Short packet received\x00" as
                                                *const u8 as
                                                *const libc::c_char;
                                        return 1i32
                                    }
                                    /* Packets shouldn't cross SSL record boundaries (we hope!), so if there
		 * was a header there, then rest of that packet should be there too. */
                                    if load_be32(&mut (*pkt).c2rust_unnamed.pulse.len
                                                     as *mut uint32_t as
                                                     *const libc::c_void) >
                                           (receive_mtu + 0x10i32) as
                                               libc::c_uint {
                                        /* This doesn't look right. Pull the rest of the SSL record
			 * and complain about it (which we will, since the length
			 * won't match the header */
                                        len = receive_mtu
                                    } else {
                                        len =
                                            load_be32(&mut (*pkt).c2rust_unnamed.pulse.len
                                                          as *mut uint32_t as
                                                          *const libc::c_void).wrapping_sub(0x10i32
                                                                                                as
                                                                                                libc::c_uint)
                                                as libc::c_int
                                    }
                                    payload_len =
                                        ssl_nonblock_read(vpninfo,
                                                          &mut (*pkt).data as
                                                              *mut [libc::c_uchar; 0]
                                                              as
                                                              *mut libc::c_void,
                                                          len);
                                    if payload_len as libc::c_uint !=
                                           load_be32(&mut (*pkt).c2rust_unnamed.pulse.len
                                                         as *mut uint32_t as
                                                         *const libc::c_void).wrapping_sub(0x10i32
                                                                                               as
                                                                                               libc::c_uint)
                                       {
                                        if payload_len < 0i32 {
                                            len = 0x10i32
                                        } else { len = payload_len + 0x10i32 }
                                    } else if !(load_be32(&mut (*pkt).c2rust_unnamed.pulse.vendor
                                                              as *mut uint32_t
                                                              as
                                                              *const libc::c_void)
                                                    !=
                                                    0xa4ci32 as libc::c_uint)
                                     {
                                        (*vpninfo).ssl_times.last_rx =
                                            time(0 as *mut time_t);
                                        len = payload_len + 0x10i32;
                                        match load_be32(&mut (*pkt).c2rust_unnamed.pulse.type_0
                                                            as *mut uint32_t
                                                            as
                                                            *const libc::c_void)
                                            {
                                            4 => {
                                                current_block =
                                                    14222255915540588208;
                                                match current_block {
                                                    6628960769279281767 => {
                                                        /* It sends the licence information once the connection is set up. For
			 * now, abuse this to deal with the race condition in ESP setup — it looks
			 * like the server doesn't process the ESP config until after we've sent
			 * the probes, in some cases. */
                                                        if (*vpninfo).dtls_state
                                                               == 3i32 {
                                                            (*(*vpninfo).proto).udp_send_probes.expect("non-null function pointer")(vpninfo);
                                                        }
                                                        current_block =
                                                            14916268686031723178;
                                                        continue ;
                                                    }
                                                    1037737505459228 => {
                                                        if !(payload_len <
                                                                 0x6ai32 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x10i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     0x21202400i32
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x18i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     payload_len
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x1ci32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     (payload_len
                                                                          -
                                                                          0x1ci32)
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x20i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     0x1000000i32
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be16((*pkt).data.as_mut_ptr().offset(0x28i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     as
                                                                     libc::c_int
                                                                     !=
                                                                     0x40i32)
                                                           {
                                                            dump_buf_hex(vpninfo,
                                                                         0i32,
                                                                         '<'
                                                                             as
                                                                             i32
                                                                             as
                                                                             libc::c_char,
                                                                         &mut (*(*vpninfo).cstp_pkt).c2rust_unnamed.pulse.vendor
                                                                             as
                                                                             *mut uint32_t
                                                                             as
                                                                             *mut libc::c_void
                                                                             as
                                                                             *mut libc::c_uchar,
                                                                         len);
                                                            ret =
                                                                handle_esp_config_packet(vpninfo,
                                                                                         &mut (*pkt).c2rust_unnamed.pulse.vendor
                                                                                             as
                                                                                             *mut uint32_t
                                                                                             as
                                                                                             *mut libc::c_void
                                                                                             as
                                                                                             *mut libc::c_uchar,
                                                                                         len);
                                                            if ret != 0 {
                                                                if (*vpninfo).verbose
                                                                       >= 0i32
                                                                   {
                                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                            0i32,
                                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                                 as
                                                                                                                                                 *const u8
                                                                                                                                                 as
                                                                                                                                                 *const libc::c_char,
                                                                                                                                             b"ESP rekey failed\n\x00"
                                                                                                                                                 as
                                                                                                                                                 *const u8
                                                                                                                                                 as
                                                                                                                                                 *const libc::c_char));
                                                                }
                                                                (*(*vpninfo).proto).udp_close.expect("non-null function pointer")(vpninfo);
                                                                current_block
                                                                    =
                                                                    14916268686031723178;
                                                                continue ;
                                                            } else {
                                                                (*vpninfo).cstp_pkt
                                                                    =
                                                                    0 as
                                                                        *mut pkt;
                                                                (*pkt).len =
                                                                    load_be32(&mut (*pkt).c2rust_unnamed.pulse.len
                                                                                  as
                                                                                  *mut uint32_t
                                                                                  as
                                                                                  *const libc::c_void).wrapping_sub(16i32
                                                                                                                        as
                                                                                                                        libc::c_uint)
                                                                        as
                                                                        libc::c_int;
                                                                queue_packet(&mut (*vpninfo).oncp_control_queue,
                                                                             pkt);
                                                                print_esp_keys(vpninfo,
                                                                               libintl_dgettext(b"openconnect\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                b"new incoming\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char),
                                                                               &mut *(*vpninfo).esp_in.as_mut_ptr().offset((*vpninfo).current_esp_in
                                                                                                                               as
                                                                                                                               isize));
                                                                print_esp_keys(vpninfo,
                                                                               libintl_dgettext(b"openconnect\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                b"new outgoing\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char),
                                                                               &mut (*vpninfo).esp_out);
                                                                current_block
                                                                    =
                                                                    14916268686031723178;
                                                                continue ;
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        if (*vpninfo).verbose
                                                               >= 3i32 {
                                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                    3i32,
                                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char,
                                                                                                                                     b"Received data packet of %d bytes\n\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char),
                                                                                                                    payload_len);
                                                        }
                                                        dump_buf_hex(vpninfo,
                                                                     3i32,
                                                                     '<' as
                                                                         i32
                                                                         as
                                                                         libc::c_char,
                                                                     &mut (*(*vpninfo).cstp_pkt).c2rust_unnamed.pulse.vendor
                                                                         as
                                                                         *mut uint32_t
                                                                         as
                                                                         *mut libc::c_void
                                                                         as
                                                                         *mut libc::c_uchar,
                                                                     len);
                                                        (*(*vpninfo).cstp_pkt).len
                                                            = payload_len;
                                                        queue_packet(&mut (*vpninfo).incoming_queue,
                                                                     pkt);
                                                        pkt = 0 as *mut pkt;
                                                        (*vpninfo).cstp_pkt =
                                                            pkt;
                                                        work_done = 1i32;
                                                        current_block =
                                                            14916268686031723178;
                                                        continue ;
                                                    }
                                                }
                                            }
                                            1 => {
                                                current_block =
                                                    1037737505459228;
                                                match current_block {
                                                    6628960769279281767 => {
                                                        if (*vpninfo).dtls_state
                                                               == 3i32 {
                                                            (*(*vpninfo).proto).udp_send_probes.expect("non-null function pointer")(vpninfo);
                                                        }
                                                        current_block =
                                                            14916268686031723178;
                                                        continue ;
                                                    }
                                                    1037737505459228 => {
                                                        if !(payload_len <
                                                                 0x6ai32 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x10i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     0x21202400i32
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x18i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     payload_len
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x1ci32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     (payload_len
                                                                          -
                                                                          0x1ci32)
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x20i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     0x1000000i32
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be16((*pkt).data.as_mut_ptr().offset(0x28i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     as
                                                                     libc::c_int
                                                                     !=
                                                                     0x40i32)
                                                           {
                                                            dump_buf_hex(vpninfo,
                                                                         0i32,
                                                                         '<'
                                                                             as
                                                                             i32
                                                                             as
                                                                             libc::c_char,
                                                                         &mut (*(*vpninfo).cstp_pkt).c2rust_unnamed.pulse.vendor
                                                                             as
                                                                             *mut uint32_t
                                                                             as
                                                                             *mut libc::c_void
                                                                             as
                                                                             *mut libc::c_uchar,
                                                                         len);
                                                            ret =
                                                                handle_esp_config_packet(vpninfo,
                                                                                         &mut (*pkt).c2rust_unnamed.pulse.vendor
                                                                                             as
                                                                                             *mut uint32_t
                                                                                             as
                                                                                             *mut libc::c_void
                                                                                             as
                                                                                             *mut libc::c_uchar,
                                                                                         len);
                                                            if ret != 0 {
                                                                if (*vpninfo).verbose
                                                                       >= 0i32
                                                                   {
                                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                            0i32,
                                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                                 as
                                                                                                                                                 *const u8
                                                                                                                                                 as
                                                                                                                                                 *const libc::c_char,
                                                                                                                                             b"ESP rekey failed\n\x00"
                                                                                                                                                 as
                                                                                                                                                 *const u8
                                                                                                                                                 as
                                                                                                                                                 *const libc::c_char));
                                                                }
                                                                (*(*vpninfo).proto).udp_close.expect("non-null function pointer")(vpninfo);
                                                                current_block
                                                                    =
                                                                    14916268686031723178;
                                                                continue ;
                                                            } else {
                                                                (*vpninfo).cstp_pkt
                                                                    =
                                                                    0 as
                                                                        *mut pkt;
                                                                (*pkt).len =
                                                                    load_be32(&mut (*pkt).c2rust_unnamed.pulse.len
                                                                                  as
                                                                                  *mut uint32_t
                                                                                  as
                                                                                  *const libc::c_void).wrapping_sub(16i32
                                                                                                                        as
                                                                                                                        libc::c_uint)
                                                                        as
                                                                        libc::c_int;
                                                                queue_packet(&mut (*vpninfo).oncp_control_queue,
                                                                             pkt);
                                                                print_esp_keys(vpninfo,
                                                                               libintl_dgettext(b"openconnect\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                b"new incoming\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char),
                                                                               &mut *(*vpninfo).esp_in.as_mut_ptr().offset((*vpninfo).current_esp_in
                                                                                                                               as
                                                                                                                               isize));
                                                                print_esp_keys(vpninfo,
                                                                               libintl_dgettext(b"openconnect\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                b"new outgoing\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char),
                                                                               &mut (*vpninfo).esp_out);
                                                                current_block
                                                                    =
                                                                    14916268686031723178;
                                                                continue ;
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        if (*vpninfo).verbose
                                                               >= 3i32 {
                                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                    3i32,
                                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char,
                                                                                                                                     b"Received data packet of %d bytes\n\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char),
                                                                                                                    payload_len);
                                                        }
                                                        dump_buf_hex(vpninfo,
                                                                     3i32,
                                                                     '<' as
                                                                         i32
                                                                         as
                                                                         libc::c_char,
                                                                     &mut (*(*vpninfo).cstp_pkt).c2rust_unnamed.pulse.vendor
                                                                         as
                                                                         *mut uint32_t
                                                                         as
                                                                         *mut libc::c_void
                                                                         as
                                                                         *mut libc::c_uchar,
                                                                     len);
                                                        (*(*vpninfo).cstp_pkt).len
                                                            = payload_len;
                                                        queue_packet(&mut (*vpninfo).incoming_queue,
                                                                     pkt);
                                                        pkt = 0 as *mut pkt;
                                                        (*vpninfo).cstp_pkt =
                                                            pkt;
                                                        work_done = 1i32;
                                                        current_block =
                                                            14916268686031723178;
                                                        continue ;
                                                    }
                                                }
                                            }
                                            150 => {
                                                current_block =
                                                    6628960769279281767;
                                                match current_block {
                                                    6628960769279281767 => {
                                                        if (*vpninfo).dtls_state
                                                               == 3i32 {
                                                            (*(*vpninfo).proto).udp_send_probes.expect("non-null function pointer")(vpninfo);
                                                        }
                                                        current_block =
                                                            14916268686031723178;
                                                        continue ;
                                                    }
                                                    1037737505459228 => {
                                                        if !(payload_len <
                                                                 0x6ai32 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x10i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     0x21202400i32
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x18i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     payload_len
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x1ci32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     (payload_len
                                                                          -
                                                                          0x1ci32)
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be32((*pkt).data.as_mut_ptr().offset(0x20i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     !=
                                                                     0x1000000i32
                                                                         as
                                                                         libc::c_uint
                                                                 ||
                                                                 load_be16((*pkt).data.as_mut_ptr().offset(0x28i32
                                                                                                               as
                                                                                                               isize)
                                                                               as
                                                                               *const libc::c_void)
                                                                     as
                                                                     libc::c_int
                                                                     !=
                                                                     0x40i32)
                                                           {
                                                            dump_buf_hex(vpninfo,
                                                                         0i32,
                                                                         '<'
                                                                             as
                                                                             i32
                                                                             as
                                                                             libc::c_char,
                                                                         &mut (*(*vpninfo).cstp_pkt).c2rust_unnamed.pulse.vendor
                                                                             as
                                                                             *mut uint32_t
                                                                             as
                                                                             *mut libc::c_void
                                                                             as
                                                                             *mut libc::c_uchar,
                                                                         len);
                                                            ret =
                                                                handle_esp_config_packet(vpninfo,
                                                                                         &mut (*pkt).c2rust_unnamed.pulse.vendor
                                                                                             as
                                                                                             *mut uint32_t
                                                                                             as
                                                                                             *mut libc::c_void
                                                                                             as
                                                                                             *mut libc::c_uchar,
                                                                                         len);
                                                            if ret != 0 {
                                                                if (*vpninfo).verbose
                                                                       >= 0i32
                                                                   {
                                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                            0i32,
                                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                                 as
                                                                                                                                                 *const u8
                                                                                                                                                 as
                                                                                                                                                 *const libc::c_char,
                                                                                                                                             b"ESP rekey failed\n\x00"
                                                                                                                                                 as
                                                                                                                                                 *const u8
                                                                                                                                                 as
                                                                                                                                                 *const libc::c_char));
                                                                }
                                                                (*(*vpninfo).proto).udp_close.expect("non-null function pointer")(vpninfo);
                                                                current_block
                                                                    =
                                                                    14916268686031723178;
                                                                continue ;
                                                            } else {
                                                                (*vpninfo).cstp_pkt
                                                                    =
                                                                    0 as
                                                                        *mut pkt;
                                                                (*pkt).len =
                                                                    load_be32(&mut (*pkt).c2rust_unnamed.pulse.len
                                                                                  as
                                                                                  *mut uint32_t
                                                                                  as
                                                                                  *const libc::c_void).wrapping_sub(16i32
                                                                                                                        as
                                                                                                                        libc::c_uint)
                                                                        as
                                                                        libc::c_int;
                                                                queue_packet(&mut (*vpninfo).oncp_control_queue,
                                                                             pkt);
                                                                print_esp_keys(vpninfo,
                                                                               libintl_dgettext(b"openconnect\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                b"new incoming\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char),
                                                                               &mut *(*vpninfo).esp_in.as_mut_ptr().offset((*vpninfo).current_esp_in
                                                                                                                               as
                                                                                                                               isize));
                                                                print_esp_keys(vpninfo,
                                                                               libintl_dgettext(b"openconnect\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                b"new outgoing\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char),
                                                                               &mut (*vpninfo).esp_out);
                                                                current_block
                                                                    =
                                                                    14916268686031723178;
                                                                continue ;
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        if (*vpninfo).verbose
                                                               >= 3i32 {
                                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                    3i32,
                                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char,
                                                                                                                                     b"Received data packet of %d bytes\n\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char),
                                                                                                                    payload_len);
                                                        }
                                                        dump_buf_hex(vpninfo,
                                                                     3i32,
                                                                     '<' as
                                                                         i32
                                                                         as
                                                                         libc::c_char,
                                                                     &mut (*(*vpninfo).cstp_pkt).c2rust_unnamed.pulse.vendor
                                                                         as
                                                                         *mut uint32_t
                                                                         as
                                                                         *mut libc::c_void
                                                                         as
                                                                         *mut libc::c_uchar,
                                                                     len);
                                                        (*(*vpninfo).cstp_pkt).len
                                                            = payload_len;
                                                        queue_packet(&mut (*vpninfo).incoming_queue,
                                                                     pkt);
                                                        pkt = 0 as *mut pkt;
                                                        (*vpninfo).cstp_pkt =
                                                            pkt;
                                                        work_done = 1i32;
                                                        current_block =
                                                            14916268686031723178;
                                                        continue ;
                                                    }
                                                }
                                            }
                                            _ => { }
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
                                                                                                                 b"Unknown Pulse packet\n\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char));
                                    }
                                    dump_buf_hex(vpninfo, 3i32,
                                                 '<' as i32 as libc::c_char,
                                                 &mut (*(*vpninfo).cstp_pkt).c2rust_unnamed.pulse.vendor
                                                     as *mut uint32_t as
                                                     *mut libc::c_void as
                                                     *mut libc::c_uchar, len);
                                    current_block = 14916268686031723178;
                                    continue ;
                                }
                            }
                        }
                    }
                    /* If SSL_write() fails we are expected to try again. With exactly
	   the same data, at exactly the same location. So we keep the
	   packet we had before.... */
                    if !(*vpninfo).current_ssl_pkt.is_null() {
                        current_block = 10784599433387736960;
                    } else { current_block = 14483658890531361756; }
                    loop  {
                        match current_block {
                            10784599433387736960 => {
                                (*vpninfo).ssl_times.last_tx =
                                    time(0 as *mut time_t);
                                let mut __fd: libc::c_int = (*vpninfo).ssl_fd;
                                (*vpninfo)._select_wfds.fds_bits[(__fd as
                                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                                                       as
                                                                                                       libc::c_ulong).wrapping_mul(8i32
                                                                                                                                       as
                                                                                                                                       libc::c_ulong))
                                                                     as usize]
                                    &=
                                    !(((1i32 as libc::c_ulong) <<
                                           (__fd as
                                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_mul(8i32
                                                                                                                 as
                                                                                                                 libc::c_ulong)))
                                          as __int32_t);
                                if (*vpninfo).verbose >= 3i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            3i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"Packet outgoing:\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char));
                                }
                                dump_buf_hex(vpninfo, 3i32,
                                             '>' as i32 as libc::c_char,
                                             &mut (*(*vpninfo).current_ssl_pkt).c2rust_unnamed.pulse.vendor
                                                 as *mut uint32_t as
                                                 *mut libc::c_void as
                                                 *mut libc::c_uchar,
                                             (*(*vpninfo).current_ssl_pkt).len
                                                 + 16i32);
                                ret =
                                    ssl_nonblock_write(vpninfo,
                                                       &mut (*(*vpninfo).current_ssl_pkt).c2rust_unnamed.pulse.vendor
                                                           as *mut uint32_t as
                                                           *mut libc::c_void,
                                                       (*(*vpninfo).current_ssl_pkt).len
                                                           + 16i32);
                                if ret < 0i32 {
                                    current_block = 14901311691614577181;
                                    continue 'c_38118 ;
                                }
                                if ret == 0 {
                                    /* Not for Pulse yet */
                                    return work_done
                                }
                                if ret !=
                                       (*(*vpninfo).current_ssl_pkt).len +
                                           16i32 {
                                    if (*vpninfo).verbose >= 0i32 {
                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                0i32,
                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char,
                                                                                                                 b"SSL wrote too few bytes! Asked for %d, sent %d\n\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char),
                                                                                                (*(*vpninfo).current_ssl_pkt).len
                                                                                                    +
                                                                                                    8i32,
                                                                                                ret);
                                    }
                                    (*vpninfo).quit_reason =
                                        b"Internal error\x00" as *const u8 as
                                            *const libc::c_char;
                                    return 1i32
                                }
                                /* Don't free the 'special' packets */
                                if (*vpninfo).current_ssl_pkt ==
                                       (*vpninfo).deflate_pkt {
                                    free((*vpninfo).pending_deflated_pkt as
                                             *mut libc::c_void);
                                    (*vpninfo).pending_deflated_pkt =
                                        0 as *mut pkt
                                } else {
                                    free((*vpninfo).current_ssl_pkt as
                                             *mut libc::c_void);
                                }
                                (*vpninfo).current_ssl_pkt = 0 as *mut pkt;
                                current_block = 14483658890531361756;
                            }
                            _ => {
                                /* Not understood for Pulse yet */
                                if (*vpninfo).dtls_state == 4i32 {
                                    /* We don't currently do anything to make the server start sending
		 * data packets in ESP instead of over IF-T/TLS. Just go straight
		 * to CONNECTED mode. */
                                    (*vpninfo).dtls_state = 5i32;
                                    work_done = 1i32
                                }
                                (*vpninfo).current_ssl_pkt =
                                    dequeue_packet(&mut (*vpninfo).oncp_control_queue);
                                if !(*vpninfo).current_ssl_pkt.is_null() {
                                    /* Anything on the control queue will have the rest of its
		   header filled in already. */
                                    let fresh7 = (*vpninfo).ift_seq;
                                    (*vpninfo).ift_seq =
                                        (*vpninfo).ift_seq.wrapping_add(1);
                                    store_be32(&mut (*(*vpninfo).current_ssl_pkt).c2rust_unnamed.pulse.ident
                                                   as *mut uint32_t as
                                                   *mut libc::c_void, fresh7);
                                    current_block = 10784599433387736960;
                                } else {
                                    /* Service outgoing packet queue, if no DTLS */
                                    if !((*vpninfo).dtls_state != 5i32 &&
                                             {
                                                 (*vpninfo).current_ssl_pkt =
                                                     dequeue_packet(&mut (*vpninfo).outgoing_queue);
                                                 !(*vpninfo).current_ssl_pkt.is_null()
                                             }) {
                                        break ;
                                    }
                                    let mut this: *mut pkt =
                                        (*vpninfo).current_ssl_pkt;
                                    store_be32(&mut (*this).c2rust_unnamed.pulse.vendor
                                                   as *mut uint32_t as
                                                   *mut libc::c_void,
                                               0xa4ci32 as uint32_t);
                                    store_be32(&mut (*this).c2rust_unnamed.pulse.type_0
                                                   as *mut uint32_t as
                                                   *mut libc::c_void,
                                               4i32 as uint32_t);
                                    store_be32(&mut (*this).c2rust_unnamed.pulse.len
                                                   as *mut uint32_t as
                                                   *mut libc::c_void,
                                               ((*this).len + 16i32) as
                                                   uint32_t);
                                    let fresh8 = (*vpninfo).ift_seq;
                                    (*vpninfo).ift_seq =
                                        (*vpninfo).ift_seq.wrapping_add(1);
                                    store_be32(&mut (*this).c2rust_unnamed.pulse.ident
                                                   as *mut uint32_t as
                                                   *mut libc::c_void, fresh8);
                                    if (*vpninfo).verbose >= 3i32 {
                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                3i32,
                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char,
                                                                                                                 b"Sending IF-T/TLS data packet of %d bytes\n\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char),
                                                                                                (*this).len);
                                    }
                                    (*vpninfo).current_ssl_pkt = this;
                                    current_block = 10784599433387736960;
                                }
                            }
                        }
                    }
                    /* Work is not done if we just got rid of packets off the queue */
                    return work_done
                }
                _ => {
                    /* XXX: Do we have to do this or can we leave it open?
			 * Perhaps we could even reconnect asynchronously while
			 * the ESP is still running? */
                    esp_shutdown(vpninfo);
                    ret = ssl_reconnect(vpninfo);
                    if ret != 0 {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Reconnect failed\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                        (*vpninfo).quit_reason =
                            b"Pulse reconnect failed\x00" as *const u8 as
                                *const libc::c_char;
                        return ret
                    }
                    (*vpninfo).dtls_need_reconnect = 1i32;
                    return 1i32
                }
            }
        };
}
/* GlobalProtect magic ping address (network-endian) */
/* For Juniper TNCC */
/* We need to give it back in the same form */
/* The SHA1 and SHA256 hashes of the peer's public key */
/* this value is cache for openconnect_get_peer_cert_hash */
/* Pointer to within cookies list */
/* OPENCONNECT_GNUTLS */
/* It may need to be larger than MTU */
/* For compressing outbound packets into */
/* The original packet associated with above */
/* Partially sent SSL packet */
/* Control packets to be sent on oNCP next */
/* For packetising incoming oNCP stream */
/* Packet buffers for receiving into */
/* How many bytes after payload for encryption (ESP HMAC) */
/* Local static configured values */
/* Returned by server */
/* Returned by server */
/* What we requested */
/* Accepted for CSTP */
/* Accepted for DTLS */
/* Attempt to redo DNS lookup on each CSTP reconnect */
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
#[no_mangle]
#[src_loc = "2577:1"]
pub unsafe extern "C" fn pulse_bye(mut vpninfo: *mut openconnect_info,
                                   mut reason: *const libc::c_char)
 -> libc::c_int {
    if (*vpninfo).ssl_fd != -1i32 {
        let mut buf: *mut oc_text_buf = buf_alloc();
        buf_append_ift_hdr(buf, 0xa4ci32 as uint32_t, 0x89i32 as uint32_t);
        if buf_error(buf) == 0 { send_ift_packet(vpninfo, buf); }
        buf_free(buf);
        openconnect_close_https(vpninfo, 0i32);
    }
    return 0i32;
}
