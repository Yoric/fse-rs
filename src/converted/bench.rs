#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fclose(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn stat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn XXH32(input: *const libc::c_void, length: size_t, seed: libc::c_uint)
     -> XXH32_hash_t;
    #[no_mangle]
    fn FSE_getErrorName(code: size_t) -> *const libc::c_char;
    /*-*****************************************
*  Tool functions
******************************************/
    #[no_mangle]
    fn FSE_compressBound(size: size_t) -> size_t;
    /* Error Management */
    #[no_mangle]
    fn FSE_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZLIBH_decompress(dest: *mut libc::c_char,
                        compressed: *const libc::c_char) -> libc::c_int;
    /* ******************************************************************
   FSE : Finite State Entropy coder
   header file
   Copyright (C) 2013-2015, Yann Collet.
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
   - Public forum : https://groups.google.com/forum/#!forum/lz4c
****************************************************************** */
    /* *************************************
*  Compiler Options
**************************************/
    // Visual Studio
    /* ***************************
*  zlib simple functions
****************************/
    #[no_mangle]
    fn ZLIBH_compress(dest: *mut libc::c_char, source: *const libc::c_char,
                      inputSize: libc::c_int) -> libc::c_int;
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
    /* ***   Advanced function   *** */
    /* * HUF_compress2() :
 *  Same as HUF_compress(), but offers control over `maxSymbolValue` and `tableLog`.
 * `maxSymbolValue` must be <= HUF_SYMBOLVALUE_MAX .
 * `tableLog` must be `<= HUF_TABLELOG_MAX` . */
    #[no_mangle]
    fn HUF_compress2(dst: *mut libc::c_void, dstCapacity: size_t,
                     src: *const libc::c_void, srcSize: size_t,
                     maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> size_t;
    /* ! FSE_decompress():
    Decompress FSE data from buffer 'cSrc', of size 'cSrcSize',
    into already allocated destination buffer 'dst', of size 'dstCapacity'.
    @return : size of regenerated data (<= maxDstSize),
              or an error code, which can be tested using FSE_isError() .

    ** Important ** : FSE_decompress() does not decompress non-compressible nor RLE data !!!
    Why ? : making this distinction requires a header.
    Header management is intentionally delegated to the user layer, which can better manage special cases.
*/
    #[no_mangle]
    fn FSE_decompress(dst: *mut libc::c_void, dstCapacity: size_t,
                      cSrc: *const libc::c_void, cSrcSize: size_t) -> size_t;
    /*-*****************************************
*  FSE advanced functions
******************************************/
/* FSE_compress2() :
    Same as FSE_compress(), but allows the selection of 'maxSymbolValue' and 'tableLog'
    Both parameters can be defined as '0' to mean : use default value
    @return : size of compressed data
    Special values : if return == 0, srcData is not compressible => Nothing is stored within cSrc !!!
                     if return == 1, srcData is a single byte symbol * srcSize times. Use RLE compression.
                     if FSE_isError(return), it's an error code.
*/
    #[no_mangle]
    fn FSE_compress2(dst: *mut libc::c_void, dstSize: size_t,
                     src: *const libc::c_void, srcSize: size_t,
                     maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> size_t;
    #[no_mangle]
    fn FSE_decompressU16(dst: *mut libc::c_ushort, dstCapacity: size_t,
                         cSrc: *const libc::c_void, cSrcSize: size_t)
     -> size_t;
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
    fn XXH64(input: *const libc::c_void, length: size_t,
             seed: libc::c_ulonglong) -> XXH64_hash_t;
    /* ! FSE_decompress_usingDTable():
    Decompress compressed source `cSrc` of size `cSrcSize` using `dt`
    into `dst` which must be already allocated.
    @return : size of regenerated data (necessarily <= `dstCapacity`),
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_decompress_usingDTable(dst: *mut libc::c_void, dstCapacity: size_t,
                                  cSrc: *const libc::c_void, cSrcSize: size_t,
                                  dt: *const FSE_DTable) -> size_t;
    /* ! FSE_compress_usingCTable():
    Compress `src` using `ct` into `dst` which must be already allocated.
    @return : size of compressed data (<= `dstCapacity`),
              or 0 if compressed data could not fit into `dst`,
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_compress_usingCTable(dst: *mut libc::c_void, dstCapacity: size_t,
                                src: *const libc::c_void, srcSize: size_t,
                                ct: *const FSE_CTable) -> size_t;
    /* ! FSE_buildDTable():
    Builds 'dt', which must be already allocated, using FSE_createDTable().
    return : 0, or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_buildDTable(dt: *mut FSE_DTable,
                       normalizedCounter: *const libc::c_short,
                       maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> size_t;
    #[no_mangle]
    fn FSE_createDTable(tableLog: libc::c_uint) -> *mut FSE_DTable;
    /* ! FSE_buildCTable():
    Builds `ct`, which must be already allocated, using FSE_createCTable().
    @return : 0, or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_buildCTable(ct: *mut FSE_CTable,
                       normalizedCounter: *const libc::c_short,
                       maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> size_t;
    #[no_mangle]
    fn FSE_createCTable(maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> *mut FSE_CTable;
    /* ! FSE_normalizeCount():
    normalize counts so that sum(count[]) == Power_of_2 (2^tableLog)
    'normalizedCounter' is a table of short, of minimum size (maxSymbolValue+1).
    @return : tableLog,
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_normalizeCount(normalizedCounter: *mut libc::c_short,
                          tableLog: libc::c_uint, count: *const libc::c_uint,
                          srcSize: size_t, maxSymbolValue: libc::c_uint)
     -> size_t;
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
    fn HIST_count(count: *mut libc::c_uint,
                  maxSymbolValuePtr: *mut libc::c_uint,
                  src: *const libc::c_void, srcSize: size_t) -> size_t;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __uint64_t = libc::c_ulonglong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_clock_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
pub type uid_t = __darwin_uid_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
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
pub type off_t = __darwin_off_t;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type nlink_t = __uint16_t;
pub type clock_t = __darwin_clock_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: libc::c_long,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
/*-**************************************************************
*  Basic Types
*****************************************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type U16 = uint16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct chunkParameters_t {
    pub id: libc::c_uint,
    pub origBuffer: *mut libc::c_char,
    pub origSize: size_t,
    pub compressedBuffer: *mut libc::c_char,
    pub compressedSize: size_t,
    pub destBuffer: *mut libc::c_char,
    pub destSize: size_t,
}
/* ****************************
*  Simple Hash Functions
******************************/
pub type XXH32_hash_t = libc::c_uint;
/* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
/* don't allocate that. It's just a way to be more restrictive than void* */
pub type FSE_DTable = libc::c_uint;
/* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
/* don't allocate that. It's only meant to be more restrictive than void* */
pub type FSE_CTable = libc::c_uint;
pub type XXH64_hash_t = libc::c_ulonglong;
/*
    bench.h - Demo program to benchmark open-source compression algorithm
    Copyright (C) Yann Collet 2012-2014
    GPLv2 License

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
    Public forum : https://groups.google.com/forum/#!forum/lz4c
*/
// bench functions
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFiles(mut fileNamesTable:
                                            *mut *const libc::c_char,
                                        mut nbFiles: libc::c_int)
 -> libc::c_int {
    let mut fileIdx: libc::c_int = 0i32;
    let mut totalSourceSize: U64 = 0i32 as U64;
    let mut totalCompressedSize: U64 = 0i32 as U64;
    let mut totalc: libc::c_double = 0.0f64;
    let mut totald: libc::c_double = 0.0f64;
    while fileIdx < nbFiles {
        let fresh0 = fileIdx;
        fileIdx = fileIdx + 1;
        let inFileName: *const libc::c_char =
            *fileNamesTable.offset(fresh0 as isize);
        let inFile: *mut FILE =
            fopen(inFileName, b"rb\x00" as *const u8 as *const libc::c_char);
        let mut inFileSize: U64 = 0;
        let mut benchedSize: size_t = 0;
        let mut orig_buff: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nbChunks: libc::c_int = 0;
        let mut maxCompressedChunkSize: libc::c_int = 0;
        let mut readSize: size_t = 0;
        let mut compressedBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut compressedBuffSize: libc::c_int = 0;
        let mut destBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut chunkP: *mut chunkParameters_t = 0 as *mut chunkParameters_t;
        if inFile.is_null() {
            fprintf(__stderrp,
                    b"Pb opening %s\n\x00" as *const u8 as
                        *const libc::c_char, inFileName);
            return 11i32
        }
        inFileSize = BMK_GetFileSize(inFileName);
        if inFileSize == 0i32 as libc::c_ulonglong {
            fprintf(__stderrp,
                    b"file is empty\n\x00" as *const u8 as
                        *const libc::c_char);
            fclose(inFile);
            return 11i32
        }
        benchedSize =
            BMK_findMaxMem(inFileSize.wrapping_mul(3i32 as
                                                       libc::c_ulonglong)).wrapping_div(3i32
                                                                                            as
                                                                                            libc::c_ulong);
        if benchedSize as U64 > inFileSize {
            benchedSize = inFileSize as size_t
        }
        if (benchedSize as libc::c_ulonglong) < inFileSize {
            fprintf(__stderrp,
                    b"Not enough memory for \'%s\' full size; testing %i MB only...\n\x00"
                        as *const u8 as *const libc::c_char, inFileName,
                    (benchedSize >> 20i32) as libc::c_int);
        }
        chunkP =
            malloc(benchedSize.wrapping_div(chunkSize as
                                                libc::c_ulong).wrapping_add(1i32
                                                                                as
                                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<chunkParameters_t>()
                                                                                                                as
                                                                                                                libc::c_ulong))
                as *mut chunkParameters_t;
        orig_buff = malloc(benchedSize) as *mut libc::c_char;
        nbChunks =
            benchedSize.wrapping_div(chunkSize as libc::c_ulong) as
                libc::c_int + 1i32;
        maxCompressedChunkSize =
            FSE_compressBound(chunkSize as size_t) as libc::c_int;
        compressedBuffSize = nbChunks * maxCompressedChunkSize;
        compressedBuffer =
            malloc(compressedBuffSize as size_t) as *mut libc::c_char;
        destBuffer = malloc(benchedSize) as *mut libc::c_char;
        if orig_buff.is_null() || compressedBuffer.is_null() ||
               destBuffer.is_null() || chunkP.is_null() {
            fprintf(__stderrp,
                    b"\nError: not enough memory!\n\x00" as *const u8 as
                        *const libc::c_char);
            free(orig_buff as *mut libc::c_void);
            free(compressedBuffer as *mut libc::c_void);
            free(destBuffer as *mut libc::c_void);
            free(chunkP as *mut libc::c_void);
            fclose(inFile);
            return 12i32
        }
        let mut i: libc::c_int = 0;
        let mut remaining: size_t = benchedSize;
        let mut in_0: *mut libc::c_char = orig_buff;
        let mut out: *mut libc::c_char = compressedBuffer;
        let mut dst: *mut libc::c_char = destBuffer;
        i = 0i32;
        while i < nbChunks {
            (*chunkP.offset(i as isize)).id = i as libc::c_uint;
            let ref mut fresh1 = (*chunkP.offset(i as isize)).origBuffer;
            *fresh1 = in_0;
            in_0 = in_0.offset(chunkSize as isize);
            if remaining > chunkSize as libc::c_ulong {
                (*chunkP.offset(i as isize)).origSize = chunkSize as size_t;
                remaining =
                    (remaining as
                         libc::c_ulong).wrapping_sub(chunkSize as
                                                         libc::c_ulong) as
                        size_t as size_t
            } else {
                (*chunkP.offset(i as isize)).origSize =
                    remaining as libc::c_int as size_t;
                remaining = 0i32 as size_t
            }
            let ref mut fresh2 =
                (*chunkP.offset(i as isize)).compressedBuffer;
            *fresh2 = out;
            out = out.offset(maxCompressedChunkSize as isize);
            (*chunkP.offset(i as isize)).compressedSize = 0i32 as size_t;
            let ref mut fresh3 = (*chunkP.offset(i as isize)).destBuffer;
            *fresh3 = dst;
            dst = dst.offset(chunkSize as isize);
            i += 1
        }
        fprintf(__stderrp,
                b"Loading %s...       \r\x00" as *const u8 as
                    *const libc::c_char, inFileName);
        readSize =
            fread(orig_buff as *mut libc::c_void, 1i32 as libc::c_ulong,
                  benchedSize, inFile);
        fclose(inFile);
        if readSize != benchedSize {
            fprintf(__stderrp,
                    b"\nError: problem reading file \'%s\' (%i read, should be %i) !!    \n\x00"
                        as *const u8 as *const libc::c_char, inFileName,
                    readSize as libc::c_int, benchedSize as libc::c_int);
            free(orig_buff as *mut libc::c_void);
            free(compressedBuffer as *mut libc::c_void);
            free(destBuffer as *mut libc::c_void);
            free(chunkP as *mut libc::c_void);
            return 13i32
        }
        BMK_benchMem(chunkP, nbChunks, inFileName, benchedSize as libc::c_int,
                     &mut totalCompressedSize, &mut totalc, &mut totald,
                     255i32, BMK_tableLog);
        totalSourceSize =
            (totalSourceSize as
                 libc::c_ulonglong).wrapping_add(benchedSize as
                                                     libc::c_ulonglong) as U64
                as U64;
        free(orig_buff as *mut libc::c_void);
        free(compressedBuffer as *mut libc::c_void);
        free(destBuffer as *mut libc::c_void);
        free(chunkP as *mut libc::c_void);
    }
    if nbFiles > 1i32 {
        fprintf(__stderrp,
                b"%-17.17s :%10llu ->%10llu (%5.2f%%), %6.1f MB/s , %6.1f MB/s\n\x00"
                    as *const u8 as *const libc::c_char,
                b"  TOTAL\x00" as *const u8 as *const libc::c_char,
                totalSourceSize, totalCompressedSize,
                totalCompressedSize as libc::c_double /
                    totalSourceSize as libc::c_double * 100.0f64,
                totalSourceSize as libc::c_double / totalc /
                    1000000i32 as libc::c_double,
                totalSourceSize as libc::c_double / totald /
                    1000000i32 as libc::c_double);
    }
    return 0i32;
}
static mut BMK_tableLog: libc::c_int = 12i32;
/* BMK_benchMem() :
 * chunkP is expected to be correctly filled */
#[no_mangle]
pub unsafe extern "C" fn BMK_benchMem(mut chunkP: *mut chunkParameters_t,
                                      mut nbChunks: libc::c_int,
                                      mut inFileName: *const libc::c_char,
                                      mut benchedSize: libc::c_int,
                                      mut totalCompressedSize: *mut U64,
                                      mut totalCompressionTime:
                                          *mut libc::c_double,
                                      mut totalDecompressionTime:
                                          *mut libc::c_double,
                                      mut nbSymbols: libc::c_int,
                                      mut memLog: libc::c_int) {
    let mut trial: libc::c_int = 0;
    let mut chunkNb: libc::c_int = 0;
    let mut cSize: size_t = 0i32 as size_t;
    let mut fastestC: libc::c_double = 100000000.0f64;
    let mut fastestD: libc::c_double = 100000000.0f64;
    let mut ratio: libc::c_double = 0.0f64;
    let mut crcCheck: U32 = 0i32 as U32;
    let mut nbDecodeLoops: libc::c_int =
        (100i32 as
             libc::c_uint).wrapping_mul(1u32 <<
                                            20i32).wrapping_div((benchedSize +
                                                                     1i32) as
                                                                    libc::c_uint).wrapping_add(1i32
                                                                                                   as
                                                                                                   libc::c_uint)
            as libc::c_int;
    let crcOrig: U32 =
        XXH32((*chunkP.offset(0isize)).origBuffer as *const libc::c_void,
              benchedSize as size_t, 0i32 as libc::c_uint);
    let mut compressor:
            Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t,
                                        _: *const libc::c_void, _: size_t,
                                        _: libc::c_uint, _: libc::c_uint)
                       -> size_t> = None;
    let mut decompressor:
            Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t,
                                        _: *const libc::c_void, _: size_t)
                       -> size_t> = None;
    let nameLength: size_t = strlen(inFileName);
    if nameLength > 17i32 as libc::c_ulong {
        inFileName =
            inFileName.offset(nameLength.wrapping_sub(17i32 as libc::c_ulong)
                                  as isize)
    }
    if nbSymbols == 3i32 {
        BMK_benchMem285(chunkP, nbChunks, inFileName, benchedSize,
                        totalCompressedSize, totalCompressionTime,
                        totalDecompressionTime, memLog);
        return
    }
    match BMK_byteCompressor {
        2 => {
            compressor = Some(HUF_compress2);
            decompressor = Some(HUF_decompress)
        }
        3 => {
            compressor = Some(BMK_ZLIBH_compress);
            decompressor = Some(BMK_ZLIBH_decompress)
        }
        1 | _ => {
            compressor = Some(FSE_compress2);
            decompressor = Some(FSE_decompress)
        }
    }
    fprintf(__stderrp, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char);
    trial = 1i32;
    while trial <= nbIterations {
        let mut nbLoops: libc::c_int = 0i32;
        let mut clockStart: clock_t = 0;
        let mut clockDuration: clock_t = 0;
        fprintf(__stderrp,
                b"%1i-%-15.15s : %9i ->\r\x00" as *const u8 as
                    *const libc::c_char, trial, inFileName, benchedSize);
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < benchedSize {
            *(*chunkP.offset(0isize)).compressedBuffer.offset(i as isize) =
                i as libc::c_char;
            i += 1
        }
        clockStart = clock();
        while clock() == clockStart { }
        clockStart = clock();
        while BMK_clockSpan(clockStart) < (1000000i32 * 2i32) as libc::c_ulong
              {
            chunkNb = 0i32;
            while chunkNb < nbChunks {
                let cBSize: size_t =
                    compressor.expect("non-null function pointer")((*chunkP.offset(chunkNb
                                                                                       as
                                                                                       isize)).compressedBuffer
                                                                       as
                                                                       *mut libc::c_void,
                                                                   FSE_compressBound((*chunkP.offset(chunkNb
                                                                                                         as
                                                                                                         isize)).origSize),
                                                                   (*chunkP.offset(chunkNb
                                                                                       as
                                                                                       isize)).origBuffer
                                                                       as
                                                                       *const libc::c_void,
                                                                   (*chunkP.offset(chunkNb
                                                                                       as
                                                                                       isize)).origSize,
                                                                   nbSymbols
                                                                       as
                                                                       libc::c_uint,
                                                                   memLog as
                                                                       libc::c_uint);
                if 0 != FSE_isError(cBSize) {
                    fprintf(__stderrp,
                            b"!!! Error compressing block %i  !!!!  => %s   \n\x00"
                                as *const u8 as *const libc::c_char, chunkNb,
                            FSE_getErrorName(cBSize));
                    return
                }
                (*chunkP.offset(chunkNb as isize)).compressedSize = cBSize;
                chunkNb += 1
            }
            nbLoops += 1
        }
        clockDuration = BMK_clockSpan(clockStart);
        clockDuration =
            (clockDuration as
                 libc::c_ulong).wrapping_add((0 == clockDuration) as
                                                 libc::c_int as libc::c_ulong)
                as clock_t as clock_t;
        if (clockDuration as libc::c_double) <
               fastestC * nbLoops as libc::c_double *
                   1000000i32 as libc::c_double {
            fastestC =
                clockDuration as libc::c_double / 1000000i32 as libc::c_double
                    / nbLoops as libc::c_double
        }
        cSize = 0i32 as size_t;
        chunkNb = 0i32;
        while chunkNb < nbChunks {
            cSize =
                (cSize as
                     libc::c_ulong).wrapping_add(if 0 !=
                                                        (*chunkP.offset(chunkNb
                                                                            as
                                                                            isize)).compressedSize
                                                    {
                                                     (*chunkP.offset(chunkNb
                                                                         as
                                                                         isize)).compressedSize
                                                 } else {
                                                     (*chunkP.offset(chunkNb
                                                                         as
                                                                         isize)).origSize
                                                 }) as size_t as size_t;
            chunkNb += 1
        }
        ratio =
            cSize as libc::c_double / benchedSize as libc::c_double *
                100.0f64;
        fprintf(__stderrp,
                b"%1i-%-15.15s : %9i -> %9i (%5.2f%%),%7.1f MB/s\r\x00" as
                    *const u8 as *const libc::c_char, trial, inFileName,
                benchedSize, cSize as libc::c_int, ratio,
                benchedSize as libc::c_double /
                    (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as
                        libc::c_double / fastestC);
        let mut i_0: libc::c_int = 0;
        i_0 = 0i32;
        while i_0 < benchedSize {
            *(*chunkP.offset(0isize)).destBuffer.offset(i_0 as isize) =
                0i32 as libc::c_char;
            i_0 += 1
        }
        clockStart = clock();
        while clock() == clockStart { }
        clockStart = clock();
        nbLoops = 0i32;
        while nbLoops < nbDecodeLoops {
            chunkNb = 0i32;
            while chunkNb < nbChunks {
                let mut regenSize: size_t = 0;
                match (*chunkP.offset(chunkNb as isize)).compressedSize {
                    0 => {
                        regenSize =
                            (*chunkP.offset(chunkNb as isize)).origSize;
                        memcpy((*chunkP.offset(chunkNb as isize)).destBuffer
                                   as *mut libc::c_void,
                               (*chunkP.offset(chunkNb as isize)).origBuffer
                                   as *const libc::c_void, regenSize);
                    }
                    1 => {
                        regenSize =
                            (*chunkP.offset(chunkNb as isize)).origSize;
                        memset((*chunkP.offset(chunkNb as isize)).destBuffer
                                   as *mut libc::c_void,
                               *(*chunkP.offset(chunkNb as
                                                    isize)).origBuffer.offset(0isize)
                                   as libc::c_int,
                               (*chunkP.offset(chunkNb as isize)).origSize);
                    }
                    _ => {
                        regenSize =
                            decompressor.expect("non-null function pointer")((*chunkP.offset(chunkNb
                                                                                                 as
                                                                                                 isize)).destBuffer
                                                                                 as
                                                                                 *mut libc::c_void,
                                                                             (*chunkP.offset(chunkNb
                                                                                                 as
                                                                                                 isize)).origSize,
                                                                             (*chunkP.offset(chunkNb
                                                                                                 as
                                                                                                 isize)).compressedBuffer
                                                                                 as
                                                                                 *const libc::c_void,
                                                                             (*chunkP.offset(chunkNb
                                                                                                 as
                                                                                                 isize)).compressedSize)
                    }
                }
                if regenSize != (*chunkP.offset(chunkNb as isize)).origSize {
                    fprintf(__stderrp,
                            b"!! Error decompressing block %i of cSize %u !! => (%s)  \n\x00"
                                as *const u8 as *const libc::c_char, chunkNb,
                            (*chunkP.offset(chunkNb as isize)).compressedSize
                                as U32, FSE_getErrorName(regenSize));
                    return
                }
                chunkNb += 1
            }
            nbLoops += 1
        }
        clockDuration = BMK_clockSpan(clockStart);
        if clockDuration > 0i32 as libc::c_ulong {
            if (clockDuration as libc::c_double) <
                   fastestD * nbDecodeLoops as libc::c_double *
                       1000000i32 as libc::c_double {
                fastestD =
                    clockDuration as libc::c_double /
                        1000000i32 as libc::c_double /
                        nbDecodeLoops as libc::c_double
            }
            if 0 !=
                   !(fastestD > 1.0f64 / 1000000000i32 as libc::c_double) as
                       libc::c_int as libc::c_long {
                __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                                       &[libc::c_char; 13]>(b"BMK_benchMem\x00")).as_ptr(),
                             b"bench.c\x00" as *const u8 as
                                 *const libc::c_char, 431i32,
                             b"fastestD > 1./1000000000\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            nbDecodeLoops =
                ((1.0f64 / fastestD) as
                     U32).wrapping_add(1i32 as libc::c_uint) as libc::c_int
        } else {
            if 0 !=
                   !(nbDecodeLoops < 20000000i32) as libc::c_int as
                       libc::c_long {
                __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                                       &[libc::c_char; 13]>(b"BMK_benchMem\x00")).as_ptr(),
                             b"bench.c\x00" as *const u8 as
                                 *const libc::c_char, 434i32,
                             b"nbDecodeLoops < 20000000\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            nbDecodeLoops *= 100i32
        }
        fprintf(__stderrp,
                b"%1i-%-15.15s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\r\x00"
                    as *const u8 as *const libc::c_char, trial, inFileName,
                benchedSize, cSize as libc::c_int, ratio,
                benchedSize as libc::c_double /
                    (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as
                        libc::c_double / fastestC,
                benchedSize as libc::c_double /
                    (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as
                        libc::c_double / fastestD);
        crcCheck =
            XXH32((*chunkP.offset(0isize)).destBuffer as *const libc::c_void,
                  benchedSize as size_t, 0i32 as libc::c_uint);
        if crcOrig != crcCheck {
            let mut src_0: *const libc::c_char =
                (*chunkP.offset(0isize)).origBuffer;
            let mut fin: *const libc::c_char =
                (*chunkP.offset(0isize)).destBuffer;
            let srcStart: *const libc::c_char = src_0;
            while *src_0 as libc::c_int == *fin as libc::c_int {
                src_0 = src_0.offset(1isize);
                fin = fin.offset(1isize)
            }
            fprintf(__stderrp,
                    b"\n!!! %15s : Invalid Checksum !!! pos %i/%i\n\x00" as
                        *const u8 as *const libc::c_char, inFileName,
                    src_0.wrapping_offset_from(srcStart) as libc::c_long as
                        libc::c_int, benchedSize);
            break ;
        } else { trial += 1 }
    }
    if crcOrig == crcCheck {
        if ratio < 100.0f64 {
            fprintf(__stderrp,
                    b"%-17.17s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\n\x00"
                        as *const u8 as *const libc::c_char, inFileName,
                    benchedSize, cSize as libc::c_int, ratio,
                    benchedSize as libc::c_double /
                        (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as
                            libc::c_double / fastestC,
                    benchedSize as libc::c_double /
                        (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as
                            libc::c_double / fastestD);
        } else {
            fprintf(__stderrp,
                    b"%-17.17s : %9i -> %9i (%5.1f%%),%7.1f MB/s ,%7.1f MB/s \n\x00"
                        as *const u8 as *const libc::c_char, inFileName,
                    benchedSize, cSize as libc::c_int, ratio,
                    benchedSize as libc::c_double /
                        (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as
                            libc::c_double / fastestC,
                    benchedSize as libc::c_double /
                        (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as
                            libc::c_double / fastestD);
        }
    } else {
        fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    *totalCompressedSize =
        (*totalCompressedSize as
             libc::c_ulonglong).wrapping_add(cSize as libc::c_ulonglong) as
            U64 as U64;
    *totalCompressionTime += fastestC;
    *totalDecompressionTime += fastestD;
}
/*-*******************************************************
*  local functions
*********************************************************/
unsafe extern "C" fn BMK_clockSpan(mut start: clock_t) -> clock_t {
    return clock().wrapping_sub(start);
}
static mut nbIterations: libc::c_int = 4i32;
#[no_mangle]
pub unsafe extern "C" fn BMK_ZLIBH_decompress(mut dest: *mut libc::c_void,
                                              mut originalSize: size_t,
                                              mut compressed:
                                                  *const libc::c_void,
                                              mut cSize: size_t) -> size_t {
    ZLIBH_decompress(dest as *mut libc::c_char,
                     compressed as *const libc::c_char);
    return originalSize;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_ZLIBH_compress(mut dst: *mut libc::c_void,
                                            mut dstSize: size_t,
                                            mut src: *const libc::c_void,
                                            mut srcSize: size_t,
                                            mut nbSymbols: libc::c_uint,
                                            mut tableLog: libc::c_uint)
 -> size_t {
    return ZLIBH_compress(dst as *mut libc::c_char,
                          src as *const libc::c_char, srcSize as libc::c_int)
               as size_t;
}
static mut BMK_byteCompressor: libc::c_int = 1i32;
/*-*******************************************************
*  Public function
*********************************************************/
#[no_mangle]
pub unsafe extern "C" fn BMK_benchMem285(mut chunkP: *mut chunkParameters_t,
                                         mut nbChunks: libc::c_int,
                                         mut inFileName: *const libc::c_char,
                                         mut benchedSize: libc::c_int,
                                         mut totalCompressedSize: *mut U64,
                                         mut totalCompressionTime:
                                             *mut libc::c_double,
                                         mut totalDecompressionTime:
                                             *mut libc::c_double,
                                         mut memLog: libc::c_int) {
    let mut loopNb: libc::c_int = 0;
    let mut chunkNb: libc::c_int = 0;
    let mut cSize: size_t = 0i32 as size_t;
    let mut fastestC: libc::c_double = 100000000.0f64;
    let mut fastestD: libc::c_double = 100000000.0f64;
    let mut ratio: libc::c_double = 0.0f64;
    let mut crcCheck: U32 = 0i32 as U32;
    let mut crcOrig: U32 = 0;
    crcOrig =
        XXH32((*chunkP.offset(0isize)).origBuffer as *const libc::c_void,
              benchedSize as size_t, 0i32 as libc::c_uint);
    fprintf(__stderrp, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char);
    loopNb = 1i32;
    while loopNb <= nbIterations {
        let mut nbLoops: libc::c_int = 0;
        let mut clockStart: clock_t = 0;
        let mut clockDuration: clock_t = 0;
        fprintf(__stderrp,
                b"%1i-%-14.14s : %9i ->\r\x00" as *const u8 as
                    *const libc::c_char, loopNb, inFileName, benchedSize);
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < benchedSize {
            *(*chunkP.offset(0isize)).compressedBuffer.offset(i as isize) =
                i as libc::c_char;
            i += 1
        }
        nbLoops = 0i32;
        clockStart = clock();
        while clock() == clockStart { }
        clockStart = clock();
        while BMK_clockSpan(clockStart) < (1000000i32 * 2i32) as libc::c_ulong
              {
            chunkNb = 0i32;
            while chunkNb < nbChunks {
                let mut rawPtr: *const libc::c_void =
                    (*chunkP.offset(chunkNb as isize)).origBuffer as
                        *const libc::c_void;
                let mut U16chunkPtr: *const U16 = rawPtr as *const U16;
                (*chunkP.offset(chunkNb as isize)).compressedSize =
                    FSE_compressU16((*chunkP.offset(chunkNb as
                                                        isize)).compressedBuffer
                                        as *mut libc::c_void,
                                    (*chunkP.offset(chunkNb as
                                                        isize)).origSize,
                                    U16chunkPtr,
                                    (*chunkP.offset(chunkNb as
                                                        isize)).origSize.wrapping_div(2i32
                                                                                          as
                                                                                          libc::c_ulong),
                                    0i32 as libc::c_uint,
                                    memLog as libc::c_uint);
                chunkNb += 1
            }
            nbLoops += 1
        }
        clockDuration = BMK_clockSpan(clockStart);
        if (clockDuration as libc::c_double) <
               fastestC * nbLoops as libc::c_double {
            fastestC =
                clockDuration as libc::c_double / nbLoops as libc::c_double
        }
        cSize = 0i32 as size_t;
        chunkNb = 0i32;
        while chunkNb < nbChunks {
            cSize =
                (cSize as
                     libc::c_ulong).wrapping_add((*chunkP.offset(chunkNb as
                                                                     isize)).compressedSize)
                    as size_t as size_t;
            chunkNb += 1
        }
        ratio =
            cSize as libc::c_double / benchedSize as libc::c_double *
                100.0f64;
        fprintf(__stderrp,
                b"%1i-%-14.14s : %9i -> %9i (%5.2f%%),%7.1f MB/s\r\x00" as
                    *const u8 as *const libc::c_char, loopNb, inFileName,
                benchedSize, cSize as libc::c_int, ratio,
                benchedSize as libc::c_double / fastestC / 1000.0f64);
        nbLoops = 0i32;
        clockStart = clock();
        while clock() == clockStart { }
        clockStart = clock();
        while BMK_clockSpan(clockStart) < (1000000i32 * 2i32) as libc::c_ulong
              {
            chunkNb = 0i32;
            while chunkNb < nbChunks {
                let mut rawPtr_0: *mut libc::c_void =
                    (*chunkP.offset(chunkNb as isize)).destBuffer as
                        *mut libc::c_void;
                let mut U16dstPtr: *mut U16 = rawPtr_0 as *mut U16;
                (*chunkP.offset(chunkNb as isize)).compressedSize =
                    FSE_decompressU16(U16dstPtr,
                                      (*chunkP.offset(chunkNb as
                                                          isize)).origSize.wrapping_div(2i32
                                                                                            as
                                                                                            libc::c_ulong),
                                      (*chunkP.offset(chunkNb as
                                                          isize)).compressedBuffer
                                          as *const libc::c_void,
                                      (*chunkP.offset(chunkNb as
                                                          isize)).compressedSize);
                chunkNb += 1
            }
            nbLoops += 1
        }
        clockDuration = BMK_clockSpan(clockStart);
        if (clockDuration as libc::c_double) <
               fastestC * nbLoops as libc::c_double {
            fastestC =
                clockDuration as libc::c_double / nbLoops as libc::c_double
        }
        fprintf(__stderrp,
                b"%1i-%-14.14s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\r\x00"
                    as *const u8 as *const libc::c_char, loopNb, inFileName,
                benchedSize, cSize as libc::c_int, ratio,
                benchedSize as libc::c_double / fastestC / 1000.0f64,
                benchedSize as libc::c_double / fastestD / 1000.0f64);
        crcCheck =
            XXH32((*chunkP.offset(0isize)).destBuffer as *const libc::c_void,
                  benchedSize as size_t, 0i32 as libc::c_uint);
        if crcOrig != crcCheck {
            let mut src: *const libc::c_char =
                (*chunkP.offset(0isize)).origBuffer;
            let mut fin: *const libc::c_char =
                (*chunkP.offset(0isize)).destBuffer;
            let srcStart: *const libc::c_char = src;
            while *src as libc::c_int == *fin as libc::c_int {
                src = src.offset(1isize);
                fin = fin.offset(1isize)
            }
            fprintf(__stderrp,
                    b"\n!!! %14s : Invalid Checksum !!! pos %i/%i\n\x00" as
                        *const u8 as *const libc::c_char, inFileName,
                    src.wrapping_offset_from(srcStart) as libc::c_long as
                        libc::c_int, benchedSize);
            break ;
        } else { loopNb += 1 }
    }
    if crcOrig == crcCheck {
        if ratio < 100.0f64 {
            fprintf(__stderrp,
                    b"%-16.16s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\n\x00"
                        as *const u8 as *const libc::c_char, inFileName,
                    benchedSize, cSize as libc::c_int, ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64,
                    benchedSize as libc::c_double / fastestD / 1000.0f64);
        } else {
            fprintf(__stderrp,
                    b"%-16.16s : %9i -> %9i (%5.1f%%),%7.1f MB/s ,%7.1f MB/s \n\x00"
                        as *const u8 as *const libc::c_char, inFileName,
                    benchedSize, cSize as libc::c_int, ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64,
                    benchedSize as libc::c_double / fastestD / 1000.0f64);
        }
    }
    *totalCompressedSize =
        (*totalCompressedSize as
             libc::c_ulonglong).wrapping_add(cSize as libc::c_ulonglong) as
            U64 as U64;
    *totalCompressionTime += fastestC;
    *totalDecompressionTime += fastestD;
}
// Initialized in run_static_initializers
static mut chunkSize: U32 = 0;
unsafe extern "C" fn BMK_findMaxMem(mut requiredMem: U64) -> size_t {
    let mut step: size_t =
        (64i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as size_t;
    let mut testmem: *mut BYTE = 0 as *mut BYTE;
    requiredMem =
        (requiredMem >> 26i32).wrapping_add(1i32 as libc::c_ulonglong) <<
            26i32;
    requiredMem =
        (requiredMem as
             libc::c_ulonglong).wrapping_add((2i32 as
                                                  libc::c_ulong).wrapping_mul(step)
                                                 as libc::c_ulonglong) as U64
            as U64;
    if requiredMem >
           if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong ==
                  4i32 as libc::c_ulong {
               (2i32 as
                    libc::c_uint).wrapping_mul(1u32 <<
                                                   30i32).wrapping_sub((64i32
                                                                            as
                                                                            libc::c_uint).wrapping_mul(1u32
                                                                                                           <<
                                                                                                           20i32))
                   as libc::c_ulonglong
           } else { 9u64.wrapping_mul((1u32 << 30i32) as libc::c_ulonglong) }
       {
        requiredMem =
            if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong ==
                   4i32 as libc::c_ulong {
                (2i32 as
                     libc::c_uint).wrapping_mul(1u32 <<
                                                    30i32).wrapping_sub((64i32
                                                                             as
                                                                             libc::c_uint).wrapping_mul(1u32
                                                                                                            <<
                                                                                                            20i32))
                    as libc::c_ulonglong
            } else { 9u64.wrapping_mul((1u32 << 30i32) as libc::c_ulonglong) }
    }
    while testmem.is_null() {
        requiredMem =
            (requiredMem as
                 libc::c_ulonglong).wrapping_sub(step as libc::c_ulonglong) as
                U64 as U64;
        if requiredMem <= step as libc::c_ulonglong {
            requiredMem = step.wrapping_add(64i32 as libc::c_ulong) as U64;
            break ;
        } else { testmem = malloc(requiredMem as size_t) as *mut BYTE }
    }
    free(testmem as *mut libc::c_void);
    return requiredMem.wrapping_sub(step as libc::c_ulonglong) as size_t;
}
unsafe extern "C" fn BMK_GetFileSize(mut infilename: *const libc::c_char)
 -> U64 {
    let mut r: libc::c_int = 0;
    let mut statbuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    r = stat(infilename, &mut statbuf);
    if 0 != r ||
           !(statbuf.st_mode as libc::c_int & 0o170000i32 == 0o100000i32) {
        return 0i32 as U64
    }
    return statbuf.st_size as U64;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_benchCore_Files(mut fileNamesTable:
                                                 *mut *const libc::c_char,
                                             mut nbFiles: libc::c_int)
 -> libc::c_int {
    let mut fileIdx: libc::c_int = 0i32;
    let mut totals: U64 = 0i32 as U64;
    let mut totalz: U64 = 0i32 as U64;
    let mut totalc: libc::c_double = 0.0f64;
    let mut totald: libc::c_double = 0.0f64;
    while fileIdx < nbFiles {
        let mut inFile: *mut FILE = 0 as *mut FILE;
        let mut inFileName: *const libc::c_char = 0 as *const libc::c_char;
        let mut inFileSize: U64 = 0;
        let mut benchedSize: size_t = 0;
        let mut nbChunks: libc::c_int = 0;
        let mut maxCompressedChunkSize: size_t = 0;
        let mut readSize: size_t = 0;
        let mut orig_buff: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut compressedBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut compressedBuffSize: size_t = 0;
        let fresh4 = fileIdx;
        fileIdx = fileIdx + 1;
        inFileName = *fileNamesTable.offset(fresh4 as isize);
        inFile =
            fopen(inFileName, b"rb\x00" as *const u8 as *const libc::c_char);
        if inFile.is_null() {
            fprintf(__stderrp,
                    b"Pb opening %s\n\x00" as *const u8 as
                        *const libc::c_char, inFileName);
            return 11i32
        }
        inFileSize = BMK_GetFileSize(inFileName);
        if inFileSize == 0i32 as libc::c_ulonglong {
            fprintf(__stderrp,
                    b"%s is empty\n\x00" as *const u8 as *const libc::c_char,
                    inFileName);
            return 11i32
        }
        benchedSize =
            (256i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as size_t;
        if benchedSize as U64 > inFileSize {
            benchedSize = inFileSize as size_t
        } else {
            fprintf(__stderrp,
                    b"FSE Core Loop speed evaluation, testing %i KB ...\n\x00"
                        as *const u8 as *const libc::c_char,
                    (benchedSize >> 10i32) as libc::c_int);
        }
        orig_buff = malloc(benchedSize) as *mut libc::c_char;
        nbChunks = 1i32;
        maxCompressedChunkSize =
            FSE_compressBound(benchedSize as libc::c_int as size_t);
        compressedBuffSize =
            (nbChunks as libc::c_ulong).wrapping_mul(maxCompressedChunkSize);
        compressedBuffer = malloc(compressedBuffSize) as *mut libc::c_char;
        if orig_buff.is_null() || compressedBuffer.is_null() {
            fprintf(__stderrp,
                    b"\nError: not enough memory!\n\x00" as *const u8 as
                        *const libc::c_char);
            free(orig_buff as *mut libc::c_void);
            free(compressedBuffer as *mut libc::c_void);
            fclose(inFile);
            return 12i32
        }
        fprintf(__stderrp,
                b"Loading %s...       \r\x00" as *const u8 as
                    *const libc::c_char, inFileName);
        readSize =
            fread(orig_buff as *mut libc::c_void, 1i32 as libc::c_ulong,
                  benchedSize, inFile);
        fclose(inFile);
        if readSize != benchedSize {
            fprintf(__stderrp,
                    b"\nError: problem reading file \'%s\' (%i read, should be %i) !!    \n\x00"
                        as *const u8 as *const libc::c_char, inFileName,
                    readSize as libc::c_int, benchedSize as libc::c_int);
            free(orig_buff as *mut libc::c_void);
            free(compressedBuffer as *mut libc::c_void);
            return 13i32
        }
        BMK_benchCore_Mem(compressedBuffer, orig_buff,
                          benchedSize as libc::c_int as libc::c_uint,
                          255i32 as libc::c_uint,
                          BMK_tableLog as libc::c_uint, inFileName,
                          &mut totalz, &mut totalc, &mut totald);
        totals =
            (totals as
                 libc::c_ulonglong).wrapping_add(benchedSize as
                                                     libc::c_ulonglong) as U64
                as U64;
        free(orig_buff as *mut libc::c_void);
        free(compressedBuffer as *mut libc::c_void);
    }
    if nbFiles > 1i32 {
        fprintf(__stderrp,
                b"%-16.16s :%10llu ->%10llu (%5.2f%%), %6.1f MB/s , %6.1f MB/s\n\x00"
                    as *const u8 as *const libc::c_char,
                b"  TOTAL\x00" as *const u8 as *const libc::c_char, totals,
                totalz,
                totalz as libc::c_double / totals as libc::c_double *
                    100.0f64, totals as libc::c_double / totalc / 1000.0f64,
                totals as libc::c_double / totald / 1000.0f64);
    }
    return 0i32;
}
/*-********************************************************************
*  BenchCore
**********************************************************************/
unsafe extern "C" fn BMK_benchCore_Mem(mut dst: *mut libc::c_char,
                                       mut src: *mut libc::c_char,
                                       mut benchedSize: libc::c_uint,
                                       mut nbSymbols: libc::c_uint,
                                       mut tableLog: libc::c_uint,
                                       mut inFileName: *const libc::c_char,
                                       mut totalCompressedSize: *mut U64,
                                       mut totalCompressionTime:
                                           *mut libc::c_double,
                                       mut totalDecompressionTime:
                                           *mut libc::c_double) {
    let mut loopNb: libc::c_int = 0;
    let mut cSize: size_t = 0i32 as size_t;
    let mut dSize: size_t = 0i32 as size_t;
    let mut fastestC: libc::c_double = 100000000.0f64;
    let mut fastestD: libc::c_double = 100000000.0f64;
    let mut ratio: libc::c_double = 0.0f64;
    let mut crcCheck: U64 = 0i32 as U64;
    let mut crcOrig: U64 = 0;
    let mut count: [U32; 256] = [0; 256];
    let mut norm: [libc::c_short; 256] = [0; 256];
    let mut ct: *mut FSE_CTable = 0 as *mut FSE_CTable;
    let mut dt: *mut FSE_DTable = 0 as *mut FSE_DTable;
    crcOrig =
        XXH64(src as *const libc::c_void, benchedSize as size_t,
              0i32 as libc::c_ulonglong);
    HIST_count(count.as_mut_ptr(), &mut nbSymbols,
               src as *mut BYTE as *const libc::c_void,
               benchedSize as size_t);
    tableLog =
        FSE_normalizeCount(norm.as_mut_ptr(), tableLog, count.as_mut_ptr(),
                           benchedSize as size_t, nbSymbols) as U32;
    ct = FSE_createCTable(tableLog, nbSymbols);
    FSE_buildCTable(ct, norm.as_mut_ptr(), nbSymbols, tableLog);
    dt = FSE_createDTable(tableLog);
    FSE_buildDTable(dt, norm.as_mut_ptr(), nbSymbols, tableLog);
    fprintf(__stderrp, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char);
    loopNb = 1i32;
    while loopNb <= nbIterations {
        let mut nbLoops: libc::c_int = 0;
        let mut clockStart: clock_t = 0;
        let mut clockDuration: clock_t = 0;
        fprintf(__stderrp,
                b"%1i-%-14.14s : %9u ->\r\x00" as *const u8 as
                    *const libc::c_char, loopNb, inFileName, benchedSize);
        let mut i: libc::c_uint = 0;
        i = 0i32 as libc::c_uint;
        while i < benchedSize {
            *dst.offset(i as isize) = i as libc::c_char;
            i = i.wrapping_add(1)
        }
        nbLoops = 0i32;
        clockStart = clock();
        while clock() == clockStart { }
        clockStart = clock();
        while BMK_clockSpan(clockStart) < (1000000i32 * 2i32) as libc::c_ulong
              {
            cSize =
                FSE_compress_usingCTable(dst as *mut libc::c_void,
                                         FSE_compressBound(benchedSize as
                                                               size_t),
                                         src as *const libc::c_void,
                                         benchedSize as size_t, ct);
            nbLoops += 1
        }
        clockDuration = BMK_clockSpan(clockStart);
        if 0 != FSE_isError(cSize) {
            fprintf(__stderrp,
                    b"!!! Error compressing file %s !!!!    \n\x00" as
                        *const u8 as *const libc::c_char, inFileName);
            break ;
        } else {
            if (clockDuration as libc::c_double) <
                   fastestC * nbLoops as libc::c_double {
                fastestC =
                    clockDuration as libc::c_double /
                        nbLoops as libc::c_double
            }
            ratio =
                cSize as libc::c_double / benchedSize as libc::c_double *
                    100.0f64;
            fprintf(__stderrp,
                    b"%1i-%-14.14s : %9i -> %9i (%5.2f%%),%7.1f MB/s\r\x00" as
                        *const u8 as *const libc::c_char, loopNb, inFileName,
                    benchedSize as libc::c_int, cSize as libc::c_int, ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64);
            let mut i_0: libc::c_uint = 0;
            i_0 = 0i32 as libc::c_uint;
            while i_0 < benchedSize {
                *src.offset(i_0 as isize) = 0i32 as libc::c_char;
                i_0 = i_0.wrapping_add(1)
            }
            nbLoops = 0i32;
            clockStart = clock();
            while clock() == clockStart { }
            clockStart = clock();
            while BMK_clockSpan(clockStart) <
                      (1000000i32 * 2i32) as libc::c_ulong {
                dSize =
                    FSE_decompress_usingDTable(src as *mut libc::c_void,
                                               benchedSize as size_t,
                                               dst as *const libc::c_void,
                                               cSize, dt);
                nbLoops += 1
            }
            clockDuration = BMK_clockSpan(clockStart);
            if 0 != FSE_isError(dSize) {
                fprintf(__stderrp,
                        b"\n!!! Error decompressing file %s !!!!    \n\x00" as
                            *const u8 as *const libc::c_char, inFileName);
                break ;
            } else if dSize != benchedSize as libc::c_ulong {
                fprintf(__stderrp,
                        b"\n!!! Error decompressing file %s !!!!    \n\x00" as
                            *const u8 as *const libc::c_char, inFileName);
                break ;
            } else {
                if (clockDuration as libc::c_double) <
                       fastestD * nbLoops as libc::c_double {
                    fastestD =
                        clockDuration as libc::c_double /
                            nbLoops as libc::c_double
                }
                fprintf(__stderrp,
                        b"%1i-%-14.14s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\r\x00"
                            as *const u8 as *const libc::c_char, loopNb,
                        inFileName, benchedSize as libc::c_int,
                        cSize as libc::c_int, ratio,
                        benchedSize as libc::c_double / fastestC / 1000.0f64,
                        benchedSize as libc::c_double / fastestD / 1000.0f64);
                crcCheck =
                    XXH64(src as *const libc::c_void, benchedSize as size_t,
                          0i32 as libc::c_ulonglong);
                if crcOrig != crcCheck {
                    fprintf(__stderrp,
                            b"\n!!! WARNING !!! %14s : Invalid Checksum : %x != %x\n\x00"
                                as *const u8 as *const libc::c_char,
                            inFileName, crcOrig as libc::c_uint,
                            crcCheck as libc::c_uint);
                    break ;
                } else { loopNb += 1 }
            }
        }
    }
    if crcOrig == crcCheck {
        if ratio < 100.0f64 {
            fprintf(__stderrp,
                    b"%-16.16s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\n\x00"
                        as *const u8 as *const libc::c_char, inFileName,
                    benchedSize as libc::c_int, cSize as libc::c_int, ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64,
                    benchedSize as libc::c_double / fastestD / 1000.0f64);
        } else {
            fprintf(__stderrp,
                    b"%-16.16s : %9i -> %9i (%5.1f%%),%7.1f MB/s ,%7.1f MB/s \n\x00"
                        as *const u8 as *const libc::c_char, inFileName,
                    benchedSize as libc::c_int, cSize as libc::c_int, ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64,
                    benchedSize as libc::c_double / fastestD / 1000.0f64);
        }
    }
    *totalCompressedSize =
        (*totalCompressedSize as
             libc::c_ulonglong).wrapping_add(cSize as libc::c_ulonglong) as
            U64 as U64;
    *totalCompressionTime += fastestC;
    *totalDecompressionTime += fastestD;
    free(ct as *mut libc::c_void);
    free(dt as *mut libc::c_void);
}
// Parameters
#[no_mangle]
pub unsafe extern "C" fn BMK_SetBlocksize(mut bsize: U32) {
    chunkSize = bsize;
    fprintf(__stderrp,
            b"- Blocks %u KB -\n\x00" as *const u8 as *const libc::c_char,
            chunkSize >> 10i32);
}
#[no_mangle]
pub unsafe extern "C" fn BMK_SetNbIterations(mut nbLoops: libc::c_int) {
    nbIterations = nbLoops;
    fprintf(__stderrp,
            b"- %i iterations -\n\x00" as *const u8 as *const libc::c_char,
            nbIterations);
}
#[no_mangle]
pub unsafe extern "C" fn BMK_SetByteCompressor(mut id: libc::c_int) {
    BMK_byteCompressor = id;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_SetTableLog(mut tableLog: libc::c_int) {
    BMK_tableLog = 5i32 + tableLog;
}
unsafe extern "C" fn run_static_initializers() {
    chunkSize = (32i32 as libc::c_uint).wrapping_mul(1u32 << 10i32)
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];