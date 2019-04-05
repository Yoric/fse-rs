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
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn getchar() -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn ftime(_: *mut timeb) -> libc::c_int;
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
    fn HIST_count(
        count: *mut libc::c_uint,
        maxSymbolValuePtr: *mut libc::c_uint,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
    /*-****************************************
     *  FSE simple functions
     ******************************************/
    /* FSE_compress() :
        Compress content of buffer 'src', of size 'srcSize', into destination buffer 'dst'.
        'dst' buffer must be already allocated. Compression runs faster is dstCapacity >= FSE_compressBound(srcSize).
        @return : size of compressed data (<= dstCapacity).
        Special values : if return == 0, srcData is not compressible => Nothing is stored within dst !!!
                         if return == 1, srcData is a single byte symbol * srcSize times. Use RLE compression instead.
                         if FSE_isError(return), compression failed (more details using FSE_getErrorName())
    */
    #[no_mangle]
    fn FSE_compress(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
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
    fn FSE_decompress(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        cSrc: *const libc::c_void,
        cSrcSize: size_t,
    ) -> size_t;
    /* Error Management */
    #[no_mangle]
    fn FSE_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn FSE_getErrorName(code: size_t) -> *const libc::c_char;
    /*-*****************************************
     *  FSE detailed API
     ******************************************/
    /*
    FSE_compress() does the following:
    1. count symbol occurrence from source[] into table count[] (see hist.h)
    2. normalize counters so that sum(count[]) == Power_of_2 (2^tableLog)
    3. save normalized counters to memory buffer using writeNCount()
    4. build encoding table 'CTable' from normalized counters
    5. encode the data stream using encoding table 'CTable'

    FSE_decompress() does the following:
    1. read normalized counters with readNCount()
    2. build decoding table 'DTable' from normalized counters
    3. decode the data stream using decoding table 'DTable'

    The following API allows targeting specific sub-functions for advanced tasks.
    For example, it's possible to compress several blocks using the same 'CTable',
    or to save and provide normalized distribution using external method.
    */
    /* *** COMPRESSION *** */
    /* ! FSE_optimalTableLog():
    dynamically downsize 'tableLog' when conditions are met.
    It saves CPU time, by using smaller tables, while preserving or even improving compression ratio.
    @return : recommended tableLog (necessarily <= 'maxTableLog') */
    #[no_mangle]
    fn FSE_optimalTableLog(
        maxTableLog: libc::c_uint,
        srcSize: size_t,
        maxSymbolValue: libc::c_uint,
    ) -> libc::c_uint;
    /* ! FSE_normalizeCount():
    normalize counts so that sum(count[]) == Power_of_2 (2^tableLog)
    'normalizedCounter' is a table of short, of minimum size (maxSymbolValue+1).
    @return : tableLog,
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_normalizeCount(
        normalizedCounter: *mut libc::c_short,
        tableLog: libc::c_uint,
        count: *const libc::c_uint,
        srcSize: size_t,
        maxSymbolValue: libc::c_uint,
    ) -> size_t;
    /* ! FSE_NCountWriteBound():
    Provides the maximum possible size of an FSE normalized table, given 'maxSymbolValue' and 'tableLog'.
    Typically useful for allocation purpose. */
    #[no_mangle]
    fn FSE_NCountWriteBound(maxSymbolValue: libc::c_uint, tableLog: libc::c_uint) -> size_t;
    /* ! FSE_writeNCount():
    Compactly save 'normalizedCounter' into 'buffer'.
    @return : size of the compressed table,
              or an errorCode, which can be tested using FSE_isError(). */
    #[no_mangle]
    fn FSE_writeNCount(
        buffer: *mut libc::c_void,
        bufferSize: size_t,
        normalizedCounter: *const libc::c_short,
        maxSymbolValue: libc::c_uint,
        tableLog: libc::c_uint,
    ) -> size_t;
    /* ! FSE_compress_usingCTable():
    Compress `src` using `ct` into `dst` which must be already allocated.
    @return : size of compressed data (<= `dstCapacity`),
              or 0 if compressed data could not fit into `dst`,
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_compress_usingCTable(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        src: *const libc::c_void,
        srcSize: size_t,
        ct: *const FSE_CTable,
    ) -> size_t;
    /* !
    Tutorial :
    ----------
    The first step is to count all symbols. FSE_count() does this job very fast.
    Result will be saved into 'count', a table of unsigned int, which must be already allocated, and have 'maxSymbolValuePtr[0]+1' cells.
    'src' is a table of bytes of size 'srcSize'. All values within 'src' MUST be <= maxSymbolValuePtr[0]
    maxSymbolValuePtr[0] will be updated, with its real value (necessarily <= original value)
    FSE_count() will return the number of occurrence of the most frequent symbol.
    This can be used to know if there is a single symbol within 'src', and to quickly evaluate its compressibility.
    If there is an error, the function will return an ErrorCode (which can be tested using FSE_isError()).

    The next step is to normalize the frequencies.
    FSE_normalizeCount() will ensure that sum of frequencies is == 2 ^'tableLog'.
    It also guarantees a minimum of 1 to any Symbol with frequency >= 1.
    You can use 'tableLog'==0 to mean "use default tableLog value".
    If you are unsure of which tableLog value to use, you can ask FSE_optimalTableLog(),
    which will provide the optimal valid tableLog given sourceSize, maxSymbolValue, and a user-defined maximum (0 means "default").

    The result of FSE_normalizeCount() will be saved into a table,
    called 'normalizedCounter', which is a table of signed short.
    'normalizedCounter' must be already allocated, and have at least 'maxSymbolValue+1' cells.
    The return value is tableLog if everything proceeded as expected.
    It is 0 if there is a single symbol within distribution.
    If there is an error (ex: invalid tableLog value), the function will return an ErrorCode (which can be tested using FSE_isError()).

    'normalizedCounter' can be saved in a compact manner to a memory area using FSE_writeNCount().
    'buffer' must be already allocated.
    For guaranteed success, buffer size must be at least FSE_headerBound().
    The result of the function is the number of bytes written into 'buffer'.
    If there is an error, the function will return an ErrorCode (which can be tested using FSE_isError(); ex : buffer size too small).

    'normalizedCounter' can then be used to create the compression table 'CTable'.
    The space required by 'CTable' must be already allocated, using FSE_createCTable().
    You can then use FSE_buildCTable() to fill 'CTable'.
    If there is an error, both functions will return an ErrorCode (which can be tested using FSE_isError()).

    'CTable' can then be used to compress 'src', with FSE_compress_usingCTable().
    Similar to FSE_count(), the convention is that 'src' is assumed to be a table of char of size 'srcSize'
    The function returns the size of compressed data (without header), necessarily <= `dstCapacity`.
    If it returns '0', compressed data could not fit into 'dst'.
    If there is an error, the function will return an ErrorCode (which can be tested using FSE_isError()).
    */
    /* *** DECOMPRESSION *** */
    /* ! FSE_readNCount():
    Read compactly saved 'normalizedCounter' from 'rBuffer'.
    @return : size read from 'rBuffer',
              or an errorCode, which can be tested using FSE_isError().
              maxSymbolValuePtr[0] and tableLogPtr[0] will also be updated with their respective values */
    #[no_mangle]
    fn FSE_readNCount(
        normalizedCounter: *mut libc::c_short,
        maxSymbolValuePtr: *mut libc::c_uint,
        tableLogPtr: *mut libc::c_uint,
        rBuffer: *const libc::c_void,
        rBuffSize: size_t,
    ) -> size_t;
    /* ! FSE_decompress_usingDTable():
    Decompress compressed source `cSrc` of size `cSrcSize` using `dt`
    into `dst` which must be already allocated.
    @return : size of regenerated data (necessarily <= `dstCapacity`),
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_decompress_usingDTable(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
        cSrc: *const libc::c_void,
        cSrcSize: size_t,
        dt: *const FSE_DTable,
    ) -> size_t;
    #[no_mangle]
    fn FSE_buildCTable_raw(ct: *mut FSE_CTable, nbBits: libc::c_uint) -> size_t;
    #[no_mangle]
    fn FSE_buildDTable_raw(dt: *mut FSE_DTable, nbBits: libc::c_uint) -> size_t;
    #[no_mangle]
    fn XXH32(input: *const libc::c_void, length: size_t, seed: libc::c_uint) -> XXH32_hash_t;
    #[no_mangle]
    fn XXH64(input: *const libc::c_void, length: size_t, seed: libc::c_ulonglong) -> XXH64_hash_t;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type int16_t = libc::c_short;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
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
    pub _close: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub _seek:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: fpos_t, _: libc::c_int) -> fpos_t>,
    pub _write: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const libc::c_char,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
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
#[derive(Copy, Clone)]
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
pub type S16 = int16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
/* ! Constructor and Destructor of FSE_CTable.
Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
/* don't allocate that. It's only meant to be more restrictive than void* */
pub type FSE_CTable = libc::c_uint;
/* ! Constructor and Destructor of FSE_DTable.
Note that its size depends on 'tableLog' */
/* don't allocate that. It's just a way to be more restrictive than void* */
pub type FSE_DTable = libc::c_uint;
/* ****************************
*  Simple Hash Functions
******************************/
pub type XXH32_hash_t = libc::c_uint;
pub type XXH64_hash_t = libc::c_ulonglong;
/*
Fuzzer.c
Automated test program for FSE
Copyright (C) Yann Collet 2013-2015

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
- FSE source repository : https://github.com/Cyan4973/FiniteStateEntropy
- Public forum : https://groups.google.com/forum/#!forum/lz4c
*/
/* *****************************
*  Compiler options
******************************/
/* Visual warning */
/* *****************************
*  Include
*******************************/
/* malloc, abs */
/* printf */
/* memset */
/* timeb */
/* **************************************************
*  Constants
***************************************************/
/* **************************************************
*  Macros
***************************************************/
// 0 : no display  // 1: errors  // 2 : + result + interaction + warnings ;  // 3 : + progression;  // 4 : + information
static mut displayLevel: libc::c_uint = 2i32 as libc::c_uint;
/* **************************************************
*  local functions
***************************************************/
unsafe extern "C" fn FUZ_GetMilliStart() -> libc::c_int {
    let mut tb: timeb = timeb {
        time: 0,
        millitm: 0,
        timezone: 0,
        dstflag: 0,
    };
    let mut nCount: libc::c_int = 0;
    ftime(&mut tb);
    nCount = (tb.millitm as libc::c_long
        + (tb.time & 0xfffffi32 as libc::c_long) * 1000i32 as libc::c_long)
        as libc::c_int;
    return nCount;
}
unsafe extern "C" fn FUZ_GetMilliSpan(mut nTimeStart: libc::c_int) -> libc::c_int {
    let mut nSpan: libc::c_int = FUZ_GetMilliStart() - nTimeStart;
    if nSpan < 0i32 {
        nSpan += 0x100000i32 * 1000i32
    }
    return nSpan;
}
unsafe extern "C" fn FUZ_rand(mut src: *mut libc::c_uint) -> libc::c_uint {
    *src = (*src)
        .wrapping_mul(2654435761u32)
        .wrapping_add(2246822519u32);
    return *src >> 11i32;
}
unsafe extern "C" fn generate(
    mut buffer: *mut libc::c_void,
    mut buffSize: size_t,
    mut p: libc::c_double,
    mut seed: *mut U32,
) {
    let mut table: [libc::c_char; 4096] = [
        0i32 as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut remaining: libc::c_int = 4i32 * (1i32 << 10i32);
    let mut pos: libc::c_int = 0i32;
    let mut s: libc::c_int = 0i32;
    let mut op: *mut libc::c_char = buffer as *mut libc::c_char;
    let mut oend: *mut libc::c_char = op.offset(buffSize as isize);
    while 0 != remaining {
        let mut n: libc::c_int = (remaining as libc::c_double * p) as libc::c_int;
        let mut end: libc::c_int = 0;
        if 0 == n {
            n = 1i32
        }
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
        let r: libc::c_int =
            (FUZ_rand(seed) & (4i32 * (1i32 << 10i32) - 1i32) as libc::c_uint) as libc::c_int;
        let fresh1 = op;
        op = op.offset(1);
        *fresh1 = table[r as usize]
    }
}
unsafe extern "C" fn generateNoise(
    mut buffer: *mut libc::c_void,
    mut buffSize: size_t,
    mut seed: *mut U32,
) {
    let mut op: *mut BYTE = buffer as *mut BYTE;
    let oend: *mut BYTE = op.offset(buffSize as isize);
    while op < oend {
        let fresh2 = op;
        op = op.offset(1);
        *fresh2 = FUZ_rand(seed) as BYTE
    }
}
unsafe extern "C" fn FUZ_checkCount(
    mut normalizedCount: *mut libc::c_short,
    mut tableLog: libc::c_int,
    mut maxSV: libc::c_int,
) -> libc::c_int {
    let mut total: libc::c_int = 1i32 << tableLog;
    let mut count: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    if tableLog > 20i32 {
        return -1i32;
    }
    i = 0i32;
    while i <= maxSV {
        count += abs(*normalizedCount.offset(i as isize) as libc::c_int);
        i += 1
    }
    if count != total {
        return -1i32;
    }
    return 0i32;
}
unsafe extern "C" fn FUZ_tests(mut seed: U32, mut totalTest: U32, mut startTestNb: U32) {
    let mut bufferP0: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as *mut BYTE;
    let mut bufferP1: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as *mut BYTE;
    let mut bufferP15: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as *mut BYTE;
    let mut bufferP90: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as *mut BYTE;
    let mut bufferP100: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as *mut BYTE;
    let mut bufferDst: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as *mut BYTE;
    let mut bufferVerif: *mut BYTE =
        malloc((1i32 * (1i32 << 20i32) - 1i32 + 64i32) as libc::c_ulong) as *mut BYTE;
    let mut bufferDstSize: size_t = (1i32 * (1i32 << 20i32) - 1i32 + 64i32) as size_t;
    let mut testNb: libc::c_uint = 0;
    let mut maxSV: libc::c_uint = 0;
    let mut tableLog: libc::c_uint = 0;
    let maxTestSizeMask: size_t = 0x1ffffi32 as size_t;
    let mut rootSeed: U32 = seed;
    let mut time: U32 = FUZ_GetMilliStart() as U32;
    generateNoise(
        bufferP0 as *mut libc::c_void,
        (1i32 * (1i32 << 20i32) - 1i32) as size_t,
        &mut rootSeed,
    );
    generate(
        bufferP1 as *mut libc::c_void,
        (1i32 * (1i32 << 20i32) - 1i32) as size_t,
        0.01f64,
        &mut rootSeed,
    );
    generate(
        bufferP15 as *mut libc::c_void,
        (1i32 * (1i32 << 20i32) - 1i32) as size_t,
        0.15f64,
        &mut rootSeed,
    );
    generate(
        bufferP90 as *mut libc::c_void,
        (1i32 * (1i32 << 20i32) - 1i32) as size_t,
        0.90f64,
        &mut rootSeed,
    );
    memset(
        bufferP100 as *mut libc::c_void,
        FUZ_rand(&mut rootSeed) as BYTE as libc::c_int,
        (1i32 * (1i32 << 20i32) - 1i32) as libc::c_ulong,
    );
    if 0 != startTestNb {
        let mut i: U32 = 0;
        i = 0i32 as U32;
        while i < startTestNb {
            FUZ_rand(&mut rootSeed);
            i = i.wrapping_add(1)
        }
    }
    testNb = startTestNb;
    while testNb < totalTest {
        let mut bufferTest: *mut BYTE = 0 as *mut BYTE;
        let mut tag: libc::c_int = 0i32;
        let mut roundSeed: U32 = rootSeed ^ 0xeda5b371u32;
        FUZ_rand(&mut rootSeed);
        if displayLevel >= 4i32 as libc::c_uint {
            fprintf(
                __stderrp,
                b"\r test %5u  \x00" as *const u8 as *const libc::c_char,
                testNb,
            );
        }
        if FUZ_GetMilliSpan(time as libc::c_int) > 200i32 {
            fprintf(
                __stderrp,
                b"\r test %5u  \x00" as *const u8 as *const libc::c_char,
                testNb,
            );
            time = FUZ_GetMilliStart() as U32
        }
        let mut sizeOrig: size_t = (FUZ_rand(&mut roundSeed) as libc::c_ulong & maxTestSizeMask)
            .wrapping_add(1i32 as libc::c_ulong);
        let mut offset: size_t = (FUZ_rand(&mut roundSeed) as libc::c_ulong).wrapping_rem(
            ((1i32 * (1i32 << 20i32) - 1i32 - 64i32) as libc::c_ulong)
                .wrapping_sub(maxTestSizeMask),
        );
        let mut sizeCompressed: size_t = 0;
        let mut hashOrig: U32 = 0;
        if 0 != FUZ_rand(&mut roundSeed) & 7i32 as libc::c_uint {
            bufferTest = bufferP15.offset(offset as isize)
        } else {
            match FUZ_rand(&mut roundSeed) & 3i32 as libc::c_uint {
                0 => bufferTest = bufferP0.offset(offset as isize),
                1 => bufferTest = bufferP1.offset(offset as isize),
                2 => bufferTest = bufferP90.offset(offset as isize),
                _ => bufferTest = bufferP100.offset(offset as isize),
            }
        }
        if displayLevel >= 4i32 as libc::c_uint {
            let fresh3 = tag;
            tag = tag + 1;
            fprintf(
                __stderrp,
                b"%3i \x00" as *const u8 as *const libc::c_char,
                fresh3,
            );
        }
        hashOrig = XXH32(
            bufferTest as *const libc::c_void,
            sizeOrig,
            0i32 as libc::c_uint,
        );
        sizeCompressed = FSE_compress(
            bufferDst as *mut libc::c_void,
            bufferDstSize,
            bufferTest as *const libc::c_void,
            sizeOrig,
        );
        if 0 != FSE_isError(sizeCompressed) {
            fprintf(
                __stderrp,
                b"Error => \x00" as *const u8 as *const libc::c_char,
            );
            fprintf(
                __stderrp,
                b"Compression failed !\x00" as *const u8 as *const libc::c_char,
            );
            fprintf(
                __stderrp,
                b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                seed,
                testNb,
            );
            exit(-1i32);
        }
        if sizeCompressed > 1i32 as libc::c_ulong {
            let mut errorCode: size_t = 0;
            let mut tooSmallDBuffer: *mut libc::c_void =
                malloc(sizeCompressed.wrapping_sub(1i32 as libc::c_ulong));
            if tooSmallDBuffer.is_null() {
                fprintf(
                    __stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b"Not enough memory for tooSmallDBuffer test\x00" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                    seed,
                    testNb,
                );
                exit(-1i32);
            }
            errorCode = FSE_compress(
                tooSmallDBuffer,
                sizeCompressed.wrapping_sub(1i32 as libc::c_ulong),
                bufferTest as *const libc::c_void,
                sizeOrig,
            );
            if errorCode != 0i32 as libc::c_ulong {
                fprintf(
                    __stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b"Compression should have failed : destination buffer too small\x00"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                    seed,
                    testNb,
                );
                exit(-1i32);
            }
            free(tooSmallDBuffer);
            let mut hashEnd: U32 = 0;
            let ref mut fresh4 = *bufferVerif.offset(sizeOrig as isize);
            *fresh4 = 254i32 as BYTE;
            let mut saved: BYTE = *fresh4;
            let mut result: size_t = FSE_decompress(
                bufferVerif as *mut libc::c_void,
                sizeOrig,
                bufferDst as *const libc::c_void,
                sizeCompressed,
            );
            if *bufferVerif.offset(sizeOrig as isize) as libc::c_int != saved as libc::c_int {
                fprintf(
                    __stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b"Output buffer overrun (bufferVerif) : write beyond specified end\x00"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                    seed,
                    testNb,
                );
                exit(-1i32);
            }
            if 0 != FSE_isError(result) {
                fprintf(
                    __stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b"Decompression failed\x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                    seed,
                    testNb,
                );
                exit(-1i32);
            }
            hashEnd = XXH32(
                bufferVerif as *const libc::c_void,
                sizeOrig,
                0i32 as libc::c_uint,
            );
            if hashEnd != hashOrig {
                fprintf(
                    __stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b"Decompressed data corrupted\x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                    seed,
                    testNb,
                );
                exit(-1i32);
            }
        }
        let mut count: [libc::c_short; 256] = [0; 256];
        let mut result_0: size_t = 0;
        if displayLevel >= 4i32 as libc::c_uint {
            let fresh5 = tag;
            tag = tag + 1;
            fprintf(
                __stderrp,
                b"\x08\x08\x08\x08%3i \x00" as *const u8 as *const libc::c_char,
                fresh5,
            );
        }
        maxSV = 255i32 as libc::c_uint;
        result_0 = FSE_readNCount(
            count.as_mut_ptr(),
            &mut maxSV,
            &mut tableLog,
            bufferTest as *const libc::c_void,
            512i32 as size_t,
        );
        if 0 == FSE_isError(result_0) {
            let mut checkCount: libc::c_int = 0;
            if result_0 > 512i32 as libc::c_ulong {
                fprintf(
                    __stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b"FSE_readHeader() reads too far (buffer overflow)\x00" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                    seed,
                    testNb,
                );
                exit(-1i32);
            }
            if maxSV > 255i32 as libc::c_uint {
                fprintf(
                    __stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b"count table overflow (%u)\x00" as *const u8 as *const libc::c_char,
                    maxSV.wrapping_add(1i32 as libc::c_uint),
                );
                fprintf(
                    __stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                    seed,
                    testNb,
                );
                exit(-1i32);
            }
            checkCount = FUZ_checkCount(
                count.as_mut_ptr(),
                tableLog as libc::c_int,
                maxSV as libc::c_int,
            );
            if checkCount == -1i32 {
                fprintf(
                    __stderrp,
                    b"Error => \x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b"symbol distribution corrupted\x00" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    __stderrp,
                    b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                    seed,
                    testNb,
                );
                exit(-1i32);
            }
        }
        let mut maxDstSize: size_t = FUZ_rand(&mut roundSeed) as libc::c_ulong & maxTestSizeMask;
        let mut sizeCompressed_0: size_t =
            FUZ_rand(&mut roundSeed) as libc::c_ulong & maxTestSizeMask;
        let ref mut fresh6 = *bufferDst.offset(maxDstSize as isize);
        *fresh6 = 253i32 as BYTE;
        let mut saved_0: BYTE = *fresh6;
        let mut result_1: size_t = 0;
        if displayLevel >= 4i32 as libc::c_uint {
            let fresh7 = tag;
            tag = tag + 1;
            fprintf(
                __stderrp,
                b"\x08\x08\x08\x08%3i \x00" as *const u8 as *const libc::c_char,
                fresh7,
            );
        }
        result_1 = FSE_decompress(
            bufferDst as *mut libc::c_void,
            maxDstSize,
            bufferTest as *const libc::c_void,
            sizeCompressed_0,
        );
        if 0 == FSE_isError(result_1) && result_1 > maxDstSize {
            fprintf(
                __stderrp,
                b"Error => \x00" as *const u8 as *const libc::c_char,
            );
            fprintf(
                __stderrp,
                b"Decompression overran output buffer\x00" as *const u8 as *const libc::c_char,
            );
            fprintf(
                __stderrp,
                b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                seed,
                testNb,
            );
            exit(-1i32);
        }
        if *bufferDst.offset(maxDstSize as isize) as libc::c_int != saved_0 as libc::c_int {
            fprintf(
                __stderrp,
                b"Error => \x00" as *const u8 as *const libc::c_char,
            );
            fprintf(
                __stderrp,
                b"FSE_decompress on bogus data : bufferDst write overflow\x00" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                __stderrp,
                b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
                seed,
                testNb,
            );
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
unsafe extern "C" fn unitTest() {
    let mut testBuff: *mut BYTE = malloc((16i32 * (1i32 << 10i32)) as libc::c_ulong) as *mut BYTE;
    let mut cBuff: *mut BYTE = malloc(
        (512i32 + (16i32 * (1i32 << 10i32) + (16i32 * (1i32 << 10i32) >> 7i32))) as libc::c_ulong,
    ) as *mut BYTE;
    let mut verifBuff: *mut BYTE = malloc((16i32 * (1i32 << 10i32)) as libc::c_ulong) as *mut BYTE;
    let mut errorCode: size_t = 0;
    let mut seed: U32 = 0i32 as U32;
    let mut testNb: U32 = 0i32 as U32;
    let mut lseed: U32 = 0i32 as U32;
    let mut count: [U32; 256] = [0; 256];
    if testBuff.is_null() || cBuff.is_null() || verifBuff.is_null() {
        fprintf(
            __stderrp,
            b"Not enough memory, exiting ... \n\x00" as *const u8 as *const libc::c_char,
        );
        free(testBuff as *mut libc::c_void);
        free(cBuff as *mut libc::c_void);
        free(verifBuff as *mut libc::c_void);
        return;
    }
    let mut max: U32 = 0;
    let mut i: U32 = 0;
    i = 0i32 as U32;
    while i < (16i32 * (1i32 << 10i32)) as libc::c_uint {
        *testBuff.offset(i as isize) = (FUZ_rand(&mut lseed) & 63i32 as libc::c_uint)
            .wrapping_add('0' as i32 as libc::c_uint)
            as BYTE;
        i = i.wrapping_add(1)
    }
    max = ('0' as i32 + 63i32) as U32;
    errorCode = HIST_count(
        count.as_mut_ptr(),
        &mut max,
        testBuff as *const libc::c_void,
        (16i32 * (1i32 << 10i32)) as size_t,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_count() should have worked\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    max = (max as libc::c_uint).wrapping_sub(1i32 as libc::c_uint) as U32 as U32;
    errorCode = HIST_count(
        count.as_mut_ptr(),
        &mut max,
        testBuff as *const libc::c_void,
        (16i32 * (1i32 << 10i32)) as size_t,
    );
    if 0 == FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_count() should have failed : value > max\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    max = 65000i32 as U32;
    errorCode = HIST_count(
        count.as_mut_ptr(),
        &mut max,
        testBuff as *const libc::c_void,
        (16i32 * (1i32 << 10i32)) as size_t,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_count() should have worked\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    let mut max_0: U32 = 0;
    let mut i_0: U32 = 0;
    let mut tableLog: U32 = 12i32 as U32;
    let mut testSize: size_t = 999i32 as size_t;
    i_0 = 0i32 as U32;
    while (i_0 as libc::c_ulong) < testSize {
        *testBuff.offset(i_0 as isize) = FUZ_rand(&mut lseed) as BYTE;
        i_0 = i_0.wrapping_add(1)
    }
    max_0 = 256i32 as U32;
    HIST_count(
        count.as_mut_ptr(),
        &mut max_0,
        testBuff as *const libc::c_void,
        testSize,
    );
    tableLog = FSE_optimalTableLog(tableLog, testSize, max_0);
    if tableLog <= 8i32 as libc::c_uint {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Too small tableLog\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    let mut norm: [S16; 256] = [0; 256];
    let mut max_1: U32 = 256i32 as U32;
    HIST_count(
        count.as_mut_ptr(),
        &mut max_1,
        testBuff as *const libc::c_void,
        (16i32 * (1i32 << 10i32)) as size_t,
    );
    errorCode = FSE_normalizeCount(
        norm.as_mut_ptr(),
        10i32 as libc::c_uint,
        count.as_mut_ptr(),
        (16i32 * (1i32 << 10i32)) as size_t,
        max_1,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_normalizeCount() should have worked\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    errorCode = FSE_normalizeCount(
        norm.as_mut_ptr(),
        8i32 as libc::c_uint,
        count.as_mut_ptr(),
        (16i32 * (1i32 << 10i32)) as size_t,
        256i32 as libc::c_uint,
    );
    if 0 == FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_normalizeCount() should have failed (max >= 1<<tableLog)\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    let mut i_1: U32 = 0;
    let mut total: U32 = 0i32 as U32;
    count[0usize] = 940i32 as U32;
    count[1usize] = 910i32 as U32;
    count[2usize] = 470i32 as U32;
    count[3usize] = 190i32 as U32;
    count[4usize] = 90i32 as U32;
    i_1 = 5i32 as U32;
    while i_1 <= 255i32 as libc::c_uint {
        count[i_1 as usize] = 6i32 as U32;
        i_1 = i_1.wrapping_add(1)
    }
    i_1 = 0i32 as U32;
    while i_1 <= 255i32 as libc::c_uint {
        total = (total as libc::c_uint).wrapping_add(count[i_1 as usize]) as U32 as U32;
        i_1 = i_1.wrapping_add(1)
    }
    errorCode = FSE_normalizeCount(
        norm.as_mut_ptr(),
        10i32 as libc::c_uint,
        count.as_mut_ptr(),
        total as size_t,
        255i32 as libc::c_uint,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_normalizeCount() should have worked\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    count[0usize] = 300i32 as U32;
    count[1usize] = 300i32 as U32;
    count[2usize] = 300i32 as U32;
    count[3usize] = 300i32 as U32;
    count[4usize] = 50i32 as U32;
    i_1 = 5i32 as U32;
    while i_1 <= 80i32 as libc::c_uint {
        count[i_1 as usize] = 4i32 as U32;
        i_1 = i_1.wrapping_add(1)
    }
    total = 0i32 as U32;
    i_1 = 0i32 as U32;
    while i_1 <= 80i32 as libc::c_uint {
        total = (total as libc::c_uint).wrapping_add(count[i_1 as usize]) as U32 as U32;
        i_1 = i_1.wrapping_add(1)
    }
    errorCode = FSE_normalizeCount(
        norm.as_mut_ptr(),
        10i32 as libc::c_uint,
        count.as_mut_ptr(),
        total as size_t,
        80i32 as libc::c_uint,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_normalizeCount() should have worked\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    let mut i_2: U32 = 0i32 as U32;
    i_2 = 0i32 as U32;
    while i_2 < 22i32 as libc::c_uint {
        count[i_2 as usize] = 0i32 as U32;
        i_2 = i_2.wrapping_add(1)
    }
    while i_2 < 44i32 as libc::c_uint {
        count[i_2 as usize] = 1i32 as U32;
        i_2 = i_2.wrapping_add(1)
    }
    errorCode = FSE_normalizeCount(
        norm.as_mut_ptr(),
        5i32 as libc::c_uint,
        count.as_mut_ptr(),
        22i32 as size_t,
        43i32 as libc::c_uint,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_normalizeCount() should have worked\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    let mut norm_0: [S16; 129] = [0; 129];
    let mut header: [BYTE; 513] = [0; 513];
    let mut max_2: U32 = 0;
    let mut tableLog_0: U32 = 0;
    let mut i_3: U32 = 0;
    let mut headerSize: size_t = 0;
    i_3 = 0i32 as U32;
    while i_3 < (16i32 * (1i32 << 10i32)) as libc::c_uint {
        *testBuff.offset(i_3 as isize) = i_3.wrapping_rem(127i32 as libc::c_uint) as BYTE;
        i_3 = i_3.wrapping_add(1)
    }
    max_2 = 128i32 as U32;
    errorCode = HIST_count(
        count.as_mut_ptr(),
        &mut max_2,
        testBuff as *const libc::c_void,
        (16i32 * (1i32 << 10i32)) as size_t,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_count() should have worked\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    tableLog_0 = FSE_optimalTableLog(
        0i32 as libc::c_uint,
        (16i32 * (1i32 << 10i32)) as size_t,
        max_2,
    );
    errorCode = FSE_normalizeCount(
        norm_0.as_mut_ptr(),
        tableLog_0,
        count.as_mut_ptr(),
        (16i32 * (1i32 << 10i32)) as size_t,
        max_2,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_normalizeCount() should have worked\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    headerSize = FSE_NCountWriteBound(max_2, tableLog_0);
    if headerSize > 513i32 as libc::c_ulong {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : not enough memory for NCount\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    headerSize = FSE_writeNCount(
        header.as_mut_ptr() as *mut libc::c_void,
        headerSize,
        norm_0.as_mut_ptr(),
        max_2,
        tableLog_0,
    );
    if 0 != FSE_isError(headerSize) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_writeNCount() should have worked\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    header[headerSize.wrapping_sub(1i32 as libc::c_ulong) as usize] = 0i32 as BYTE;
    errorCode = FSE_writeNCount(
        header.as_mut_ptr() as *mut libc::c_void,
        headerSize.wrapping_sub(1i32 as libc::c_ulong),
        norm_0.as_mut_ptr(),
        max_2,
        tableLog_0,
    );
    if 0 == FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_writeNCount() should have failed\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    if header[headerSize.wrapping_sub(1i32 as libc::c_ulong) as usize] as libc::c_int != 0i32 {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_writeNCount() buffer overwrite\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    errorCode = FSE_writeNCount(
        header.as_mut_ptr() as *mut libc::c_void,
        headerSize.wrapping_add(1i32 as libc::c_ulong),
        norm_0.as_mut_ptr(),
        max_2,
        tableLog_0,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_writeNCount() should have worked\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    let mut maxN: libc::c_uint = 128i32 as libc::c_uint;
    let err: size_t = FSE_readNCount(
        norm_0.as_mut_ptr(),
        &mut maxN,
        &mut tableLog_0,
        header.as_mut_ptr() as *const libc::c_void,
        headerSize,
    );
    if 0 != FSE_isError(err) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_readNCount() should have worked : (error %s)\x00" as *const u8
                as *const libc::c_char,
            FSE_getErrorName(err),
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    max_2 = 64i32 as U32;
    errorCode = FSE_readNCount(
        norm_0.as_mut_ptr(),
        &mut max_2,
        &mut tableLog_0,
        header.as_mut_ptr() as *const libc::c_void,
        headerSize,
    );
    if 0 == FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_readNCount() should have failed (max too small)\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    max_2 = 128i32 as U32;
    errorCode = FSE_readNCount(
        norm_0.as_mut_ptr(),
        &mut max_2,
        &mut tableLog_0,
        header.as_mut_ptr() as *const libc::c_void,
        headerSize.wrapping_sub(1i32 as libc::c_ulong),
    );
    if 0 == FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_readNCount() should have failed (size too small)\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    let mut smallBuffer: *mut libc::c_void = malloc(headerSize.wrapping_sub(1i32 as libc::c_ulong));
    if smallBuffer.is_null() {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : Not enough memory (FSE_readNCount unit test)\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    memcpy(
        smallBuffer,
        header.as_mut_ptr() as *const libc::c_void,
        headerSize.wrapping_sub(1i32 as libc::c_ulong),
    );
    max_2 = 129i32 as U32;
    errorCode = FSE_readNCount(
        norm_0.as_mut_ptr(),
        &mut max_2,
        &mut tableLog_0,
        smallBuffer,
        headerSize.wrapping_sub(1i32 as libc::c_ulong),
    );
    if 0 == FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Error : FSE_readNCount() should have failed (size too small)\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    free(smallBuffer);
    let mut ct: [U32; 643] = [0; 643];
    let mut dt: [U32; 257] = [0; 257];
    let mut crcOrig: U64 = 0;
    let mut crcVerif: U64 = 0;
    let mut cSize: size_t = 0;
    let mut verifSize: size_t = 0;
    let mut i_4: U32 = 0;
    i_4 = 0i32 as U32;
    while i_4 < (16i32 * (1i32 << 10i32)) as libc::c_uint {
        *testBuff.offset(i_4 as isize) = (FUZ_rand(&mut seed) & 63i32 as libc::c_uint)
            .wrapping_add('0' as i32 as libc::c_uint)
            as BYTE;
        i_4 = i_4.wrapping_add(1)
    }
    crcOrig = XXH64(
        testBuff as *const libc::c_void,
        (16i32 * (1i32 << 10i32)) as size_t,
        0i32 as libc::c_ulonglong,
    );
    errorCode = FSE_buildCTable_raw(ct.as_mut_ptr(), 8i32 as libc::c_uint);
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"FSE_buildCTable_raw should have worked\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    errorCode = FSE_buildDTable_raw(dt.as_mut_ptr(), 8i32 as libc::c_uint);
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"FSE_buildDTable_raw should have worked\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    cSize = FSE_compress_usingCTable(
        cBuff as *mut libc::c_void,
        (512i32 + (16i32 * (1i32 << 10i32) + (16i32 * (1i32 << 10i32) >> 7i32))) as size_t,
        testBuff as *const libc::c_void,
        (16i32 * (1i32 << 10i32)) as size_t,
        ct.as_mut_ptr(),
    );
    if 0 != FSE_isError(cSize) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"FSE_compress_usingCTable should have worked using raw CTable\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    verifSize = FSE_decompress_usingDTable(
        verifBuff as *mut libc::c_void,
        (16i32 * (1i32 << 10i32)) as size_t,
        cBuff as *const libc::c_void,
        cSize,
        dt.as_mut_ptr(),
    );
    if 0 != FSE_isError(verifSize) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"FSE_decompress_usingDTable should have worked using raw DTable\x00" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    crcVerif = XXH64(
        verifBuff as *const libc::c_void,
        verifSize,
        0i32 as libc::c_ulonglong,
    );
    if crcOrig != crcVerif {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Raw regenerated data is corrupted\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    let mut sample8: [BYTE; 8] = [
        0i32 as BYTE,
        0i32 as BYTE,
        0i32 as BYTE,
        2i32 as BYTE,
        0i32 as BYTE,
        0i32 as BYTE,
        0i32 as BYTE,
        0i32 as BYTE,
    ];
    let mut rBuff: *mut BYTE = 0 as *mut BYTE;
    errorCode = FSE_compress(
        cBuff as *mut libc::c_void,
        (16i32 * (1i32 << 10i32)) as size_t,
        sample8.as_mut_ptr() as *const libc::c_void,
        8i32 as size_t,
    );
    if 0 != FSE_isError(errorCode) {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"FSE_compress failed compressing sample8\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    rBuff = malloc(errorCode) as *mut BYTE;
    if rBuff.is_null() {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"Not enough memory for rBuff\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    memcpy(
        rBuff as *mut libc::c_void,
        cBuff as *const libc::c_void,
        errorCode,
    );
    errorCode = FSE_decompress(
        verifBuff as *mut libc::c_void,
        ::std::mem::size_of::<[BYTE; 8]>() as libc::c_ulong,
        rBuff as *const libc::c_void,
        errorCode,
    );
    if errorCode != ::std::mem::size_of::<[BYTE; 8]>() as libc::c_ulong {
        fprintf(
            __stderrp,
            b"Error => \x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b"FSE_decompress failed regenerating sample8\x00" as *const u8 as *const libc::c_char,
        );
        fprintf(
            __stderrp,
            b" (seed %u, test nb %u)  \n\x00" as *const u8 as *const libc::c_char,
            seed,
            testNb,
        );
        exit(-1i32);
    }
    free(rBuff as *mut libc::c_void);
    free(testBuff as *mut libc::c_void);
    free(cBuff as *mut libc::c_void);
    free(verifBuff as *mut libc::c_void);
    fprintf(
        __stderrp,
        b"Unit tests completed\n\x00" as *const u8 as *const libc::c_char,
    );
}
/* ****************************************************************
*  Command line
*****************************************************************/
#[no_mangle]
pub unsafe extern "C" fn badUsage(mut exename: *const libc::c_char) -> libc::c_int {
    fprintf(
        __stderrp,
        b"wrong parameter\n\x00" as *const u8 as *const libc::c_char,
    );
    return 1i32;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut seed: U32 = 0;
    let mut startTestNb: U32 = 0i32 as U32;
    let mut pause: U32 = 0i32 as U32;
    let mut totalTest: U32 = (128i32 * (1i32 << 10i32)) as U32;
    let mut argNb: libc::c_int = 0;
    seed = (FUZ_GetMilliStart() % 10000i32) as U32;
    if displayLevel >= 1i32 as libc::c_uint {
        fprintf(
            __stderrp,
            b"FSE (%2i bits) automated test\n\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_int * 8i32,
        );
    }
    argNb = 1i32;
    while argNb < argc {
        let mut argument: *mut libc::c_char = *argv.offset(argNb as isize);
        if *argument.offset(0isize) as libc::c_int == '-' as i32 {
            argument = argument.offset(1isize);
            while *argument.offset(0isize) as libc::c_int != 0i32 {
                match *argument.offset(0isize) as libc::c_int {
                    115 => {
                        argument = argument.offset(1isize);
                        seed = 0i32 as U32;
                        while *argument as libc::c_int >= '0' as i32
                            && *argument as libc::c_int <= '9' as i32
                        {
                            seed = (seed as libc::c_uint).wrapping_mul(10i32 as libc::c_uint) as U32
                                as U32;
                            seed = (seed as libc::c_uint).wrapping_add(
                                (*argument as libc::c_int - '0' as i32) as libc::c_uint,
                            ) as U32 as U32;
                            argument = argument.offset(1isize)
                        }
                    }
                    105 => {
                        argument = argument.offset(1isize);
                        totalTest = 0i32 as U32;
                        while *argument as libc::c_int >= '0' as i32
                            && *argument as libc::c_int <= '9' as i32
                        {
                            totalTest = (totalTest as libc::c_uint)
                                .wrapping_mul(10i32 as libc::c_uint)
                                as U32 as U32;
                            totalTest = (totalTest as libc::c_uint).wrapping_add(
                                (*argument as libc::c_int - '0' as i32) as libc::c_uint,
                            ) as U32 as U32;
                            argument = argument.offset(1isize)
                        }
                    }
                    116 => {
                        argument = argument.offset(1isize);
                        startTestNb = 0i32 as U32;
                        while *argument as libc::c_int >= '0' as i32
                            && *argument as libc::c_int <= '9' as i32
                        {
                            startTestNb = (startTestNb as libc::c_uint)
                                .wrapping_mul(10i32 as libc::c_uint)
                                as U32 as U32;
                            startTestNb = (startTestNb as libc::c_uint).wrapping_add(
                                (*argument as libc::c_int - '0' as i32) as libc::c_uint,
                            ) as U32 as U32;
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
                    _ => return badUsage(*argv.offset(0isize)),
                }
            }
        }
        argNb += 1
    }
    if startTestNb == 0i32 as libc::c_uint {
        unitTest();
    }
    fprintf(
        __stderrp,
        b"Fuzzer seed : %u \n\x00" as *const u8 as *const libc::c_char,
        seed,
    );
    FUZ_tests(seed, totalTest, startTestNb);
    fprintf(
        __stderrp,
        b"\rAll %u tests passed               \n\x00" as *const u8 as *const libc::c_char,
        totalTest,
    );
    if 0 != pause {
        let mut unused: libc::c_int = 0;
        fprintf(
            __stderrp,
            b"press enter ...\n\x00" as *const u8 as *const libc::c_char,
        );
        unused = getchar()
    }
    return 0i32;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
