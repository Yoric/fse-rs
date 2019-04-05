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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ftime(_: *mut timeb) -> libc::c_int;
    /* ******************************************************************
   huff0 huffman codec,
   part of Finite State Entropy library
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
   - Source repository : https://github.com/Cyan4973/FiniteStateEntropy
****************************************************************** */
    /* *** Dependencies *** */
    /* size_t */
    /* *** library symbols visibility *** */
/* Note : when linking with -fvisibility=hidden on gcc, or by default on Visual,
 *        HUF symbols remain "private" (internal symbols for library only).
 *        Set macro FSE_DLL_EXPORT to 1 if you want HUF symbols visible on DLL interface */
    /* Visual expected */
    /* ========================== */
/* ***  simple functions  *** */
/* ========================== */
    /* * HUF_compress() :
 *  Compress content from buffer 'src', of size 'srcSize', into buffer 'dst'.
 * 'dst' buffer must be already allocated.
 *  Compression runs faster if `dstCapacity` >= HUF_compressBound(srcSize).
 * `srcSize` must be <= `HUF_BLOCKSIZE_MAX` == 128 KB.
 * @return : size of compressed data (<= `dstCapacity`).
 *  Special values : if return == 0, srcData is not compressible => Nothing is stored within dst !!!
 *                   if HUF_isError(return), compression failed (more details using HUF_getErrorName())
 */
    #[no_mangle]
    fn HUF_compress(dst: *mut libc::c_void, dstCapacity: size_t,
                    src: *const libc::c_void, srcSize: size_t) -> size_t;
    /* * HUF_decompress() :
 *  Decompress HUF data from buffer 'cSrc', of size 'cSrcSize',
 *  into already allocated buffer 'dst', of minimum size 'dstSize'.
 * `originalSize` : **must** be the ***exact*** size of original (uncompressed) data.
 *  Note : in contrast with FSE, HUF_decompress can regenerate
 *         RLE (cSrcSize==1) and uncompressed (cSrcSize==dstSize) data,
 *         because it knows size to regenerate (originalSize).
 * @return : size of regenerated data (== originalSize),
 *           or an error code, which can be tested using HUF_isError()
 */
    #[no_mangle]
    fn HUF_decompress(dst: *mut libc::c_void, originalSize: size_t,
                      cSrc: *const libc::c_void, cSrcSize: size_t) -> size_t;
    /* Error Management */
    #[no_mangle]
    fn HUF_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn HUF_getErrorName(code: size_t) -> *const libc::c_char;
    #[no_mangle]
    fn XXH32(input: *const libc::c_void, length: size_t, seed: libc::c_uint)
     -> XXH32_hash_t;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint32_t = libc::c_uint;
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
/*-**************************************************************
*  Basic Types
*****************************************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
/* ****************************
*  Simple Hash Functions
******************************/
pub type XXH32_hash_t = libc::c_uint;
/*
FuzzerHuff0.c
Automated test program for HUF
Copyright (C) Yann Collet 2015

GPL v2 License

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

You can contact the author at :
- FSE+HUF source repository : https://github.com/Cyan4973/FiniteStateEntropy
- Public forum : https://groups.google.com/forum/#!forum/lz4c
*/
/*-****************************
*  Compiler options
******************************/
/* Visual warning */
/*-****************************
*  Dependencies
*******************************/
/* malloc, abs */
/* printf */
/* memset */
/* timeb */
/*-*************************************************
*  Constants
***************************************************/
/*-*************************************************
*  Macros
***************************************************/
/* 0 : no display; 1: errors; 2 : + result + interaction + warnings; 3 : + progression; 4 : + information */
static mut displayLevel: libc::c_uint = 2i32 as libc::c_uint;
/*-*************************************************
*  local functions
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
    static mut prime1: libc::c_uint = 2654435761u32;
    static mut prime2: libc::c_uint = 2246822519u32;
    *src = (*src).wrapping_mul(prime1).wrapping_add(prime2);
    return *src >> 11i32;
}
unsafe extern "C" fn generate(mut buffer: *mut libc::c_void,
                              mut buffSize: size_t, mut p: libc::c_double,
                              mut seed: *mut U32) {
    let mut table: [libc::c_char; 4096] =
        [0i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0];
    let mut remaining: libc::c_int = 4i32 * (1i32 << 10i32);
    let mut pos: libc::c_int = 0i32;
    let mut s: libc::c_int = 0i32;
    let mut op: *mut libc::c_char = buffer as *mut libc::c_char;
    let mut oend: *mut libc::c_char = op.offset(buffSize as isize);
    while 0 != remaining {
        let mut n: libc::c_int =
            (remaining as libc::c_double * p) as libc::c_int;
        let mut end: libc::c_int = 0;
        if 0 == n { n = 1i32 }
        end = pos + n;
        while pos < end {
            let fresh0 = pos;
            pos = pos + 1;
            table[fresh0 as usize] = s as libc::c_char
        }
        s += 1;
        remaining -= n
    }
    while op < oend {
        let fresh1 = op;
        op = op.offset(1);
        *fresh1 =
            table[(FUZ_rand(seed) &
                       (4i32 * (1i32 << 10i32) - 1i32) as libc::c_uint) as
                      usize]
    };
}
unsafe extern "C" fn generateNoise(mut buffer: *mut libc::c_void,
                                   mut buffSize: size_t, mut seed: *mut U32) {
    let mut op: *mut BYTE = buffer as *mut BYTE;
    let oend: *mut BYTE = op.offset(buffSize as isize);
    while op < oend {
        let fresh2 = op;
        op = op.offset(1);
        *fresh2 = FUZ_rand(seed) as BYTE
    };
}
unsafe extern "C" fn findDifferentByte(mut buf1: *const libc::c_void,
                                       mut buf1Size: size_t,
                                       mut buf2: *const libc::c_void,
                                       mut buf2Size: size_t) {
    let maxSize: size_t =
        if buf1Size < buf2Size { buf1Size } else { buf2Size };
    let B1: *const BYTE = buf1 as *const BYTE;
    let B2: *const BYTE = buf2 as *const BYTE;
    let mut n: size_t = 0;
    n = 0i32 as size_t;
    while n < maxSize {
        if *B1.offset(n as isize) as libc::c_int !=
               *B2.offset(n as isize) as libc::c_int {
            break ;
        }
        n = n.wrapping_add(1)
    }
    if n == maxSize {
        fprintf(__stderrp,
                b"No difference found \n\x00" as *const u8 as
                    *const libc::c_char);
        return
    }
    fprintf(__stderrp,
            b"Buffers are different at byte %u / %u : %02X!=%02X\n\x00" as
                *const u8 as *const libc::c_char, n as U32, maxSize as U32,
            *B1.offset(n as isize) as libc::c_int,
            *B2.offset(n as isize) as libc::c_int);
}
unsafe extern "C" fn FUZ_tests(mut seed: U32, mut totalTest: U32,
                               mut startTestNb: U32) {
    let mut bufferP0: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as
            *mut BYTE;
    let mut bufferP1: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as
            *mut BYTE;
    let mut bufferP15: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as
            *mut BYTE;
    let mut bufferP90: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as
            *mut BYTE;
    let mut bufferP100: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as
            *mut BYTE;
    let mut bufferDst: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as
            *mut BYTE;
    let mut bufferVerif: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as
            *mut BYTE;
    let bufferDstSize: size_t =
        (1i32 * (1i32 << 20i32) - 1i32 + 64i32) as size_t;
    let mut testNb: libc::c_uint = 0;
    /* 128 KB - 1 */
    let maxTestSizeMask: size_t = 0x1ffffi32 as size_t;
    let mut rootSeed: U32 = seed;
    let mut time: U32 = FUZ_GetMilliStart() as U32;
    generateNoise(bufferP0 as *mut libc::c_void,
                  (1i32 * (1i32 << 20i32) - 1i32) as size_t, &mut rootSeed);
    generate(bufferP1 as *mut libc::c_void,
             (1i32 * (1i32 << 20i32) - 1i32) as size_t, 0.01f64,
             &mut rootSeed);
    generate(bufferP15 as *mut libc::c_void,
             (1i32 * (1i32 << 20i32) - 1i32) as size_t, 0.15f64,
             &mut rootSeed);
    generate(bufferP90 as *mut libc::c_void,
             (1i32 * (1i32 << 20i32) - 1i32) as size_t, 0.90f64,
             &mut rootSeed);
    memset(bufferP100 as *mut libc::c_void,
           FUZ_rand(&mut rootSeed) as BYTE as libc::c_int,
           (1i32 * (1i32 << 20i32) - 1i32) as libc::c_ulong);
    memset(bufferDst as *mut libc::c_void, 0i32,
           (1i32 * (1i32 << 20i32) - 1i32) as libc::c_ulong);
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < startTestNb { FUZ_rand(&mut rootSeed); u = u.wrapping_add(1) }
    testNb = startTestNb;
    while testNb < totalTest {
        let mut roundSeed: U32 = rootSeed ^ 0xeda5b371u32;
        FUZ_rand(&mut rootSeed);
        let mut tag: libc::c_int = 0i32;
        let mut bufferTest: *mut BYTE = 0 as *mut BYTE;
        if displayLevel >= 4i32 as libc::c_uint {
            fprintf(__stderrp,
                    b"\r test %5u  \x00" as *const u8 as *const libc::c_char,
                    testNb);
        }
        if FUZ_GetMilliSpan(time as libc::c_int) > 200i32 {
            fprintf(__stderrp,
                    b"\r test %5u  \x00" as *const u8 as *const libc::c_char,
                    testNb);
            time = FUZ_GetMilliStart() as U32
        }
        if displayLevel >= 4i32 as libc::c_uint {
            let fresh3 = tag;
            tag = tag + 1;
            fprintf(__stderrp,
                    b"%3i \x00" as *const u8 as *const libc::c_char, fresh3);
        }
        let sizeOrig: size_t =
            (FUZ_rand(&mut roundSeed) as libc::c_ulong &
                 maxTestSizeMask).wrapping_add(1i32 as libc::c_ulong);
        let offset: size_t =
            (FUZ_rand(&mut roundSeed) as
                 libc::c_ulong).wrapping_rem(((1i32 * (1i32 << 20i32) - 1i32 -
                                                   64i32) as
                                                  libc::c_ulong).wrapping_sub(maxTestSizeMask));
        let mut sizeCompressed: size_t = 0;
        let mut hashOrig: U32 = 0;
        if 0 != FUZ_rand(&mut roundSeed) & 7i32 as libc::c_uint {
            bufferTest = bufferP15.offset(offset as isize)
        } else {
            match FUZ_rand(&mut roundSeed) & 3i32 as libc::c_uint {
                0 => { bufferTest = bufferP0.offset(offset as isize) }
                1 => { bufferTest = bufferP1.offset(offset as isize) }
                2 => { bufferTest = bufferP90.offset(offset as isize) }
                _ => { bufferTest = bufferP100.offset(offset as isize) }
            }
        }
        hashOrig =
            XXH32(bufferTest as *const libc::c_void, sizeOrig,
                  0i32 as libc::c_uint);
        sizeCompressed =
            HUF_compress(bufferDst as *mut libc::c_void, bufferDstSize,
                         bufferTest as *const libc::c_void, sizeOrig);
        if 0 != HUF_isError(sizeCompressed) {
            fprintf(__stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char);
            fprintf(__stderrp,
                    b"HUF_compress failed\x00" as *const u8 as
                        *const libc::c_char);
            fprintf(__stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                        *const libc::c_char, seed, testNb);
            exit(-1i32);
        }
        if sizeCompressed > 1i32 as libc::c_ulong {
            let ref mut fresh4 =
                *bufferVerif.offset(sizeCompressed.wrapping_sub(1i32 as
                                                                    libc::c_ulong)
                                        as isize);
            *fresh4 = 253i32 as BYTE;
            let saved: BYTE = *fresh4;
            let errorCode: size_t =
                HUF_compress(bufferVerif as *mut libc::c_void,
                             sizeCompressed.wrapping_sub(1i32 as
                                                             libc::c_ulong),
                             bufferTest as *const libc::c_void, sizeOrig);
            if errorCode != 0i32 as libc::c_ulong {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"HUF_compress should have failed (too small destination buffer)\x00"
                            as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, seed, testNb);
                exit(-1i32);
            }
            if *bufferVerif.offset(sizeCompressed.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                       as isize) as libc::c_int !=
                   saved as libc::c_int {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"HUF_compress w/ too small dst : bufferVerif overflow\x00"
                            as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, seed, testNb);
                exit(-1i32);
            }
            let ref mut fresh5 = *bufferVerif.offset(sizeOrig as isize);
            *fresh5 = 253i32 as BYTE;
            let saved_0: BYTE = *fresh5;
            let result: size_t =
                HUF_decompress(bufferVerif as *mut libc::c_void, sizeOrig,
                               bufferDst as *const libc::c_void,
                               sizeCompressed);
            if *bufferVerif.offset(sizeOrig as isize) as libc::c_int !=
                   saved_0 as libc::c_int {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"HUF_decompress : bufferVerif overflow\x00" as
                            *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, seed, testNb);
                exit(-1i32);
            }
            if 0 != HUF_isError(result) {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"HUF_decompress failed : %s\x00" as *const u8 as
                            *const libc::c_char, HUF_getErrorName(result));
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, seed, testNb);
                exit(-1i32);
            }
            let hashEnd: U32 =
                XXH32(bufferVerif as *const libc::c_void, sizeOrig,
                      0i32 as libc::c_uint);
            if hashEnd != hashOrig {
                findDifferentByte(bufferVerif as *const libc::c_void,
                                  sizeOrig, bufferTest as *const libc::c_void,
                                  sizeOrig);
            }
            if hashEnd != hashOrig {
                fprintf(__stderrp,
                        b"Error => \x00" as *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b"HUF_decompress : Decompressed data corrupted\x00" as
                            *const u8 as *const libc::c_char);
                fprintf(__stderrp,
                        b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                            *const libc::c_char, seed, testNb);
                exit(-1i32);
            }
            if sizeCompressed > 4i32 as libc::c_ulong {
                let missing: size_t =
                    (FUZ_rand(&mut roundSeed) as
                         libc::c_ulong).wrapping_rem(sizeCompressed.wrapping_sub(3i32
                                                                                     as
                                                                                     libc::c_ulong)).wrapping_add(2i32
                                                                                                                      as
                                                                                                                      libc::c_ulong);
                let tooSmallSize: size_t =
                    sizeCompressed.wrapping_sub(missing);
                let mut cBufferTooSmall: *mut libc::c_void =
                    malloc(tooSmallSize);
                if cBufferTooSmall.is_null() {
                    fprintf(__stderrp,
                            b"Error => \x00" as *const u8 as
                                *const libc::c_char);
                    fprintf(__stderrp,
                            b"not enough memory !\x00" as *const u8 as
                                *const libc::c_char);
                    fprintf(__stderrp,
                            b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                                *const libc::c_char, seed, testNb);
                    exit(-1i32);
                }
                memcpy(cBufferTooSmall, bufferDst as *const libc::c_void,
                       tooSmallSize);
                let errorCode_0: size_t =
                    HUF_decompress(bufferVerif as *mut libc::c_void, sizeOrig,
                                   cBufferTooSmall, tooSmallSize);
                if 0 == HUF_isError(errorCode_0) && errorCode_0 != sizeOrig {
                    fprintf(__stderrp,
                            b"Error => \x00" as *const u8 as
                                *const libc::c_char);
                    fprintf(__stderrp,
                            b"HUF_decompress should have failed ! (truncated src buffer)\x00"
                                as *const u8 as *const libc::c_char);
                    fprintf(__stderrp,
                            b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                                *const libc::c_char, seed, testNb);
                    exit(-1i32);
                }
                free(cBufferTooSmall);
            }
        }
        let maxDstSize: size_t =
            FUZ_rand(&mut roundSeed) as libc::c_ulong & maxTestSizeMask;
        let sizeCompressed_0: size_t =
            FUZ_rand(&mut roundSeed) as libc::c_ulong & maxTestSizeMask;
        let ref mut fresh6 = *bufferDst.offset(maxDstSize as isize);
        *fresh6 = 253i32 as BYTE;
        let saved_1: BYTE = *fresh6;
        let mut result_0: size_t = 0;
        if displayLevel >= 4i32 as libc::c_uint {
            let fresh7 = tag;
            tag = tag + 1;
            fprintf(__stderrp,
                    b"\x08\x08\x08\x08%3i \x00" as *const u8 as
                        *const libc::c_char, fresh7);
        }
        result_0 =
            HUF_decompress(bufferDst as *mut libc::c_void, maxDstSize,
                           bufferTest as *const libc::c_void,
                           sizeCompressed_0);
        if 0 == HUF_isError(result_0) && result_0 > maxDstSize {
            fprintf(__stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char);
            fprintf(__stderrp,
                    b"Decompression overran output buffer\x00" as *const u8 as
                        *const libc::c_char);
            fprintf(__stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                        *const libc::c_char, seed, testNb);
            exit(-1i32);
        }
        if *bufferDst.offset(maxDstSize as isize) as libc::c_int !=
               saved_1 as libc::c_int {
            fprintf(__stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char);
            fprintf(__stderrp,
                    b"HUF_decompress noise : bufferDst overflow\x00" as
                        *const u8 as *const libc::c_char);
            fprintf(__stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as
                        *const libc::c_char, seed, testNb);
            exit(-1i32);
        }
        testNb = testNb.wrapping_add(1)
    }
    free(bufferP0 as *mut libc::c_void);
    free(bufferP1 as *mut libc::c_void);
    free(bufferP15 as *mut libc::c_void);
    free(bufferP90 as *mut libc::c_void);
    free(bufferP100 as *mut libc::c_void);
    free(bufferDst as *mut libc::c_void);
    free(bufferVerif as *mut libc::c_void);
}
/*-***************************************************************
*  Unitary tests
*****************************************************************/
unsafe extern "C" fn unitTest() {
    let mut testBuff: *mut BYTE =
        malloc((16i32 * (1i32 << 10i32)) as libc::c_ulong) as *mut BYTE;
    let mut cBuff: *mut BYTE =
        malloc((129i32 +
                    (16i32 * (1i32 << 10i32) +
                         (16i32 * (1i32 << 10i32) >> 8i32) + 8i32)) as
                   libc::c_ulong) as *mut BYTE;
    let mut verifBuff: *mut BYTE =
        malloc((16i32 * (1i32 << 10i32)) as libc::c_ulong) as *mut BYTE;
    if testBuff.is_null() || cBuff.is_null() || verifBuff.is_null() {
        fprintf(__stderrp,
                b"Not enough memory, exiting ... \n\x00" as *const u8 as
                    *const libc::c_char);
        free(testBuff as *mut libc::c_void);
        free(cBuff as *mut libc::c_void);
        free(verifBuff as *mut libc::c_void);
        return
    }
    free(testBuff as *mut libc::c_void);
    free(cBuff as *mut libc::c_void);
    free(verifBuff as *mut libc::c_void);
    fprintf(__stderrp,
            b"Unit tests completed\n\x00" as *const u8 as
                *const libc::c_char);
}
/*-***************************************************************
*  Command line
*****************************************************************/
unsafe extern "C" fn badUsage(mut exename: *const libc::c_char)
 -> libc::c_int {
    fprintf(__stderrp,
            b"wrong parameter\n\x00" as *const u8 as *const libc::c_char);
    return 1i32;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    let mut seed: U32 = 0;
    let mut startTestNb: U32 = 0i32 as U32;
    let mut pause: U32 = 0i32 as U32;
    let mut totalTest: U32 = (128i32 * (1i32 << 10i32)) as U32;
    let mut argNb: libc::c_int = 0;
    seed = (FUZ_GetMilliStart() % 10000i32) as U32;
    if displayLevel >= 1i32 as libc::c_uint {
        fprintf(__stderrp,
                b"HUF (%2i bits) automated test\n\x00" as *const u8 as
                    *const libc::c_char,
                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as
                    libc::c_int * 8i32);
    }
    argNb = 1i32;
    while argNb < argc {
        let mut argument: *const libc::c_char = *argv.offset(argNb as isize);
        if *argument.offset(0isize) as libc::c_int == '-' as i32 {
            argument = argument.offset(1isize);
            while *argument.offset(0isize) as libc::c_int != 0i32 {
                match *argument.offset(0isize) as libc::c_int {
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
                            let fresh8 = argument;
                            argument = argument.offset(1);
                            seed =
                                (seed as
                                     libc::c_uint).wrapping_add((*fresh8 as
                                                                     libc::c_int
                                                                     -
                                                                     '0' as
                                                                         i32)
                                                                    as
                                                                    libc::c_uint)
                                    as U32 as U32
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
                            let fresh9 = argument;
                            argument = argument.offset(1);
                            totalTest =
                                (totalTest as
                                     libc::c_uint).wrapping_add((*fresh9 as
                                                                     libc::c_int
                                                                     -
                                                                     '0' as
                                                                         i32)
                                                                    as
                                                                    libc::c_uint)
                                    as U32 as U32
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
                            let fresh10 = argument;
                            argument = argument.offset(1);
                            startTestNb =
                                (startTestNb as
                                     libc::c_uint).wrapping_add((*fresh10 as
                                                                     libc::c_int
                                                                     -
                                                                     '0' as
                                                                         i32)
                                                                    as
                                                                    libc::c_uint)
                                    as U32 as U32
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
                    _ => { return badUsage(*argv.offset(0isize)) }
                }
            }
        }
        argNb += 1
    }
    if startTestNb == 0i32 as libc::c_uint { unitTest(); }
    fprintf(__stderrp,
            b"Fuzzer seed : %u \n\x00" as *const u8 as *const libc::c_char,
            seed);
    FUZ_tests(seed, totalTest, startTestNb);
    fprintf(__stderrp,
            b"\rAll %u tests passed               \n\x00" as *const u8 as
                *const libc::c_char, totalTest);
    if 0 != pause {
        let mut unused: libc::c_int = 0;
        fprintf(__stderrp,
                b"press enter ...\n\x00" as *const u8 as *const libc::c_char);
        unused = getchar()
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