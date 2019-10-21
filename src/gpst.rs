#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, extern_types,
           linkage, ptr_wrapping_offset_from)]
extern crate libc;
extern crate c2rust_bitfields;
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
    #[src_loc = "109:1"]
    pub type __darwin_rune_t = __darwin_wchar_t;
    /* clock() */
    #[src_loc = "118:1"]
    pub type __darwin_socklen_t = __uint32_t;
    /* socklen_t (duh) */
    #[src_loc = "119:1"]
    pub type __darwin_ssize_t = libc::c_long;
    /* byte count or error */
    #[src_loc = "120:1"]
    pub type __darwin_time_t = libc::c_long;
    use super::libc;
    /* _BSD_I386__TYPES_H_ */
    /* time() */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:20"]
pub mod sys__types_h {
    #[src_loc = "60:1"]
    pub type __darwin_gid_t = __uint32_t;
    #[src_loc = "72:1"]
    pub type __darwin_pid_t = __int32_t;
    #[src_loc = "74:1"]
    pub type __darwin_suseconds_t = __int32_t;
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
    use super::libc;
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
    use super::libc;
    /* _UINT32_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int16_t.h:20"]
pub mod _u_int16_t_h {
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
    #[src_loc = "30:1"]
    pub type u_int16_t = libc::c_ushort;
    use super::libc;
    /* _U_INT16_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int32_t.h:20"]
pub mod _u_int32_t_h {
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
    #[src_loc = "30:1"]
    pub type u_int32_t = libc::c_uint;
    use super::libc;
    /* _U_INT32_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:20"]
pub mod _ssize_t_h {
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
    #[src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
    /* _SSIZE_T */
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:20"]
pub mod _pid_t_h {
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
    /* __darwin_pid_t */
    #[src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
    /* _PID_T */
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
    use super::libc;
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
    use super::libc;
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
    use super::libc;
    /* _UINT16_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_char.h:28"]
pub mod _u_char_h {
    /*
 * Copyright (c) 2017 Apple Inc. All rights reserved.
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
    #[src_loc = "30:1"]
    pub type u_char = libc::c_uchar;
    use super::libc;
    /* _U_CHAR */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_short.h:28"]
pub mod _u_short_h {
    /*
 * Copyright (c) 2017 Apple Inc. All rights reserved.
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
    #[src_loc = "30:1"]
    pub type u_short = libc::c_ushort;
    use super::libc;
    /* _U_SHORT */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int.h:28"]
pub mod _u_int_h {
    /*
 * Copyright (c) 2017 Apple Inc. All rights reserved.
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
    #[src_loc = "30:1"]
    pub type u_int = libc::c_uint;
    use super::libc;
    /* _U_INT */
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in_systm.h:44"]
pub mod in_systm_h {
    /*
 * Copyright (c) 2000-2018 Apple Computer, Inc. All rights reserved.
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
 * Copyright (c) 1982, 1986, 1993
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
 *	@(#)in_systm.h	8.1 (Berkeley) 6/10/93
 * $FreeBSD: src/sys/netinet/in_systm.h,v 1.9 1999/12/29 04:41:00 peter Exp $
 */
    /*
 * Miscellaneous internetwork
 * definitions for kernel.
 */
    /*
 * Network types.
 *
 * Internally the system keeps counters in the headers with the bytes
 * swapped so that VAX instructions will work on them.  It reverses
 * the bytes before transmission at each protocol level.  The n_ types
 * represent the types with the bytes in ``high-ender'' order.
 */
    #[src_loc = "82:1"]
    pub type n_short = __uint16_t;
    /* long as received from the net */
    #[src_loc = "85:1"]
    pub type n_time = __uint32_t;
    use super::_types_h::{__uint16_t, __uint32_t};
    /* _NETINET_IN_SYSTM_H_ */
    /* ms since 00:00 GMT, byte rev */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:45"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:45"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:45"]
pub mod socket_h {
    /* __MSFILTERREQ_DEFINED */
    /*
 * Definitions for inet sysctl operations.
 *
 * Third level is protocol number.
 * Fourth level is desired variable within that protocol.
 */
    /* don't list to IPPROTO_MAX */
    /*
 * Names for IP sysctl objects
 */
    /* act as router */
    /* may send redirects when forwarding */
    /* default TTL */
    /* cloned route expiration time */
    /* min value for expiration time */
    /* trigger level for dynamic expire */
    /* may perform source routes */
    /* may re-broadcast received packets */
    /* max length of netisr queue */
    /* number of netisr q drops */
    /* ipstat structure */
    /* may accept source routed packets */
    /* use fast IP forwarding code */
    /* deprecated */
    /* default TTL for gif encap packet */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* INET6 stuff */
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
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "705:1"]
        pub fn send(_: libc::c_int, _: *const libc::c_void, _: size_t,
                    _: libc::c_int) -> ssize_t;
    }
    /* !_SYS_SOCKET_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:45"]
pub mod in_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "301:1"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    use super::_in_addr_t_h::in_addr_t;
    /* _NETINET_IN_H_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/ip.h:46"]
pub mod ip_h {
    /*
 * Copyright (c) 2000-2016 Apple Inc. All rights reserved.
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
 * Copyright (c) 1982, 1986, 1993
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
 *	@(#)ip.h	8.2 (Berkeley) 6/1/94
 * $FreeBSD: src/sys/netinet/ip.h,v 1.17 1999/12/22 19:13:20 shin Exp $
 */
    /* XXX temporary hack to get u_ types */
    /*
 * Definitions for internet protocol version 4.
 * Per RFC 791, September 1981.
 */
    /*
 * Structure of an internet header, naked of options.
 */
    #[derive ( Copy, Clone, BitfieldStruct )]
    #[repr(C)]
    #[src_loc = "81:1"]
    pub struct ip {
        #[bitfield(name = "ip_hl", ty = "u_int", bits = "0..=3")]
        #[bitfield(name = "ip_v", ty = "u_int", bits = "4..=7")]
        pub ip_hl_ip_v: [u8; 1],
        pub ip_tos: u_char,
        pub ip_len: u_short,
        pub ip_id: u_short,
        pub ip_off: u_short,
        pub ip_ttl: u_char,
        pub ip_p: u_char,
        pub ip_sum: u_short,
        pub ip_src: in_addr,
        pub ip_dst: in_addr,
        /* source and dest address */
    }
    use super::_u_int_h::u_int;
    use super::_u_char_h::u_char;
    use super::_u_short_h::u_short;
    use super::in_h::in_addr;
    /* default maximum segment size */
    /* subtracted when forwarding */
    /* time to live for frags (seconds) */
    /* default ttl, from RFC 1340 */
    /* maximum time to live (seconds) */
    /*
 * Internet implementation parameters.
 */
    /* bits for security (not byte swapped) */
    /* specified modules only */
    /* timestamps and addresses */
    /* timestamps only */
    /* flag bits for ipt_flg */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/ip_icmp.h:47"]
pub mod ip_icmp_h {
    /*
 * Copyright (c) 2008-2018 Apple Inc. All rights reserved.
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
 * Copyright (c) 1982, 1986, 1993
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
 *	@(#)ip_icmp.h	8.1 (Berkeley) 6/10/93
 * $FreeBSD: src/sys/netinet/ip_icmp.h,v 1.16 1999/12/29 04:41:01 peter Exp $
 */
    /*
 * Interface Control Message Protocol Definitions.
 * Per RFC 792, September 1981.
 */
    /*
 * Internal of an ICMP Router Advertisement
 */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "76:1"]
    pub struct icmp_ra_addr {
        pub ira_addr: u_int32_t,
        pub ira_preference: u_int32_t,
    }
    /*
 * Structure of an icmp header.
 */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "84:1"]
    pub struct icmp {
        pub icmp_type: u_char,
        pub icmp_code: u_char,
        pub icmp_cksum: u_short,
        pub icmp_hun: C2RustUnnamed_0,
        pub icmp_dun: C2RustUnnamed,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "119:2"]
    pub union C2RustUnnamed {
        pub id_ts: id_ts,
        pub id_ip: id_ip,
        pub id_radv: icmp_ra_addr,
        pub id_mask: u_int32_t,
        pub id_data: [libc::c_char; 1],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "125:3"]
    pub struct id_ip {
        pub idi_ip: ip,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "120:3"]
    pub struct id_ts {
        pub its_otime: n_time,
        pub its_rtime: n_time,
        pub its_ttime: n_time,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "88:2"]
    pub union C2RustUnnamed_0 {
        pub ih_pptr: u_char,
        pub ih_gwaddr: in_addr,
        pub ih_idseq: ih_idseq,
        pub ih_void: libc::c_int,
        pub ih_pmtu: ih_pmtu,
        pub ih_rtradv: ih_rtradv,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "103:3"]
    pub struct ih_rtradv {
        pub irt_num_addrs: u_char,
        pub irt_wpa: u_char,
        pub irt_lifetime: u_int16_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "98:3"]
    pub struct ih_pmtu {
        pub ipm_void: n_short,
        pub ipm_nextmtu: n_short,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "91:3"]
    pub struct ih_idseq {
        pub icd_id: n_short,
        pub icd_seq: n_short,
    }
    use super::_u_int32_t_h::u_int32_t;
    use super::_u_char_h::u_char;
    use super::_u_short_h::u_short;
    use super::libc;
    use super::ip_h::ip;
    use super::in_systm_h::{n_time, n_short};
    use super::in_h::in_addr;
    use super::_u_int16_t_h::u_int16_t;
    /* _NETINET_IP_ICMP_H_ */
    /* decrypt failed */
    /* auth failed */
    /* unknown sec index */
    /* Photuris */
    /* SKIP */
    /* mobile registration reply */
    /* mobile registration req */
    /* IPv6 i-am-here */
    /* IPv6 where-are-you */
    /* mobile host redirect */
    /* data conversion error */
    /* traceroute */
    /* address mask reply */
    /* address mask request */
    /* information reply */
    /* information request */
    /* timestamp reply */
    /* timestamp request */
    /* bad length */
    /* req. opt. absent */
    /* error at param ptr */
    /* ip header bad */
    /* ttl==0 in reass */
    /* ttl==0 in transit */
    /* time exceeded, code: */
    /* router solicitation */
    /* selective routing */
    /* normal advertisement */
    /* router advertisement */
    /* echo service */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netdb.h:57"]
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
    use super::libc;
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
    /* !_NETDB_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src = "/Users/cameron/github/openconnect/openconnect.h:57"]
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
        pub oath_hmac_alg: C2RustUnnamed_14,
        pub hotp_secret_format: C2RustUnnamed_13,
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
                                        C2RustUnnamed_14, C2RustUnnamed_13,
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
#[header_src = "/Users/cameron/github/openconnect/openconnect-internal.h:57"]
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
        pub c2rust_unnamed: C2RustUnnamed_1,
        pub data: [libc::c_uchar; 0],
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "126:2"]
    pub union C2RustUnnamed_1 {
        pub esp: C2RustUnnamed_6,
        pub oncp: C2RustUnnamed_5,
        pub cstp: C2RustUnnamed_4,
        pub gpst: C2RustUnnamed_3,
        pub pulse: C2RustUnnamed_2,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "146:3"]
    pub struct C2RustUnnamed_2 {
        pub pad: [libc::c_uchar; 8],
        pub vendor: uint32_t,
        pub type_0: uint32_t,
        pub len: uint32_t,
        pub ident: uint32_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "142:3"]
    pub struct C2RustUnnamed_3 {
        pub pad: [libc::c_uchar; 8],
        pub hdr: [libc::c_uchar; 16],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "138:3"]
    pub struct C2RustUnnamed_4 {
        pub pad: [libc::c_uchar; 16],
        pub hdr: [libc::c_uchar; 8],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "133:3"]
    pub struct C2RustUnnamed_5 {
        pub pad: [libc::c_uchar; 2],
        pub rec: [libc::c_uchar; 2],
        pub kmp: [libc::c_uchar; 20],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "127:3"]
    pub struct C2RustUnnamed_6 {
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
    pub type C2RustUnnamed_13 = libc::c_uint;
    #[src_loc = "475:3"]
    pub const HOTP_SECRET_PSKC: C2RustUnnamed_13 = 4;
    #[src_loc = "474:3"]
    pub const HOTP_SECRET_HEX: C2RustUnnamed_13 = 3;
    #[src_loc = "473:3"]
    pub const HOTP_SECRET_RAW: C2RustUnnamed_13 = 2;
    #[src_loc = "472:3"]
    pub const HOTP_SECRET_BASE32: C2RustUnnamed_13 = 1;
    #[src_loc = "466:2"]
    pub type C2RustUnnamed_14 = libc::c_uint;
    #[src_loc = "469:3"]
    pub const OATH_ALG_HMAC_SHA512: C2RustUnnamed_14 = 2;
    #[src_loc = "468:3"]
    pub const OATH_ALG_HMAC_SHA256: C2RustUnnamed_14 = 1;
    #[src_loc = "467:3"]
    pub const OATH_ALG_HMAC_SHA1: C2RustUnnamed_14 = 0;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "237:1"]
    pub struct http_auth_state {
        pub state: libc::c_int,
        pub challenge: *mut libc::c_char,
        pub c2rust_unnamed: C2RustUnnamed_15,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "240:2"]
    pub union C2RustUnnamed_15 {
        pub c2rust_unnamed: C2RustUnnamed_17,
        pub c2rust_unnamed_0: C2RustUnnamed_16,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "258:3"]
    pub struct C2RustUnnamed_16 {
        pub ntlm_helper_fd: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "242:3"]
    pub struct C2RustUnnamed_17 {
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
    #[derive ( Copy, Clone )]
    #[repr(C, packed)]
    #[src_loc = "1189:1"]
    pub struct oc_packed_uint16_t {
        pub d: uint16_t,
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
    #[src_loc = "1231:1"]
    pub unsafe extern "C" fn load_le32(mut _p: *const libc::c_void)
     -> uint32_t {
        let mut p: *const oc_packed_uint32_t =
            _p as *const oc_packed_uint32_t;
        return (*p).d;
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
    #[src_loc = "783:1"]
    pub unsafe extern "C" fn set_fd_cloexec(mut fd: libc::c_int)
     -> libc::c_int {
        return fcntl(fd, 2i32, fcntl(fd, 1i32) | 1i32);
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
    use super::libc;
    use super::_uint32_t_h::uint32_t;
    use super::_time_t_h::time_t;
    use super::gssapi_h::{gss_name_t, gss_ctx_id_t};
    use super::hmac_h::HMAC_CTX;
    use super::ossl_typ_h::EVP_CIPHER_CTX;
    use super::_uint64_t_h::uint64_t;
    use super::openconnect_h::{openconnect_info, oc_vpn_option};
    use super::_uint16_t_h::uint16_t;
    use super::_uint8_t_h::uint8_t;
    use super::_OSByteOrder_h::{_OSSwapInt32, _OSSwapInt16};
    use super::_types_h::__uint16_t;
    use super::tree_h::xmlNode;
    use super::fcntl_h::fcntl;
    extern "C" {
        #[src_loc = "363:1"]
        pub type oc_pcsc_ctx;
        #[no_mangle]
        #[src_loc = "954:1"]
        pub fn udp_connect(vpninfo: *mut openconnect_info) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "977:1"]
        pub fn construct_esp_packet(vpninfo: *mut openconnect_info,
                                    pkt: *mut pkt, next_hdr: uint8_t)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1016:1"]
        pub fn ka_check_deadline(timeout: *mut libc::c_int, now: time_t,
                                 due: time_t) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "986:1"]
        pub fn ssl_nonblock_read(vpninfo: *mut openconnect_info,
                                 buf: *mut libc::c_void, maxlen: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1084:1"]
        pub fn dump_buf_hex(vpninfo: *mut openconnect_info,
                            loglevel: libc::c_int, prefix: libc::c_char,
                            buf: *mut libc::c_uchar, len: libc::c_int);
        #[no_mangle]
        #[src_loc = "987:1"]
        pub fn ssl_nonblock_write(vpninfo: *mut openconnect_info,
                                  buf: *mut libc::c_void, buflen: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1015:1"]
        pub fn ka_stalled_action(ka: *mut keepalive_info,
                                 timeout: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1014:1"]
        pub fn keepalive_action(ka: *mut keepalive_info,
                                timeout: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "955:1"]
        pub fn ssl_reconnect(vpninfo: *mut openconnect_info) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "953:1"]
        pub fn udp_sockaddr(vpninfo: *mut openconnect_info, port: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "976:1"]
        pub fn openconnect_setup_esp_keys(vpninfo: *mut openconnect_info,
                                          new_keys: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1136:1"]
        pub fn free_optlist(opt: *mut oc_vpn_option);
        #[no_mangle]
        #[src_loc = "1093:1"]
        pub fn buf_truncate(buf: *mut oc_text_buf);
        #[no_mangle]
        #[src_loc = "1085:1"]
        pub fn buf_ensure_space(buf: *mut oc_text_buf, len: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "995:1"]
        pub fn openconnect_md5(result: *mut libc::c_uchar,
                               data: *mut libc::c_void, len: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1068:1"]
        pub fn append_opt(body: *mut oc_text_buf, opt: *const libc::c_char,
                          name: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1102:1"]
        pub fn do_https_request(vpninfo: *mut openconnect_info,
                                method: *const libc::c_char,
                                request_body_type: *const libc::c_char,
                                request_body: *mut oc_text_buf,
                                form_buf: *mut *mut libc::c_char,
                                fetch_redirect: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1061:1"]
        pub fn set_csd_user(vpninfo: *mut openconnect_info) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "988:1"]
        pub fn openconnect_open_https(vpninfo: *mut openconnect_info)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1082:1"]
        pub fn buf_alloc() -> *mut oc_text_buf;
        #[no_mangle]
        #[src_loc = "1088:1"]
        pub fn buf_append_bytes(buf: *mut oc_text_buf,
                                bytes: *const libc::c_void, len: libc::c_int);
        #[no_mangle]
        #[src_loc = "1086:1"]
        pub fn buf_append(buf: *mut oc_text_buf, fmt: *const libc::c_char,
                          _: ...);
        #[no_mangle]
        #[src_loc = "1096:1"]
        pub fn buf_error(buf: *mut oc_text_buf) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1083:1"]
        pub fn dump_buf(vpninfo: *mut openconnect_info, prefix: libc::c_char,
                        buf: *mut libc::c_char);
        #[no_mangle]
        #[src_loc = "989:1"]
        pub fn openconnect_close_https(vpninfo: *mut openconnect_info,
                                       final_0: libc::c_int);
        #[no_mangle]
        #[src_loc = "1097:1"]
        pub fn buf_free(buf: *mut oc_text_buf) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1067:1"]
        pub fn xmlnode_match_prop(xml_node: *mut xmlNode,
                                  name: *const libc::c_char,
                                  match_0: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1064:1"]
        pub fn xmlnode_is_named(xml_node: *mut xmlNode,
                                name: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1065:1"]
        pub fn xmlnode_get_val(xml_node: *mut xmlNode,
                               name: *const libc::c_char,
                               var: *mut *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "912:1"]
        pub fn gpst_os_name(vpninfo: *mut openconnect_info)
         -> *const libc::c_char;
        #[no_mangle]
        #[src_loc = "849:1"]
        pub fn free_split_routes(vpninfo: *mut openconnect_info);
        #[no_mangle]
        #[src_loc = "843:1"]
        pub fn unhex(data: *const libc::c_char) -> libc::c_uchar;
        #[no_mangle]
        #[src_loc = "835:1"]
        pub fn openconnect_utf8_to_legacy(vpninfo: *mut openconnect_info,
                                          utf8: *const libc::c_char)
         -> *mut libc::c_char;
    }
    /* __OPENCONNECT_INTERNAL_H__ */
    /* !Not known to be little-endian */
}
#[header_src =
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ossl_typ.h:57"]
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
        pub stats: C2RustUnnamed_10,
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
        pub pkey: C2RustUnnamed_8,
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
        pub cb: C2RustUnnamed_9,
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
    use super::libc;
    use super::ssl_h::{SSL_METHOD, ssl2_state_st, ssl3_state_st,
                       dtls1_state_st, stack_st_SSL_CIPHER, cert_st,
                       SSL_SESSION, GEN_SESSION_CB, stack_st_OCSP_RESPID,
                       TLS_SESSION_TICKET_EXT, tls_session_ticket_ext_cb_fn,
                       tls_session_secret_cb_fn,
                       stack_st_SRTP_PROTECTION_PROFILE,
                       SRTP_PROTECTION_PROFILE, SRP_CTX, lhash_st_SSL_SESSION,
                       ssl_session_st, C2RustUnnamed_10, stack_st_SSL_COMP,
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
    use super::evp_h::{C2RustUnnamed_8, stack_st_X509_ATTRIBUTE};
    use super::bn_h::C2RustUnnamed_9;
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl.h:57"]
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
    pub struct C2RustUnnamed_10 {
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
        pub tmp: C2RustUnnamed_11,
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
        pub tmp: C2RustUnnamed_12,
    }
    use super::libc;
    use super::ossl_typ_h::{SSL, BIGNUM, X509, CRYPTO_EX_DATA, SSL_CTX,
                            EVP_MD_CTX, COMP_METHOD};
    use super::stack_h::_STACK;
    use super::_size_t_h::size_t;
    use super::dtls1_h::{DTLS1_BITMAP, record_pqueue, hm_header_st,
                         dtls1_timeout_st};
    use super::pqueue_h::pqueue;
    use super::_timeval_h::timeval;
    use super::ssl3_h::{SSL3_BUFFER, SSL3_RECORD, C2RustUnnamed_11};
    use super::bio_h::BIO;
    use super::ssl2_h::C2RustUnnamed_12;
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
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the SSL functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/stack.h:57"]
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
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/hmac.h:57"]
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
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/asn1.h:57"]
pub mod asn1_h {
    /* crypto/asn1/asn1.h */
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
    /* let the recipient choose */
    /* used in ASN1_TYPE */
    /* used in ASN1 template code */
    /* negative flag */
    /* */
    /* */
    /* alias */
    /* */
    /* */
    /* */
    /* */
    /* alias */
    /* */
    /* */
    /* For use with d2i_ASN1_type_bytes() */
    /* For use with ASN1_mbstring_copy() */
    /* filled in by mkstack.pl */
    /* nothing, no longer needed */
    /*
 * We MUST make sure that, except for constness, asn1_ctx_st and
 * asn1_const_ctx are exactly the same.  Fortunately, as soon as the old ASN1
 * parsing macros are gone, we can throw this away as well...
 */
    /* work char pointer */
    /* end of sequence read for indefinite
                                 * encoding */
    /* error code to use when returning an error */
    /* constructed if 0x20, indefinite is 0x21 */
    /* tag from last 'get object' */
    /* class from last 'get object' */
    /* length of last 'get object' */
    /* largest value of p allowed */
    /* temporary variable */
    /* variable */
    /* used in error processing */
    /* work char pointer */
    /* end of sequence read for indefinite
                                 * encoding */
    /* error code to use when returning an error */
    /* constructed if 0x20, indefinite is 0x21 */
    /* tag from last 'get object' */
    /* class from last 'get object' */
    /* length of last 'get object' */
    /* largest value of p allowed */
    /* temporary variable */
    /* variable */
    /* used in error processing */
    /*
 * These are used internally in the ASN1_OBJECT to keep track of whether the
 * names and data need to be free()ed
 */
    /* internal use */
    /* critical x509v3 object id */
    /* internal use */
    /* internal use */
    /* data remains const after init */
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
    #[src_loc = "524:1"]
    pub type ASN1_TYPE = asn1_type_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "524:9"]
    pub struct asn1_type_st {
        pub type_0: libc::c_int,
        pub value: C2RustUnnamed_7,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "526:5"]
    pub union C2RustUnnamed_7 {
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
    use super::libc;
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
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509_vfy.h:57"]
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
    extern "C" {
        #[src_loc = "159:9"]
        pub type X509_VERIFY_PARAM_ID_st;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/crypto.h:57"]
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
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the CRYPTO functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509.h:57"]
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
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "301:1"]
    pub struct stack_st_X509 {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "476:1"]
    pub struct stack_st_X509_CRL {
        pub stack: _STACK,
    }
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/evp.h:57"]
pub mod evp_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "151:5"]
    pub struct stack_st_X509_ATTRIBUTE {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "135:5"]
    pub union C2RustUnnamed_8 {
        pub ptr: *mut libc::c_char,
        pub rsa: *mut rsa_st,
        pub dsa: *mut dsa_st,
        pub dh: *mut dh_st,
        pub ec: *mut ec_key_st,
    }
    use super::stack_h::_STACK;
    use super::libc;
    use super::ossl_typ_h::{rsa_st, dsa_st, dh_st};
    extern "C" {
        #[src_loc = "147:9"]
        pub type ec_key_st;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the EVP functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bn.h:57"]
pub mod bn_h {
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "352:5"]
    pub union C2RustUnnamed_9 {
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dsa.h:57"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pem.h:57"]
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
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the PEM functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/comp.h:57"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dtls1.h:57"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pqueue.h:57"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl3.h:57"]
pub mod ssl3_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "553:5"]
    pub struct C2RustUnnamed_11 {
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ec.h:57"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bio.h:57"]
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
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the BIO functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl2.h:57"]
pub mod ssl2_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "203:5"]
    pub struct C2RustUnnamed_12 {
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zlib.h:57"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zconf.h:57"]
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
#[header_src = "/usr/local/Cellar/stoken/0.92/include/stoken.h:57"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/gssapi/gssapi.h:57"]
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
#[header_src = "/usr/local/Cellar/libproxy/0.4.15_1/include/proxy.h:57"]
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
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/tree.h:57"]
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
    #[src_loc = "488:1"]
    pub type xmlNodePtr = *mut xmlNode;
    /* application data */
    /* type number, must be second ! */
    /* the name of the node, or the entity */
    /* parent->childs link */
    /* last child link */
    /* child->parent link */
    /* next sibling link  */
    /* previous sibling link  */
    /* the containing document */
    /* End of common part */
    /* pointer to the associated namespace */
    /* the content */
    /* properties list */
    /* namespace definitions on this node */
    /* for type/PSVI informations */
    /* line number */
    /* extra data for XPath/XSLT */
    /* *
 * XML_GET_CONTENT:
 *
 * Macro to extract the content pointer of a node.
 */
    /* *
 * XML_GET_LINE:
 *
 * Macro to extract the line number of an element node.
 */
    /* *
 * xmlDocProperty
 *
 * Set of properties of the document as found by the parser
 * Some of them are linked to similary named xmlParserOption
 */
    /* document is XML well formed */
    /* document is Namespace valid */
    /* parsed with old XML-1.0 parser */
    /* DTD validation was successful */
    /* XInclude substitution was done */
    /* Document was built using the API
                                           and not by parsing an instance */
    /* built for internal processing */
    /* parsed or built HTML document */
    /* *
 * xmlDoc:
 *
 * An XML document.
 */
    #[src_loc = "550:1"]
    pub type xmlDocPtr = *mut xmlDoc;
    #[src_loc = "549:1"]
    pub type xmlDoc = _xmlDoc;
    use super::libc;
    use super::xmlstring_h::xmlChar;
    use super::dict_h::_xmlDict;
    extern "C" {
        #[no_mangle]
        #[src_loc = "781:11"]
        pub fn xmlFreeDoc(cur: xmlDocPtr);
        /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
        #[no_mangle]
        #[src_loc = "919:11"]
        pub fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
        #[no_mangle]
        #[src_loc = "1075:11"]
        pub fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
    }
    /* __XML_TREE_H__ */
}
#[header_src =
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/dict.h:57"]
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
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/xmlstring.h:57"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/iconv.h:57"]
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
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/parser.h:57"]
pub mod parser_h {
    /*
 * New set of simpler/more flexible APIs
 */
/* *
 * xmlParserOption:
 *
 * This is the set of XML parser options that can be passed down
 * to the xmlReadDoc() and similar calls.
 */
    #[src_loc = "1089:9"]
    pub type C2RustUnnamed_18 = libc::c_uint;
    /* Store big lines numbers in text PSVI field */
    /* ignore internal document encoding hint */
    #[src_loc = "1114:5"]
    pub const XML_PARSE_BIG_LINES: C2RustUnnamed_18 = 4194304;
    /* parse using SAX2 interface before 2.7.0 */
    #[src_loc = "1113:5"]
    pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_18 = 2097152;
    /* relax any hardcoded limit from the parser */
    #[src_loc = "1112:5"]
    pub const XML_PARSE_OLDSAX: C2RustUnnamed_18 = 1048576;
    /* do not fixup XINCLUDE xml:base uris */
    #[src_loc = "1111:5"]
    pub const XML_PARSE_HUGE: C2RustUnnamed_18 = 524288;
    /* parse using XML-1.0 before update 5 */
    #[src_loc = "1110:5"]
    pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_18 = 262144;
    /* compact small text nodes; no modification of
                                   the tree allowed afterwards (will possibly
				   crash if you try to modify the tree) */
    #[src_loc = "1109:5"]
    pub const XML_PARSE_OLD10: C2RustUnnamed_18 = 131072;
    /* do not generate XINCLUDE START/END nodes */
    #[src_loc = "1106:5"]
    pub const XML_PARSE_COMPACT: C2RustUnnamed_18 = 65536;
    /* merge CDATA as text nodes */
    #[src_loc = "1105:5"]
    pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_18 = 32768;
    /* remove redundant namespaces declarations */
    #[src_loc = "1104:5"]
    pub const XML_PARSE_NOCDATA: C2RustUnnamed_18 = 16384;
    /* Do not reuse the context dictionary */
    #[src_loc = "1103:5"]
    pub const XML_PARSE_NSCLEAN: C2RustUnnamed_18 = 8192;
    /* Forbid network access */
    #[src_loc = "1102:5"]
    pub const XML_PARSE_NODICT: C2RustUnnamed_18 = 4096;
    /* Implement XInclude substitition  */
    #[src_loc = "1101:5"]
    pub const XML_PARSE_NONET: C2RustUnnamed_18 = 2048;
    /* use the SAX1 interface internally */
    #[src_loc = "1100:5"]
    pub const XML_PARSE_XINCLUDE: C2RustUnnamed_18 = 1024;
    /* remove blank nodes */
    #[src_loc = "1099:5"]
    pub const XML_PARSE_SAX1: C2RustUnnamed_18 = 512;
    /* pedantic error reporting */
    #[src_loc = "1098:5"]
    pub const XML_PARSE_NOBLANKS: C2RustUnnamed_18 = 256;
    /* suppress warning reports */
    #[src_loc = "1097:5"]
    pub const XML_PARSE_PEDANTIC: C2RustUnnamed_18 = 128;
    /* suppress error reports */
    #[src_loc = "1096:5"]
    pub const XML_PARSE_NOWARNING: C2RustUnnamed_18 = 64;
    /* validate with the DTD */
    #[src_loc = "1095:5"]
    pub const XML_PARSE_NOERROR: C2RustUnnamed_18 = 32;
    /* default DTD attributes */
    #[src_loc = "1094:5"]
    pub const XML_PARSE_DTDVALID: C2RustUnnamed_18 = 16;
    /* load the external subset */
    #[src_loc = "1093:5"]
    pub const XML_PARSE_DTDATTR: C2RustUnnamed_18 = 8;
    /* substitute entities */
    #[src_loc = "1092:5"]
    pub const XML_PARSE_DTDLOAD: C2RustUnnamed_18 = 4;
    /* recover on errors */
    #[src_loc = "1091:5"]
    pub const XML_PARSE_NOENT: C2RustUnnamed_18 = 2;
    #[src_loc = "1090:5"]
    pub const XML_PARSE_RECOVER: C2RustUnnamed_18 = 1;
    use super::libc;
    use super::tree_h::xmlDocPtr;
    extern "C" {
        #[no_mangle]
        #[src_loc = "1137:11"]
        pub fn xmlReadMemory(buffer: *const libc::c_char, size: libc::c_int,
                             URL: *const libc::c_char,
                             encoding: *const libc::c_char,
                             options: libc::c_int) -> xmlDocPtr;
    }
    /* __XML_PARSER_H__ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:20"]
pub mod unistd_h {
    use super::libc;
    use super::_pid_t_h::pid_t;
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "437:1"]
        pub fn close(_: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "440:1"]
        pub fn dup2(_: libc::c_int, _: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "444:1"]
        pub fn execv(__path: *const libc::c_char,
                     __argv: *const *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "447:1"]
        pub fn fork() -> pid_t;
        #[no_mangle]
        #[src_loc = "470:1"]
        pub fn pipe(_: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "472:1"]
        pub fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t)
         -> ssize_t;
    }
    /* _UNISTD_H_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/fcntl.h:21"]
pub mod fcntl_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "535:1"]
        pub fn fcntl(_: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    }
    /* !_SYS_FCNTL_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/time.h:22"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:23"]
pub mod string_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "71:6"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "72:7"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:24"]
pub mod _ctype_h {
    #[inline]
    #[src_loc = "134:1"]
    pub unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
        return (_c & !0x7fi32 == 0i32) as libc::c_int;
    }
    #[inline]
    #[src_loc = "152:1"]
    pub unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t,
                                      mut _f: libc::c_ulong) -> libc::c_int {
        return if isascii(_c) != 0 {
                   (_DefaultRuneLocale.__runetype[_c as usize] as
                        libc::c_ulong & _f != 0) as libc::c_int
               } else { (__maskrune(_c, _f) != 0) as libc::c_int };
    }
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[src_loc = "266:1"]
    pub unsafe extern "C" fn isspace(mut _c: libc::c_int) -> libc::c_int {
        return __istype(_c, 0x4000i64 as libc::c_ulong);
    }
    use super::libc;
    use super::_types_h::{__darwin_ct_rune_t, __uint32_t};
    use super::runetype_h::_DefaultRuneLocale;
    extern "C" {
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
    use super::libc;
    /* ! _OS__OSBYTEORDERI386_H */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/wait.h:26"]
pub mod wait_h {
    use super::_pid_t_h::pid_t;
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "249:1"]
        pub fn waitpid(_: pid_t, _: *mut libc::c_int, _: libc::c_int)
         -> pid_t;
    }
    /* !_SYS_WAIT_H_ */
    /* (!defined(_POSIX_C_SOURCE) || defined(_DARWIN_C_SOURCE)) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:26"]
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
    }
    /* _MALLOC_UNDERSCORE_MALLOC_H_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:26"]
pub mod stdlib_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "135:1"]
        pub fn atoi(_: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "145:7"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[src_loc = "175:3"]
        pub fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char,
                       _: libc::c_int) -> libc::c_ulong;
    }
    /* _STDLIB_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* Poison the following routines if -fshort-wchar is set */
    /* !_ANSI_SOURCE && !_POSIX_SOURCE */
    /* valloc is now declared in _malloc.h */
    /* getsubopt(3) external variable */
    /* !__DARWIN_NO_LONG_LONG */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::libc;
    extern "C" {
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arpa/inet.h:57"]
pub mod inet_h {
    use super::libc;
    use super::_in_addr_t_h::in_addr_t;
    extern "C" {
        /*
 * ++Copyright++ 1983, 1993
 * -
 * Copyright (c) 1983, 1993
 *    The Regents of the University of California.  All rights reserved.
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
 * 	This product includes software developed by the University of
 * 	California, Berkeley and its contributors.
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
 * -
 * Portions Copyright (c) 1993 by Digital Equipment Corporation.
 * 
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies, and that
 * the name of Digital Equipment Corporation not be used in advertising or
 * publicity pertaining to distribution of the document or software without
 * specific, written prior permission.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS" AND DIGITAL EQUIPMENT CORP. DISCLAIMS ALL
 * WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS.   IN NO EVENT SHALL DIGITAL EQUIPMENT
 * CORPORATION BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL
 * DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR
 * PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS
 * ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
 * SOFTWARE.
 * -
 * --Copyright--
 */
        /*
 *	@(#)inet.h	8.1 (Berkeley) 6/2/93
 *	$Id: inet.h,v 1.10 2006/02/01 18:09:47 majka Exp $
 */
        /* External definitions for functions in inet(3), addr2ascii(3) */
        /* uint32_t uint16_t */
        /* htonl() and family if (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
        /* htonl() and family if (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
        /* in_addr */
        #[no_mangle]
        #[src_loc = "75:1"]
        pub fn inet_addr(_: *const libc::c_char) -> in_addr_t;
    }
    /* !_ARPA_INET_H_ */
    /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
}
#[header_src = "/usr/local/Cellar/gettext/0.20.1/include/libintl.h:57"]
pub mod libintl_h {
    use super::libc;
    extern "C" {
        #[no_mangle]
        #[src_loc = "156:1"]
        pub fn libintl_dgettext(__domainname: *const libc::c_char,
                                __msgid: *const libc::c_char)
         -> *mut libc::c_char;
    }
    /* libintl.h */
}
use c2rust_bitfields::BitfieldStruct;
pub use self::_types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                         __darwin_ct_rune_t, __darwin_size_t,
                         __darwin_wchar_t, __darwin_rune_t,
                         __darwin_socklen_t, __darwin_ssize_t,
                         __darwin_time_t};
