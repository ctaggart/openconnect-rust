use libc;
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
}
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
    #[src_loc = "46:1"]
    pub type __int64_t = libc::c_longlong;
    /* ptr1 - ptr2 */
    /* __GNUC__ */
    #[src_loc = "92:1"]
    pub type __darwin_size_t = libc::c_ulong;
    /* sizeof() */
    #[src_loc = "98:1"]
    pub type __darwin_va_list = __builtin_va_list;
    /* clock() */
    #[src_loc = "118:1"]
    pub type __darwin_socklen_t = __uint32_t;
    /* socklen_t (duh) */
    #[src_loc = "119:1"]
    pub type __darwin_ssize_t = libc::c_long;
    /* byte count or error */
    #[src_loc = "120:1"]
    pub type __darwin_time_t = libc::c_long;
    use super::internal::__builtin_va_list;
    /* _BSD_I386__TYPES_H_ */
    /* time() */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int32_t.h:20"]
pub mod _int32_t_h {
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
    pub type int32_t = libc::c_int;
    /* _INT32_T */
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
    /* _U_INT32_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:20"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:20"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_port_t.h:20"]
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
    /* __darwin_ssize_t */
    #[src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
    /* _SSIZE_T */
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_fd_def.h:20"]
pub mod _fd_def_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "49:9"]
    pub struct fd_set {
        pub fds_bits: [__int32_t; 32],
    }
    /* This inline avoids argument side-effect issues with FD_ISSET() */
    #[inline]
    #[src_loc = "55:1"]
    pub unsafe extern "C" fn __darwin_fd_isset(mut _n: libc::c_int,
                                               mut _p: *const fd_set)
     -> libc::c_int {
        return (*_p).fds_bits[(_n as
                                   libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8i32
                                                                                                    as
                                                                                                    libc::c_ulong))
                                  as usize] &
                   ((1i32 as libc::c_ulong) <<
                        (_n as
                             libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                              as
                                                              libc::c_ulong).wrapping_mul(8i32
                                                                                              as
                                                                                              libc::c_ulong)))
                       as __int32_t;
    }
    use super::_types_h::__int32_t;
    /* _FD_SET */
    /*
 * Use the built-in bzero function instead of the library version so that
 * we do not pollute the namespace or introduce prototype warnings.
 */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:22"]
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
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timeval.h:22"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h:25"]
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
    /* __darwin_va_list */
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
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:27"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_fsid_t.h:34"]
pub mod _fsid_t_h {
    /*
 * Copyright (c) 2014 Apple Inc. All rights reserved.
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
    /* int32_t */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "31:9"]
    pub struct fsid {
        pub val: [int32_t; 2],
    }
    #[src_loc = "31:1"]
    pub type fsid_t = fsid;
    use super::_int32_t_h::int32_t;
    /* _FSID_T */
    /* file system id type */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/mount.h:34"]
pub mod mount_h {
    /* !__DARWIN_ONLY_64_BIT_INO_T */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "129:1"]
    pub struct statfs {
        pub f_bsize: uint32_t,
        pub f_iosize: int32_t,
        pub f_blocks: uint64_t,
        pub f_bfree: uint64_t,
        pub f_bavail: uint64_t,
        pub f_files: uint64_t,
        pub f_ffree: uint64_t,
        pub f_fsid: fsid_t,
        pub f_owner: uid_t,
        pub f_type: uint32_t,
        pub f_flags: uint32_t,
        pub f_fssubtype: uint32_t,
        pub f_fstypename: [libc::c_char; 16],
        pub f_mntonname: [libc::c_char; 1024],
        pub f_mntfromname: [libc::c_char; 1024],
        pub f_flags_ext: uint32_t,
        pub f_reserved: [uint32_t; 7],
    }
    use super::_uint32_t_h::uint32_t;
    use super::_int32_t_h::int32_t;
    use super::_uint64_t_h::uint64_t;
    use super::_fsid_t_h::fsid_t;
    use super::_uid_t_h::uid_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "415:1"]
        pub fn statfs(_: *const libc::c_char, _: *mut statfs) -> libc::c_int;
    }
    /* !_SYS_MOUNT_H_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:41"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:41"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:41"]
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
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "440:1"]
    pub struct sockaddr_storage {
        pub ss_len: __uint8_t,
        pub ss_family: sa_family_t,
        pub __ss_pad1: [libc::c_char; 6],
        pub __ss_align: __int64_t,
        pub __ss_pad2: [libc::c_char; 112],
    }
    use super::_types_h::{__uint8_t, __int64_t};
    use super::_sa_family_t_h::sa_family_t;
    use super::_socklen_t_h::socklen_t;
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "712:1"]
        pub fn socket(_: libc::c_int, _: libc::c_int, _: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "709:1"]
        pub fn setsockopt(_: libc::c_int, _: libc::c_int, _: libc::c_int,
                          _: *const libc::c_void, _: socklen_t)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "705:1"]
        pub fn send(_: libc::c_int, _: *const libc::c_void, _: size_t,
                    _: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[src_loc = "701:1"]
        pub fn recv(_: libc::c_int, _: *mut libc::c_void, _: size_t,
                    _: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[src_loc = "693:1"]
        pub fn bind(_: libc::c_int, _: *const sockaddr, _: socklen_t)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "694:1"]
        pub fn connect(_: libc::c_int, _: *const sockaddr, _: socklen_t)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "695:1"]
        pub fn getpeername(_: libc::c_int, _: *mut sockaddr,
                           _: *mut socklen_t) -> libc::c_int;
    }
    /* !_SYS_SOCKET_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:41"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:41"]
pub mod in6_h {
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
 * Copyright (C) 1995, 1996, 1997, and 1998 WIDE Project.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the project nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE PROJECT AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE PROJECT OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
    /*
 * Copyright (c) 1982, 1986, 1990, 1993
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
 *	@(#)in.h	8.3 (Berkeley) 1/3/94
 */
    /*
 * Identification of the network protocol stack
 * for *BSD-current/release: http://www.kame.net/dev/cvsweb.cgi/kame/COVERAGE
 * has the table of implementation/integration differences.
 */
    /*
 * Local port number conventions:
 *
 * Ports < IPPORT_RESERVED are reserved for privileged processes (e.g. root),
 * unless a kernel is compiled with IPNOPRIVPORTS defined.
 *
 * When a user does a bind(2) or connect(2) with a port number of zero,
 * a non-conflicting local port address is chosen.
 *
 * The default range is IPPORT_ANONMIN to IPPORT_ANONMAX, although
 * that is settable by sysctl(3); net.inet.ip.anonportmin and
 * net.inet.ip.anonportmax respectively.
 *
 * A user may set the IPPROTO_IP option IP_PORTRANGE to change this
 * default assignment range.
 *
 * The value IP_PORTRANGE_DEFAULT causes the default behavior.
 *
 * The value IP_PORTRANGE_HIGH is the same as IP_PORTRANGE_DEFAULT,
 * and exists only for FreeBSD compatibility purposes.
 *
 * The value IP_PORTRANGE_LOW changes the range to the "low" are
 * that is (by convention) restricted to privileged processes.
 * This convention is based on "vouchsafe" principles only.
 * It is only secure if you trust the remote host to restrict these ports.
 * The range is IPPORT_RESERVEDMIN to IPPORT_RESERVEDMAX.
 */
    /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
    /*
 * IPv6 address
 */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "151:9"]
    pub struct in6_addr {
        pub __u6_addr: C2RustUnnamed,
        /* 128-bit IP6 address */
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "152:2"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [__uint8_t; 16],
        pub __u6_addr16: [__uint16_t; 8],
        pub __u6_addr32: [__uint32_t; 4],
    }
    /*
 * Socket address for IPv6
 */
    /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
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
        /* scope zone index */
    }
    use super::_types_h::{__uint8_t, __uint16_t, __uint32_t};
    use super::_sa_family_t_h::sa_family_t;
    use super::_in_port_t_h::in_port_t;
    extern "C" {
        /*
 * Definition of some useful macros to handle IP6 addresses
 */
        /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
        #[no_mangle]
        #[src_loc = "211:30"]
        pub static in6addr_any: in6_addr;
    }
    /* !_NETINET6_IN6_H_ */
    /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netdb.h:41"]
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
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
    extern "C" {
        #[no_mangle]
        #[src_loc = "269:1"]
        pub fn freeaddrinfo(_: *mut addrinfo);
        #[no_mangle]
        #[src_loc = "270:1"]
        pub fn gai_strerror(_: libc::c_int) -> *const libc::c_char;
        #[no_mangle]
        #[src_loc = "271:1"]
        pub fn getaddrinfo(_: *const libc::c_char, _: *const libc::c_char,
                           _: *const addrinfo, _: *mut *mut addrinfo)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "277:1"]
        pub fn getnameinfo(_: *const sockaddr, _: socklen_t,
                           _: *mut libc::c_char, _: socklen_t,
                           _: *mut libc::c_char, _: socklen_t, _: libc::c_int)
         -> libc::c_int;
    }
    /* !_NETDB_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src = "/Users/cameron/github/openconnect/openconnect.h:41"]
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
#[header_src = "/Users/cameron/github/openconnect/openconnect-internal.h:41"]
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
    #[inline]
    #[src_loc = "783:1"]
    pub unsafe extern "C" fn set_fd_cloexec(mut fd: libc::c_int)
     -> libc::c_int {
        return fcntl(fd, 2i32, fcntl(fd, 1i32) | 1i32);
    }
    #[inline]
    #[src_loc = "774:1"]
    pub unsafe extern "C" fn set_sock_nonblock(mut fd: libc::c_int)
     -> libc::c_int {
        return fcntl(fd, 4i32, fcntl(fd, 3i32) | 0x4i32);
    }
    use super::_uint32_t_h::uint32_t;
    use super::_time_t_h::time_t;
    use super::gssapi_h::{gss_name_t, gss_ctx_id_t};
    use super::hmac_h::HMAC_CTX;
    use super::ossl_typ_h::EVP_CIPHER_CTX;
    use super::_uint64_t_h::uint64_t;
    use super::openconnect_h::{openconnect_info, oc_auth_form};
    use super::fcntl_h::fcntl;
    extern "C" {
        #[src_loc = "363:1"]
        pub type oc_pcsc_ctx;
        #[no_mangle]
        #[src_loc = "835:1"]
        pub fn openconnect_utf8_to_legacy(vpninfo: *mut openconnect_info,
                                          utf8: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[src_loc = "989:1"]
        pub fn openconnect_close_https(vpninfo: *mut openconnect_info,
                                       final_0: libc::c_int);
        #[no_mangle]
        #[src_loc = "1137:1"]
        pub fn process_auth_form(vpninfo: *mut openconnect_info,
                                 form: *mut oc_auth_form) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1086:1"]
        pub fn buf_append(buf: *mut oc_text_buf, fmt: *const libc::c_char,
                          _: ...);
        #[no_mangle]
        #[src_loc = "1096:1"]
        pub fn buf_error(buf: *mut oc_text_buf) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1100:1"]
        pub fn internal_parse_url(url: *const libc::c_char,
                                  res_proto: *mut *mut libc::c_char,
                                  res_host: *mut *mut libc::c_char,
                                  res_port: *mut libc::c_int,
                                  res_path: *mut *mut libc::c_char,
                                  default_port: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1097:1"]
        pub fn buf_free(buf: *mut oc_text_buf) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1082:1"]
        pub fn buf_alloc() -> *mut oc_text_buf;
        #[no_mangle]
        #[src_loc = "1099:1"]
        pub fn process_proxy(vpninfo: *mut openconnect_info,
                             ssl_sock: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1116:1"]
        pub fn clear_auth_states(vpninfo: *mut openconnect_info,
                                 auth_states: *mut http_auth_state,
                                 reset: libc::c_int);
        #[no_mangle]
        #[src_loc = "847:1"]
        pub fn script_config_tun(vpninfo: *mut openconnect_info,
                                 reason: *const libc::c_char) -> libc::c_int;
    }
    /* __OPENCONNECT_INTERNAL_H__ */
    /* !Not known to be little-endian */
}
#[header_src =
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ossl_typ.h:41"]
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
        pub pkey: C2RustUnnamed_7,
        pub save_parameters: libc::c_int,
        pub attributes: *mut stack_st_X509_ATTRIBUTE,
    }
    /* Methods here */
    /* Can be null */
    /* If this is non-NULL, it will be used to generate parameters */
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
    /* crypto/dh/dh.h */
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
    /*
 * new with 0.9.7h; the built-in DH
 * implementation now uses constant time
 * modular exponentiation for secret exponents
 * by default. This flag causes the
 * faster variable sliding window method to
 * be used for all exponents.
 */
    /*
 * If this flag is set the DH method is FIPS compliant and can be used in
 * FIPS mode. This is set in the validated module method. If an application
 * sets this flag in its own methods it is its reposibility to ensure the
 * result is compliant.
 */
    /*
 * If this flag is set the operations normally disabled in FIPS mode are
 * permitted it is then the applications responsibility to ensure that the
 * usage is compliant.
 */
    /* Already defined in ossl_typ.h */
