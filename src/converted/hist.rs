use libc;
#[header_src = "/usr/local/Cellar/llvm/7.0.1/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
}
#[header_src = "/usr/include/_types/_uint8_t.h"]
pub mod _uint8_t_h {
    pub type uint8_t = libc::c_uchar;
}
#[header_src = "/usr/include/_types/_uint32_t.h"]
pub mod _uint32_t_h {
    pub type uint32_t = libc::c_uint;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/mem.h"]
pub mod mem_h {
    /*-**************************************************************
     *  Basic Types
     *****************************************************************/
    /* C99 */
    pub type BYTE = uint8_t;
    pub type U32 = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    pub struct unalign32 {
        pub v: U32,
    }
    use super::_uint32_t_h::uint32_t;
    use super::_uint8_t_h::uint8_t;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/error_public.h"]
pub mod error_public_h {
    /* ******************************************************************
       Error codes list
       Copyright (C) 2016, Yann Collet

       BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)

       Redistribution and use in source and binary forms, with or without
       modification, are permitted provided that the following conditions are
       met:

           * Redistributions of source code must retain the above copyright
       notice, this list of conditions and the following disclaimer.
           * Redistributions in binary form must reproduce the above
       copyright notice, this list of conditions and the following disclaimer
       in the documentation and/or other materials provided with the
       distribution.

       THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
       "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
       LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
       A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
       OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
       SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
       LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
       DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
       THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
       (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
       OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

       You can contact the author at :
       - Source repository : https://github.com/Cyan4973/FiniteStateEntropy
       - Public forum : https://groups.google.com/forum/#!forum/lz4c
    ****************************************************************** */
    /* *****************************************
     *  error codes list
     ******************************************/
    pub type unnamed = libc::c_uint;
    pub const FSE_error_maxCode: unnamed = 9;
    pub const FSE_error_workSpace_tooSmall: unnamed = 8;
    pub const FSE_error_maxSymbolValue_tooSmall: unnamed = 7;
    pub const FSE_error_maxSymbolValue_tooLarge: unnamed = 6;
    pub const FSE_error_tableLog_tooLarge: unnamed = 5;
    pub const FSE_error_corruption_detected: unnamed = 4;
    pub const FSE_error_srcSize_wrong: unnamed = 3;
    pub const FSE_error_dstSize_tooSmall: unnamed = 2;
    pub const FSE_error_GENERIC: unnamed = 1;
    pub const FSE_error_no_error: unnamed = 0;
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[header_src = "/usr/include/assert.h"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        pub fn __assert_rtn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *const libc::c_char,
        ) -> !;
    }
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/error_private.h"]
pub mod error_private_h {}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/hist.h"]
pub mod hist_h {
    use super::stddef_h::size_t;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/hist.c"]
pub mod hist_c {}
use self::_uint32_t_h::uint32_t;
use self::_uint8_t_h::uint8_t;
use self::assert_h::__assert_rtn;
use self::error_public_h::{
    unnamed, FSE_error_GENERIC, FSE_error_corruption_detected, FSE_error_dstSize_tooSmall,
    FSE_error_maxCode, FSE_error_maxSymbolValue_tooLarge, FSE_error_maxSymbolValue_tooSmall,
    FSE_error_no_error, FSE_error_srcSize_wrong, FSE_error_tableLog_tooLarge,
    FSE_error_workSpace_tooSmall,
};
use self::mem_h::{unalign32, BYTE, U32};
use self::stddef_h::size_t;
use self::string_h::memset;
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign32)).v;
}
/*-****************************************
*  Error codes handling
******************************************/
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(FSE_error_maxCode as libc::c_int) as size_t) as libc::c_int as libc::c_uint;
}
/* ******************************************************************
   hist : Histogram functions
   part of Finite State Entropy project
   Copyright (C) 2013-present, Yann Collet.

   BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions are
   met:

       * Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
       * Redistributions in binary form must reproduce the above
   copyright notice, this list of conditions and the following disclaimer
   in the documentation and/or other materials provided with the
   distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
   OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
   DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
   THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
   (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

    You can contact the author at :
    - FSE source repository : https://github.com/Cyan4973/FiniteStateEntropy
    - Public forum : https://groups.google.com/forum/#!forum/lz4c
****************************************************************** */
/* --- dependencies --- */
/* size_t */
/* --- simple histogram functions --- */
/* ! HIST_count():
 *  Provides the precise count of each byte within a table 'count'.
 *  'count' is a table of unsigned int, of minimum size (*maxSymbolValuePtr+1).
 *  Updates *maxSymbolValuePtr with actual largest symbol value detected.
 *  @return : count of the most frequent symbol (which isn't identified).
 *            or an error code, which can be tested using HIST_isError().
 *            note : if return == srcSize, there is only one symbol.
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_count(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut tmpCounters: [libc::c_uint; 1024] = [0; 1024];
    return HIST_count_wksp(
        count,
        maxSymbolValuePtr,
        src,
        srcSize,
        tmpCounters.as_mut_ptr(),
    );
}
/* --- advanced histogram functions --- */
/* * HIST_count_wksp() :
 *  Same as HIST_count(), but using an externally provided scratch buffer.
 *  Benefit is this function will use very little stack space.
 * `workSpace` must be a table of unsigned of size >= HIST_WKSP_SIZE_U32
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_count_wksp(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut source: *const libc::c_void,
    mut sourceSize: size_t,
    mut workSpace: *mut libc::c_uint,
) -> size_t {
    if *maxSymbolValuePtr < 255i32 as libc::c_uint {
        return HIST_count_parallel_wksp(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            1i32 as libc::c_uint,
            workSpace,
        );
    }
    *maxSymbolValuePtr = 255i32 as libc::c_uint;
    return HIST_countFast_wksp(count, maxSymbolValuePtr, source, sourceSize, workSpace);
}
/* * HIST_countFast_wksp() :
 *  Same as HIST_countFast(), but using an externally provided scratch buffer.
 * `workSpace` must be a table of unsigned of size >= HIST_WKSP_SIZE_U32
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_countFast_wksp(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut source: *const libc::c_void,
    mut sourceSize: size_t,
    mut workSpace: *mut libc::c_uint,
) -> size_t {
    if sourceSize < 1500i32 as libc::c_ulong {
        return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize) as size_t;
    }
    return HIST_count_parallel_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        0i32 as libc::c_uint,
        workSpace,
    );
}
/* HIST_count_parallel_wksp() :
 * store histogram into 4 intermediate tables, recombined at the end.
 * this design makes better use of OoO cpus,
 * and is noticeably faster when some values are heavily repeated.
 * But it needs some additional workspace for intermediate tables.
 * `workSpace` size must be a table of size >= HIST_WKSP_SIZE_U32.
 * @return : largest histogram frequency,
 *           or an error code (notably when histogram would be larger than *maxSymbolValuePtr). */