pub use self::sys__types_h::{__darwin_gid_t, __darwin_pid_t,
                             __darwin_suseconds_t, __darwin_uid_t};
pub use self::_size_t_h::size_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_u_int16_t_h::u_int16_t;
pub use self::_u_int32_t_h::u_int32_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_uid_t_h::uid_t;
pub use self::_gid_t_h::gid_t;
pub use self::_pid_t_h::pid_t;
pub use self::_fd_def_h::fd_set;
pub use self::_timeval_h::timeval;
pub use self::_time_t_h::time_t;
pub use self::runetype_h::{_RuneEntry, _RuneRange, _RuneCharClass,
                           _RuneLocale, _DefaultRuneLocale};
pub use self::_uint8_t_h::uint8_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_u_char_h::u_char;
pub use self::_u_short_h::u_short;
pub use self::_u_int_h::u_int;
pub use self::_in_addr_t_h::in_addr_t;
pub use self::in_systm_h::{n_short, n_time};
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::socket_h::{sockaddr, send};
pub use self::in_h::in_addr;
pub use self::ip_h::ip;
pub use self::ip_icmp_h::{icmp_ra_addr, icmp, C2RustUnnamed, id_ip, id_ts,
                          C2RustUnnamed_0, ih_rtradv, ih_pmtu, ih_idseq};
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
pub use self::openconnect_internal_h::{pkt_q, pkt, C2RustUnnamed_1,
                                       C2RustUnnamed_2, C2RustUnnamed_3,
                                       C2RustUnnamed_4, C2RustUnnamed_5,
                                       C2RustUnnamed_6, keepalive_info,
                                       pin_cache, oc_text_buf,
                                       C2RustUnnamed_13, HOTP_SECRET_PSKC,
                                       HOTP_SECRET_HEX, HOTP_SECRET_RAW,
                                       HOTP_SECRET_BASE32, C2RustUnnamed_14,
                                       OATH_ALG_HMAC_SHA512,
                                       OATH_ALG_HMAC_SHA256,
                                       OATH_ALG_HMAC_SHA1, http_auth_state,
                                       C2RustUnnamed_15, C2RustUnnamed_16,
                                       C2RustUnnamed_17, esp, vpn_proto,
                                       oc_packed_uint32_t, oc_packed_uint16_t,
                                       load_be32, load_be16, load_le32,
                                       store_be32, store_be16, store_le32,
                                       set_fd_cloexec, queue_packet,
                                       dequeue_packet, oc_pcsc_ctx,
                                       udp_connect, construct_esp_packet,
                                       ka_check_deadline, ssl_nonblock_read,
                                       dump_buf_hex, ssl_nonblock_write,
                                       ka_stalled_action, keepalive_action,
                                       ssl_reconnect, udp_sockaddr,
                                       openconnect_setup_esp_keys,
                                       free_optlist, buf_truncate,
                                       buf_ensure_space, openconnect_md5,
                                       append_opt, do_https_request,
                                       set_csd_user, openconnect_open_https,
                                       buf_alloc, buf_append_bytes,
                                       buf_append, buf_error, dump_buf,
                                       openconnect_close_https, buf_free,
                                       xmlnode_match_prop, xmlnode_is_named,
                                       xmlnode_get_val, gpst_os_name,
                                       free_split_routes, unhex,
                                       openconnect_utf8_to_legacy};
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
                      stack_st_SSL_COMP, C2RustUnnamed_10, SSL_SESSION,
                      ssl_session_st, stack_st_SSL_CIPHER, SSL_CIPHER,
                      ssl_cipher_st, lhash_st_SSL_SESSION, SSL_METHOD,
                      ssl_method_st, tls_session_secret_cb_fn,
                      tls_session_ticket_ext_cb_fn, TLS_SESSION_TICKET_EXT,
                      tls_session_ticket_ext_st, dtls1_state_st,
                      ssl3_state_st, SSL_COMP, ssl_comp_st, ssl2_state_st,
                      ssl3_buf_freelist_st, cert_st, sess_cert_st,
                      ssl3_enc_method, stack_st_OCSP_RESPID};