/* typedef struct dh_st DH; */
/* typedef struct dh_method DH_METHOD; */
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl.h:41"]
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
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the SSL functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/stack.h:41"]
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
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/hmac.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/asn1.h:41"]
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
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509_vfy.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/crypto.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/evp.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bn.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dsa.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pem.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/comp.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dtls1.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pqueue.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl3.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ec.h:41"]
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
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bio.h:41"]
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
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the BIO functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl2.h:41"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zlib.h:41"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zconf.h:41"]
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
#[header_src = "/usr/local/Cellar/stoken/0.92/include/stoken.h:41"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/gssapi/gssapi.h:41"]
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
#[header_src = "/usr/local/Cellar/libproxy/0.4.15_1/include/proxy.h:41"]
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
        /* *
 * Get which proxies to use for the specified URL.
 *
 * A NULL-terminated array of proxy strings is returned.
 * If the first proxy fails, the second should be tried, etc...
 * Don't forget to free the strings/array when you are done.
 * If an unrecoverable error occurs, this function returns NULL.
 *
 * Regarding performance: this method always blocks and may be called
 * in a separate thread (is thread-safe).  In most cases, the time
 * required to complete this function call is simply the time required
 * to read the configuration (i.e. from gconf, kconfig, etc).
 *
 * In the case of PAC, if no valid PAC is found in the cache (i.e.
 * configuration has changed, cache is invalid, etc), the PAC file is
 * downloaded and inserted into the cache. This is the most expensive
 * operation as the PAC is retrieved over the network. Once a PAC exists
 * in the cache, it is merely a javascript invocation to evaluate the PAC.
 * One should note that DNS can be called from within a PAC during
 * javascript invocation.
 *
 * In the case of WPAD, WPAD is used to automatically locate a PAC on the
 * network.  Currently, we only use DNS for this, but other methods may
 * be implemented in the future.  Once the PAC is located, normal PAC
 * performance (described above) applies.
 *
 * The format of the returned proxy strings are as follows:
 *   - http://[username:password@]proxy:port
 *   - socks://[username:password@]proxy:port
 *   - socks5://[username:password@]proxy:port
 *   - socks4://[username:password@]proxy:port
 *   - <procotol>://[username:password@]proxy:port
 *   - direct://
 * Please note that the username and password in the above URLs are optional
 * and should be use to authenticate the connection if present.
 *
 * For SOCKS proxies, when the protocol version is specified (socks4:// or
 * sock5://), it is expected that only this version is used. When only
 * socks:// is set, the client MUST try SOCKS version 5 protocol and, on
 * connection failure, fallback to SOCKS version 4.
 *
 * Other proxying protocols may exist. It is expected that the returned
 * configuration scheme shall match the network service name of the
 * proxy protocol or the service name of the protocol being proxied if the
 * previous does not exist. As an example, on Mac OS X you can configure a
 * RTSP streaming proxy. The expected returned configuration would be:
 *   - rtsp://[username:password@]proxy:port
 * 
 * @url The URL we are trying to reach
 * @return A NULL-terminated array of proxy strings to use
 */
        #[no_mangle]
        #[src_loc = "91:1"]
        pub fn px_proxy_factory_get_proxies(self_0: *mut pxProxyFactory,
                                            url: *const libc::c_char)
         -> *mut *mut libc::c_char;
    }
    /*PROXY_H_*/
}
#[header_src =
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/tree.h:41"]
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
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/dict.h:41"]
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
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/xmlstring.h:41"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/iconv.h:41"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/libkern/i386/_OSByteOrder.h:20"]
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
    use super::_types_h::__uint16_t;
    /* ! _OS__OSBYTEORDERI386_H */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:22"]
