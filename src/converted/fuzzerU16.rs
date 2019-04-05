#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn getchar() -> libc::c_int;
    #[no_mangle]
    fn ftime(_: *mut timeb) -> libc::c_int;
    /* Error Management */
    #[no_mangle]
    fn FSE_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn FSE_getErrorName(code: size_t) -> *const libc::c_char;
    /* ******************************************************************
   FSEU16 : Finite State Entropy coder for 16-bits input
   header file
   Copyright (C) 2013-2016, Yann Collet.

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
    /*-*****************************************
*  Tuning parameters
*******************************************/
/* FSE_MAX_SYMBOL_VALUE :
*  Maximum nb of symbol values authorized.
*  Required for allocation purposes */
    /* This is just an example, typical value for zlib */
    /*-*****************************************
*  Includes
*******************************************/
    /* size_t, ptrdiff_t */
    /* *****************************************
*  FSE U16 functions
*******************************************/
    /* !FSE_compressU16() :
   data is presented or regenerated as a table of unsigned short (2 bytes per symbol),
   which is useful for alphabet size > 256.
   Important ! All symbol values within input table must be <= 'maxSymbolValue'.
   Maximum allowed 'maxSymbolValue' is controlled by constant FSE_MAX_SYMBOL_VALUE
   Special values : if result == 0, data is not compressible => Nothing is stored within cSrc !!
                    if result == 1, data is one constant element x srcSize times. Use RLE compression.
                    if FSE_isError(result), it's an error code.*/
    #[no_mangle]
    fn FSE_compressU16(dst: *mut libc::c_void, dstCapacity: size_t,
                       src: *const libc::c_ushort, srcSize: size_t,
                       maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> size_t;
    #[no_mangle]
    fn FSE_decompressU16(dst: *mut libc::c_ushort, dstCapacity: size_t,
                         cSrc: *const libc::c_void, cSrcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn XXH64(input: *const libc::c_void, length: size_t,
             seed: libc::c_ulonglong) -> XXH64_hash_t;
    /*-***************************************************************
*  Unitary tests
*****************************************************************/
    #[no_mangle]
    fn FSE_countU16(count: *mut libc::c_uint,
                    maxSymbolValuePtr: *mut libc::c_uint,
                    source: *const libc::c_ushort, sourceSize: size_t)
     -> size_t;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type fpos_t = __darwin_off_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive ( Copy , Clone )]
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
pub type FILE = __sFILE;
pub type time_t = __darwin_time_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timeb {
    pub time: time_t,
    pub millitm: libc::c_ushort,
    pub timezone: libc::c_short,
    pub dstflag: libc::c_short,
}
pub type XXH64_hash_t = libc::c_ulonglong;
pub type U16 = uint16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
/*-****************************
*  Constants
******************************/
/*-*************************************************
*  Macros
***************************************************/
/* 0: no display;  1: errors;  2: + result + interaction + warnings;  3: + progression;  4: + information */
static mut displayLevel: libc::c_uint = 2i32 as libc::c_uint;
/*-*************************************************
*  Local functions
***************************************************/
unsafe extern "C" fn FUZ_GetMilliStart() -> libc::c_int {
    let mut tb: timeb = timeb{time: 0, millitm: 0, timezone: 0, dstflag: 0,};
    let mut nCount: libc::c_int = 0;
    ftime(&mut tb);
    nCount =
        (tb.millitm as libc::c_long +
             (tb.time & 0xfffffi32 as libc::c_long) * 1000i32 as libc::c_long)
            as libc::c_int;
    return nCount;
}
unsafe extern "C" fn FUZ_GetMilliSpan(mut nTimeStart: libc::c_int)
 -> libc::c_int {
    let mut nSpan: libc::c_int = FUZ_GetMilliStart() - nTimeStart;
    if nSpan < 0i32 { nSpan += 0x100000i32 * 1000i32 }
    return nSpan;
}
unsafe extern "C" fn FUZ_rand(mut src: *mut libc::c_uint) -> libc::c_uint {
    *src = (*src).wrapping_mul(2654435761u32).wrapping_add(2246822519u32);
    return *src >> 11i32;
}
unsafe extern "C" fn generateU16(mut buffer: *mut U16, mut buffSize: size_t,
                                 mut start: U16, mut p: libc::c_double,
                                 mut seedSrc: U32) {
    let mut tableU16: [U16; 4096] = [0; 4096];
    let mut remaining: U32 = (4i32 * (1i32 << 10i32)) as U32;
    let mut pos: U32 = 0i32 as U32;
    let mut op: *mut U16 = buffer;
    let oend: *mut U16 = op.offset(buffSize as isize);
    let mut val16: U16 = start;
    let mut max16: U16 = 286i32 as U16;
    let mut seed: U32 = seedSrc;
    while 0 != remaining {
        let n: U32 =
            ((remaining as libc::c_double * p) as
                 U32).wrapping_add(1i32 as libc::c_uint);
        let end: U32 = pos.wrapping_add(n);
        while pos < end {
            let fresh0 = pos;
            pos = pos.wrapping_add(1);
            tableU16[fresh0 as usize] = val16
        }
        val16 = val16.wrapping_add(1);
        if val16 as libc::c_int >= max16 as libc::c_int {
            val16 = 1i32 as U16
        }
        remaining = (remaining as libc::c_uint).wrapping_sub(n) as U32 as U32
    }
    while op < oend {
        let v16: U32 =
            FUZ_rand(&mut seed) &
                (4i32 * (1i32 << 10i32) - 1i32) as libc::c_uint;
        let fresh1 = op;
        op = op.offset(1);
        *fresh1 = tableU16[v16 as usize]
    };
}
unsafe extern "C" fn FUZ_tests(startSeed: U32, mut totalTest: U32,
                               mut startTestNb: U32) {
    let bufferDstSize: size_t =
        ((1i32 * (1i32 << 20i32) - 1i32) as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<U16>() as
                                             libc::c_ulong).wrapping_add(64i32
                                                                             as
                                                                             libc::c_ulong);
    let bufferP8: *mut U16 = malloc(bufferDstSize) as *mut U16;
    let bufferP80: *mut U16 = malloc(bufferDstSize) as *mut U16;
    let bufferDst: *mut libc::c_void = malloc(bufferDstSize);
    let bufferVerif: *mut U16 = malloc(bufferDstSize) as *mut U16;
    let maxTestSizeMask: size_t = 0x1ffffi32 as size_t;
    let mut time: U32 = FUZ_GetMilliStart() as U32;
    let mut seed: U32 = startSeed;
    let mut testNb: libc::c_uint = 0;
    if bufferP8.is_null() || bufferP80.is_null() || bufferDst.is_null() ||
           bufferVerif.is_null() {
        fprintf(__stderrp,
                b"memory allocation error \n\x00" as *const u8 as
                    *const libc::c_char);
        exit(1i32);
    }
    generateU16(bufferP8, (1i32 * (1i32 << 20i32) - 1i32) as size_t,
                240i32 as U16, 0.08f64, seed);
    generateU16(bufferP80, (1i32 * (1i32 << 20i32) - 1i32) as size_t,
                257i32 as U16, 0.80f64,
                seed.wrapping_add(1i32 as libc::c_uint));
    if 0 != startTestNb {
        let mut u: U32 = 0;
        u = 0i32 as U32;
        while u < startTestNb { FUZ_rand(&mut seed); u = u.wrapping_add(1) }
    }
    testNb = startTestNb;
    while testNb < totalTest {
        let mut tag: libc::c_int = 0i32;
        let mut roundSeed: U32 = seed ^ 0xeda5b371u32;
        FUZ_rand(&mut seed);
        if displayLevel >= 4i32 as libc::c_uint {
            fprintf(__stderrp,
                    b"\r test %5u      \x00" as *const u8 as
                        *const libc::c_char, testNb);
        }
        if FUZ_GetMilliSpan(time as libc::c_int) > 200i32 {
            fprintf(__stderrp,
                    b"\r test %5u      \x00" as *const u8 as
                        *const libc::c_char, testNb);
            time = FUZ_GetMilliStart() as U32
        }
        let sizeOrig: size_t =
            (FUZ_rand(&mut roundSeed) as libc::c_ulong &
                 maxTestSizeMask).wrapping_add(1i32 as libc::c_ulong);
        let offset: size_t =
            (FUZ_rand(&mut roundSeed) as
                 libc::c_ulong).wrapping_rem(((1i32 * (1i32 << 20i32) - 1i32 -
                                                   64i32) as
                                                  libc::c_ulong).wrapping_sub(maxTestSizeMask));
        let bufferSrc: *const U16 =
            if 0 != FUZ_rand(&mut roundSeed) & 0x1ffi32 as libc::c_uint {
                bufferP8
            } else { bufferP80 };
        let bufferTest: *const U16 = bufferSrc.offset(offset as isize);
        let hashOrig: U64 =
            XXH64(bufferTest as *const libc::c_void,
                  sizeOrig.wrapping_mul(::std::mem::size_of::<U16>() as
                                            libc::c_ulong),
                  0i32 as libc::c_ulonglong);
        let sizeCompressed: size_t =
            FSE_compressU16(bufferDst, bufferDstSize, bufferTest, sizeOrig,
                            286i32 as libc::c_uint, 12i32 as libc::c_uint);
        if 0 != FSE_isError(sizeCompressed) {
            fprintf(__stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char);
            fprintf(__stderrp,
                    b"\r test %5u : FSE_compressU16 failed !\x00" as *const u8
                        as *const libc::c_char, testNb);
            fprintf(__stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                        *const libc::c_char, startSeed, testNb);
            exit(-1i32);
        }
        if displayLevel >= 4i32 as libc::c_uint {
            let fresh2 = tag;
            tag = tag + 1;
            fprintf(__stderrp,
                    b"\x08\x08\x08\x08%3i \x00" as *const u8 as
                        *const libc::c_char, fresh2);
        }
        if sizeCompressed > 1i32 as libc::c_ulong {
            let guardValue: U16 = (1024i32 + 250i32) as U16;
            if displayLevel >= 4i32 as libc::c_uint {
                let fresh3 = tag;
                tag = tag + 1;
                fprintf(__stderrp,
                        b"\x08\x08\x08\x08%3i \x00" as *const u8 as
                            *const libc::c_char, fresh3);
            }
            *bufferVerif.offset(sizeOrig as isize) = guardValue;
            let dSize: size_t =
                FSE_decompressU16(bufferVerif, sizeOrig, bufferDst,
                                  sizeCompressed);
            if *bufferVerif.offset(sizeOrig as isize) as libc::c_int !=
                   guardValue as libc::c_int {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"\r test %5u : FSE_decompressU16 overrun output buffer (write beyond specified end) !\x00"
                            as *const u8 as *const libc::c_char, testNb);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, startSeed, testNb);
                exit(-1i32);
            }
            if 0 != FSE_isError(dSize) {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"\r test %5u : FSE_decompressU16 failed : %s ! (origSize = %u shorts, cSize = %u bytes)\x00"
                            as *const u8 as *const libc::c_char, testNb,
                        FSE_getErrorName(dSize), sizeOrig as U32,
                        sizeCompressed as U32);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, startSeed, testNb);
                exit(-1i32);
            }
            let hashEnd: U64 =
                XXH64(bufferVerif as *const libc::c_void,
                      dSize.wrapping_mul(::std::mem::size_of::<U16>() as
                                             libc::c_ulong),
                      0i32 as libc::c_ulonglong);
            if hashEnd != hashOrig {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"\r test %5u : Decompressed data corrupted !!\x00" as
                            *const u8 as *const libc::c_char, testNb);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, startSeed, testNb);
                exit(-1i32);
            }
            if displayLevel >= 4i32 as libc::c_uint {
                let fresh4 = tag;
                tag = tag + 1;
                fprintf(__stderrp,
                        b"\x08\x08\x08\x08%3i \x00" as *const u8 as
                            *const libc::c_char, fresh4);
            }
            let dSize_0: size_t =
                FSE_decompressU16(bufferVerif,
                                  sizeOrig.wrapping_add((FUZ_rand(&mut roundSeed)
                                                             &
                                                             31i32 as
                                                                 libc::c_uint)
                                                            as
                                                            libc::c_ulong).wrapping_add(1i32
                                                                                            as
                                                                                            libc::c_ulong),
                                  bufferDst, sizeCompressed);
            if 0 != FSE_isError(dSize_0) {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"\r test %5u : FSE_decompressU16 failed : %s ! (origSize = %u shorts, cSize = %u bytes)\x00"
                            as *const u8 as *const libc::c_char, testNb,
                        FSE_getErrorName(dSize_0), sizeOrig as U32,
                        sizeCompressed as U32);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, startSeed, testNb);
                exit(-1i32);
            }
            let hashEnd_0: U64 =
                XXH64(bufferVerif as *const libc::c_void,
                      dSize_0.wrapping_mul(::std::mem::size_of::<U16>() as
                                               libc::c_ulong),
                      0i32 as libc::c_ulonglong);
            if hashEnd_0 != hashOrig {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"\r test %5u : Decompressed data corrupted !!\x00" as
                            *const u8 as *const libc::c_char, testNb);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, startSeed, testNb);
                exit(-1i32);
            }
            if displayLevel >= 4i32 as libc::c_uint {
                let fresh5 = tag;
                tag = tag + 1;
                fprintf(__stderrp,
                        b"\x08\x08\x08\x08%3i \x00" as *const u8 as
                            *const libc::c_char, fresh5);
            }
            let missing: size_t =
                (FUZ_rand(&mut roundSeed) &
                     31i32 as libc::c_uint).wrapping_add(1i32 as libc::c_uint)
                    as size_t;
            let missing_fixed: size_t =
                if missing >= sizeOrig {
                    1i32 as libc::c_ulong
                } else { missing };
            let dstSize: size_t = sizeOrig.wrapping_sub(missing_fixed);
            *bufferVerif.offset(dstSize as isize) = guardValue;
            let dSize_1: size_t =
                FSE_decompressU16(bufferVerif, dstSize, bufferDst,
                                  sizeCompressed);
            if *bufferVerif.offset(dstSize as isize) as libc::c_int !=
                   guardValue as libc::c_int {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"\r test %5u : FSE_decompressU16 overrun output buffer (write beyond specified end) !\x00"
                            as *const u8 as *const libc::c_char, testNb);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, startSeed, testNb);
                exit(-1i32);
            }
            if 0 == FSE_isError(dSize_1) {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"\r test %5u : FSE_decompressU16 should have failed ! (origSize = %u shorts, dstSize = %u bytes)\x00"
                            as *const u8 as *const libc::c_char, testNb,
                        sizeOrig as U32, dstSize as U32);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, startSeed, testNb);
                exit(-1i32);
            }
        }
        testNb = testNb.wrapping_add(1)
    }
    free(bufferP8 as *mut libc::c_void);
    free(bufferP80 as *mut libc::c_void);
    free(bufferDst);
    free(bufferVerif as *mut libc::c_void);
}
unsafe extern "C" fn unitTest() {
    let mut testBuffU16: [U16; 16384] = [0; 16384];
    /* just to re-use CHECK */
    let mut startSeed: U32 = 0i32 as U32;
    let mut testNb: U32 = 0i32 as U32;
    /* FSE_countU16 */
    let mut table: [U32; 288] = [0; 288];
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < (16i32 * (1i32 << 10i32)) as libc::c_uint {
        testBuffU16[u as usize] =
            u.wrapping_rem((286i32 + 1i32) as libc::c_uint) as U16;
        u = u.wrapping_add(1)
    }
    let mut max: U32 = 286i32 as U32;
    let errC: size_t =
        FSE_countU16(table.as_mut_ptr(), &mut max, testBuffU16.as_mut_ptr(),
                     (16i32 * (1i32 << 10i32)) as size_t);
    if 0 != FSE_isError(errC) {
        fprintf(__stderrp,
                b"Error => \x00" as *const u8 as *const libc::c_char);
        fprintf(__stderrp,
                b"FSE_countU16() should have worked\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(__stderrp,
                b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                    *const libc::c_char, startSeed, testNb);
        exit(-1i32);
    }
    let mut max_0: U32 = (286i32 - 1i32) as U32;
    let errC_0: size_t =
        FSE_countU16(table.as_mut_ptr(), &mut max_0, testBuffU16.as_mut_ptr(),
                     (16i32 * (1i32 << 10i32)) as size_t);
    if 0 == FSE_isError(errC_0) {
        fprintf(__stderrp,
                b"Error => \x00" as *const u8 as *const libc::c_char);
        fprintf(__stderrp,
                b"FSE_countU16() should have failed : max too low\x00" as
                    *const u8 as *const libc::c_char);
        fprintf(__stderrp,
                b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                    *const libc::c_char, startSeed, testNb);
        exit(-1i32);
    }
    fprintf(__stderrp,
            b"Unit tests completed\n\x00" as *const u8 as
                *const libc::c_char);
}
/* ****************************************************************
*  Command line
*****************************************************************/
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    let mut startTestNb: U32 = 0i32 as U32;
    let mut pause: U32 = 0i32 as U32;
    let mut totalTest: U32 = (32i32 * (1i32 << 10i32)) as U32;
    let mut argNb: libc::c_int = 0;
    let mut seed: U32 = (FUZ_GetMilliStart() % 10000i32) as U32;
    if displayLevel >= 1i32 as libc::c_uint {
        fprintf(__stderrp,
                b"FSE U16 (%2i bits) automated test\n\x00" as *const u8 as
                    *const libc::c_char,
                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as
                    libc::c_int * 8i32);
    }
    argNb = 1i32;
    while argNb < argc {
        let mut argument: *const libc::c_char = *argv.offset(argNb as isize);
        if *argument.offset(0isize) as libc::c_int == '-' as i32 {
            argument = argument.offset(1isize);
            while *argument as libc::c_int != 0i32 {
                match *argument as libc::c_int {
                    115 => {
                        argument = argument.offset(1isize);
                        seed = 0i32 as U32;
                        while *argument as libc::c_int >= '0' as i32 &&
                                  *argument as libc::c_int <= '9' as i32 {
                            seed =
                                (seed as
                                     libc::c_uint).wrapping_mul(10i32 as
                                                                    libc::c_uint)
                                    as U32 as U32;
                            seed =
                                (seed as
                                     libc::c_uint).wrapping_add((*argument as
                                                                     libc::c_int
                                                                     -
                                                                     '0' as
                                                                         i32)
                                                                    as
                                                                    libc::c_uint)
                                    as U32 as U32;
                            argument = argument.offset(1isize)
                        }
                    }
                    105 => {
                        argument = argument.offset(1isize);
                        totalTest = 0i32 as U32;
                        while *argument as libc::c_int >= '0' as i32 &&
                                  *argument as libc::c_int <= '9' as i32 {
                            totalTest =
                                (totalTest as
                                     libc::c_uint).wrapping_mul(10i32 as
                                                                    libc::c_uint)
                                    as U32 as U32;
                            totalTest =
                                (totalTest as
                                     libc::c_uint).wrapping_add((*argument as
                                                                     libc::c_int
                                                                     -
                                                                     '0' as
                                                                         i32)
                                                                    as
                                                                    libc::c_uint)
                                    as U32 as U32;
                            argument = argument.offset(1isize)
                        }
                    }
                    116 => {
                        argument = argument.offset(1isize);
                        startTestNb = 0i32 as U32;
                        while *argument as libc::c_int >= '0' as i32 &&
                                  *argument as libc::c_int <= '9' as i32 {
                            startTestNb =
                                (startTestNb as
                                     libc::c_uint).wrapping_mul(10i32 as
                                                                    libc::c_uint)
                                    as U32 as U32;
                            startTestNb =
                                (startTestNb as
                                     libc::c_uint).wrapping_add((*argument as
                                                                     libc::c_int
                                                                     -
                                                                     '0' as
                                                                         i32)
                                                                    as
                                                                    libc::c_uint)
                                    as U32 as U32;
                            argument = argument.offset(1isize)
                        }
                    }
                    118 => {
                        argument = argument.offset(1isize);
                        displayLevel = 4i32 as libc::c_uint
                    }
                    112 => {
                        argument = argument.offset(1isize);
                        pause = 1i32 as U32
                    }
                    _ => { }
                }
            }
        }
        argNb += 1
    }
    unitTest();
    fprintf(__stderrp,
            b"Fuzzer seed : %u \n\x00" as *const u8 as *const libc::c_char,
            seed);
    FUZ_tests(seed, totalTest, startTestNb);
    fprintf(__stderrp,
            b"\rAll %u tests passed               \n\x00" as *const u8 as
                *const libc::c_char, totalTest);
    if 0 != pause {
        fprintf(__stderrp,
                b"press enter ...\n\x00" as *const u8 as *const libc::c_char);
        getchar();
    }
    return 0i32;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *const libc::c_char) as i32)
    }
}