pub use self::stack_h::{_STACK, stack_st};
pub use self::hmac_h::{HMAC_CTX, hmac_ctx_st};
pub use self::asn1_h::{ASN1_TYPE, asn1_type_st, C2RustUnnamed_7, ASN1_VALUE,
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
pub use self::evp_h::{stack_st_X509_ATTRIBUTE, C2RustUnnamed_8, ec_key_st};
pub use self::bn_h::C2RustUnnamed_9;
pub use self::dsa_h::{DSA_SIG, DSA_SIG_st};
pub use self::pem_h::pem_password_cb;
pub use self::comp_h::{COMP_CTX, comp_ctx_st};
pub use self::dtls1_h::{dtls1_timeout_st, hm_header_st,
                        dtls1_retransmit_state, record_pqueue,
                        record_pqueue_st, DTLS1_BITMAP, dtls1_bitmap_st};
pub use self::pqueue_h::{pqueue, _pqueue};
pub use self::ssl3_h::{C2RustUnnamed_11, SSL3_RECORD, ssl3_record_st,
                       SSL3_BUFFER, ssl3_buffer_st};
pub use self::ec_h::EC_KEY;
pub use self::bio_h::{BIO, BIO_METHOD, bio_method_st, bio_info_cb};
pub use self::ssl2_h::C2RustUnnamed_12;
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
                       XML_ATTRIBUTE_CDATA, xmlNodePtr, xmlDocPtr, xmlDoc,
                       xmlFreeDoc, xmlDocGetRootElement, xmlNodeGetContent};
use self::dict_h::_xmlDict;
pub use self::xmlstring_h::xmlChar;
pub use self::iconv_h::iconv_t;
pub use self::parser_h::{C2RustUnnamed_18, XML_PARSE_BIG_LINES,
                         XML_PARSE_IGNORE_ENC, XML_PARSE_OLDSAX,
                         XML_PARSE_HUGE, XML_PARSE_NOBASEFIX, XML_PARSE_OLD10,
                         XML_PARSE_COMPACT, XML_PARSE_NOXINCNODE,
                         XML_PARSE_NOCDATA, XML_PARSE_NSCLEAN,
                         XML_PARSE_NODICT, XML_PARSE_NONET,
                         XML_PARSE_XINCLUDE, XML_PARSE_SAX1,
                         XML_PARSE_NOBLANKS, XML_PARSE_PEDANTIC,
                         XML_PARSE_NOWARNING, XML_PARSE_NOERROR,
                         XML_PARSE_DTDVALID, XML_PARSE_DTDATTR,
                         XML_PARSE_DTDLOAD, XML_PARSE_NOENT,
                         XML_PARSE_RECOVER, xmlReadMemory};
use self::unistd_h::{close, dup2, execv, fork, pipe, read};
use self::fcntl_h::fcntl;
use self::time_h::time;
use self::string_h::{memcmp, memcpy, memset, strchr, strcmp, strerror, strlen,
                     strncmp, strdup, strndup};
pub use self::_ctype_h::{isascii, __istype, isspace, __maskrune};
pub use self::_OSByteOrder_h::{_OSSwapInt16, _OSSwapInt32};
use self::wait_h::waitpid;
use self::_malloc_h::{malloc, free};
use self::stdlib_h::{atoi, exit, strtoul};
use self::stdio_h::sprintf;
use self::inet_h::inet_addr;
use self::libintl_h::libintl_dgettext;
/*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2016-2017 Daniel Lenski
 *
 * Author: Daniel Lenski <dlenski@gmail.com>
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
/* The BSDs require the first two headers before netinet/ip.h
 * (Linux and macOS already #include them within netinet/ip.h)
 */
/*
 * Data packets are encapsulated in the SSL stream as follows:
 *
 * 0000: Magic "\x1a\x2b\x3c\x4d"
 * 0004: Big-endian EtherType (0x0800 for IPv4)
 * 0006: Big-endian 16-bit length (not including 16-byte header)
 * 0008: Always "\x01\0\0\0\0\0\0\0"
 * 0010: data payload
 */
/* Strange initialisers here to work around GCC PR#10676 (which was
 * fixed in GCC 4.6 but it takes a while for some systems to catch
 * up. */
#[src_loc = "72:25"]
static mut dpd_pkt: pkt =
    {
        let mut init =
            pkt{len: 0,
                next: 0 as *const pkt as *mut pkt,
                c2rust_unnamed:
                    C2RustUnnamed_1{gpst:
                                        {
                                            let mut init =
                                                C2RustUnnamed_3{pad: [0; 8],
                                                                hdr:
                                                                    [0x1ai32
                                                                         as
                                                                         libc::c_uchar,
                                                                     0x2bi32
                                                                         as
                                                                         libc::c_uchar,
                                                                     0x3ci32
                                                                         as
                                                                         libc::c_uchar,
                                                                     0x4di32
                                                                         as
                                                                         libc::c_uchar,
                                                                     0, 0, 0,
                                                                     0, 0, 0,
                                                                     0, 0, 0,
                                                                     0, 0,
                                                                     0],};
                                            init
                                        },},
                data: [],};
        init
    };
/* We behave like CSTP — create a linked list in vpninfo->cstp_options
 * with the strings containing the information we got from the server,
 * and oc_ip_info contains const copies of those pointers.
 *
 * (unlike version in oncp.c, val is stolen rather than strdup'ed) */