pub mod unistd_h {
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "437:1"]
        pub fn close(_: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "472:1"]
        pub fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t)
         -> ssize_t;
    }
    /* _UNISTD_H_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_select.h:22"]
pub mod _select_h {
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
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/fcntl.h:23"]
pub mod fcntl_h {
    extern "C" {
        /* XXX backwards compatibility */
        /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
        #[no_mangle]
        #[src_loc = "530:1"]
        pub fn open(_: *const libc::c_char, _: libc::c_int, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "535:1"]
        pub fn fcntl(_: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    }
    /* !_SYS_FCNTL_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:24"]
pub mod string_h {
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:25"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
        /* Additional functionality provided by:
 * POSIX.1-1988
 */
        /* size for ctermid(); PATH_MAX */
        /* !_DARWIN_UNLIMITED_STREAMS && !_DARWIN_C_SOURCE */
        #[no_mangle]
        #[src_loc = "210:1"]
        pub fn fdopen(_: libc::c_int, _: *const libc::c_char) -> *mut FILE;
        /* __DARWIN_C_LEVEL >= 200112L */
        #[no_mangle]
        #[src_loc = "334:6"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "337:6"]
        pub fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                         _: *const libc::c_char, _: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "370:1"]
        pub fn asprintf(_: *mut *mut libc::c_char, _: *const libc::c_char,
                        _: ...) -> libc::c_int;
    }
    /* _STDIO_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:26"]
pub mod errno_h {
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
    /* This value is only discrete when compiling __DARWIN_UNIX03, or KERNEL */
    /* STREAM ioctl timeout */
    /* Protocol error */
    /* Not a STREAM */
    /* No STREAM resources */
    /* Reserved */
    /* No message available on STREAM */
    /* Reserved */
    /* Bad message */
    /* Attribute not found */
    /* Illegal byte sequence */
    /* No message of desired type */
    /* Identifier removed */
    /* Operation canceled */
    /* Malformed Macho file */
    /* Shared library version mismatch */
    /* Bad CPU type in executable */
    /* Bad executable */
    /* Program loading errors */
    /* Value too large to be stored in data type */
    /* Device error, e.g. paper out */
    /* Device power is off */
    /* Intelligent device errors */
    /* Need authenticator */
    /* Authentication error */
    /* Inappropriate file type or format */
    /* Function not implemented */
    /* No locks available */
    /* Bad procedure for program */
    /* Program version wrong */
    /* RPC prog. not avail */
    /* RPC version wrong */
    /* RPC struct is bad */
    /* Too many levels of remote in path */
    /* Stale NFS file handle */
    /* Network File System */
    /* Disc quota exceeded */
    /* Too many users */
    /* Too many processes */
    /* quotas & mush */
    /* Directory not empty */
    /* No route to host */
    /* Host is down */
    /* should be rearranged */
    /* File name too long */
    /* Too many levels of symbolic links */
    /* Connection refused */
    /* Operation timed out */
    /* Too many references: can't splice */
    /* Can't send after socket shutdown */
    /* Socket is not connected */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:27"]
pub mod _malloc_h {
    extern "C" {
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/time.h:29"]
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
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arpa/inet.h:41"]
pub mod inet_h {
    use super::in_h::in_addr;
    extern "C" {
        #[no_mangle]
        #[src_loc = "83:1"]
        pub fn inet_aton(_: *const libc::c_char, _: *mut in_addr)
         -> libc::c_int;
    }
    /* !_ARPA_INET_H_ */
    /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
}
#[header_src = "/usr/local/Cellar/gettext/0.20.1/include/libintl.h:41"]
pub mod libintl_h {
    extern "C" {
        /* Message catalogs for internationalization.
   Copyright (C) 1995-1997, 2000-2016, 2018-2019 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as published by
   the Free Software Foundation; either version 2.1 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
        /* The LC_MESSAGES locale category is the category used by the functions
   gettext() and dgettext().  It is specified in POSIX, but not in ANSI C.
   On systems that don't define it, use an arbitrary value instead.
   On Solaris, <locale.h> defines __LOCALE_H (or _LOCALE_H in Solaris 2.5)
   then includes <libintl.h> (i.e. this file!) and then only defines
   LC_MESSAGES.  To avoid a redefinition warning, don't define LC_MESSAGES
   in this case.  */
        /* We define an additional symbol to signal that we use the GNU
   implementation of gettext.  */
        /* Provide information about the supported file formats.  Returns the
   maximum minor revision number supported for a given major revision.  */
        /* Resolve a platform specific conflict on DJGPP.  GNU gettext takes
   precedence over _conio_gettext.  */
        /* Version number: (major<<16) + (minor<<8) + subminor */
        /* We redirect the functions to those prefixed with "libintl_".  This is
   necessary, because some systems define gettext/textdomain/... in the C
   library (namely, Solaris 2.4 and newer, and GNU libc 2.0 and newer).
   If we used the unprefixed names, there would be cases where the
   definition in the C library would override the one in the libintl.so
   shared library.  Recall that on ELF systems, the symbols are looked
   up in the following order:
     1. in the executable,
     2. in the shared libraries specified on the link command line, in order,
     3. in the dependencies of the shared libraries specified on the link
        command line,
     4. in the dlopen()ed shared libraries, in the order in which they were
        dlopen()ed.
   The definition in the C library would override the one in libintl.so if
   either
     * -lc is given on the link command line and -lintl isn't, or
     * -lc is given on the link command line before -lintl, or
     * libintl.so is a dependency of a dlopen()ed shared library but not
       linked to the executable at link time.
   Since Solaris gettext() behaves differently than GNU gettext(), this
   would be unacceptable.

   The redirection happens by default through macros in C, so that &gettext
   is independent of the compilation unit, but through inline functions in
   C++, in order not to interfere with the name mangling of class fields or
   class methods called 'gettext'.  */
        /* The user can define _INTL_REDIRECT_INLINE or _INTL_REDIRECT_MACROS.
   If he doesn't, we choose the method.  A third possible method is
   _INTL_REDIRECT_ASM, supported only by GCC.  */
        /* Auxiliary macros.  */
        /* _INTL_MAY_RETURN_STRING_ARG(n) declares that the given function may return
   its n-th argument literally.  This enables GCC to warn for example about
   printf (gettext ("foo %y")).  */
        /* Look up MSGID in the current default message catalog for the current
   LC_MESSAGES locale.  If not found, returns MSGID itself (the default
   text).  */
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
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::_types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                         __int64_t, __darwin_size_t, __darwin_va_list,
                         __darwin_socklen_t, __darwin_ssize_t,
                         __darwin_time_t};
pub use self::_int32_t_h::int32_t;
pub use self::_u_int32_t_h::u_int32_t;
pub use self::sys__types_h::{__darwin_gid_t, __darwin_off_t,
                             __darwin_suseconds_t, __darwin_uid_t};
pub use self::_gid_t_h::gid_t;
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_uid_t_h::uid_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_time_t_h::time_t;
pub use self::_fd_def_h::{fd_set, __darwin_fd_isset};
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_timeval_h::timeval;
pub use self::_va_list_h::va_list;
pub use self::_stdio_h::{fpos_t, __sbuf, __sFILE, FILE, __sFILEX};
pub use self::_uint8_t_h::uint8_t;
pub use self::_fsid_t_h::{fsid, fsid_t};
pub use self::mount_h::statfs;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::socket_h::{sockaddr, sockaddr_storage, socket, setsockopt, send,
                         recv, bind, connect, getpeername};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::in6_h::{in6_addr, C2RustUnnamed, sockaddr_in6, in6addr_any};
pub use self::netdb_h::{addrinfo, freeaddrinfo, gai_strerror, getaddrinfo,
                        getnameinfo};
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
                                       set_fd_cloexec, set_sock_nonblock,
                                       oc_pcsc_ctx,
                                       openconnect_utf8_to_legacy,
                                       openconnect_close_https,
                                       process_auth_form, buf_append,
                                       buf_error, internal_parse_url,
                                       buf_free, buf_alloc, process_proxy,
                                       clear_auth_states, script_config_tun};
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
                      ssl3_enc_method, stack_st_OCSP_RESPID};
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
pub use self::proxy_h::{pxProxyFactory, pxProxyFactory_,
                        px_proxy_factory_get_proxies};
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
pub use self::_OSByteOrder_h::_OSSwapInt16;
use self::unistd_h::{close, read};
use self::_select_h::select;
use self::fcntl_h::{open, fcntl};
use self::string_h::{memcmp, memcpy, memset, strchr, strcmp, strerror, strlen,
                     strncmp, strdup, strndup};
use self::stdio_h::{fdopen, snprintf, vsnprintf, asprintf};
use self::errno_h::__error;
use self::_malloc_h::{malloc, free};
use self::time_h::time;
use self::inet_h::inet_aton;
use self::libintl_h::libintl_dgettext;
#[derive ( Copy, Clone )]
#[repr ( C )]
#[src_loc = "960:3"]
pub union C2RustUnnamed_17 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
}
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
/* OSX < 1.6 doesn't have AI_NUMERICSERV */
/* GNU Hurd doesn't yet declare IPV6_TCLASS */
#[inline]
#[src_loc = "61:1"]
unsafe extern "C" fn connect_pending() -> libc::c_int {
    return (*__error() == 36i32) as libc::c_int;
}
/* Windows is interminably horrid, and has disjoint errno spaces.
 * So if we return a positive value, that's a WSA Error and should
 * be handled with openconnect__win32_strerror(). But if we return a
 * negative value, that's a normal errno and should be handled with
 * strerror(). No, you can't just pass the latter value (negated) to
 * openconnect__win32_strerror() because it gives nonsense results. */