unsafe extern "C" fn HIST_count_parallel_wksp(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut source: *const libc::c_void,
    mut sourceSize: size_t,
    mut checkMax: libc::c_uint,
    workSpace: *mut libc::c_uint,
) -> size_t {
    let mut ip: *const BYTE = source as *const BYTE;
    let iend: *const BYTE = ip.offset(sourceSize as isize);
    let mut maxSymbolValue: libc::c_uint = *maxSymbolValuePtr;
    let mut max: libc::c_uint = 0i32 as libc::c_uint;
    let Counting1: *mut U32 = workSpace;
    let Counting2: *mut U32 = Counting1.offset(256isize);
    let Counting3: *mut U32 = Counting2.offset(256isize);
    let Counting4: *mut U32 = Counting3.offset(256isize);
    memset(
        workSpace as *mut libc::c_void,
        0i32,
        ((4i32 * 256i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    if 0 == sourceSize {
        memset(
            count as *mut libc::c_void,
            0i32,
            maxSymbolValue.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong,
        );
        *maxSymbolValuePtr = 0i32 as libc::c_uint;
        return 0i32 as size_t;
    }
    if 0 == maxSymbolValue {
        maxSymbolValue = 255i32 as libc::c_uint
    }
    let mut cached: U32 = MEM_read32(ip as *const libc::c_void);
    ip = ip.offset(4isize);
    while ip < iend.offset(-15isize) {
        let mut c: U32 = cached;
        cached = MEM_read32(ip as *const libc::c_void);
        ip = ip.offset(4isize);
        let ref mut fresh0 = *Counting1.offset(c as BYTE as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        let ref mut fresh1 = *Counting2.offset((c >> 8i32) as BYTE as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        let ref mut fresh2 = *Counting3.offset((c >> 16i32) as BYTE as isize);
        *fresh2 = (*fresh2).wrapping_add(1);
        let ref mut fresh3 = *Counting4.offset((c >> 24i32) as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        c = cached;
        cached = MEM_read32(ip as *const libc::c_void);
        ip = ip.offset(4isize);
        let ref mut fresh4 = *Counting1.offset(c as BYTE as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        let ref mut fresh5 = *Counting2.offset((c >> 8i32) as BYTE as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        let ref mut fresh6 = *Counting3.offset((c >> 16i32) as BYTE as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        let ref mut fresh7 = *Counting4.offset((c >> 24i32) as isize);
        *fresh7 = (*fresh7).wrapping_add(1);
        c = cached;
        cached = MEM_read32(ip as *const libc::c_void);
        ip = ip.offset(4isize);
        let ref mut fresh8 = *Counting1.offset(c as BYTE as isize);
        *fresh8 = (*fresh8).wrapping_add(1);
        let ref mut fresh9 = *Counting2.offset((c >> 8i32) as BYTE as isize);
        *fresh9 = (*fresh9).wrapping_add(1);
        let ref mut fresh10 = *Counting3.offset((c >> 16i32) as BYTE as isize);
        *fresh10 = (*fresh10).wrapping_add(1);
        let ref mut fresh11 = *Counting4.offset((c >> 24i32) as isize);
        *fresh11 = (*fresh11).wrapping_add(1);
        c = cached;
        cached = MEM_read32(ip as *const libc::c_void);
        ip = ip.offset(4isize);
        let ref mut fresh12 = *Counting1.offset(c as BYTE as isize);
        *fresh12 = (*fresh12).wrapping_add(1);
        let ref mut fresh13 = *Counting2.offset((c >> 8i32) as BYTE as isize);
        *fresh13 = (*fresh13).wrapping_add(1);
        let ref mut fresh14 = *Counting3.offset((c >> 16i32) as BYTE as isize);
        *fresh14 = (*fresh14).wrapping_add(1);
        let ref mut fresh15 = *Counting4.offset((c >> 24i32) as isize);
        *fresh15 = (*fresh15).wrapping_add(1)
    }
    ip = ip.offset(-4isize);
    while ip < iend {
        let fresh16 = ip;
        ip = ip.offset(1);
        let ref mut fresh17 = *Counting1.offset(*fresh16 as isize);
        *fresh17 = (*fresh17).wrapping_add(1)
    }
    if 0 != checkMax {
        let mut s: U32 = 0;
        s = 255i32 as U32;
        while s > maxSymbolValue {
            let ref mut fresh18 = *Counting1.offset(s as isize);
            *fresh18 = (*fresh18 as libc::c_uint).wrapping_add(
                (*Counting2.offset(s as isize))
                    .wrapping_add(*Counting3.offset(s as isize))
                    .wrapping_add(*Counting4.offset(s as isize)),
            ) as U32 as U32;
            if 0 != *Counting1.offset(s as isize) {
                return -(FSE_error_maxSymbolValue_tooSmall as libc::c_int) as size_t;
            }
            s = s.wrapping_sub(1)
        }
    }
    let mut s_0: U32 = 0;
    if maxSymbolValue > 255i32 as libc::c_uint {
        maxSymbolValue = 255i32 as libc::c_uint
    }
    s_0 = 0i32 as U32;
    while s_0 <= maxSymbolValue {
        *count.offset(s_0 as isize) = (*Counting1.offset(s_0 as isize))
            .wrapping_add(*Counting2.offset(s_0 as isize))
            .wrapping_add(*Counting3.offset(s_0 as isize))
            .wrapping_add(*Counting4.offset(s_0 as isize));
        if *count.offset(s_0 as isize) > max {
            max = *count.offset(s_0 as isize)
        }
        s_0 = s_0.wrapping_add(1)
    }
    while 0 == *count.offset(maxSymbolValue as isize) {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1)
    }
    *maxSymbolValuePtr = maxSymbolValue;
    return max as size_t;
}
/* ! HIST_count_simple() :
 *  Same as HIST_countFast(), this function is unsafe,
 *  and will segfault if any value within `src` is `> *maxSymbolValuePtr`.
 *  It is also a bit slower for large inputs.
 *  However, it does not need any additional memory (not even on stack).
 * @return : count of the most frequent symbol.
 *  Note this function doesn't produce any error (i.e. it must succeed).
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_count_simple(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> libc::c_uint {
    let mut ip: *const BYTE = src as *const BYTE;
    let end: *const BYTE = ip.offset(srcSize as isize);
    let mut maxSymbolValue: libc::c_uint = *maxSymbolValuePtr;
    let mut largestCount: libc::c_uint = 0i32 as libc::c_uint;
    memset(
        count as *mut libc::c_void,
        0i32,
        (maxSymbolValue.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    if srcSize == 0i32 as libc::c_ulong {
        *maxSymbolValuePtr = 0i32 as libc::c_uint;
        return 0i32 as libc::c_uint;
    }
    while ip < end {
        if 0 != !(*ip as libc::c_uint <= maxSymbolValue) as libc::c_int as libc::c_long {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                    b"HIST_count_simple\x00",
                ))
                .as_ptr(),
                b"../lib/hist.c\x00" as *const u8 as *const libc::c_char,
                61i32,
                b"*ip <= maxSymbolValue\x00" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        let fresh19 = ip;
        ip = ip.offset(1);
        let ref mut fresh20 = *count.offset(*fresh19 as isize);
        *fresh20 = (*fresh20).wrapping_add(1)
    }
    while 0 == *count.offset(maxSymbolValue as isize) {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1)
    }
    *maxSymbolValuePtr = maxSymbolValue;
    let mut s: U32 = 0;
    s = 0i32 as U32;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) > largestCount {
            largestCount = *count.offset(s as isize)
        }
        s = s.wrapping_add(1)
    }
    return largestCount;
}
/*< tells if a return value is an error code */
#[no_mangle]
pub unsafe extern "C" fn HIST_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
/* * HIST_countFast() :
 *  same as HIST_count(), but blindly trusts that all byte values within src are <= *maxSymbolValuePtr.
 *  This function is unsafe, and will segfault if any value within `src` is `> *maxSymbolValuePtr`
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_countFast(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut source: *const libc::c_void,
    mut sourceSize: size_t,
) -> size_t {
    let mut tmpCounters: [libc::c_uint; 1024] = [0; 1024];
    return HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        tmpCounters.as_mut_ptr(),
    );
}