#[src_loc = "83:1"]
unsafe extern "C" fn add_option(mut vpninfo: *mut openconnect_info,
                                mut opt: *const libc::c_char,
                                mut val: *mut *mut libc::c_char)
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
    (*new).value = *val;
    *val = 0 as *mut libc::c_char;
    (*new).next = (*vpninfo).cstp_options;
    (*vpninfo).cstp_options = new;
    return (*new).value;
}
#[src_loc = "102:1"]
unsafe extern "C" fn filter_opts(mut buf: *mut oc_text_buf,
                                 mut query: *const libc::c_char,
                                 mut incexc: *const libc::c_char,
                                 mut include: libc::c_int) -> libc::c_int {
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    let mut endf: *const libc::c_char = 0 as *const libc::c_char;
    let mut eq: *const libc::c_char = 0 as *const libc::c_char;
    let mut found: *const libc::c_char = 0 as *const libc::c_char;
    let mut comma: *const libc::c_char = 0 as *const libc::c_char;
    f = query;
    while *f != 0 {
        let ref mut fresh0 = strchr(f, '&' as i32);
        endf =
            if !(*fresh0).is_null() {
                *fresh0
            } else { f.offset(strlen(f) as isize) };
        eq = strchr(f, '=' as i32);
        if eq.is_null() || eq > endf { eq = endf }
        found = incexc;
        while *found != 0 {
            let ref mut fresh1 = strchr(found, ',' as i32);
            comma =
                if !(*fresh1).is_null() {
                    *fresh1
                } else { found.offset(strlen(found) as isize) };
            if strncmp(found, f,
                       if comma.wrapping_offset_from(found) as libc::c_long >
                              eq.wrapping_offset_from(f) as libc::c_long {
                           comma.wrapping_offset_from(found) as libc::c_long
                       } else { eq.wrapping_offset_from(f) as libc::c_long }
                           as libc::c_ulong) == 0 {
                break ;
            }
            found =
                if *comma as libc::c_int != 0 {
                    comma.offset(1)
                } else { comma }
        }
        if include != 0 && *found as libc::c_int != 0 ||
               include == 0 && *found == 0 {
            if (*buf).pos != 0 &&
                   *(*buf).data.offset(((*buf).pos - 1i32) as isize) as
                       libc::c_int != '?' as i32 &&
                   *(*buf).data.offset(((*buf).pos - 1i32) as isize) as
                       libc::c_int != '&' as i32 {
                buf_append(buf, b"&\x00" as *const u8 as *const libc::c_char);
            }
            buf_append_bytes(buf, f as *const libc::c_void,
                             endf.wrapping_offset_from(f) as libc::c_long as
                                 libc::c_int);
        }
        f = if *endf as libc::c_int != 0 { endf.offset(1) } else { endf }
    }
    return buf_error(buf);
}
/* Parse this JavaScript-y mess:

	"var respStatus = \"Challenge|Error\";\n"
	"var respMsg = \"<prompt>\";\n"
	"thisForm.inputStr.value = "<inputStr>";\n"
*/
#[src_loc = "134:1"]
unsafe extern "C" fn parse_javascript(mut buf: *mut libc::c_char,
                                      mut prompt: *mut *mut libc::c_char,
                                      mut inputStr: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = buf;
    let mut status: libc::c_int = 0;
    let mut pre_status: *const libc::c_char =
        b"var respStatus = \"\x00" as *const u8 as *const libc::c_char;
    let mut pre_prompt: *const libc::c_char =
        b"var respMsg = \"\x00" as *const u8 as *const libc::c_char;
    let mut pre_inputStr: *const libc::c_char =
        b"thisForm.inputStr.value = \"\x00" as *const u8 as
            *const libc::c_char;
    /* Status */
    while isspace(*end as libc::c_int) != 0 { end = end.offset(1) }
    if !(strncmp(end, pre_status, strlen(pre_status)) != 0) {
        start = end.offset(strlen(pre_status) as isize);
        end = strchr(start, '\n' as i32);
        if !(end.is_null() ||
                 *end.offset(-1i32 as isize) as libc::c_int != ';' as i32 ||
                 *end.offset(-2i32 as isize) as libc::c_int != '\"' as i32) {
            if strncmp(start,
                       b"Challenge\x00" as *const u8 as *const libc::c_char,
                       8i32 as libc::c_ulong) == 0 {
                status = 0i32;
                current_block = 17965632435239708295;
            } else if strncmp(start,
                              b"Error\x00" as *const u8 as
                                  *const libc::c_char, 5i32 as libc::c_ulong)
                          == 0 {
                status = 1i32;
                current_block = 17965632435239708295;
            } else { current_block = 15465179129558584698; }
            match current_block {
                15465179129558584698 => { }
                _ => {
                    /* Prompt */
                    while isspace(*end as libc::c_int) != 0 {
                        end = end.offset(1)
                    }
                    if !(strncmp(end, pre_prompt, strlen(pre_prompt)) != 0) {
                        start = end.offset(strlen(pre_prompt) as isize);
                        end = strchr(start, '\n' as i32);
                        if !(end.is_null() ||
                                 *end.offset(-1i32 as isize) as libc::c_int !=
                                     ';' as i32 ||
                                 *end.offset(-2i32 as isize) as libc::c_int !=
                                     '\"' as i32 || end < start.offset(2)) {
                            if !prompt.is_null() {
                                *prompt =
                                    strndup(start,
                                            (end.wrapping_offset_from(start)
                                                 as libc::c_long -
                                                 2i32 as libc::c_long) as
                                                libc::c_ulong)
                            }
                            /* inputStr */
                            while isspace(*end as libc::c_int) != 0 {
                                end = end.offset(1)
                            }
                            if !(strncmp(end, pre_inputStr,
                                         strlen(pre_inputStr)) != 0) {
                                start =
                                    end.offset(strlen(pre_inputStr) as isize);
                                end = strchr(start, '\n' as i32);
                                if !(end.is_null() ||
                                         *end.offset(-1i32 as isize) as
                                             libc::c_int != ';' as i32 ||
                                         *end.offset(-2i32 as isize) as
                                             libc::c_int != '\"' as i32 ||
                                         end < start.offset(2)) {
                                    if !inputStr.is_null() {
                                        *inputStr =
                                            strndup(start,
                                                    (end.wrapping_offset_from(start)
                                                         as libc::c_long -
                                                         2i32 as libc::c_long)
                                                        as libc::c_ulong)
                                    }
                                    while isspace(*end as libc::c_int) != 0 {
                                        end = end.offset(1)
                                    }
                                    if *end as libc::c_int != '\u{0}' as i32 {
                                        if !inputStr.is_null() {
                                            free(*inputStr as
                                                     *mut libc::c_void);
                                        }
                                    } else { return status }
                                }
                            }
                            if !prompt.is_null() {
                                free(*prompt as *mut libc::c_void);
                            }
                        }
                    }
                }
            }
        }
    }
    return -22i32;
}
#[no_mangle]
#[src_loc = "201:1"]
pub unsafe extern "C" fn gpst_xml_or_error(mut vpninfo: *mut openconnect_info,
                                           mut response: *mut libc::c_char,
                                           mut xml_cb:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut openconnect_info,
                                                                           _:
                                                                               *mut xmlNode,
                                                                           _:
                                                                               *mut libc::c_void)
                                                          -> libc::c_int>,
                                           mut challenge_cb:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut openconnect_info,
                                                                           _:
                                                                               *mut libc::c_char,
                                                                           _:
                                                                               *mut libc::c_char,
                                                                           _:
                                                                               *mut libc::c_void)
                                                          -> libc::c_int>,
                                           mut cb_data: *mut libc::c_void)
 -> libc::c_int {
    let mut current_block: u64;
    let mut xml_doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut xml_node: *mut xmlNode = 0 as *mut xmlNode;
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inputStr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = -22i32;
    if response.is_null() {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Empty response from server\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        return -22i32
    }
    /* is it XML? */
    xml_doc =
        xmlReadMemory(response, strlen(response) as libc::c_int,
                      b"noname.xml\x00" as *const u8 as *const libc::c_char,
                      0 as *const libc::c_char,
                      XML_PARSE_NOERROR as libc::c_int);
    if xml_doc.is_null() {
        /* is it Javascript? */
        result = parse_javascript(response, &mut prompt, &mut inputStr);
        match result {
            1 => {
                current_block = 6470822782134901710;
                match current_block {
                    6470822782134901710 => {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"%s\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char),
                                                                                    prompt);
                        }
                    }
                    _ => {
                        if (*vpninfo).verbose >= 1i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    1i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Challenge: %s\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char),
                                                                                    prompt);
                        }
                        result =
                            if challenge_cb.is_some() {
                                challenge_cb.expect("non-null function pointer")(vpninfo,
                                                                                 prompt,
                                                                                 inputStr,
                                                                                 cb_data)
                            } else { -22i32 }
                    }
                }
                free(prompt as *mut libc::c_void);
                free(inputStr as *mut libc::c_void);
                current_block = 16766476312343843419;
            }
            0 => {
                current_block = 7277133277968711435;
                match current_block {
                    6470822782134901710 => {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"%s\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char),
                                                                                    prompt);
                        }
                    }
                    _ => {
                        if (*vpninfo).verbose >= 1i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    1i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Challenge: %s\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char),
                                                                                    prompt);
                        }
                        result =
                            if challenge_cb.is_some() {
                                challenge_cb.expect("non-null function pointer")(vpninfo,
                                                                                 prompt,
                                                                                 inputStr,
                                                                                 cb_data)
                            } else { -22i32 }
                    }
                }
                free(prompt as *mut libc::c_void);
                free(inputStr as *mut libc::c_void);
                current_block = 16766476312343843419;
            }
            _ => { current_block = 16766476312343843419; }
        }
    } else {
        xml_node = xmlDocGetRootElement(xml_doc as *const xmlDoc);
        /* is it <response status="error"><error>..</error></response> ? */
        if xmlnode_is_named(xml_node,
                            b"response\x00" as *const u8 as
                                *const libc::c_char) != 0 &&
               xmlnode_match_prop(xml_node,
                                  b"status\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"error\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
            xml_node = (*xml_node).children;
            loop  {
                if xml_node.is_null() {
                    current_block = 16766476312343843419;
                    break ;
                }
                if xmlnode_get_val(xml_node,
                                   b"error\x00" as *const u8 as
                                       *const libc::c_char, &mut err) == 0 {
                    current_block = 15390401390535582915;
                    break ;
                }
                xml_node = (*xml_node).next
            }
        } else {
            /* Is it <prelogin-response><status>Error</status><msg>..</msg></prelogin-response> ? */
            if xmlnode_is_named(xml_node,
                                b"prelogin-response\x00" as *const u8 as
                                    *const libc::c_char) != 0 {
                let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut has_err: libc::c_int = 0i32;
                let mut x: *mut xmlNode = 0 as *mut xmlNode;
                x = (*xml_node).children;
                while !x.is_null() {
                    if xmlnode_get_val(x,
                                       b"status\x00" as *const u8 as
                                           *const libc::c_char, &mut s) == 0 {
                        has_err =
                            strcmp(s,
                                   b"Success\x00" as *const u8 as
                                       *const libc::c_char)
                    } else {
                        xmlnode_get_val(x,
                                        b"msg\x00" as *const u8 as
                                            *const libc::c_char, &mut err);
                    }
                    x = (*x).next
                }
                free(s as *mut libc::c_void);
                if has_err != 0 {
                    current_block = 15390401390535582915;
                } else {
                    free(err as *mut libc::c_void);
                    err = 0 as *mut libc::c_char;
                    current_block = 12381812505308290051;
                }
            } else { current_block = 12381812505308290051; }
            match current_block {
                15390401390535582915 => { }
                _ =>
                /* is it <challenge><user>user.name</user><inputstr>...</inputstr><respmsg>...</respmsg></challenge> */
                {
                    if xmlnode_is_named(xml_node,
                                        b"challenge\x00" as *const u8 as
                                            *const libc::c_char) != 0 {
                        xml_node = (*xml_node).children;
                        while !xml_node.is_null() {
                            xmlnode_get_val(xml_node,
                                            b"inputstr\x00" as *const u8 as
                                                *const libc::c_char,
                                            &mut inputStr);
                            xmlnode_get_val(xml_node,
                                            b"respmsg\x00" as *const u8 as
                                                *const libc::c_char,
                                            &mut prompt);
                            xml_node = (*xml_node).next
                            /* XXX: override the username passed to the next form from <user> ? */
                        }
                        result =
                            if challenge_cb.is_some() {
                                challenge_cb.expect("non-null function pointer")(vpninfo,
                                                                                 prompt,
                                                                                 inputStr,
                                                                                 cb_data)
                            } else { -22i32 };
                        free(prompt as *mut libc::c_void);
                        free(inputStr as *mut libc::c_void);
                    } else {
                        /* if it's XML, invoke callback (or default to success) */
                        result =
                            if xml_cb.is_some() {
                                xml_cb.expect("non-null function pointer")(vpninfo,
                                                                           xml_node,
                                                                           cb_data)
                            } else { 0i32 }
                    }
                    current_block = 16766476312343843419;
                }
            }
        }
    }
    match current_block {
        16766476312343843419 => {
            if result == -22i32 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to parse server response\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Response was:%s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            response);
                }
            }
        }
        _ => { }
    }
    if !err.is_null() {
        if strcmp(err,
                  b"GlobalProtect gateway does not exist\x00" as *const u8 as
                      *const libc::c_char) == 0 ||
               strcmp(err,
                      b"GlobalProtect portal does not exist\x00" as *const u8
                          as *const libc::c_char) == 0 {
            if (*vpninfo).verbose >= 2i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        2i32,
                                                                        b"%s\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        err);
            }
            result = -17i32
        } else if strcmp(err,
                         b"Invalid authentication cookie\x00" as *const u8 as
                             *const libc::c_char) == 0 ||
                      strcmp(err,
                             b"Valid client certificate is required\x00" as
                                 *const u8 as *const libc::c_char) == 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        b"%s\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        err);
            }
            result = -1i32
        } else {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        b"%s\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        err);
            }
            result = -22i32
        }
        free(err as *mut libc::c_void);
    }
    if !xml_doc.is_null() { xmlFreeDoc(xml_doc); }
    return result;
}
/* Based on cstp.c's calculate_mtu().
 *
 * With HTTPS tunnel, there are 21 bytes of overhead beyond the
 * TCP MSS: 5 bytes for TLS and 16 for GPST.
 */
#[src_loc = "328:1"]
unsafe extern "C" fn calculate_mtu(mut vpninfo: *mut openconnect_info,
                                   mut can_use_esp: libc::c_int)
 -> libc::c_int {
    let mut mtu: libc::c_int = (*vpninfo).reqmtu;
    let mut base_mtu: libc::c_int = (*vpninfo).basemtu;
    let mut mss: libc::c_int = 0i32;
    if base_mtu == 0 {
        /* Default */
        base_mtu = 1406i32
    }
    if base_mtu < 1280i32 { base_mtu = 1280i32 }
    /* If we can use the ESP tunnel then we should pick the optimal MTU for ESP. */
    if mtu == 0 && can_use_esp != 0 {
        /* remove ESP, UDP, IP headers from base (wire) MTU */
        mtu =
            base_mtu - 8i32 - (4i32 + 4i32) - (*vpninfo).hmac_out_len - 16i32;
        if (*(*vpninfo).peer_addr).sa_family as libc::c_int == 30i32 {
            mtu -= 40i32
        } else { mtu -= 20i32 }
        /* round down to a multiple of blocksize (16 bytes for both AES-128 and AES-256) */
        mtu -= mtu % 16i32;
        /* subtract ESP footer, which is included in the payload before padding to the blocksize */
        mtu -= 1i32 + 1i32
    } else if mtu == 0 {
        if mss != 0 {
            mtu = mss - 21i32
        } else {
            mtu = base_mtu - 20i32 - 21i32;
            if (*(*vpninfo).peer_addr).sa_family as libc::c_int == 30i32 {
                mtu -= 40i32
            } else { mtu -= 20i32 }
        }
    }
    return mtu;
}
#[src_loc = "407:1"]
unsafe extern "C" fn check_hmac_algo(mut v: *mut openconnect_info,
                                     mut s: *const libc::c_char)
 -> libc::c_int {
    if strcmp(s, b"sha1\x00" as *const u8 as *const libc::c_char) == 0 {
        return 2i32
    }
    if strcmp(s, b"md5\x00" as *const u8 as *const libc::c_char) == 0 {
        return 1i32
    }
    if strcmp(s, b"sha256\x00" as *const u8 as *const libc::c_char) == 0 {
        return 3i32
    }
    if (*v).verbose >= 0i32 {
        (*v).progress.expect("non-null function pointer")((*v).cbdata, 0i32,
                                                          libintl_dgettext(b"openconnect\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           b"Unknown ESP MAC algorithm: %s\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char),
                                                          s);
    }
    return -2i32;
}
#[src_loc = "416:1"]
unsafe extern "C" fn check_enc_algo(mut v: *mut openconnect_info,
                                    mut s: *const libc::c_char)
 -> libc::c_int {
    if strcmp(s, b"aes128\x00" as *const u8 as *const libc::c_char) == 0 ||
           strcmp(s, b"aes-128-cbc\x00" as *const u8 as *const libc::c_char)
               == 0 {
        return 2i32
    }
    if strcmp(s, b"aes-256-cbc\x00" as *const u8 as *const libc::c_char) == 0
       {
        return 5i32
    }
    if (*v).verbose >= 0i32 {
        (*v).progress.expect("non-null function pointer")((*v).cbdata, 0i32,
                                                          libintl_dgettext(b"openconnect\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           b"Unknown ESP encryption algorithm: %s\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char),
                                                          s);
    }
    return -2i32;
}
/* We are definitely using the TLS tunnel, so we should base our MTU on the TCP MSS. */
/* Reads <KEYTAG/><bits>N</bits><val>hex digits</val></KEYTAG> and saves the
 * key in dest, returning its length in bytes.
 */
#[src_loc = "427:1"]
unsafe extern "C" fn xml_to_key(mut xml_node: *mut xmlNode,
                                mut dest: *mut libc::c_uchar,
                                mut dest_size: libc::c_int) -> libc::c_int {
    let mut explen: libc::c_int = -1i32;
    let mut len: libc::c_int = 0i32;
    let mut child: *mut xmlNode = 0 as *mut xmlNode;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    child = (*xml_node).children;
    while !child.is_null() {
        if xmlnode_get_val(child,
                           b"bits\x00" as *const u8 as *const libc::c_char,
                           &mut s) == 0i32 {
            explen = atoi(s);
            if explen & 0x7i32 != 0 { break ; }
            explen >>= 3i32
        } else if xmlnode_get_val(child,
                                  b"val\x00" as *const u8 as
                                      *const libc::c_char, &mut s) == 0i32 {
            p = s;
            while *p.offset(0) as libc::c_int != 0 &&
                      *p.offset(1) as libc::c_int != 0 {
                let fresh2 = len;
                len = len + 1;
                if fresh2 < dest_size {
                    let fresh3 = dest;
                    dest = dest.offset(1);
                    *fresh3 = unhex(p)
                }
                p = p.offset(2)
            }
        }
        child = (*child).next
    }
    free(s as *mut libc::c_void);
    return if len == explen { len } else { -22i32 };
}
/* Return value:
 *  < 0, on error
 *  = 0, on success; *form is populated
 */