#[src_loc = "76:1"]
unsafe extern "C" fn cancellable_connect(mut vpninfo: *mut openconnect_info,
                                         mut sockfd: libc::c_int,
                                         mut addr: *const sockaddr,
                                         mut addrlen: socklen_t)
 -> libc::c_int {
    let mut peer: sockaddr_storage =
        sockaddr_storage{ss_len: 0,
                         ss_family: 0,
                         __ss_pad1: [0; 6],
                         __ss_align: 0,
                         __ss_pad2: [0; 112],};
    let mut peerlen: socklen_t =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    let mut wr_set: fd_set = fd_set{fds_bits: [0; 32],};
    let mut rd_set: fd_set = fd_set{fds_bits: [0; 32],};
    let mut ex_set: fd_set = fd_set{fds_bits: [0; 32],};
    let mut maxfd: libc::c_int = sockfd;
    let mut err: libc::c_int = 0;
    set_sock_nonblock(sockfd);
    if (*vpninfo).protect_socket.is_some() {
        (*vpninfo).protect_socket.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                      sockfd);
    }
    if connect(sockfd, addr, addrlen) < 0i32 && connect_pending() == 0 {
        return -*__error()
    }
    loop  {
        let mut __fd: libc::c_int = sockfd;
        wr_set.fds_bits[(__fd as
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
        /* Windows indicates failure this way, not in wr_set */
        cmd_fd_set(vpninfo, &mut rd_set, &mut maxfd);
        select(maxfd + 1i32, &mut rd_set, &mut wr_set, &mut ex_set,
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
                                                                                         b"Socket connect cancelled\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            return -4i32
        }
        if !(__darwin_fd_isset(sockfd, &mut wr_set) == 0 &&
                 __darwin_fd_isset(sockfd, &mut ex_set) == 0 &&
                 (*vpninfo).got_pause_cmd == 0) {
            break ;
        }
    }
    /* Check whether connect() succeeded or failed by using
	   getpeername(). See http://cr.yp.to/docs/connect.html */
    if getpeername(sockfd,
                   &mut peer as *mut sockaddr_storage as *mut libc::c_void as
                       *mut sockaddr, &mut peerlen) == 0 {
        return 0i32
    }
    /* On Windows, use getsockopt() to determine the error.
	       * We don't ddo this on Windows because it just reports
	       * -ENOTCONN, which we already knew. */
    err = -*__error();
    if err == -57i32 {
        let mut ch: libc::c_int = 0;
        if read(sockfd, &mut ch as *mut libc::c_int as *mut libc::c_void,
                1i32 as size_t) < 0i32 as libc::c_long {
            err = -*__error()
        }
        /* It should *always* fail! */
    }
    return err;
}
/* checks whether the provided string is an IP or a hostname.
 */
#[no_mangle]
#[src_loc = "145:1"]
pub unsafe extern "C" fn string_is_hostname(mut str: *const libc::c_char)
 -> libc::c_uint {
    let mut buf: in_addr = in_addr{s_addr: 0,};
    /* We don't use inet_pton() because an IPv6 literal is likely to
	   be encased in []. So just check for a colon, which shouldn't
	   occur in hostnames anyway. */
    if str.is_null() || inet_aton(str, &mut buf) != 0 ||
           !strchr(str, ':' as i32).is_null() {
        return 0i32 as libc::c_uint
    }
    return 1i32 as libc::c_uint;
}
#[src_loc = "158:1"]
unsafe extern "C" fn match_sockaddr(mut a: *mut sockaddr,
                                    mut b: *mut sockaddr) -> libc::c_int {
    if (*a).sa_family as libc::c_int == 2i32 {
        let mut a4: *mut sockaddr_in =
            a as *mut libc::c_void as *mut sockaddr_in;
        let mut b4: *mut sockaddr_in =
            b as *mut libc::c_void as *mut sockaddr_in;
        return ((*a4).sin_addr.s_addr == (*b4).sin_addr.s_addr &&
                    (*a4).sin_port as libc::c_int ==
                        (*b4).sin_port as libc::c_int) as libc::c_int
    } else if (*a).sa_family as libc::c_int == 30i32 {
        let mut a6: *mut sockaddr_in6 =
            a as *mut libc::c_void as *mut sockaddr_in6;
        let mut b6: *mut sockaddr_in6 =
            b as *mut libc::c_void as *mut sockaddr_in6;
        return (memcmp(&mut (*a6).sin6_addr as *mut in6_addr as
                           *const libc::c_void,
                       &mut (*b6).sin6_addr as *mut in6_addr as
                           *const libc::c_void,
                       ::std::mem::size_of::<in6_addr>() as libc::c_ulong) ==
                    0 &&
                    (*a6).sin6_port as libc::c_int ==
                        (*b6).sin6_port as libc::c_int) as libc::c_int
    } else { return 0i32 };
}
#[no_mangle]
#[src_loc = "175:1"]
pub unsafe extern "C" fn connect_https_socket(mut vpninfo:
                                                  *mut openconnect_info)
 -> libc::c_int {
    let mut errstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut ssl_sock: libc::c_int = -1i32;
    let mut err: libc::c_int = 0;
    if (*vpninfo).port == 0 { (*vpninfo).port = 443i32 }
    /* If we're talking to a server which told us it has dynamic DNS, don't
	   just re-use its previous IP address. If we're talking to a proxy, we
	   can use *its* previous IP address. We expect it'll re-do the DNS
	   lookup for the server anyway. */
    if !(*vpninfo).peer_addr.is_null() &&
           ((*vpninfo).is_dyndns == 0 || !(*vpninfo).proxy.is_null()) {
        current_block = 17179679302217393232;
    } else {
        let mut hints: addrinfo =
            addrinfo{ai_flags: 0,
                     ai_family: 0,
                     ai_socktype: 0,
                     ai_protocol: 0,
                     ai_addrlen: 0,
                     ai_canonname: 0 as *mut libc::c_char,
                     ai_addr: 0 as *mut sockaddr,
                     ai_next: 0 as *mut addrinfo,};
        let mut result: *mut addrinfo = 0 as *mut addrinfo;
        let mut rp: *mut addrinfo = 0 as *mut addrinfo;
        let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut port: [libc::c_char; 6] = [0; 6];
        memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
        hints.ai_family = 0i32;
        hints.ai_socktype = 1i32;
        hints.ai_flags = 0x1i32 | 0x1000i32;
        hints.ai_protocol = 0i32;
        hints.ai_canonname = 0 as *mut libc::c_char;
        hints.ai_addr = 0 as *mut sockaddr;
        hints.ai_next = 0 as *mut addrinfo;
        /* The 'port' variable is a string because it's easier
		   this way than if we pass NULL to getaddrinfo() and
		   then try to fill in the numeric value into
		   different types of returned sockaddr_in{6,}. */
        if !(*vpninfo).proxy_factory.is_null() {
            let mut url_buf: *mut oc_text_buf = buf_alloc();
            let mut proxies: *mut *mut libc::c_char =
                0 as *mut *mut libc::c_char;
            let mut i: libc::c_int = 0i32;
            free((*vpninfo).proxy_type as *mut libc::c_void);
            (*vpninfo).proxy_type = 0 as *mut libc::c_char;
            free((*vpninfo).proxy as *mut libc::c_void);
            (*vpninfo).proxy = 0 as *mut libc::c_char;
            buf_append(url_buf,
                       b"https://%s\x00" as *const u8 as *const libc::c_char,
                       (*vpninfo).hostname);
            if (*vpninfo).port != 443i32 {
                buf_append(url_buf,
                           b":%d\x00" as *const u8 as *const libc::c_char,
                           (*vpninfo).port);
            }
            buf_append(url_buf,
                       b"/%s\x00" as *const u8 as *const libc::c_char,
                       if !(*vpninfo).urlpath.is_null() {
                           (*vpninfo).urlpath
                       } else {
                           b"\x00" as *const u8 as *const libc::c_char
                       });
            if buf_error(url_buf) != 0 {
                buf_free(url_buf);
                ssl_sock = -12i32;
                current_block = 16930233754088119189;
            } else {
                proxies =
                    px_proxy_factory_get_proxies((*vpninfo).proxy_factory,
                                                 (*url_buf).data);
                i = 0i32;
                while !proxies.is_null() &&
                          !(*proxies.offset(i as isize)).is_null() {
                    if (*vpninfo).proxy.is_null() &&
                           (strncmp(*proxies.offset(i as isize),
                                    b"http://\x00" as *const u8 as
                                        *const libc::c_char,
                                    7i32 as libc::c_ulong) == 0 ||
                                strncmp(*proxies.offset(i as isize),
                                        b"socks://\x00" as *const u8 as
                                            *const libc::c_char,
                                        8i32 as libc::c_ulong) == 0 ||
                                strncmp(*proxies.offset(i as isize),
                                        b"socks5://\x00" as *const u8 as
                                            *const libc::c_char,
                                        9i32 as libc::c_ulong) == 0) {
                        internal_parse_url(*proxies.offset(i as isize),
                                           &mut (*vpninfo).proxy_type,
                                           &mut (*vpninfo).proxy,
                                           &mut (*vpninfo).proxy_port,
                                           0 as *mut *mut libc::c_char, 0i32);
                    }
                    i += 1
                }
                buf_free(url_buf);
                free(proxies as *mut libc::c_void);
                if !(*vpninfo).proxy.is_null() {
                    if (*vpninfo).verbose >= 2i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                2i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Proxy from libproxy: %s://%s:%d/\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).proxy_type,
                                                                                (*vpninfo).proxy,
                                                                                (*vpninfo).port);
                    }
                }
                current_block = 17075014677070940716;
            }
        } else { current_block = 17075014677070940716; }
        match current_block {
            16930233754088119189 => { }
            _ => {
                if !(*vpninfo).proxy.is_null() {
                    hostname = (*vpninfo).proxy;
                    snprintf(port.as_mut_ptr(), 6i32 as libc::c_ulong,
                             b"%d\x00" as *const u8 as *const libc::c_char,
                             (*vpninfo).proxy_port);
                } else {
                    hostname = (*vpninfo).hostname;
                    snprintf(port.as_mut_ptr(), 6i32 as libc::c_ulong,
                             b"%d\x00" as *const u8 as *const libc::c_char,
                             (*vpninfo).port);
                }
                if *hostname.offset(0) as libc::c_int == '[' as i32 &&
                       *hostname.offset(strlen(hostname).wrapping_sub(1i32 as
                                                                          libc::c_ulong)
                                            as isize) as libc::c_int ==
                           ']' as i32 {
                    hostname =
                        strndup(hostname.offset(1),
                                strlen(hostname).wrapping_sub(2i32 as
                                                                  libc::c_ulong));
                    if hostname.is_null() {
                        ssl_sock = -12i32;
                        current_block = 16930233754088119189;
                    } else {
                        hints.ai_flags |= 0x4i32;
                        current_block = 17239133558811367971;
                    }
                } else { current_block = 17239133558811367971; }
                match current_block {
                    16930233754088119189 => { }
                    _ => {
                        if (*vpninfo).getaddrinfo_override.is_some() {
                            err =
                                (*vpninfo).getaddrinfo_override.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                    hostname,
                                                                                                    port.as_mut_ptr(),
                                                                                                    &mut hints,
                                                                                                    &mut result)
                        } else {
                            err =
                                getaddrinfo(hostname, port.as_mut_ptr(),
                                            &mut hints, &mut result)
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
                                                                                                         b"getaddrinfo failed for host \'%s\': %s\n\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char),
                                                                                        hostname,
                                                                                        gai_strerror(err));
                            }
                            if hints.ai_flags & 0x4i32 != 0 {
                                free(hostname as *mut libc::c_void);
                            }
                            ssl_sock = -22i32;
                            /* If we were just retrying for dynamic DNS, reconnct using
			   the previously-known IP address */
                            if !(*vpninfo).peer_addr.is_null() {
                                if (*vpninfo).verbose >= 0i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            0i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"Reconnecting to DynDNS server using previously cached IP address\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char));
                                }
                                current_block = 17179679302217393232;
                            } else { current_block = 16930233754088119189; }
                        } else {
                            if hints.ai_flags & 0x4i32 != 0 {
                                free(hostname as *mut libc::c_void);
                            }
                            rp = result;
                            loop  {
                                if rp.is_null() {
                                    current_block = 11354253847736050364;
                                    break ;
                                }
                                let mut host: [libc::c_char; 80] = [0; 80];
                                host[0] = 0i32 as libc::c_char;
                                if getnameinfo((*rp).ai_addr,
                                               (*rp).ai_addrlen,
                                               host.as_mut_ptr(),
                                               ::std::mem::size_of::<[libc::c_char; 80]>()
                                                   as libc::c_ulong as
                                                   socklen_t,
                                               0 as *mut libc::c_char,
                                               0i32 as socklen_t, 0x2i32) == 0
                                   {
                                    if (*vpninfo).verbose >= 2i32 {
                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                2i32,
                                                                                                if !(*vpninfo).proxy_type.is_null()
                                                                                                   {
                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char,
                                                                                                                     b"Attempting to connect to proxy %s%s%s:%s\n\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char)
                                                                                                } else {
                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char,
                                                                                                                     b"Attempting to connect to server %s%s%s:%s\n\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char)
                                                                                                },
                                                                                                if (*rp).ai_family
                                                                                                       ==
                                                                                                       30i32
                                                                                                   {
                                                                                                    b"[\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char
                                                                                                } else {
                                                                                                    b"\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char
                                                                                                },
                                                                                                host.as_mut_ptr(),
                                                                                                if (*rp).ai_family
                                                                                                       ==
                                                                                                       30i32
                                                                                                   {
                                                                                                    b"]\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char
                                                                                                } else {
                                                                                                    b"\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char
                                                                                                },
                                                                                                port.as_mut_ptr());
                                    }
                                }
                                ssl_sock =
                                    socket((*rp).ai_family, (*rp).ai_socktype,
                                           (*rp).ai_protocol);
                                if !(ssl_sock < 0i32) {
                                    set_fd_cloexec(ssl_sock);
                                    err =
                                        cancellable_connect(vpninfo, ssl_sock,
                                                            (*rp).ai_addr,
                                                            (*rp).ai_addrlen);
                                    if err == 0 {
                                        /* Store the peer address we actually used, so that DTLS can
				   use it again later */
                                        free((*vpninfo).ip_info.gateway_addr
                                                 as *mut libc::c_void);
                                        (*vpninfo).ip_info.gateway_addr =
                                            0 as *mut libc::c_char;
                                        if host[0] != 0 {
                                            (*vpninfo).ip_info.gateway_addr =
                                                strdup(host.as_mut_ptr());
                                            if (*vpninfo).verbose >= 1i32 {
                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                        1i32,
                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char,
                                                                                                                         b"Connected to %s%s%s:%s\n\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char),
                                                                                                        if (*rp).ai_family
                                                                                                               ==
                                                                                                               30i32
                                                                                                           {
                                                                                                            b"[\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char
                                                                                                        } else {
                                                                                                            b"\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char
                                                                                                        },
                                                                                                        host.as_mut_ptr(),
                                                                                                        if (*rp).ai_family
                                                                                                               ==
                                                                                                               30i32
                                                                                                           {
                                                                                                            b"]\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char
                                                                                                        } else {
                                                                                                            b"\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char
                                                                                                        },
                                                                                                        port.as_mut_ptr());
                                            }
                                        }
                                        free((*vpninfo).peer_addr as
                                                 *mut libc::c_void);
                                        (*vpninfo).peer_addrlen =
                                            0i32 as socklen_t;
                                        (*vpninfo).peer_addr =
                                            malloc((*rp).ai_addrlen as
                                                       libc::c_ulong) as
                                                *mut sockaddr;
                                        if (*vpninfo).peer_addr.is_null() {
                                            if (*vpninfo).verbose >= 0i32 {
                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                        0i32,
                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char,
                                                                                                                         b"Failed to allocate sockaddr storage\n\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char));
                                            }
                                            close(ssl_sock);
                                            ssl_sock = -12i32;
                                            current_block =
                                                16930233754088119189;
                                            break ;
                                        } else {
                                            (*vpninfo).peer_addrlen =
                                                (*rp).ai_addrlen;
                                            memcpy((*vpninfo).peer_addr as
                                                       *mut libc::c_void,
                                                   (*rp).ai_addr as
                                                       *const libc::c_void,
                                                   (*rp).ai_addrlen as
                                                       libc::c_ulong);
                                            /* If no proxy, ensure that we output *this* IP address in
				 * authentication results because we're going to need to
				 * reconnect to the *same* server from the rotation. And with
				 * some trick DNS setups, it might possibly be a "rotation"
				 * even if we only got one result from getaddrinfo() this
				 * time.
				 *
				 * If there's a proxy, we're kind of screwed; we can't know
				 * which IP address we connected to. Perhaps we ought to do
				 * the DNS lookup locally and connect to a specific IP? */
                                            if (*vpninfo).proxy.is_null() &&
                                                   host[0] as libc::c_int != 0
                                               {
                                                let mut p: *mut libc::c_char =
                                                    malloc(strlen(host.as_mut_ptr()).wrapping_add(3i32
                                                                                                      as
                                                                                                      libc::c_ulong))
                                                        as *mut libc::c_char;
                                                if !p.is_null() {
                                                    free((*vpninfo).unique_hostname
                                                             as
                                                             *mut libc::c_void);
                                                    (*vpninfo).unique_hostname
                                                        = p;
                                                    if (*rp).ai_family ==
                                                           30i32 {
                                                        let fresh0 = p;
                                                        p = p.offset(1);
                                                        *fresh0 =
                                                            '[' as i32 as
                                                                libc::c_char
                                                    }
                                                    memcpy(p as
                                                               *mut libc::c_void,
                                                           host.as_mut_ptr()
                                                               as
                                                               *const libc::c_void,
                                                           strlen(host.as_mut_ptr()));
                                                    p =
                                                        p.offset(strlen(host.as_mut_ptr())
                                                                     as
                                                                     isize);
                                                    if (*rp).ai_family ==
                                                           30i32 {
                                                        let fresh1 = p;
                                                        p = p.offset(1);
                                                        *fresh1 =
                                                            ']' as i32 as
                                                                libc::c_char
                                                    }
                                                    *p = 0i32 as libc::c_char
                                                }
                                            }
                                            current_block =
                                                11354253847736050364;
                                            break ;
                                        }
                                    } else {
                                        if host[0] != 0 {
                                            let mut errstr_0:
                                                    *mut libc::c_char =
                                                0 as *mut libc::c_char;
                                            errstr_0 = strerror(-err);
                                            if (*vpninfo).verbose >= 1i32 {
                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                        1i32,
                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char,
                                                                                                                         b"Failed to connect to %s%s%s:%s: %s\n\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char),
                                                                                                        if (*rp).ai_family
                                                                                                               ==
                                                                                                               30i32
                                                                                                           {
                                                                                                            b"[\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char
                                                                                                        } else {
                                                                                                            b"\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char
                                                                                                        },
                                                                                                        host.as_mut_ptr(),
                                                                                                        if (*rp).ai_family
                                                                                                               ==
                                                                                                               30i32
                                                                                                           {
                                                                                                            b"]\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char
                                                                                                        } else {
                                                                                                            b"\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char
                                                                                                        },
                                                                                                        port.as_mut_ptr(),
                                                                                                        errstr_0);
                                            }
                                        }
                                        close(ssl_sock);
                                        ssl_sock = -1i32;
                                        /* If we're in DynDNS mode but this *was* the cached IP address,
			 * don't bother falling back to it if it didn't work. */
                                        if !(*vpninfo).peer_addr.is_null() &&
                                               (*vpninfo).peer_addrlen ==
                                                   (*rp).ai_addrlen &&
                                               match_sockaddr((*vpninfo).peer_addr,
                                                              (*rp).ai_addr)
                                                   != 0 {
                                            if (*vpninfo).verbose >= 3i32 {
                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                        3i32,
                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char,
                                                                                                                         b"Forgetting non-functional previous peer address\n\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char));
                                            }
                                            free((*vpninfo).peer_addr as
                                                     *mut libc::c_void);
                                            (*vpninfo).peer_addr =
                                                0 as *mut sockaddr;
                                            (*vpninfo).peer_addrlen =
                                                0i32 as socklen_t;
                                            free((*vpninfo).ip_info.gateway_addr
                                                     as *mut libc::c_void);
                                            (*vpninfo).ip_info.gateway_addr =
                                                0 as *mut libc::c_char
                                        }
                                    }
                                }
                                rp = (*rp).ai_next
                            }
                            match current_block {
                                16930233754088119189 => { }
                                _ => {
                                    freeaddrinfo(result);
                                    if ssl_sock < 0i32 {
                                        if (*vpninfo).verbose >= 0i32 {
                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                    0i32,
                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char,
                                                                                                                     b"Failed to connect to host %s\n\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char),
                                                                                                    if !(*vpninfo).proxy.is_null()
                                                                                                       {
                                                                                                        (*vpninfo).proxy
                                                                                                    } else {
                                                                                                        (*vpninfo).hostname
                                                                                                    });
                                        }
                                        ssl_sock = -22i32;
                                        if !(*vpninfo).peer_addr.is_null() {
                                            if (*vpninfo).verbose >= 0i32 {
                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                        0i32,
                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char,
                                                                                                                         b"Reconnecting to DynDNS server using previously cached IP address\n\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char));
                                            }
                                            current_block =
                                                17179679302217393232;
                                        } else {
                                            current_block =
                                                16930233754088119189;
                                        }
                                    } else {
                                        current_block = 8466485602140941539;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    loop  {
        match current_block {
            16930233754088119189 => {
                /* If proxy processing returned -EAGAIN to reconnect before attempting
	   further auth, and we failed to reconnect, we have to clean up here. */
                clear_auth_states(vpninfo, (*vpninfo).proxy_auth.as_mut_ptr(),
                                  1i32);
                break ;
            }
            17179679302217393232 => {
                ssl_sock =
                    socket((*(*vpninfo).peer_addr).sa_family as libc::c_int,
                           1i32, 0i32);
                if ssl_sock < 0i32 {
                    err = -*__error()
                } else {
                    set_fd_cloexec(ssl_sock);
                    err =
                        cancellable_connect(vpninfo, ssl_sock,
                                            (*vpninfo).peer_addr,
                                            (*vpninfo).peer_addrlen);
                    if !(err != 0) {
                        current_block = 8466485602140941539;
                        continue ;
                    }
                    errstr = 0 as *mut libc::c_char;
                }
                errstr = strerror(-err);
                if !(*vpninfo).proxy.is_null() {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Failed to reconnect to proxy %s: %s\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).proxy,
                                                                                errstr);
                    }
                } else if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to reconnect to host %s: %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            (*vpninfo).hostname,
                                                                            errstr);
                }
                if ssl_sock >= 0i32 { close(ssl_sock); }
                ssl_sock = -22i32;
                current_block = 16930233754088119189;
            }
            _ => {
                if (*vpninfo).proxy.is_null() {
                    current_block = 16930233754088119189;
                    continue ;
                }
                err = process_proxy(vpninfo, ssl_sock);
                if !(err != 0) {
                    current_block = 16930233754088119189;
                    continue ;
                }
                close(ssl_sock);
                if err == -35i32 {
                    /* Proxy authentication failed and we need to retry */
                    if (*vpninfo).verbose >= 2i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                2i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Reconnecting to proxy %s\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).proxy);
                    }
                    current_block = 17179679302217393232;
                } else {
                    ssl_sock = err;
                    current_block = 16930233754088119189;
                }
            }
        }
    }
    return ssl_sock;
}
#[no_mangle]
#[src_loc = "478:1"]
pub unsafe extern "C" fn openconnect_SSL_printf(mut vpninfo:
                                                    *mut openconnect_info,
                                                mut fmt: *const libc::c_char,
                                                mut args: ...)
 -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut args_0: ::std::ffi::VaListImpl;
    buf[1023] = 0i32 as libc::c_char;
    args_0 = args.clone();
    vsnprintf(buf.as_mut_ptr(), 1023i32 as libc::c_ulong, fmt,
              args_0.as_va_list());
    return (*vpninfo).ssl_write.expect("non-null function pointer")(vpninfo,
                                                                    buf.as_mut_ptr(),
                                                                    strlen(buf.as_mut_ptr()));
}
#[no_mangle]
#[src_loc = "493:1"]
pub unsafe extern "C" fn request_passphrase(mut vpninfo:
                                                *mut openconnect_info,
                                            mut label: *const libc::c_char,
                                            mut response:
                                                *mut *mut libc::c_char,
                                            mut fmt: *const libc::c_char,
                                            mut args: ...) -> libc::c_int {
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
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut args_0: ::std::ffi::VaListImpl;
    let mut ret: libc::c_int = 0;
    buf[1023] = 0i32 as libc::c_char;
    memset(&mut f as *mut oc_auth_form as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<oc_auth_form>() as libc::c_ulong);
    args_0 = args.clone();
    vsnprintf(buf.as_mut_ptr(), 1023i32 as libc::c_ulong, fmt,
              args_0.as_va_list());
    f.auth_id = label as *mut libc::c_char;
    f.opts = &mut o;
    o.next = 0 as *mut oc_form_opt;
    o.type_0 = 2i32;
    o.name = label as *mut libc::c_char;
    o.label = buf.as_mut_ptr();
    o._value = 0 as *mut libc::c_char;
    ret = process_auth_form(vpninfo, &mut f);
    if ret == 0 { *response = o._value; return 0i32 }
    return -5i32;
}
#[no_mangle]
#[src_loc = "593:1"]
pub unsafe extern "C" fn openconnect_passphrase_from_fsid(mut vpninfo:
                                                              *mut openconnect_info)
 -> libc::c_int {
    let mut sslkey: *mut libc::c_char =
        openconnect_utf8_to_legacy(vpninfo, (*vpninfo).sslkey);
    let mut buf: statfs =
        statfs{f_bsize: 0,
               f_iosize: 0,
               f_blocks: 0,
               f_bfree: 0,
               f_bavail: 0,
               f_files: 0,
               f_ffree: 0,
               f_fsid: fsid_t{val: [0; 2],},
               f_owner: 0,
               f_type: 0,
               f_flags: 0,
               f_fssubtype: 0,
               f_fstypename: [0; 16],
               f_mntonname: [0; 1024],
               f_mntfromname: [0; 1024],
               f_flags_ext: 0,
               f_reserved: [0; 7],};
    let mut fsid: *mut libc::c_uint =
        &mut buf.f_fsid as *mut fsid_t as *mut libc::c_uint;
    let mut fsid64: libc::c_ulonglong = 0;
    let mut err: libc::c_int = 0i32;
    if statfs(sslkey, &mut buf) != 0 {
        err = -*__error();
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"statfs: %s\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    strerror(*__error()));
        }
        return -err
    } else {
        fsid64 =
            (*fsid.offset(0) as libc::c_ulonglong) << 32i32 |
                *fsid.offset(1) as libc::c_ulonglong;
        if asprintf(&mut (*vpninfo).cert_password as *mut *mut libc::c_char,
                    b"%llx\x00" as *const u8 as *const libc::c_char, fsid64)
               == -1i32 {
            err = -12i32
        }
    }
    if sslkey != (*vpninfo).sslkey { free(sslkey as *mut libc::c_void); }
    return err;
}
/* We put this here rather than in openssl.c because it might be needed
   for OpenSSL DTLS support even when GnuTLS is being used for HTTPS */