#[src_loc = "454:1"]
unsafe extern "C" fn gpst_parse_config_xml(mut vpninfo: *mut openconnect_info,
                                           mut xml_node: *mut xmlNode,
                                           mut cb_data: *mut libc::c_void)
 -> libc::c_int {
    let mut member: *mut xmlNode = 0 as *mut xmlNode;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ii: libc::c_int = 0;
    if xml_node.is_null() ||
           xmlnode_is_named(xml_node,
                            b"response\x00" as *const u8 as
                                *const libc::c_char) == 0 {
        return -22i32
    }
    /* Clear old options which will be overwritten */
    (*vpninfo).ip_info.netmask = 0 as *const libc::c_char;
    (*vpninfo).ip_info.addr = (*vpninfo).ip_info.netmask;
    (*vpninfo).ip_info.netmask6 = 0 as *const libc::c_char;
    (*vpninfo).ip_info.addr6 = (*vpninfo).ip_info.netmask6;
    (*vpninfo).ip_info.domain = 0 as *const libc::c_char;
    (*vpninfo).ip_info.mtu = 0i32;
    (*vpninfo).esp_magic = inet_addr((*vpninfo).ip_info.gateway_addr);
    (*vpninfo).esp_replay_protect = 1i32 as uint32_t;
    (*vpninfo).ssl_times.rekey_method = 0i32;
    (*vpninfo).cstp_options = 0 as *mut oc_vpn_option;
    ii = 0i32;
    while ii < 3i32 {
        (*vpninfo).ip_info.nbns[ii as usize] = 0 as *const libc::c_char;
        (*vpninfo).ip_info.dns[ii as usize] =
            (*vpninfo).ip_info.nbns[ii as usize];
        ii += 1
    }
    free_split_routes(vpninfo);
    /* Parse config */
    xml_node = (*xml_node).children;
    while !xml_node.is_null() {
        if xmlnode_get_val(xml_node,
                           b"ip-address\x00" as *const u8 as
                               *const libc::c_char, &mut s) == 0 {
            (*vpninfo).ip_info.addr =
                add_option(vpninfo,
                           b"ipaddr\x00" as *const u8 as *const libc::c_char,
                           &mut s)
        } else if xmlnode_get_val(xml_node,
                                  b"netmask\x00" as *const u8 as
                                      *const libc::c_char, &mut s) == 0 {
            (*vpninfo).ip_info.netmask =
                add_option(vpninfo,
                           b"netmask\x00" as *const u8 as *const libc::c_char,
                           &mut s)
        } else if xmlnode_get_val(xml_node,
                                  b"mtu\x00" as *const u8 as
                                      *const libc::c_char, &mut s) == 0 {
            (*vpninfo).ip_info.mtu = atoi(s)
        } else if xmlnode_get_val(xml_node,
                                  b"lifetime\x00" as *const u8 as
                                      *const libc::c_char, &mut s) == 0 {
            if (*vpninfo).verbose >= 1i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        1i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Session will expire after %d minutes.\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        atoi(s)
                                                                            /
                                                                            60i32);
            }
        } else if xmlnode_get_val(xml_node,
                                  b"disconnect-on-idle\x00" as *const u8 as
                                      *const libc::c_char, &mut s) == 0 {
            let mut sec: libc::c_int = atoi(s);
            if (*vpninfo).verbose >= 1i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        1i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Idle timeout is %d minutes.\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        sec /
                                                                            60i32);
            }
            (*vpninfo).idle_timeout = sec
        } else if xmlnode_get_val(xml_node,
                                  b"ssl-tunnel-url\x00" as *const u8 as
                                      *const libc::c_char, &mut s) == 0 {
            free((*vpninfo).urlpath as *mut libc::c_void);
            (*vpninfo).urlpath = s;
            if strcmp(s,
                      b"/ssl-tunnel-connect.sslvpn\x00" as *const u8 as
                          *const libc::c_char) != 0 {
                if (*vpninfo).verbose >= 1i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            1i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Non-standard SSL tunnel path: %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            s);
                }
            }
            s = 0 as *mut libc::c_char
        } else if xmlnode_get_val(xml_node,
                                  b"timeout\x00" as *const u8 as
                                      *const libc::c_char, &mut s) == 0 {
            let mut sec_0: libc::c_int = atoi(s);
            if (*vpninfo).verbose >= 1i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        1i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Tunnel timeout (rekey interval) is %d minutes.\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        sec_0
                                                                            /
                                                                            60i32);
            }
            (*vpninfo).ssl_times.last_rekey = time(0 as *mut time_t);
            (*vpninfo).ssl_times.rekey = sec_0 - 60i32;
            (*vpninfo).ssl_times.rekey_method = 1i32
        } else if xmlnode_get_val(xml_node,
                                  b"gw-address\x00" as *const u8 as
                                      *const libc::c_char, &mut s) == 0 {
            /* As remarked in oncp.c, "this is a tunnel; having a
			 * gateway is meaningless." See esp_send_probes_gp for the
			 * gory details of what this field actually means.
			 */
            if strcmp(s, (*vpninfo).ip_info.gateway_addr) != 0 {
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Gateway address in config XML (%s) differs from external gateway address (%s).\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            s,
                                                                            (*vpninfo).ip_info.gateway_addr);
                }
            }
            (*vpninfo).esp_magic = inet_addr(s)
        } else if xmlnode_is_named(xml_node,
                                   b"dns\x00" as *const u8 as
                                       *const libc::c_char) != 0 {
            ii = 0i32;
            member = (*xml_node).children;
            while !member.is_null() && ii < 3i32 {
                if xmlnode_get_val(member,
                                   b"member\x00" as *const u8 as
                                       *const libc::c_char, &mut s) == 0 {
                    let fresh4 = ii;
                    ii = ii + 1;
                    (*vpninfo).ip_info.dns[fresh4 as usize] =
                        add_option(vpninfo,
                                   b"DNS\x00" as *const u8 as
                                       *const libc::c_char, &mut s)
                }
                member = (*member).next
            }
        } else if xmlnode_is_named(xml_node,
                                   b"wins\x00" as *const u8 as
                                       *const libc::c_char) != 0 {
            ii = 0i32;
            member = (*xml_node).children;
            while !member.is_null() && ii < 3i32 {
                if xmlnode_get_val(member,
                                   b"member\x00" as *const u8 as
                                       *const libc::c_char, &mut s) == 0 {
                    let fresh5 = ii;
                    ii = ii + 1;
                    (*vpninfo).ip_info.nbns[fresh5 as usize] =
                        add_option(vpninfo,
                                   b"WINS\x00" as *const u8 as
                                       *const libc::c_char, &mut s)
                }
                member = (*member).next
            }
        } else if xmlnode_is_named(xml_node,
                                   b"dns-suffix\x00" as *const u8 as
                                       *const libc::c_char) != 0 {
            let mut domains: *mut oc_text_buf = buf_alloc();
            member = (*xml_node).children;
            while !member.is_null() {
                if xmlnode_get_val(member,
                                   b"member\x00" as *const u8 as
                                       *const libc::c_char, &mut s) == 0 {
                    buf_append(domains,
                               b"%s \x00" as *const u8 as *const libc::c_char,
                               s);
                }
                member = (*member).next
            }
            if buf_error(domains) == 0i32 && (*domains).pos > 0i32 {
                *(*domains).data.offset(((*domains).pos - 1i32) as isize) =
                    '\u{0}' as i32 as libc::c_char;
                (*vpninfo).ip_info.domain =
                    add_option(vpninfo,
                               b"search\x00" as *const u8 as
                                   *const libc::c_char, &mut (*domains).data)
            }
            buf_free(domains);
        } else if xmlnode_is_named(xml_node,
                                   b"access-routes\x00" as *const u8 as
                                       *const libc::c_char) != 0 ||
                      xmlnode_is_named(xml_node,
                                       b"exclude-access-routes\x00" as
                                           *const u8 as *const libc::c_char)
                          != 0 {
            member = (*xml_node).children;
            while !member.is_null() {
                if xmlnode_get_val(member,
                                   b"member\x00" as *const u8 as
                                       *const libc::c_char, &mut s) == 0 {
                    let mut inc: *mut oc_split_include =
                        malloc(::std::mem::size_of::<oc_split_include>() as
                                   libc::c_ulong) as *mut oc_split_include;
                    if !inc.is_null() {
                        if xmlnode_is_named(xml_node,
                                            b"access-routes\x00" as *const u8
                                                as *const libc::c_char) != 0 {
                            (*inc).route =
                                add_option(vpninfo,
                                           b"split-include\x00" as *const u8
                                               as *const libc::c_char,
                                           &mut s);
                            (*inc).next = (*vpninfo).ip_info.split_includes;
                            (*vpninfo).ip_info.split_includes = inc
                        } else {
                            (*inc).route =
                                add_option(vpninfo,
                                           b"split-exclude\x00" as *const u8
                                               as *const libc::c_char,
                                           &mut s);
                            (*inc).next = (*vpninfo).ip_info.split_excludes;
                            (*vpninfo).ip_info.split_excludes = inc
                        }
                    }
                }
                member = (*member).next
            }
        } else if xmlnode_is_named(xml_node,
                                   b"ipsec\x00" as *const u8 as
                                       *const libc::c_char) != 0 {
            if (*vpninfo).dtls_state != 2i32 {
                (*vpninfo).current_esp_in ^= 1i32;
                let mut c: libc::c_int = (*vpninfo).current_esp_in;
                let mut ei: *mut esp =
                    &mut *(*vpninfo).esp_in.as_mut_ptr().offset(c as isize) as
                        *mut esp;
                let mut eo: *mut esp = &mut (*vpninfo).esp_out;
                (*vpninfo).old_esp_maxseq =
                    (*vpninfo).esp_in[(c ^ 1i32) as
                                          usize].seq.wrapping_add(32i32 as
                                                                      libc::c_ulonglong)
                        as libc::c_int;
                member = (*xml_node).children;
                while !member.is_null() {
                    if xmlnode_get_val(member,
                                       b"udp-port\x00" as *const u8 as
                                           *const libc::c_char, &mut s) == 0 {
                        udp_sockaddr(vpninfo, atoi(s));
                    } else if xmlnode_get_val(member,
                                              b"enc-algo\x00" as *const u8 as
                                                  *const libc::c_char, &mut s)
                                  == 0 {
                        (*vpninfo).esp_enc =
                            check_enc_algo(vpninfo, s) as libc::c_uchar
                    } else if xmlnode_get_val(member,
                                              b"hmac-algo\x00" as *const u8 as
                                                  *const libc::c_char, &mut s)
                                  == 0 {
                        (*vpninfo).esp_hmac =
                            check_hmac_algo(vpninfo, s) as libc::c_uchar
                    } else if xmlnode_get_val(member,
                                              b"c2s-spi\x00" as *const u8 as
                                                  *const libc::c_char, &mut s)
                                  == 0 {
                        (*eo).spi =
                            if 0 != 0 {
                                ((strtoul(s, 0 as *mut *mut libc::c_char,
                                          16i32) as __uint32_t &
                                      0xff000000u32) >> 24i32 |
                                     (strtoul(s, 0 as *mut *mut libc::c_char,
                                              16i32) as __uint32_t &
                                          0xff0000i32 as libc::c_uint) >> 8i32
                                     |
                                     (strtoul(s, 0 as *mut *mut libc::c_char,
                                              16i32) as __uint32_t &
                                          0xff00i32 as libc::c_uint) << 8i32)
                                    |
                                    (strtoul(s, 0 as *mut *mut libc::c_char,
                                             16i32) as __uint32_t &
                                         0xffi32 as libc::c_uint) << 24i32
                            } else {
                                _OSSwapInt32(strtoul(s,
                                                     0 as
                                                         *mut *mut libc::c_char,
                                                     16i32) as __uint32_t)
                            }
                    } else if xmlnode_get_val(member,
                                              b"s2c-spi\x00" as *const u8 as
                                                  *const libc::c_char, &mut s)
                                  == 0 {
                        (*ei).spi =
                            if 0 != 0 {
                                ((strtoul(s, 0 as *mut *mut libc::c_char,
                                          16i32) as __uint32_t &
                                      0xff000000u32) >> 24i32 |
                                     (strtoul(s, 0 as *mut *mut libc::c_char,
                                              16i32) as __uint32_t &
                                          0xff0000i32 as libc::c_uint) >> 8i32
                                     |
                                     (strtoul(s, 0 as *mut *mut libc::c_char,
                                              16i32) as __uint32_t &
                                          0xff00i32 as libc::c_uint) << 8i32)
                                    |
                                    (strtoul(s, 0 as *mut *mut libc::c_char,
                                             16i32) as __uint32_t &
                                         0xffi32 as libc::c_uint) << 24i32
                            } else {
                                _OSSwapInt32(strtoul(s,
                                                     0 as
                                                         *mut *mut libc::c_char,
                                                     16i32) as __uint32_t)
                            }
                    } else if xmlnode_is_named(member,
                                               b"ekey-c2s\x00" as *const u8 as
                                                   *const libc::c_char) != 0 {
                        (*vpninfo).enc_key_len =
                            xml_to_key(member, (*eo).enc_key.as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_uchar; 64]>()
                                           as libc::c_ulong as libc::c_int)
                    } else if xmlnode_is_named(member,
                                               b"ekey-s2c\x00" as *const u8 as
                                                   *const libc::c_char) != 0 {
                        (*vpninfo).enc_key_len =
                            xml_to_key(member, (*ei).enc_key.as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_uchar; 64]>()
                                           as libc::c_ulong as libc::c_int)
                    } else if xmlnode_is_named(member,
                                               b"akey-c2s\x00" as *const u8 as
                                                   *const libc::c_char) != 0 {
                        (*vpninfo).hmac_key_len =
                            xml_to_key(member, (*eo).hmac_key.as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_uchar; 64]>()
                                           as libc::c_ulong as libc::c_int)
                    } else if xmlnode_is_named(member,
                                               b"akey-s2c\x00" as *const u8 as
                                                   *const libc::c_char) != 0 {
                        (*vpninfo).hmac_key_len =
                            xml_to_key(member, (*ei).hmac_key.as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_uchar; 64]>()
                                           as libc::c_ulong as libc::c_int)
                    } else if xmlnode_get_val(member,
                                              b"ipsec-mode\x00" as *const u8
                                                  as *const libc::c_char,
                                              &mut s) == 0 &&
                                  strcmp(s,
                                         b"esp-tunnel\x00" as *const u8 as
                                             *const libc::c_char) != 0 {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"GlobalProtect config sent ipsec-mode=%s (expected esp-tunnel)\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char),
                                                                                    s);
                        }
                    }
                    member = (*member).next
                }
                if openconnect_setup_esp_keys(vpninfo, 0i32) != 0 {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                b"Failed to setup ESP keys.\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                    }
                } else {
                    /* prevent race condition between esp_mainloop() and gpst_mainloop() timers */
                    (*vpninfo).dtls_times.last_rekey =
                        time(&mut (*vpninfo).new_dtls_started)
                }
            }
        } else if !(xmlnode_is_named(xml_node,
                                     b"need-tunnel\x00" as *const u8 as
                                         *const libc::c_char) != 0 ||
                        xmlnode_is_named(xml_node,
                                         b"bw-c2s\x00" as *const u8 as
                                             *const libc::c_char) != 0 ||
                        xmlnode_is_named(xml_node,
                                         b"bw-s2c\x00" as *const u8 as
                                             *const libc::c_char) != 0 ||
                        xmlnode_is_named(xml_node,
                                         b"default-gateway\x00" as *const u8
                                             as *const libc::c_char) != 0 ||
                        xmlnode_is_named(xml_node,
                                         b"no-direct-access-to-local-network\x00"
                                             as *const u8 as
                                             *const libc::c_char) != 0 ||
                        xmlnode_is_named(xml_node,
                                         b"ip-address-preferred\x00" as
                                             *const u8 as *const libc::c_char)
                            != 0 ||
                        xmlnode_is_named(xml_node,
                                         b"portal\x00" as *const u8 as
                                             *const libc::c_char) != 0 ||
                        xmlnode_is_named(xml_node,
                                         b"user\x00" as *const u8 as
                                             *const libc::c_char) != 0) {
            if (*xml_node).type_0 as libc::c_uint ==
                   XML_ELEMENT_NODE as libc::c_int as libc::c_uint {
                /* XX: Don't know what tags are used for IPv6 addresses and networks, since
			 * we haven't yet seen a real GlobalProtect VPN with IPv6 internal addresses.
			 */
                free(s as *mut libc::c_void);
                s = xmlNodeGetContent(xml_node) as *mut libc::c_char;
                if !strchr((*xml_node).name as *mut libc::c_char,
                           '6' as i32).is_null() {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Potential IPv6-related GlobalProtect config tag <%s>: %s\nThis build does not support GlobalProtect IPv6 due to a lack of\nof information on how it is configured. Please report this\nto <openconnect-devel@lists.infradead.org>.\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*xml_node).name,
                                                                                s);
                    }
                } else if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Unknown GlobalProtect config tag <%s>: %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            (*xml_node).name,
                                                                            s);
                }
            }
        }
        xml_node = (*xml_node).next
    }
    /* Set 10-second DPD/keepalive (same as Windows client) unless
	 * overridden with --force-dpd */
    if (*vpninfo).ssl_times.dpd == 0 { (*vpninfo).ssl_times.dpd = 10i32 }
    (*vpninfo).esp_ssl_fallback = (*vpninfo).ssl_times.dpd as uint32_t;
    (*vpninfo).ssl_times.keepalive =
        (*vpninfo).esp_ssl_fallback as libc::c_int;
    free(s as *mut libc::c_void);
    return 0i32;
}
#[src_loc = "610:1"]
unsafe extern "C" fn gpst_get_config(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut current_block: u64;
    let mut orig_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    let mut request_body: *mut oc_text_buf = buf_alloc();
    let mut old_cstp_opts: *mut oc_vpn_option = (*vpninfo).cstp_options;
    let mut old_addr: *const libc::c_char = (*vpninfo).ip_info.addr;
    let mut old_netmask: *const libc::c_char = (*vpninfo).ip_info.netmask;
    let mut old_addr6: *const libc::c_char = (*vpninfo).ip_info.addr6;
    let mut old_netmask6: *const libc::c_char = (*vpninfo).ip_info.netmask6;
    let mut request_body_type: *const libc::c_char =
        b"application/x-www-form-urlencoded\x00" as *const u8 as
            *const libc::c_char;
    let mut method: *const libc::c_char =
        b"POST\x00" as *const u8 as *const libc::c_char;
    let mut xml_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    /* submit getconfig request */
    buf_append(request_body,
               b"client-type=1&protocol-version=p1&app-version=4.0.5-8\x00" as
                   *const u8 as *const libc::c_char);
    append_opt(request_body,
               b"clientos\x00" as *const u8 as *const libc::c_char,
               gpst_os_name(vpninfo));
    append_opt(request_body,
               b"os-version\x00" as *const u8 as *const libc::c_char,
               (*vpninfo).platname);
    append_opt(request_body,
               b"hmac-algo\x00" as *const u8 as *const libc::c_char,
               b"sha1,md5,sha256\x00" as *const u8 as *const libc::c_char);
    append_opt(request_body,
               b"enc-algo\x00" as *const u8 as *const libc::c_char,
               b"aes-128-cbc,aes-256-cbc\x00" as *const u8 as
                   *const libc::c_char);
    if !old_addr.is_null() || !old_addr6.is_null() {
        append_opt(request_body,
                   b"preferred-ip\x00" as *const u8 as *const libc::c_char,
                   old_addr);
        append_opt(request_body,
                   b"preferred-ipv6\x00" as *const u8 as *const libc::c_char,
                   old_addr6);
        filter_opts(request_body, (*vpninfo).cookie,
                    b"preferred-ip,preferred-ipv6\x00" as *const u8 as
                        *const libc::c_char, 0i32);
    } else {
        buf_append(request_body,
                   b"&%s\x00" as *const u8 as *const libc::c_char,
                   (*vpninfo).cookie);
    }
    result = buf_error(request_body);
    if !(result != 0) {
        orig_path = (*vpninfo).urlpath;
        (*vpninfo).urlpath =
            strdup(b"ssl-vpn/getconfig.esp\x00" as *const u8 as
                       *const libc::c_char);
        result =
            do_https_request(vpninfo, method, request_body_type, request_body,
                             &mut xml_buf, 0i32);
        free((*vpninfo).urlpath as *mut libc::c_void);
        (*vpninfo).urlpath = orig_path;
        /* parse getconfig result */
        if result >= 0i32 {
            result =
                gpst_xml_or_error(vpninfo, xml_buf,
                                  Some(gpst_parse_config_xml as
                                           unsafe extern "C" fn(_:
                                                                    *mut openconnect_info,
                                                                _:
                                                                    *mut xmlNode,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> libc::c_int), None,
                                  0 as *mut libc::c_void)
        }
        if !(result != 0) {
            if (*vpninfo).ip_info.mtu == 0 {
                /* FIXME: GP gateway config always seems to be <mtu>0</mtu> */
                let mut no_esp_reason: *mut libc::c_char =
                    0 as *mut libc::c_char; /* NOT zero-terminated */
                if (*vpninfo).dtls_state == 2i32 {
                    no_esp_reason =
                        libintl_dgettext(b"openconnect\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"ESP disabled\x00" as *const u8 as
                                             *const libc::c_char)
                } else if (*vpninfo).dtls_state == 0i32 {
                    no_esp_reason =
                        libintl_dgettext(b"openconnect\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"No ESP keys received\x00" as
                                             *const u8 as *const libc::c_char)
                }
                (*vpninfo).ip_info.mtu =
                    calculate_mtu(vpninfo,
                                  no_esp_reason.is_null() as libc::c_int);
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"No MTU received. Calculated %d for %s%s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            (*vpninfo).ip_info.mtu,
                                                                            if !no_esp_reason.is_null()
                                                                               {
                                                                                b"SSL tunnel. \x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char
                                                                            } else {
                                                                                b"ESP tunnel\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char
                                                                            },
                                                                            if !no_esp_reason.is_null()
                                                                               {
                                                                                no_esp_reason
                                                                            } else {
                                                                                b"\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char
                                                                            });
                }
            }
            if (*vpninfo).ip_info.addr.is_null() &&
                   (*vpninfo).ip_info.addr6.is_null() &&
                   (*vpninfo).ip_info.netmask6.is_null() {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"No IP address received. Aborting\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                result = -22i32
            } else {
                if !old_addr.is_null() {
                    if strcmp(old_addr, (*vpninfo).ip_info.addr) != 0 {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Reconnect gave different Legacy IP address (%s != %s)\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char),
                                                                                    (*vpninfo).ip_info.addr,
                                                                                    old_addr);
                        }
                        result = -22i32;
                        current_block = 11053012439411406010;
                    } else { current_block = 2989495919056355252; }
                } else { current_block = 2989495919056355252; }
                match current_block {
                    11053012439411406010 => { }
                    _ => {
                        if !old_netmask.is_null() {
                            if strcmp(old_netmask, (*vpninfo).ip_info.netmask)
                                   != 0 {
                                if (*vpninfo).verbose >= 0i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            0i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"Reconnect gave different Legacy IP netmask (%s != %s)\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char),
                                                                                            (*vpninfo).ip_info.netmask,
                                                                                            old_netmask);
                                }
                                result = -22i32;
                                current_block = 11053012439411406010;
                            } else { current_block = 8545136480011357681; }
                        } else { current_block = 8545136480011357681; }
                        match current_block {
                            11053012439411406010 => { }
                            _ => {
                                if !old_addr6.is_null() {
                                    if strcmp(old_addr6,
                                              (*vpninfo).ip_info.addr6) != 0 {
                                        if (*vpninfo).verbose >= 0i32 {
                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                    0i32,
                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char,
                                                                                                                     b"Reconnect gave different IPv6 address (%s != %s)\n\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char),
                                                                                                    (*vpninfo).ip_info.addr6,
                                                                                                    old_addr6);
                                        }
                                        return -22i32
                                    }
                                }
                                if !old_netmask6.is_null() {
                                    if strcmp(old_netmask6,
                                              (*vpninfo).ip_info.netmask6) !=
                                           0 {
                                        if (*vpninfo).verbose >= 0i32 {
                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                    0i32,
                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char,
                                                                                                                     b"Reconnect gave different IPv6 netmask (%s != %s)\n\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char),
                                                                                                    (*vpninfo).ip_info.netmask6,
                                                                                                    old_netmask6);
                                        }
                                        return -22i32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free_optlist(old_cstp_opts);
    buf_free(request_body);
    free(xml_buf as *mut libc::c_void);
    return result;
}
#[src_loc = "716:1"]
unsafe extern "C" fn gpst_connect(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut reqbuf: *mut oc_text_buf = 0 as *mut oc_text_buf;
    let start_tunnel: [libc::c_char; 12] =
        *::std::mem::transmute::<&[u8; 12],
                                 &[libc::c_char; 12]>(b"START_TUNNEL");
    let mut buf: [libc::c_char; 256] = [0; 256];
    /* Connect to SSL VPN tunnel */
    if (*vpninfo).verbose >= 2i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                2i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Connecting to HTTPS tunnel endpoint ...\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
    }
    ret = openconnect_open_https(vpninfo);
    if ret != 0 { return ret }
    reqbuf = buf_alloc();
    buf_append(reqbuf, b"GET %s?\x00" as *const u8 as *const libc::c_char,
               (*vpninfo).urlpath);
    filter_opts(reqbuf, (*vpninfo).cookie,
                b"user,authcookie\x00" as *const u8 as *const libc::c_char,
                1i32);
    buf_append(reqbuf,
               b" HTTP/1.1\r\n\r\n\x00" as *const u8 as *const libc::c_char);
    ret = buf_error(reqbuf);
    if !(ret != 0) {
        if (*vpninfo).dump_http_traffic != 0 {
            dump_buf(vpninfo, '>' as i32 as libc::c_char, (*reqbuf).data);
        }
        (*vpninfo).ssl_write.expect("non-null function pointer")(vpninfo,
                                                                 (*reqbuf).data,
                                                                 (*reqbuf).pos
                                                                     as
                                                                     size_t);
        ret =
            (*vpninfo).ssl_read.expect("non-null function pointer")(vpninfo,
                                                                    buf.as_mut_ptr(),
                                                                    12i32 as
                                                                        size_t);
        if ret < 0i32 {
            if !(ret == -4i32) {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Error fetching GET-tunnel HTTPS response.\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ret = -22i32
            }
        } else {
            if strncmp(buf.as_mut_ptr(), start_tunnel.as_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 12]>() as
                           libc::c_ulong) == 0 {
                ret = 0i32
            } else if ret == 0i32 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Gateway disconnected immediately after GET-tunnel request.\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ret = -32i32
            } else {
                if ret as libc::c_ulong ==
                       ::std::mem::size_of::<[libc::c_char; 12]>() as
                           libc::c_ulong {
                    ret =
                        (*vpninfo).ssl_gets.expect("non-null function pointer")(vpninfo,
                                                                                buf.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 12]>()
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            isize),
                                                                                (::std::mem::size_of::<[libc::c_char; 256]>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 12]>()
                                                                                                                     as
                                                                                                                     libc::c_ulong));
                    ret =
                        ((if ret > 0i32 { ret } else { 0i32 }) as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 12]>()
                                                             as libc::c_ulong)
                            as libc::c_int
                }
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Got inappropriate HTTP GET-tunnel response: %.*s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            ret,
                                                                            buf.as_mut_ptr());
                }
                ret = -22i32
            }
            if ret < 0i32 {
                openconnect_close_https(vpninfo, 0i32);
            } else {
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
                (*vpninfo).ssl_times.last_tx = time(0 as *mut time_t);
                (*vpninfo).ssl_times.last_rx = (*vpninfo).ssl_times.last_tx;
                /* connecting the HTTPS tunnel totally invalidates the ESP keys,
		   hence shutdown */
                if (*(*vpninfo).proto).udp_shutdown.is_some() {
                    (*(*vpninfo).proto).udp_shutdown.expect("non-null function pointer")(vpninfo);
                }
            }
        }
    }
    buf_free(reqbuf);
    return ret;
}
#[src_loc = "786:1"]
unsafe extern "C" fn parse_hip_report_check(mut vpninfo:
                                                *mut openconnect_info,
                                            mut xml_node: *mut xmlNode,
                                            mut cb_data: *mut libc::c_void)
 -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = -22i32;
    if !(xml_node.is_null() ||
             xmlnode_is_named(xml_node,
                              b"response\x00" as *const u8 as
                                  *const libc::c_char) == 0) {
        xml_node = (*xml_node).children;
        while !xml_node.is_null() {
            if xmlnode_get_val(xml_node,
                               b"hip-report-needed\x00" as *const u8 as
                                   *const libc::c_char, &mut s) == 0 {
                if strcmp(s, b"no\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                    result = 0i32
                } else if strcmp(s,
                                 b"yes\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                    result = -35i32
                } else { result = -22i32 }
                break ;
            } else { xml_node = (*xml_node).next }
        }
    }
    free(s as *mut libc::c_void);
    return result;
}
/* Unlike CSD, the HIP security checker runs during the connection
 * phase, not during the authentication phase.
 *
 * The HIP security checker will (probably) ask us to resubmit the
 * HIP report if either of the following changes:
 *   - Client IP address
 *   - Client HIP report md5sum
 *
 * I'm not sure what the md5sum is computed over in the official
 * client, but it doesn't really matter.
 *
 * We just need an identifier for the combination of the local host
 * and the VPN gateway which won't change when our IP address
 * or authcookie are changed.
 */
#[src_loc = "826:1"]
unsafe extern "C" fn build_csd_token(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut buf: *mut oc_text_buf = 0 as *mut oc_text_buf;
    let mut md5: [libc::c_uchar; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    if !(*vpninfo).csd_token.is_null() { return 0i32 }
    (*vpninfo).csd_token =
        malloc((16i32 * 2i32 + 1i32) as libc::c_ulong) as *mut libc::c_char;
    if (*vpninfo).csd_token.is_null() { return -12i32 }
    /* use cookie (excluding volatile authcookie and preferred-ip) to build md5sum */
    buf = buf_alloc();
    filter_opts(buf, (*vpninfo).cookie,
                b"authcookie,preferred-ip\x00" as *const u8 as
                    *const libc::c_char, 0i32);
    if !(buf_error(buf) != 0) {
        /* save as csd_token */
        openconnect_md5(md5.as_mut_ptr(), (*buf).data as *mut libc::c_void,
                        (*buf).pos);
        i = 0i32;
        while i < 16i32 {
            sprintf(&mut *(*vpninfo).csd_token.offset((i * 2i32) as isize) as
                        *mut libc::c_char,
                    b"%02x\x00" as *const u8 as *const libc::c_char,
                    md5[i as usize] as libc::c_int);
            i += 1
        }
    }
    return buf_free(buf);
}
/* check if HIP report is needed (to ssl-vpn/hipreportcheck.esp) or submit HIP report contents (to ssl-vpn/hipreport.esp) */
#[src_loc = "855:1"]
unsafe extern "C" fn check_or_submit_hip_report(mut vpninfo:
                                                    *mut openconnect_info,
                                                mut report:
                                                    *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut result: libc::c_int = 0;
    let mut request_body: *mut oc_text_buf = buf_alloc();
    let mut request_body_type: *const libc::c_char =
        b"application/x-www-form-urlencoded\x00" as *const u8 as
            *const libc::c_char;
    let mut method: *const libc::c_char =
        b"POST\x00" as *const u8 as *const libc::c_char;
    let mut xml_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut orig_path: *mut libc::c_char = 0 as *mut libc::c_char;
    /* cookie gives us these fields: authcookie, portal, user, domain, computer, and (maybe the unnecessary) preferred-ip */
    buf_append(request_body,
               b"client-role=global-protect-full&%s\x00" as *const u8 as
                   *const libc::c_char, (*vpninfo).cookie);
    if !(*vpninfo).ip_info.addr.is_null() {
        append_opt(request_body,
                   b"client-ip\x00" as *const u8 as *const libc::c_char,
                   (*vpninfo).ip_info.addr);
    }
    if !(*vpninfo).ip_info.addr6.is_null() {
        append_opt(request_body,
                   b"client-ipv6\x00" as *const u8 as *const libc::c_char,
                   (*vpninfo).ip_info.addr6);
    }
    if !report.is_null() {
        /* XML report contains many characters requiring URL-encoding (%xx) */
        buf_ensure_space(request_body,
                         strlen(report).wrapping_mul(3i32 as libc::c_ulong) as
                             libc::c_int);
        append_opt(request_body,
                   b"report\x00" as *const u8 as *const libc::c_char, report);
        current_block = 12599329904712511516;
    } else {
        result = build_csd_token(vpninfo);
        if result != 0 {
            current_block = 18415497256852884872;
        } else {
            append_opt(request_body,
                       b"md5\x00" as *const u8 as *const libc::c_char,
                       (*vpninfo).csd_token);
            current_block = 12599329904712511516;
        }
    }
    match current_block {
        12599329904712511516 => {
            result = buf_error(request_body);
            if !(result != 0) {
                orig_path = (*vpninfo).urlpath;
                (*vpninfo).urlpath =
                    strdup(if !report.is_null() {
                               b"ssl-vpn/hipreport.esp\x00" as *const u8 as
                                   *const libc::c_char
                           } else {
                               b"ssl-vpn/hipreportcheck.esp\x00" as *const u8
                                   as *const libc::c_char
                           });
                result =
                    do_https_request(vpninfo, method, request_body_type,
                                     request_body, &mut xml_buf, 0i32);
                free((*vpninfo).urlpath as *mut libc::c_void);
                (*vpninfo).urlpath = orig_path;
                if result >= 0i32 {
                    result =
                        gpst_xml_or_error(vpninfo, xml_buf,
                                          if !report.is_null() {
                                              None
                                          } else {
                                              Some(parse_hip_report_check as
                                                       unsafe extern "C" fn(_:
                                                                                *mut openconnect_info,
                                                                            _:
                                                                                *mut xmlNode,
                                                                            _:
                                                                                *mut libc::c_void)
                                                           -> libc::c_int)
                                          }, None, 0 as *mut libc::c_void)
                }
            }
        }
        _ => { }
    }
    buf_free(request_body);
    free(xml_buf as *mut libc::c_void);
    return result;
}
#[src_loc = "899:1"]
unsafe extern "C" fn run_hip_script(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut hip_argv: [*mut libc::c_char; 32] = [0 as *mut libc::c_char; 32];
    let mut i_0: libc::c_int = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut ret: libc::c_int = 0;
    let mut child: pid_t = 0;
    if (*vpninfo).csd_wrapper.is_null() {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"WARNING: Server asked us to submit HIP report with md5sum %s.\nVPN connectivity may be disabled or limited without HIP report submission.\nYou need to provide a --csd-wrapper argument with the HIP report submission script.\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*vpninfo).csd_token);
        }
        /* XXX: Many GlobalProtect VPNs work fine despite allegedly requiring HIP report submission */
        return 0i32
    }
    if !(pipe(pipefd.as_mut_ptr()) != 0) {
        set_fd_cloexec(pipefd[0]);
        set_fd_cloexec(pipefd[1]);
        child = fork();
        if !(child == -1i32) {
            if child > 0i32 {
                /* in parent: read report from child */
                let mut report_buf: *mut oc_text_buf = buf_alloc();
                let mut b: [libc::c_char; 256] = [0; 256];
                let mut i: libc::c_int = 0;
                let mut status: libc::c_int = 0;
                close(pipefd[1]);
                buf_truncate(report_buf);
                loop  {
                    i =
                        read(pipefd[0], b.as_mut_ptr() as *mut libc::c_void,
                             ::std::mem::size_of::<[libc::c_char; 256]>() as
                                 libc::c_ulong) as libc::c_int;
                    if !(i > 0i32) { break ; }
                    buf_append_bytes(report_buf,
                                     b.as_mut_ptr() as *const libc::c_void,
                                     i);
                }
                waitpid(child, &mut status, 0i32);
                if !(*(&mut status as *mut libc::c_int) & 0o177i32 == 0i32) {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"HIP script \'%s\' exited abnormally\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).csd_wrapper);
                    }
                    ret = -22i32
                } else if *(&mut status as *mut libc::c_int) >> 8i32 & 0xffi32
                              != 0i32 {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"HIP script \'%s\' returned non-zero status: %d\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).csd_wrapper,
                                                                                *(&mut status
                                                                                      as
                                                                                      *mut libc::c_int)
                                                                                    >>
                                                                                    8i32
                                                                                    &
                                                                                    0xffi32);
                    }
                    ret = -22i32
                } else {
                    ret =
                        check_or_submit_hip_report(vpninfo,
                                                   (*report_buf).data);
                    if ret < 0i32 {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"HIP report submission failed.\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                    } else {
                        if (*vpninfo).verbose >= 1i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    1i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"HIP report submitted successfully.\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                        ret = 0i32
                    }
                }
                buf_free(report_buf);
                return ret
            } else {
                /* in child: run HIP script */
                hip_argv = [0 as *mut libc::c_char; 32];
                i_0 = 0i32;
                close(pipefd[0]);
                /* The duplicated fd does not have O_CLOEXEC */
                dup2(pipefd[1], 1i32);
                if set_csd_user(vpninfo) < 0i32 { exit(1i32); }
                let fresh6 = i_0;
                i_0 = i_0 + 1;
                hip_argv[fresh6 as usize] =
                    openconnect_utf8_to_legacy(vpninfo,
                                               (*vpninfo).csd_wrapper);
                let fresh7 = i_0;
                i_0 = i_0 + 1;
                hip_argv[fresh7 as usize] =
                    b"--cookie\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char;
                let fresh8 = i_0;
                i_0 = i_0 + 1;
                hip_argv[fresh8 as usize] = (*vpninfo).cookie;
                if !(*vpninfo).ip_info.addr.is_null() {
                    let fresh9 = i_0;
                    i_0 = i_0 + 1;
                    hip_argv[fresh9 as usize] =
                        b"--client-ip\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    let fresh10 = i_0;
                    i_0 = i_0 + 1;
                    hip_argv[fresh10 as usize] =
                        (*vpninfo).ip_info.addr as *mut libc::c_char
                }
                if !(*vpninfo).ip_info.addr6.is_null() {
                    let fresh11 = i_0;
                    i_0 = i_0 + 1;
                    hip_argv[fresh11 as usize] =
                        b"--client-ipv6\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char;
                    let fresh12 = i_0;
                    i_0 = i_0 + 1;
                    hip_argv[fresh12 as usize] =
                        (*vpninfo).ip_info.addr6 as *mut libc::c_char
                }
                let fresh13 = i_0;
                i_0 = i_0 + 1;
                hip_argv[fresh13 as usize] =
                    b"--md5\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char;
                let fresh14 = i_0;
                i_0 = i_0 + 1;
                hip_argv[fresh14 as usize] = (*vpninfo).csd_token;
                let fresh15 = i_0;
                i_0 = i_0 + 1;
                hip_argv[fresh15 as usize] = 0 as *mut libc::c_char;
                execv(hip_argv[0], hip_argv.as_mut_ptr());
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
                                                                                 b"Failed to exec HIP script %s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                hip_argv[0]);
    }
    exit(1i32);
    /* !_WIN32 && !__native_client__ */
}
#[no_mangle]
#[src_loc = "1003:1"]
pub unsafe extern "C" fn gpst_setup(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    /* ESP keys are invalid as soon as we (re-)fetch the configuration, hence shutdown */
    if (*(*vpninfo).proto).udp_shutdown.is_some() {
        (*(*vpninfo).proto).udp_shutdown.expect("non-null function pointer")(vpninfo);
    }
    /* Get configuration */
    ret = gpst_get_config(vpninfo);
    if !(ret != 0) {
        /* Check HIP */
        ret = check_or_submit_hip_report(vpninfo, 0 as *const libc::c_char);
        if ret == -35i32 {
            if (*vpninfo).verbose >= 2i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        2i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Gateway says HIP report submission is needed.\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            ret = run_hip_script(vpninfo);
            if ret != 0i32 {
                current_block = 6665063336625948996;
            } else { current_block = 17833034027772472439; }
        } else {
            if ret == 0i32 {
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Gateway says no HIP report submission is needed.\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
            }
            current_block = 17833034027772472439;
        }
        match current_block {
            6665063336625948996 => { }
            _ => {
                /* We do NOT actually start the HTTPS tunnel yet if we want to
	 * use ESP, because the ESP tunnel won't work if the HTTPS tunnel
	 * is connected! >:-(
	 */
                if (*vpninfo).dtls_state == 2i32 ||
                       (*vpninfo).dtls_state == 0i32 {
                    ret = gpst_connect(vpninfo)
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
#[src_loc = "1039:1"]
pub unsafe extern "C" fn gpst_mainloop(mut vpninfo: *mut openconnect_info,
                                       mut timeout: *mut libc::c_int,
                                       mut readable: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut work_done: libc::c_int = 0i32;
    let mut ethertype: uint16_t = 0;
    let mut one: uint32_t = 0;
    let mut zero: uint32_t = 0;
    let mut magic: uint32_t = 0;
    /* Starting the HTTPS tunnel kills ESP, so we avoid starting
	 * it if the ESP tunnel is connected or connecting.
	 */
    match (*vpninfo).dtls_state {
        4 => {
            openconnect_close_https(vpninfo,
                                    0i32); /* don't keep stale HTTPS socket */
            if (*vpninfo).verbose >= 1i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        1i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"ESP tunnel connected; exiting HTTPS mainloop.\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            (*vpninfo).dtls_state = 5i32;
            /* fall through */
            current_block = 15152944789225942827;
        }
        5 => { current_block = 15152944789225942827; }
        1 | 3 => {
            if ka_check_deadline(timeout, time(0 as *mut time_t),
                                 (*vpninfo).new_dtls_started +
                                     5i32 as libc::c_long) == 0 {
                /* Allow 5 seconds after configuration for ESP to start */
                return 0i32
            } else {
                /* ... before we switch to HTTPS instead */
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to connect ESP tunnel; using HTTPS instead.\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                if gpst_connect(vpninfo) != 0 {
                    (*vpninfo).quit_reason =
                        b"GPST connect failed\x00" as *const u8 as
                            *const libc::c_char;
                    return 1i32
                }
            }
            current_block = 16203760046146113240;
        }
        0 => {
            /* HTTPS tunnel already started, or getconfig.esp did not provide any ESP keys */
            current_block = 16203760046146113240;
        }
        2 | _ => { current_block = 16203760046146113240; }
    }
    match current_block {
        15152944789225942827 =>
        /* Rekey if needed */
        {
            if keepalive_action(&mut (*vpninfo).ssl_times, timeout) == 4i32 {
                current_block = 3922202458087797500;
            } else { return 0i32 }
        }
        _ =>
        /* ESP is disabled */
        {
            if (*vpninfo).ssl_fd == -1i32 {
                current_block = 7933628344389254903;
            } else {
                loop  {
                    if !(readable != 0) {
                        current_block = 1658462350791934405;
                        break ;
                    }
                    /* Some servers send us packets that are larger than
		   negotiated MTU. We reserve some extra space to
		   handle that */
                    let mut receive_mtu: libc::c_int =
                        if 16384i32 > (*vpninfo).ip_info.mtu {
                            16384i32
                        } else { (*vpninfo).ip_info.mtu };
                    let mut len: libc::c_int = 0;
                    let mut payload_len: libc::c_int = 0;
                    if (*vpninfo).cstp_pkt.is_null() {
                        (*vpninfo).cstp_pkt =
                            malloc((::std::mem::size_of::<pkt>() as
                                        libc::c_ulong).wrapping_add(receive_mtu
                                                                        as
                                                                        libc::c_ulong))
                                as *mut pkt;
                        if (*vpninfo).cstp_pkt.is_null() {
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
                            current_block = 1658462350791934405;
                            break ;
                        }
                    }
                    len =
                        ssl_nonblock_read(vpninfo,
                                          (*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr()
                                              as *mut libc::c_void,
                                          receive_mtu + 16i32);
                    if len == 0 {
                        current_block = 1658462350791934405;
                        break ;
                    }
                    if len < 0i32 {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Packet receive error: %s\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char),
                                                                                    strerror(-len));
                        }
                        current_block = 7933628344389254903;
                        break ;
                    } else {
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
                                b"Short packet received\x00" as *const u8 as
                                    *const libc::c_char;
                            return 1i32
                        }
                        /* check packet header */
                        magic =
                            load_be32((*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr()
                                          as *const libc::c_void);
                        ethertype =
                            load_be16((*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(4)
                                          as *const libc::c_void);
                        payload_len =
                            load_be16((*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(6)
                                          as *const libc::c_void) as
                                libc::c_int;
                        one =
                            load_le32((*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(8)
                                          as *const libc::c_void);
                        zero =
                            load_le32((*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(12)
                                          as *const libc::c_void);
                        if !(magic != 0x1a2b3c4di32 as libc::c_uint) {
                            if len != 16i32 + payload_len {
                                if (*vpninfo).verbose >= 0i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            0i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"Unexpected packet length. SSL_read returned %d (includes 16 header bytes) but header payload_len is %d\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char),
                                                                                            len,
                                                                                            payload_len);
                                }
                                dump_buf_hex(vpninfo, 0i32,
                                             '<' as i32 as libc::c_char,
                                             (*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr(),
                                             16i32);
                                continue ;
                            } else {
                                (*vpninfo).ssl_times.last_rx =
                                    time(0 as *mut time_t);
                                match ethertype as libc::c_int {
                                    0 => {
                                        current_block = 11647255923688962889;
                                        match current_block {
                                            14254964580781485841 => {
                                                if (*vpninfo).verbose >= 3i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            3i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Received IPv%d data packet of %d bytes\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char),
                                                                                                            if ethertype
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   ==
                                                                                                                   0x86ddi32
                                                                                                               {
                                                                                                                6i32
                                                                                                            } else {
                                                                                                                4i32
                                                                                                            },
                                                                                                            payload_len);
                                                }
                                                (*(*vpninfo).cstp_pkt).len =
                                                    payload_len;
                                                queue_packet(&mut (*vpninfo).incoming_queue,
                                                             (*vpninfo).cstp_pkt);
                                                (*vpninfo).cstp_pkt =
                                                    0 as *mut pkt;
                                                work_done = 1i32;
                                                if one != 1i32 as libc::c_uint
                                                       ||
                                                       zero !=
                                                           0i32 as
                                                               libc::c_uint {
                                                    if (*vpninfo).verbose >=
                                                           2i32 {
                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                2i32,
                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char,
                                                                                                                                 b"Expected 0100000000000000 as last 8 bytes of data packet header, but got:\n\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char));
                                                    }
                                                    dump_buf_hex(vpninfo,
                                                                 2i32,
                                                                 '<' as i32 as
                                                                     libc::c_char,
                                                                 (*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(8),
                                                                 8i32);
                                                }
                                                continue ;
                                            }
                                            _ => {
                                                if (*vpninfo).verbose >= 2i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            2i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Got GPST DPD/keepalive response\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char));
                                                }
                                                if one != 0i32 as libc::c_uint
                                                       ||
                                                       zero !=
                                                           0i32 as
                                                               libc::c_uint {
                                                    if (*vpninfo).verbose >=
                                                           2i32 {
                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                2i32,
                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char,
                                                                                                                                 b"Expected 0000000000000000 as last 8 bytes of DPD/keepalive packet header, but got:\n\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char));
                                                    }
                                                    dump_buf_hex(vpninfo,
                                                                 2i32,
                                                                 '<' as i32 as
                                                                     libc::c_char,
                                                                 (*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(8),
                                                                 8i32);
                                                }
                                                continue ;
                                            }
                                        }
                                    }
                                    2048 | 34525 => {
                                        current_block = 14254964580781485841;
                                        match current_block {
                                            14254964580781485841 => {
                                                if (*vpninfo).verbose >= 3i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            3i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Received IPv%d data packet of %d bytes\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char),
                                                                                                            if ethertype
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   ==
                                                                                                                   0x86ddi32
                                                                                                               {
                                                                                                                6i32
                                                                                                            } else {
                                                                                                                4i32
                                                                                                            },
                                                                                                            payload_len);
                                                }
                                                (*(*vpninfo).cstp_pkt).len =
                                                    payload_len;
                                                queue_packet(&mut (*vpninfo).incoming_queue,
                                                             (*vpninfo).cstp_pkt);
                                                (*vpninfo).cstp_pkt =
                                                    0 as *mut pkt;
                                                work_done = 1i32;
                                                if one != 1i32 as libc::c_uint
                                                       ||
                                                       zero !=
                                                           0i32 as
                                                               libc::c_uint {
                                                    if (*vpninfo).verbose >=
                                                           2i32 {
                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                2i32,
                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char,
                                                                                                                                 b"Expected 0100000000000000 as last 8 bytes of data packet header, but got:\n\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char));
                                                    }
                                                    dump_buf_hex(vpninfo,
                                                                 2i32,
                                                                 '<' as i32 as
                                                                     libc::c_char,
                                                                 (*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(8),
                                                                 8i32);
                                                }
                                                continue ;
                                            }
                                            _ => {
                                                if (*vpninfo).verbose >= 2i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            2i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Got GPST DPD/keepalive response\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char));
                                                }
                                                if one != 0i32 as libc::c_uint
                                                       ||
                                                       zero !=
                                                           0i32 as
                                                               libc::c_uint {
                                                    if (*vpninfo).verbose >=
                                                           2i32 {
                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                2i32,
                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char,
                                                                                                                                 b"Expected 0000000000000000 as last 8 bytes of DPD/keepalive packet header, but got:\n\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char));
                                                    }
                                                    dump_buf_hex(vpninfo,
                                                                 2i32,
                                                                 '<' as i32 as
                                                                     libc::c_char,
                                                                 (*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(8),
                                                                 8i32);
                                                }
                                                continue ;
                                            }
                                        }
                                    }
                                    _ => { }
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
                                                                                                     b"Unknown packet. Header dump follows:\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                        dump_buf_hex(vpninfo, 0i32,
                                     '<' as i32 as libc::c_char,
                                     (*(*vpninfo).cstp_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr(),
                                     16i32);
                        (*vpninfo).quit_reason =
                            b"Unknown packet received\x00" as *const u8 as
                                *const libc::c_char;
                        return 1i32
                    }
                }
                match current_block {
                    7933628344389254903 => { }
                    _ =>
                    /* If SSL_write() fails we are expected to try again. With exactly
	   the same data, at exactly the same location. So we keep the
	   packet we had before.... */
                    {
                        if !(*vpninfo).current_ssl_pkt.is_null() {
                            current_block = 9661857191748018424;
                        } else { current_block = 15456862084301247793; }
                        loop  {
                            match current_block {
                                9661857191748018424 => {
                                    (*vpninfo).ssl_times.last_tx =
                                        time(0 as *mut time_t);
                                    let mut __fd: libc::c_int =
                                        (*vpninfo).ssl_fd;
                                    (*vpninfo)._select_wfds.fds_bits[(__fd as
                                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                                                           as
                                                                                                           libc::c_ulong).wrapping_mul(8i32
                                                                                                                                           as
                                                                                                                                           libc::c_ulong))
                                                                         as
                                                                         usize]
                                        &=
                                        !(((1i32 as libc::c_ulong) <<
                                               (__fd as
                                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(8i32
                                                                                                                     as
                                                                                                                     libc::c_ulong)))
                                              as __int32_t);
                                    ret =
                                        ssl_nonblock_write(vpninfo,
                                                           (*(*vpninfo).current_ssl_pkt).c2rust_unnamed.gpst.hdr.as_mut_ptr()
                                                               as
                                                               *mut libc::c_void,
                                                           (*(*vpninfo).current_ssl_pkt).len
                                                               + 16i32);
                                    if ret < 0i32 {
                                        current_block = 7933628344389254903;
                                        break ;
                                    }
                                    if ret == 0 {
                                        match ka_stalled_action(&mut (*vpninfo).ssl_times,
                                                                timeout) {
                                            4 => {
                                                current_block =
                                                    3922202458087797500;
                                                break ;
                                            }
                                            2 => {
                                                current_block =
                                                    7877770734039406270;
                                                break ;
                                            }
                                            0 => { return work_done }
                                            _ => { }
                                        }
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
                                                                                                        16i32,
                                                                                                    ret);
                                        }
                                        (*vpninfo).quit_reason =
                                            b"Internal error\x00" as *const u8
                                                as *const libc::c_char;
                                        return 1i32
                                    }
                                    /* Don't free the 'special' packets */
                                    if (*vpninfo).current_ssl_pkt !=
                                           &dpd_pkt as *const pkt as *mut pkt
                                       {
                                        free((*vpninfo).current_ssl_pkt as
                                                 *mut libc::c_void);
                                    }
                                    (*vpninfo).current_ssl_pkt =
                                        0 as *mut pkt;
                                    current_block = 15456862084301247793;
                                }
                                _ => {
                                    match keepalive_action(&mut (*vpninfo).ssl_times,
                                                           timeout) {
                                        4 => {
                                            current_block =
                                                3922202458087797500;
                                            break ;
                                        }
                                        2 => {
                                            current_block =
                                                7877770734039406270;
                                            break ;
                                        }
                                        3 => {
                                            /* No need to send an explicit keepalive
		   if we have real data to send */
                                            if (*vpninfo).dtls_state != 5i32
                                                   &&
                                                   !(*vpninfo).outgoing_queue.head.is_null()
                                               {
                                                current_block =
                                                    10265667325682070567;
                                            } else {
                                                current_block =
                                                    8999518187244832834;
                                            }
                                        }
                                        1 => {
                                            current_block =
                                                8999518187244832834;
                                        }
                                        _ => {
                                            current_block =
                                                10265667325682070567;
                                        }
                                    }
                                    match current_block {
                                        10265667325682070567 =>
                                        /* Service outgoing packet queue */
                                        {
                                            if (*vpninfo).dtls_state != 5i32
                                                   &&
                                                   {
                                                       (*vpninfo).current_ssl_pkt
                                                           =
                                                           dequeue_packet(&mut (*vpninfo).outgoing_queue);
                                                       !(*vpninfo).current_ssl_pkt.is_null()
                                                   } {
                                                let mut this: *mut pkt =
                                                    (*vpninfo).current_ssl_pkt;
                                                /* IPv4 or IPv6 EtherType */
                                                let mut ethertype_0:
                                                        libc::c_int =
                                                    if (*this).len != 0 &&
                                                           *(*this).data.as_mut_ptr().offset(0)
                                                               as libc::c_int
                                                               & 0xf0i32 ==
                                                               0x60i32 {
                                                        0x86ddi32
                                                    } else { 0x800i32 };
                                                /* store header */
                                                store_be32((*this).c2rust_unnamed.gpst.hdr.as_mut_ptr()
                                                               as
                                                               *mut libc::c_void,
                                                           0x1a2b3c4di32 as
                                                               uint32_t);
                                                store_be16((*this).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(4)
                                                               as
                                                               *mut libc::c_void,
                                                           ethertype_0 as
                                                               uint16_t);
                                                store_be16((*this).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(6)
                                                               as
                                                               *mut libc::c_void,
                                                           (*this).len as
                                                               uint16_t);
                                                store_le32((*this).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(8)
                                                               as
                                                               *mut libc::c_void,
                                                           1i32 as uint32_t);
                                                store_le32((*this).c2rust_unnamed.gpst.hdr.as_mut_ptr().offset(12)
                                                               as
                                                               *mut libc::c_void,
                                                           0i32 as uint32_t);
                                                if (*vpninfo).verbose >= 3i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            3i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Sending IPv%d data packet of %d bytes\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char),
                                                                                                            if ethertype_0
                                                                                                                   ==
                                                                                                                   0x86ddi32
                                                                                                               {
                                                                                                                6i32
                                                                                                            } else {
                                                                                                                4i32
                                                                                                            },
                                                                                                            (*this).len);
                                                }
                                                current_block =
                                                    9661857191748018424;
                                            } else {
                                                /* Work is not done if we just got rid of packets off the queue */
                                                return work_done
                                            }
                                        }
                                        _ =>
                                        /* fall through */
                                        {
                                            if (*vpninfo).verbose >= 2i32 {
                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                        2i32,
                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char,
                                                                                                                         b"Send GPST DPD/keepalive request\n\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char));
                                            }
                                            (*vpninfo).current_ssl_pkt =
                                                &dpd_pkt as *const pkt as
                                                    *mut pkt;
                                            current_block =
                                                9661857191748018424;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            7933628344389254903 => { }
                            3922202458087797500 => { }
                            _ => {
                                if (*vpninfo).verbose >= 0i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            0i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"GPST Dead Peer Detection detected dead peer!\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char));
                                }
                                current_block = 7933628344389254903;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        3922202458087797500 => {
            if (*vpninfo).verbose >= 1i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        1i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"GlobalProtect rekey due\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
        }
        _ => { }
    }
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
            b"GPST reconnect failed\x00" as *const u8 as *const libc::c_char;
        return ret
    }
    if (*(*vpninfo).proto).udp_setup.is_some() {
        (*(*vpninfo).proto).udp_setup.expect("non-null function pointer")(vpninfo,
                                                                          (*vpninfo).dtls_attempt_period);
    }
    return 1i32;
}
#[src_loc = "1271:1"]
unsafe extern "C" fn csum(mut buf: *mut uint16_t, mut nwords: libc::c_int)
 -> uint16_t {
    let mut sum: uint32_t = 0i32 as uint32_t;
    sum = 0i32 as uint32_t;
    while nwords > 0i32 {
        sum =
            (sum as
                 libc::c_uint).wrapping_add(if 0 != 0 {
                                                let fresh16 = buf;
                                                buf = buf.offset(1);
                                                let fresh17 = buf;
                                                buf = buf.offset(1);
                                                ((*fresh16 as libc::c_int &
                                                      0xff00i32) >> 8i32 |
                                                     (*fresh17 as libc::c_int
                                                          & 0xffi32) << 8i32)
                                                    as __uint16_t as
                                                    libc::c_int
                                            } else {
                                                let fresh18 = buf;
                                                buf = buf.offset(1);
                                                _OSSwapInt16(*fresh18) as
                                                    libc::c_int
                                            } as __uint16_t as libc::c_uint)
                as uint32_t as uint32_t;
        nwords -= 1
    }
    sum = (sum >> 16i32).wrapping_add(sum & 0xffffi32 as libc::c_uint);
    sum =
        (sum as libc::c_uint).wrapping_add(sum >> 16i32) as uint32_t as
            uint32_t;
    return if 0 != 0 {
               ((!sum as uint16_t as libc::c_int & 0xff00i32) >> 8i32 |
                    (!sum as uint16_t as libc::c_int & 0xffi32) << 8i32) as
                   __uint16_t as libc::c_int
           } else { _OSSwapInt16(!sum as uint16_t) as libc::c_int } as
               __uint16_t;
}
#[src_loc = "1281:13"]
static mut magic_ping_payload: [libc::c_char; 16] =
    [109, 111, 110, 105, 116, 111, 114, 0, 0, 112, 97, 110, 32, 104, 97, 32];
#[no_mangle]
#[src_loc = "1283:1"]
pub unsafe extern "C" fn gpst_esp_send_probes(mut vpninfo:
                                                  *mut openconnect_info)
 -> libc::c_int {
    /* The GlobalProtect VPN initiates and maintains the ESP connection
	 * using specially-crafted ICMP ("ping") packets.
	 *
	 * 1) These ping packets have a special magic payload. It must
	 *    include at least the 16 bytes below. The Windows client actually
	 *    sends this 56-byte version, but the remaining bytes don't
	 *    seem to matter:
	 *
	 *    "monitor\x00\x00pan ha 0123456789:;<=>? !\"#$%&\'()*+,-./\x10\x11\x12\x13\x14\x15\x16\x18";
	 *
	 * 2) The ping packets are addressed to the IP supplied in the
	 *    config XML as as <gw-address>. In most cases, this is the
	 *    same as the *external* IP address of the VPN gateway
	 *    (vpninfo->ip_info.gateway_addr), but in some cases it is a
	 *    separate address.
	 *
	 *    Don't blame me. I didn't design this.
	 */
    let mut pktlen: libc::c_int = 0;
    let mut seq: libc::c_int = 0;
    let mut pkt: *mut pkt =
        malloc((::std::mem::size_of::<pkt>() as
                    libc::c_ulong).wrapping_add(::std::mem::size_of::<ip>() as
                                                    libc::c_ulong).wrapping_add(8i32
                                                                                    as
                                                                                    libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 16]>()
                                                                                                                    as
                                                                                                                    libc::c_ulong).wrapping_add((*vpninfo).pkt_trailer
                                                                                                                                                    as
                                                                                                                                                    libc::c_ulong))
            as *mut pkt;
    let mut iph: *mut ip =
        (*pkt).data.as_mut_ptr() as *mut libc::c_void as *mut ip;
    let mut icmph: *mut icmp =
        (*pkt).data.as_mut_ptr().offset(::std::mem::size_of::<ip>() as
                                            libc::c_ulong as isize) as
            *mut libc::c_void as *mut icmp;
    let mut pmagic: *mut libc::c_char =
        (*pkt).data.as_mut_ptr().offset(::std::mem::size_of::<ip>() as
                                            libc::c_ulong as isize).offset(8)
            as *mut libc::c_void as *mut libc::c_char;
    if pkt.is_null() { return -12i32 }
    if (*vpninfo).dtls_fd == -1i32 {
        let mut fd: libc::c_int = udp_connect(vpninfo);
        if fd < 0i32 { free(pkt as *mut libc::c_void); return fd }
        /* We are not connected until we get an ESP packet back */
        (*vpninfo).dtls_state = 3i32;
        (*vpninfo).dtls_fd = fd;
        if (*vpninfo)._select_nfds <= (*vpninfo).dtls_fd {
            (*vpninfo)._select_nfds = (*vpninfo).dtls_fd + 1i32
        }
        let mut __fd: libc::c_int = (*vpninfo).dtls_fd;
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
        let mut __fd_0: libc::c_int = (*vpninfo).dtls_fd;
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
                as __int32_t
    }
    seq = 1i32;
    while seq <= (if (*vpninfo).dtls_state == 5i32 { 1i32 } else { 3i32 }) {
        memset(pkt as *mut libc::c_void, 0i32,
               (::std::mem::size_of::<pkt>() as
                    libc::c_ulong).wrapping_add(::std::mem::size_of::<ip>() as
                                                    libc::c_ulong).wrapping_add(8i32
                                                                                    as
                                                                                    libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 16]>()
                                                                                                                    as
                                                                                                                    libc::c_ulong));
        (*pkt).len =
            (::std::mem::size_of::<ip>() as
                 libc::c_ulong).wrapping_add(8i32 as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 16]>()
                                                                                 as
                                                                                 libc::c_ulong)
                as libc::c_int;
        /* IP Header */
        (*iph).set_ip_hl(5i32 as u_int); /* what the Windows client uses */
        (*iph).set_ip_v(4i32 as u_int); /* don't fragment, frag offset = 0 */
        (*iph).ip_len =
            if 0 != 0 {
                (((::std::mem::size_of::<ip>() as
                       libc::c_ulong).wrapping_add(8i32 as
                                                       libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 16]>()
                                                                                       as
                                                                                       libc::c_ulong)
                      as __uint16_t as libc::c_int & 0xff00i32) >> 8i32 |
                     ((::std::mem::size_of::<ip>() as
                           libc::c_ulong).wrapping_add(8i32 as
                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 16]>()
                                                                                           as
                                                                                           libc::c_ulong)
                          as __uint16_t as libc::c_int & 0xffi32) << 8i32) as
                    __uint16_t as libc::c_int
            } else {
                _OSSwapInt16((::std::mem::size_of::<ip>() as
                                  libc::c_ulong).wrapping_add(8i32 as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 16]>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                 as __uint16_t) as libc::c_int
            } as __uint16_t; /* hops */
        (*iph).ip_id =
            if 0 != 0 {
                ((0x4747i32 as __uint16_t as libc::c_int & 0xff00i32) >> 8i32
                     |
                     (0x4747i32 as __uint16_t as libc::c_int & 0xffi32) <<
                         8i32) as __uint16_t as libc::c_int
            } else { _OSSwapInt16(0x4747i32 as __uint16_t) as libc::c_int } as
                __uint16_t; /* ICMP */
        (*iph).ip_off =
            if 0 != 0 {
                ((0x4000i32 as __uint16_t as libc::c_int & 0xff00i32) >> 8i32
                     |
                     (0x4000i32 as __uint16_t as libc::c_int & 0xffi32) <<
                         8i32) as __uint16_t as libc::c_int
            } else { _OSSwapInt16(0x4000i32 as __uint16_t) as libc::c_int } as
                __uint16_t;
        (*iph).ip_ttl = 64i32 as u_char;
        (*iph).ip_p = 1i32 as u_char;
        (*iph).ip_src.s_addr = inet_addr((*vpninfo).ip_info.addr);
        (*iph).ip_dst.s_addr = (*vpninfo).esp_magic;
        (*iph).ip_sum =
            csum(iph as *mut uint16_t,
                 (::std::mem::size_of::<ip>() as
                      libc::c_ulong).wrapping_div(2i32 as libc::c_ulong) as
                     libc::c_int);
        /* ICMP echo request */
        (*icmph).icmp_type =
            8i32 as u_char; /* required to get gateway to respond */
        (*icmph).icmp_hun.ih_idseq.icd_id =
            if 0 != 0 {
                ((0x4747i32 as __uint16_t as libc::c_int & 0xff00i32) >> 8i32
                     |
                     (0x4747i32 as __uint16_t as libc::c_int & 0xffi32) <<
                         8i32) as __uint16_t as libc::c_int
            } else { _OSSwapInt16(0x4747i32 as __uint16_t) as libc::c_int } as
                __uint16_t;
        (*icmph).icmp_hun.ih_idseq.icd_seq =
            if 0 != 0 {
                ((seq as __uint16_t as libc::c_int & 0xff00i32) >> 8i32 |
                     (seq as __uint16_t as libc::c_int & 0xffi32) << 8i32) as
                    __uint16_t as libc::c_int
            } else { _OSSwapInt16(seq as __uint16_t) as libc::c_int } as
                __uint16_t;
        memcpy(pmagic as *mut libc::c_void,
               magic_ping_payload.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong);
        (*icmph).icmp_cksum =
            csum(icmph as *mut uint16_t,
                 (8i32 as
                      libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 16]>()
                                                      as
                                                      libc::c_ulong).wrapping_div(2i32
                                                                                      as
                                                                                      libc::c_ulong)
                     as libc::c_int);
        pktlen = construct_esp_packet(vpninfo, pkt, 4i32 as uint8_t);
        if pktlen >= 0i32 {
            send((*vpninfo).dtls_fd,
                 &mut (*pkt).c2rust_unnamed.esp as *mut C2RustUnnamed_6 as
                     *mut libc::c_void, pktlen as size_t, 0i32);
        }
        seq += 1
    }
    free(pkt as *mut libc::c_void);
    (*vpninfo).dtls_times.last_tx = time(&mut (*vpninfo).new_dtls_started);
    return 0i32;
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
#[no_mangle]
#[src_loc = "1360:1"]
pub unsafe extern "C" fn gpst_esp_catch_probe(mut vpninfo:
                                                  *mut openconnect_info,
                                              mut pkt: *mut pkt)
 -> libc::c_int {
    let mut iph: *mut ip =
        (*pkt).data.as_mut_ptr() as *mut libc::c_void as *mut ip;
    return ((*pkt).len >= 21i32 && (*iph).ip_v() as libc::c_int == 4i32 &&
                (*iph).ip_p as libc::c_int == 1i32 &&
                (*iph).ip_src.s_addr == (*vpninfo).esp_magic &&
                (*pkt).len as libc::c_ulong >=
                    (((((*iph).ip_hl() as libc::c_int) << 2i32) + 8i32) as
                         libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 16]>()
                                                         as libc::c_ulong) &&
                *(*pkt).data.as_mut_ptr().offset((((*iph).ip_hl() as
                                                       libc::c_int) << 2i32)
                                                     as isize) as libc::c_int
                    == 0i32 &&
                memcmp(&mut *(*pkt).data.as_mut_ptr().offset(((((*iph).ip_hl()
                                                                    as
                                                                    libc::c_int)
                                                                   << 2i32) +
                                                                  8i32) as
                                                                 isize) as
                           *mut libc::c_uchar as *const libc::c_void,
                       magic_ping_payload.as_mut_ptr() as *const libc::c_void,
                       ::std::mem::size_of::<[libc::c_char; 16]>() as
                           libc::c_ulong) == 0) as libc::c_int;
}
/* Same magic payload in response */
/* HAVE_ESP */