#[no_mangle]
#[src_loc = "628:1"]
pub unsafe extern "C" fn openconnect_print_err_cb(mut str:
                                                      *const libc::c_char,
                                                  mut len: size_t,
                                                  mut ptr: *mut libc::c_void)
 -> libc::c_int {
    let mut vpninfo: *mut openconnect_info = ptr as *mut openconnect_info;
    if (*vpninfo).verbose >= 0i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                0i32,
                                                                b"%s\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                str);
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "770:1"]
pub unsafe extern "C" fn cmd_fd_set(mut vpninfo: *mut openconnect_info,
                                    mut fds: *mut fd_set,
                                    mut maxfd: *mut libc::c_int) {
    if (*vpninfo).cmd_fd != -1i32 {
        let mut __fd: libc::c_int = (*vpninfo).cmd_fd;
        (*fds).fds_bits[(__fd as
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
        if (*vpninfo).cmd_fd > *maxfd { *maxfd = (*vpninfo).cmd_fd }
    };
}
#[no_mangle]
#[src_loc = "779:1"]
pub unsafe extern "C" fn check_cmd_fd(mut vpninfo: *mut openconnect_info,
                                      mut fds: *mut fd_set) {
    let mut cmd: libc::c_char = 0;
    if (*vpninfo).cmd_fd == -1i32 ||
           __darwin_fd_isset((*vpninfo).cmd_fd, fds) == 0 {
        return
    }
    if (*vpninfo).cmd_fd_write == -1i32 {
        /* legacy openconnect_set_cancel_fd() users */
        (*vpninfo).got_cancel_cmd = 1i32;
        return
    }
    if read((*vpninfo).cmd_fd,
            &mut cmd as *mut libc::c_char as *mut libc::c_void,
            1i32 as size_t) != 1i32 as libc::c_long {
        return
    }
    match cmd as libc::c_int {
        120 | 100 => {
            (*vpninfo).got_cancel_cmd = 1i32;
            (*vpninfo).cancel_type = cmd
        }
        112 => { (*vpninfo).got_pause_cmd = 1i32 }
        115 => {
            if (*vpninfo).stats_handler.is_some() {
                (*vpninfo).stats_handler.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                             &mut (*vpninfo).stats);
            }
        }
        _ => { }
    };
}
#[no_mangle]
#[src_loc = "813:1"]
pub unsafe extern "C" fn is_cancel_pending(mut vpninfo: *mut openconnect_info,
                                           mut fds: *mut fd_set)
 -> libc::c_int {
    check_cmd_fd(vpninfo, fds);
    return ((*vpninfo).got_cancel_cmd != 0 || (*vpninfo).got_pause_cmd != 0)
               as libc::c_int;
}
#[no_mangle]
#[src_loc = "819:1"]
pub unsafe extern "C" fn poll_cmd_fd(mut vpninfo: *mut openconnect_info,
                                     mut timeout: libc::c_int) {
    let mut rd_set: fd_set = fd_set{fds_bits: [0; 32],};
    let mut maxfd: libc::c_int = 0i32;
    let mut expiration: time_t =
        time(0 as *mut time_t) + timeout as libc::c_long;
    let mut now: time_t = 0i32 as time_t;
    while now < expiration && (*vpninfo).got_cancel_cmd == 0 &&
              (*vpninfo).got_pause_cmd == 0 {
        let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
        now = time(0 as *mut time_t);
        tv.tv_sec =
            if now >= expiration {
                0i32 as libc::c_long
            } else { (expiration) - now };
        tv.tv_usec = 0i32;
        cmd_fd_set(vpninfo, &mut rd_set, &mut maxfd);
        select(maxfd + 1i32, &mut rd_set, 0 as *mut fd_set, 0 as *mut fd_set,
               &mut tv);
        check_cmd_fd(vpninfo, &mut rd_set);
    };
}
#[no_mangle]
#[src_loc = "865:1"]
pub unsafe extern "C" fn openconnect_open_utf8(mut vpninfo:
                                                   *mut openconnect_info,
                                               mut fname: *const libc::c_char,
                                               mut mode: libc::c_int)
 -> libc::c_int {
    let mut legacy_fname: *mut libc::c_char =
        openconnect_utf8_to_legacy(vpninfo, fname);
    let mut fd: libc::c_int = 0;
    fd = open(legacy_fname, mode, 0o644i32);
    if legacy_fname != fname as *mut libc::c_char {
        free(legacy_fname as *mut libc::c_void);
    }
    return fd;
}
#[no_mangle]
#[src_loc = "878:1"]
pub unsafe extern "C" fn openconnect_fopen_utf8(mut vpninfo:
                                                    *mut openconnect_info,
                                                mut fname:
                                                    *const libc::c_char,
                                                mut mode: *const libc::c_char)
 -> *mut FILE {
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    if strcmp(mode, b"r\x00" as *const u8 as *const libc::c_char) == 0 {
        flags = 0i32 | 0x1000000i32
    } else if strcmp(mode, b"rb\x00" as *const u8 as *const libc::c_char) == 0
     {
        flags = 0i32 | 0x1000000i32 | 0i32
    } else if strcmp(mode, b"w\x00" as *const u8 as *const libc::c_char) == 0
     {
        flags = 0x1i32 | 0x1000000i32 | 0x200i32 | 0x400i32
    } else if strcmp(mode, b"wb\x00" as *const u8 as *const libc::c_char) == 0
     {
        flags = 0x1i32 | 0x1000000i32 | 0x200i32 | 0x400i32 | 0i32
    } else {
        /* This should never happen, but if we forget and start using other
		   modes without implementing proper mode->flags conversion, complain! */
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"openconnect_fopen_utf8() used with unsupported mode \'%s\'\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    mode);
        }
        return 0 as *mut FILE
    }
    fd = openconnect_open_utf8(vpninfo, fname, flags);
    if fd == -1i32 { return 0 as *mut FILE }
    return fdopen(fd, mode);
}
#[no_mangle]
#[src_loc = "908:1"]
pub unsafe extern "C" fn udp_sockaddr(mut vpninfo: *mut openconnect_info,
                                      mut port: libc::c_int) -> libc::c_int {
    free((*vpninfo).dtls_addr as *mut libc::c_void);
    (*vpninfo).dtls_addr =
        malloc((*vpninfo).peer_addrlen as libc::c_ulong) as *mut sockaddr;
    if (*vpninfo).dtls_addr.is_null() { return -12i32 }
    memcpy((*vpninfo).dtls_addr as *mut libc::c_void,
           (*vpninfo).peer_addr as *const libc::c_void,
           (*vpninfo).peer_addrlen as libc::c_ulong);
    if (*(*vpninfo).peer_addr).sa_family as libc::c_int == 2i32 {
        let mut sin: *mut sockaddr_in =
            (*vpninfo).dtls_addr as *mut libc::c_void as *mut sockaddr_in;
        (*sin).sin_port =
            if 0 != 0 {
                ((port as __uint16_t as libc::c_int & 0xff00i32) >> 8i32 |
                     (port as __uint16_t as libc::c_int & 0xffi32) << 8i32) as
                    __uint16_t as libc::c_int
            } else { _OSSwapInt16(port as __uint16_t) as libc::c_int } as
                __uint16_t;
        (*vpninfo).dtls_tos_proto = 0i32;
        (*vpninfo).dtls_tos_optname = 3i32
    } else if (*(*vpninfo).peer_addr).sa_family as libc::c_int == 30i32 {
        let mut sin_0: *mut sockaddr_in6 =
            (*vpninfo).dtls_addr as *mut libc::c_void as *mut sockaddr_in6;
        (*sin_0).sin6_port =
            if 0 != 0 {
                ((port as __uint16_t as libc::c_int & 0xff00i32) >> 8i32 |
                     (port as __uint16_t as libc::c_int & 0xffi32) << 8i32) as
                    __uint16_t as libc::c_int
            } else { _OSSwapInt16(port as __uint16_t) as libc::c_int } as
                __uint16_t;
        (*vpninfo).dtls_tos_proto = 41i32;
        (*vpninfo).dtls_tos_optname = 36i32
    } else {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Unknown protocol family %d. Cannot create UDP server address\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*(*vpninfo).peer_addr).sa_family
                                                                        as
                                                                        libc::c_int);
        }
        return -22i32
    }
    /* in case DTLS TOS copy is disabled, reset the optname value */
	/* so that the copy won't be applied in dtls.c / dtls_mainloop() */
    if (*vpninfo).dtls_pass_tos == 0 { (*vpninfo).dtls_tos_optname = 0i32 }
    return 0i32;
}
#[no_mangle]
#[src_loc = "944:1"]
pub unsafe extern "C" fn udp_connect(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut sndbuf: libc::c_int = 0;
    fd =
        socket((*(*vpninfo).peer_addr).sa_family as libc::c_int, 2i32, 17i32);
    if fd < 0i32 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    b"%s: %s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Open UDP socket\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    strerror(*__error()));
        }
        return -22i32
    }
    if (*vpninfo).protect_socket.is_some() {
        (*vpninfo).protect_socket.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                      fd);
    }
    sndbuf = (*vpninfo).ip_info.mtu * 2i32;
    setsockopt(fd, 0xffffi32, 0x1001i32,
               &mut sndbuf as *mut libc::c_int as *mut libc::c_void,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                   socklen_t);
    if (*vpninfo).dtls_local_port != 0 {
        let mut dtls_bind_addr: C2RustUnnamed_17 =
            C2RustUnnamed_17{in_0:
                                 sockaddr_in{sin_len: 0,
                                             sin_family: 0,
                                             sin_port: 0,
                                             sin_addr: in_addr{s_addr: 0,},
                                             sin_zero: [0; 8],},};
        let mut dtls_bind_addrlen: libc::c_int = 0;
        memset(&mut dtls_bind_addr as *mut C2RustUnnamed_17 as
                   *mut libc::c_void, 0i32,
               ::std::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong);
        if (*(*vpninfo).peer_addr).sa_family as libc::c_int == 2i32 {
            let mut addr: *mut sockaddr_in = &mut dtls_bind_addr.in_0;
            dtls_bind_addrlen =
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                    libc::c_int;
            (*addr).sin_family = 2i32 as sa_family_t;
            (*addr).sin_addr.s_addr = 0i32 as u_int32_t;
            (*addr).sin_port =
                if 0 != 0 {
                    (((*vpninfo).dtls_local_port as __uint16_t as libc::c_int
                          & 0xff00i32) >> 8i32 |
                         ((*vpninfo).dtls_local_port as __uint16_t as
                              libc::c_int & 0xffi32) << 8i32) as __uint16_t as
                        libc::c_int
                } else {
                    _OSSwapInt16((*vpninfo).dtls_local_port as __uint16_t) as
                        libc::c_int
                } as __uint16_t
        } else if (*(*vpninfo).peer_addr).sa_family as libc::c_int == 30i32 {
            let mut addr_0: *mut sockaddr_in6 = &mut dtls_bind_addr.in6;
            dtls_bind_addrlen =
                ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                    libc::c_int;
            (*addr_0).sin6_family = 30i32 as sa_family_t;
            (*addr_0).sin6_addr = in6addr_any;
            (*addr_0).sin6_port =
                if 0 != 0 {
                    (((*vpninfo).dtls_local_port as __uint16_t as libc::c_int
                          & 0xff00i32) >> 8i32 |
                         ((*vpninfo).dtls_local_port as __uint16_t as
                              libc::c_int & 0xffi32) << 8i32) as __uint16_t as
                        libc::c_int
                } else {
                    _OSSwapInt16((*vpninfo).dtls_local_port as __uint16_t) as
                        libc::c_int
                } as __uint16_t
        } else {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Unknown protocol family %d. Cannot use UDP transport\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        (*(*vpninfo).peer_addr).sa_family
                                                                            as
                                                                            libc::c_int);
            }
            (*vpninfo).dtls_attempt_period = 0i32;
            close(fd);
            return -22i32
        }
        if bind(fd,
                &mut dtls_bind_addr as *mut C2RustUnnamed_17 as *mut sockaddr,
                dtls_bind_addrlen as socklen_t) != 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        b"%s: %s\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Bind UDP socket\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        strerror(*__error()));
            }
            close(fd);
            return -22i32
        }
    }
    if connect(fd, (*vpninfo).dtls_addr, (*vpninfo).peer_addrlen) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    b"%s: %s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Connect UDP socket\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    strerror(*__error()));
        }
        close(fd);
        return -22i32
    }
    set_fd_cloexec(fd);
    set_sock_nonblock(fd);
    return fd;
}
#[no_mangle]
#[src_loc = "1007:1"]
pub unsafe extern "C" fn ssl_reconnect(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut timeout: libc::c_int = 0;
    let mut interval: libc::c_int = 0;
    openconnect_close_https(vpninfo, 0i32);
    timeout = (*vpninfo).reconnect_timeout;
    interval = (*vpninfo).reconnect_interval;
    free((*vpninfo).dtls_pkt as *mut libc::c_void);
    (*vpninfo).dtls_pkt = 0 as *mut pkt;
    free((*vpninfo).tun_pkt as *mut libc::c_void);
    (*vpninfo).tun_pkt = 0 as *mut pkt;
    loop  {
        script_config_tun(vpninfo,
                          b"attempt-reconnect\x00" as *const u8 as
                              *const libc::c_char);
        ret =
            (*(*vpninfo).proto).tcp_connect.expect("non-null function pointer")(vpninfo);
        if ret == 0 { break ; }
        if timeout <= 0i32 { return ret }
        if ret == -1i32 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Cookie is no longer valid, ending session\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            return ret
        }
        if (*vpninfo).verbose >= 1i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    1i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"sleep %ds, remaining timeout %ds\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    interval,
                                                                    timeout);
        }
        poll_cmd_fd(vpninfo, interval);
        if (*vpninfo).got_cancel_cmd != 0 { return -4i32 }
        if (*vpninfo).got_pause_cmd != 0 { return 0i32 }
        timeout -= interval;
        interval += (*vpninfo).reconnect_interval;
        if interval > 100i32 { interval = 100i32 }
    }
    script_config_tun(vpninfo,
                      b"reconnect\x00" as *const u8 as *const libc::c_char);
    if (*vpninfo).reconnected.is_some() {
        (*vpninfo).reconnected.expect("non-null function pointer")((*vpninfo).cbdata);
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "1058:1"]
pub unsafe extern "C" fn cancellable_gets(mut vpninfo: *mut openconnect_info,
                                          mut fd: libc::c_int,
                                          mut buf: *mut libc::c_char,
                                          mut len: size_t) -> libc::c_int {
    let mut i: libc::c_int = 0i32;
    let mut ret: libc::c_int = 0;
    if len < 2i32 as libc::c_ulong { return -22i32 }
    loop  {
        ret =
            cancellable_recv(vpninfo, fd,
                             buf.offset(i as isize) as *mut libc::c_void as
                                 *mut libc::c_char, 1i32 as size_t);
        if !(ret == 1i32) { break ; }
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
    }
    *buf.offset(i as isize) = 0i32 as libc::c_char;
    return if i != 0 { i } else { ret };
}
#[no_mangle]
#[src_loc = "1087:1"]
pub unsafe extern "C" fn cancellable_send(mut vpninfo: *mut openconnect_info,
                                          mut fd: libc::c_int,
                                          mut buf: *mut libc::c_char,
                                          mut len: size_t) -> libc::c_int {
    let mut count: size_t = 0;
    if fd == -1i32 { return -22i32 }
    count = 0i32 as size_t;
    while count < len {
        let mut rd_set: fd_set = fd_set{fds_bits: [0; 32],};
        let mut wr_set: fd_set = fd_set{fds_bits: [0; 32],};
        let mut maxfd: libc::c_int = fd;
        let mut i: libc::c_int = 0;
        let mut __fd: libc::c_int = fd;
        wr_set.fds_bits[(__fd as
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
        cmd_fd_set(vpninfo, &mut rd_set, &mut maxfd);
        select(maxfd + 1i32, &mut rd_set, &mut wr_set, 0 as *mut fd_set,
               0 as *mut timeval);
        if is_cancel_pending(vpninfo, &mut rd_set) != 0 { return -4i32 }
        /* Not that this should ever be able to happen... */
        if __darwin_fd_isset(fd, &mut wr_set) == 0 { continue ; }
        i =
            send(fd,
                 &mut *buf.offset(count as isize) as *mut libc::c_char as
                     *mut libc::c_void, len.wrapping_sub(count), 0i32) as
                libc::c_int;
        if i < 0i32 { return -*__error() }
        count =
            (count as libc::c_ulong).wrapping_add(i as libc::c_ulong) as
                size_t as size_t
    }
    return count as libc::c_int;
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
/* ssl.c */
#[no_mangle]
#[src_loc = "1123:1"]
pub unsafe extern "C" fn cancellable_recv(mut vpninfo: *mut openconnect_info,
                                          mut fd: libc::c_int,
                                          mut buf: *mut libc::c_char,
                                          mut len: size_t) -> libc::c_int {
    let mut count: size_t = 0;
    if fd == -1i32 { return -22i32 }
    count = 0i32 as size_t;
    while count < len {
        let mut rd_set: fd_set = fd_set{fds_bits: [0; 32],};
        let mut maxfd: libc::c_int = fd;
        let mut i: libc::c_int = 0;
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
                as __int32_t;
        cmd_fd_set(vpninfo, &mut rd_set, &mut maxfd);
        select(maxfd + 1i32, &mut rd_set, 0 as *mut fd_set, 0 as *mut fd_set,
               0 as *mut timeval);
        if is_cancel_pending(vpninfo, &mut rd_set) != 0 { return -4i32 }
        /* Not that this should ever be able to happen... */
        if __darwin_fd_isset(fd, &mut rd_set) == 0 { continue ; }
        i =
            recv(fd,
                 &mut *buf.offset(count as isize) as *mut libc::c_char as
                     *mut libc::c_void, len.wrapping_sub(count), 0i32) as
                libc::c_int;
        if i < 0i32 {
            return -*__error()
        } else { if i == 0i32 { return -54i32 } }
        count =
            (count as libc::c_ulong).wrapping_add(i as libc::c_ulong) as
                size_t as size_t
    }
    return count as libc::c_int;
}
