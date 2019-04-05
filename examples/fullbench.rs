#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(asm, extern_types, libc)]
extern crate core;
extern crate libc;
extern "C" {
    pub type __sFILEX;
    pub type HUF_CElt_s;
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
    fn getchar() -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
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
    /* * HIST_countFast() :
 *  same as HIST_count(), but blindly trusts that all byte values within src are <= *maxSymbolValuePtr.
 *  This function is unsafe, and will segfault if any value within `src` is `> *maxSymbolValuePtr`
 */
    #[no_mangle]
    fn HIST_countFast(count: *mut libc::c_uint,
                      maxSymbolValuePtr: *mut libc::c_uint,
                      src: *const libc::c_void, srcSize: size_t) -> size_t;
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
    fn FSE_compress(dst: *mut libc::c_void, dstCapacity: size_t,
                    src: *const libc::c_void, srcSize: size_t) -> size_t;
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
*  Tool functions
******************************************/
    #[no_mangle]
    fn FSE_compressBound(size: size_t) -> size_t;
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
    fn FSE_optimalTableLog(maxTableLog: libc::c_uint, srcSize: size_t,
                           maxSymbolValue: libc::c_uint) -> libc::c_uint;
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
    /* ! FSE_writeNCount():
    Compactly save 'normalizedCounter' into 'buffer'.
    @return : size of the compressed table,
              or an errorCode, which can be tested using FSE_isError(). */
    #[no_mangle]
    fn FSE_writeNCount(buffer: *mut libc::c_void, bufferSize: size_t,
                       normalizedCounter: *const libc::c_short,
                       maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> size_t;
    /* ! FSE_buildCTable():
    Builds `ct`, which must be already allocated, using FSE_createCTable().
    @return : 0, or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_buildCTable(ct: *mut FSE_CTable,
                       normalizedCounter: *const libc::c_short,
                       maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> size_t;
    /* ! FSE_compress_usingCTable():
    Compress `src` using `ct` into `dst` which must be already allocated.
    @return : size of compressed data (<= `dstCapacity`),
              or 0 if compressed data could not fit into `dst`,
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_compress_usingCTable(dst: *mut libc::c_void, dstCapacity: size_t,
                                src: *const libc::c_void, srcSize: size_t,
                                ct: *const FSE_CTable) -> size_t;
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
    fn FSE_readNCount(normalizedCounter: *mut libc::c_short,
                      maxSymbolValuePtr: *mut libc::c_uint,
                      tableLogPtr: *mut libc::c_uint,
                      rBuffer: *const libc::c_void, rBuffSize: size_t)
     -> size_t;
    /* ! FSE_buildDTable():
    Builds 'dt', which must be already allocated, using FSE_createDTable().
    return : 0, or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_buildDTable(dt: *mut FSE_DTable,
                       normalizedCounter: *const libc::c_short,
                       maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> size_t;
    /* ! FSE_decompress_usingDTable():
    Decompress compressed source `cSrc` of size `cSrcSize` using `dt`
    into `dst` which must be already allocated.
    @return : size of regenerated data (necessarily <= `dstCapacity`),
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_decompress_usingDTable(dst: *mut libc::c_void, dstCapacity: size_t,
                                  cSrc: *const libc::c_void, cSrcSize: size_t,
                                  dt: *const FSE_DTable) -> size_t;
    #[no_mangle]
    fn FSE_buildCTable_raw(ct: *mut FSE_CTable, nbBits: libc::c_uint)
     -> size_t;
    #[no_mangle]
    fn FSE_buildDTable_raw(dt: *mut FSE_DTable, nbBits: libc::c_uint)
     -> size_t;
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
    /* ****************************************
*  Advanced decompression functions
******************************************/
    #[no_mangle]
    fn HUF_decompress4X1(dst: *mut libc::c_void, dstSize: size_t,
                         cSrc: *const libc::c_void, cSrcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn HUF_decompress4X2(dst: *mut libc::c_void, dstSize: size_t,
                         cSrc: *const libc::c_void, cSrcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn HUF_buildCTable(CTable: *mut HUF_CElt, count: *const libc::c_uint,
                       maxSymbolValue: libc::c_uint, maxNbBits: libc::c_uint)
     -> size_t;
    #[no_mangle]
    fn HUF_writeCTable(dst: *mut libc::c_void, maxDstSize: size_t,
                       CTable: *const HUF_CElt, maxSymbolValue: libc::c_uint,
                       huffLog: libc::c_uint) -> size_t;
    #[no_mangle]
    fn HUF_compress4X_usingCTable(dst: *mut libc::c_void, dstSize: size_t,
                                  src: *const libc::c_void, srcSize: size_t,
                                  CTable: *const HUF_CElt) -> size_t;
    /* * HUF_compress4X_repeat() :
 *  Same as HUF_compress4X_wksp(), but considers using hufTable if *repeat != HUF_repeat_none.
 *  If it uses hufTable it does not modify hufTable or repeat.
 *  If it doesn't, it sets *repeat = HUF_repeat_none, and it sets hufTable to the table used.
 *  If preferRepeat then the old table will always be used if valid. */
    #[no_mangle]
    fn HUF_compress4X_repeat(dst: *mut libc::c_void, dstSize: size_t,
                             src: *const libc::c_void, srcSize: size_t,
                             maxSymbolValue: libc::c_uint,
                             tableLog: libc::c_uint,
                             workSpace: *mut libc::c_void, wkspSize: size_t,
                             hufTable: *mut HUF_CElt, repeat: *mut HUF_repeat,
                             preferRepeat: libc::c_int, bmi2: libc::c_int)
     -> size_t;
    /* ! HUF_readStats() :
 *  Read compact Huffman tree, saved by HUF_writeCTable().
 * `huffWeight` is destination buffer.
 * @return : size read from `src` , or an error Code .
 *  Note : Needed by HUF_readCTable() and HUF_readDTableXn() . */
    #[no_mangle]
    fn HUF_readStats(huffWeight: *mut BYTE, hwSize: size_t,
                     rankStats: *mut U32, nbSymbolsPtr: *mut U32,
                     tableLogPtr: *mut U32, src: *const libc::c_void,
                     srcSize: size_t) -> size_t;
    /* *
 *  The minimum workspace size for the `workSpace` used in
 *  HUF_readDTableX1_wksp() and HUF_readDTableX2_wksp().
 *
 *  The space used depends on HUF_TABLELOG_MAX, ranging from ~1500 bytes when
 *  HUF_TABLE_LOG_MAX=12 to ~1850 bytes when HUF_TABLE_LOG_MAX=15.
 *  Buffer overflow errors may potentially occur if code modifications result in
 *  a required workspace size greater than that specified in the following
 *  macro.
 */
    #[no_mangle]
    fn HUF_readDTableX1(DTable: *mut HUF_DTable, src: *const libc::c_void,
                        srcSize: size_t) -> size_t;
    #[no_mangle]
    fn HUF_readDTableX2(DTable: *mut HUF_DTable, src: *const libc::c_void,
                        srcSize: size_t) -> size_t;
    #[no_mangle]
    fn HUF_decompress4X1_usingDTable(dst: *mut libc::c_void,
                                     maxDstSize: size_t,
                                     cSrc: *const libc::c_void,
                                     cSrcSize: size_t,
                                     DTable: *const HUF_DTable) -> size_t;
    #[no_mangle]
    fn HUF_decompress4X2_usingDTable(dst: *mut libc::c_void,
                                     maxDstSize: size_t,
                                     cSrc: *const libc::c_void,
                                     cSrcSize: size_t,
                                     DTable: *const HUF_DTable) -> size_t;
    #[no_mangle]
    fn HUF_compress1X_usingCTable(dst: *mut libc::c_void, dstSize: size_t,
                                  src: *const libc::c_void, srcSize: size_t,
                                  CTable: *const HUF_CElt) -> size_t;
    #[no_mangle]
    fn HUF_decompress1X1(dst: *mut libc::c_void, dstSize: size_t,
                         cSrc: *const libc::c_void, cSrcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn HUF_decompress1X2(dst: *mut libc::c_void, dstSize: size_t,
                         cSrc: *const libc::c_void, cSrcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn HUF_decompress1X1_usingDTable(dst: *mut libc::c_void,
                                     maxDstSize: size_t,
                                     cSrc: *const libc::c_void,
                                     cSrcSize: size_t,
                                     DTable: *const HUF_DTable) -> size_t;
    #[no_mangle]
    fn HUF_decompress1X2_usingDTable(dst: *mut libc::c_void,
                                     maxDstSize: size_t,
                                     cSrc: *const libc::c_void,
                                     cSrcSize: size_t,
                                     DTable: *const HUF_DTable) -> size_t;
    /* BMI2 variants.
 * If the CPU has BMI2 support, pass bmi2=1, otherwise pass bmi2=0.
 */
    #[no_mangle]
    fn HUF_decompress1X_usingDTable_bmi2(dst: *mut libc::c_void,
                                         maxDstSize: size_t,
                                         cSrc: *const libc::c_void,
                                         cSrcSize: size_t,
                                         DTable: *const HUF_DTable,
                                         bmi2: libc::c_int) -> size_t;
    #[no_mangle]
    fn HUF_decompress4X_usingDTable_bmi2(dst: *mut libc::c_void,
                                         maxDstSize: size_t,
                                         cSrc: *const libc::c_void,
                                         cSrcSize: size_t,
                                         DTable: *const HUF_DTable,
                                         bmi2: libc::c_int) -> size_t;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_clock_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
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
pub type clock_t = __darwin_clock_t;
/*-**************************************************************
*  Basic Types
*****************************************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct unalign64 {
    pub v: U64,
}
/*
 * Copyright (c) 2018-present, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* *
 * Implementation taken from folly/CpuId.h
 * https://github.com/facebook/folly/blob/master/folly/CpuId.h
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_cpuid_t {
    pub f1c: U32,
    pub f1d: U32,
    pub f7b: U32,
    pub f7c: U32,
}
/* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
/* don't allocate that. It's only meant to be more restrictive than void* */
pub type FSE_CTable = libc::c_uint;
/* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
/* don't allocate that. It's just a way to be more restrictive than void* */
pub type FSE_DTable = libc::c_uint;
/* HUF_H_298734234 */
/* ******************************************************************
 *  WARNING !!
 *  The following section contains advanced and experimental definitions
 *  which shall never be used in the context of a dynamic library,
 *  because they are not guaranteed to remain stable in the future.
 *  Only consider them in association with static linking.
 * *****************************************************************/
/* *** Dependencies *** */
/* U32 */
/* *** Constants *** */
/* max runtime value of tableLog (due to static allocation); can be modified up to HUF_ABSOLUTEMAX_TABLELOG */
/* default tableLog value when none specified */
/* absolute limit of HUF_MAX_TABLELOG. Beyond that value, code does not work */
/* ****************************************
*  Static allocation
******************************************/
/* HUF buffer bounds */
/* only true when incompressible is pre-filtered with fast heuristic */
/* Macro version, useful for static allocation */
/* static allocation of HUF's Compression Table */
/* Use tables of U32, for proper alignment */
/* no final ; */
/* static allocation of HUF's DTable */
pub type HUF_DTable = U32;
/* incomplete type */
pub type HUF_CElt = HUF_CElt_s;
pub type HUF_repeat = libc::c_uint;
/* *< Can use the previous table and it is assumed to be valid */
pub const HUF_repeat_valid: HUF_repeat = 2;
/* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
pub const HUF_repeat_check: HUF_repeat = 1;
/* *< Cannot use the previous table */
pub const HUF_repeat_none: HUF_repeat = 0;
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> U64 {
    return (*(ptr as *const unalign64)).v;
}


unsafe extern "C" fn ZSTD_cpuid() -> ZSTD_cpuid_t {
    use core::arch::x86_64::__cpuid;

    let mut f1c = 0;
    let mut f1d = 0;
    let mut f7b = 0;
    let mut f7c = 0;

    // Find out which APIs are implemented.
    let cpuid = __cpuid(0);
    if cpuid.eax >= 1 {
        let cpuid = __cpuid(1);
        f1c = cpuid.ecx;
        f1d = cpuid.edx;
    }
    if cpuid.eax >= 7 {
        let cpuid = __cpuid(7);
        f7b = cpuid.ebx;
        f7c = cpuid.ecx;
    }

    return ZSTD_cpuid_t {
        f1c,
        f1d,
        f7b,
        f7c
    };
}

unsafe extern "C" fn ZSTD_cpuid_bmi2(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 8i32 != 0i32 as libc::c_uint) as libc::c_int;
}
/*
    fullbench.c - Demo program to benchmark open-source compression algorithm
    Copyright (C) Yann Collet 2012-2015

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
    - public forum : https://groups.google.com/forum/#!forum/lz4c
    - website : http://fastcompression.blogspot.com/
*/
/*=====   Compiler's specifics   =====*/
/* Remove Visual warning */
/*_************************************
*  Includes
**************************************/
/* malloc */
/* fprintf, fopen, ftello64 */
/* strcmp */
/* clock_t, clock, CLOCKS_PER_SEC */
/*_************************************
*  Constants
**************************************/
/*_************************************
*  Macros
***************************************/
/*_************************************
*  Benchmark Parameters
***************************************/
static mut no_prompt: U32 = 0i32 as U32;
/*_*******************************************************
*  Private functions
**********************************************************/
unsafe extern "C" fn BMK_clockSpan(mut clockStart: clock_t) -> clock_t {
    return clock().wrapping_sub(clockStart);
}
unsafe extern "C" fn BMK_rand(mut seed: *mut U32) -> U32 {
    *seed = (*seed).wrapping_mul(2654435761u32).wrapping_add(2246822519u32);
    return *seed >> 11i32;
}
unsafe extern "C" fn BMK_genData(mut buffer: *mut libc::c_void,
                                 mut buffSize: size_t,
                                 mut p: libc::c_double) {
    let mut table: [libc::c_char; 2048] =
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
         0, 0, 0, 0, 0, 0, 0];
    let mut remaining: libc::c_int = 2048i32;
    let mut pos: libc::c_uint = 0i32 as libc::c_uint;
    let mut s: libc::c_uint = 0i32 as libc::c_uint;
    let mut op: *mut libc::c_char = buffer as *mut libc::c_char;
    let mut oend: *mut libc::c_char = op.offset(buffSize as isize);
    let mut seed: libc::c_uint = 1i32 as libc::c_uint;
    static mut done: libc::c_uint = 0i32 as libc::c_uint;
    if p < 0.01f64 { p = 0.005f64 }
    if p > 1.0f64 { p = 1.0f64 }
    if 0 == done {
        done = 1i32 as libc::c_uint;
        fprintf(__stderrp,
                b"Generating %i KB with P=%.2f%%\n\x00" as *const u8 as
                    *const libc::c_char, (buffSize >> 10i32) as libc::c_int,
                p * 100i32 as libc::c_double);
    }
    while 0 != remaining {
        let mut n: libc::c_uint =
            (remaining as libc::c_double * p) as libc::c_uint;
        let mut end: libc::c_uint = 0;
        if 0 == n { n = 1i32 as libc::c_uint }
        end = pos.wrapping_add(n);
        while pos < end {
            let fresh0 = pos;
            pos = pos.wrapping_add(1);
            table[fresh0 as usize] = s as libc::c_char
        }
        s = s.wrapping_add(1);
        if s == 255i32 as libc::c_uint { s = 0i32 as libc::c_uint }
        remaining =
            (remaining as libc::c_uint).wrapping_sub(n) as libc::c_int as
                libc::c_int
    }
    while op < oend {
        let r: libc::c_uint =
            BMK_rand(&mut seed) & (2048i32 - 1i32) as libc::c_uint;
        let fresh1 = op;
        op = op.offset(1);
        *fresh1 = table[r as usize]
    };
}
/*_*******************************************************
*  Benchmark function
**********************************************************/
unsafe extern "C" fn local_trivialCount(mut dst: *mut libc::c_void,
                                        mut dstSize: size_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> libc::c_int {
    let mut count: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut ip: *const BYTE = src as *const BYTE;
    let end: *const BYTE = ip.offset(srcSize as isize);
    while ip < end {
        let fresh2 = ip;
        ip = ip.offset(1);
        count[*fresh2 as usize] = count[*fresh2 as usize].wrapping_add(1)
    }
    return count[*ip.offset(-1i32 as isize) as usize] as libc::c_int;
}
unsafe extern "C" fn local_count8(mut dst: *mut libc::c_void,
                                  mut dstSize: size_t,
                                  mut src: *const libc::c_void,
                                  mut srcSize: size_t) -> libc::c_int {
    let mut count: [[U32; 256]; 8] = [[0; 256]; 8];
    let mut ip: *const BYTE = src as *const BYTE;
    let end: *const BYTE =
        ip.offset(srcSize as isize).offset(-((8i32 - 1i32) as isize));
    memset(count.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[[U32; 256]; 8]>() as libc::c_ulong);
    while ip < end {
        let mut idx: libc::c_uint = 0;
        idx = 0i32 as libc::c_uint;
        while idx < 8i32 as libc::c_uint {
            let fresh3 = ip;
            ip = ip.offset(1);
            count[idx as usize][*fresh3 as usize] =
                count[idx as usize][*fresh3 as usize].wrapping_add(1);
            idx = idx.wrapping_add(1)
        }
    }
    let mut n: libc::c_uint = 0;
    n = 0i32 as libc::c_uint;
    while n < 256i32 as libc::c_uint {
        let mut idx_0: libc::c_uint = 0;
        idx_0 = 1i32 as libc::c_uint;
        while idx_0 < 8i32 as libc::c_uint {
            count[0usize][n as usize] =
                (count[0usize][n as usize] as
                     libc::c_uint).wrapping_add(count[idx_0 as
                                                          usize][n as usize])
                    as U32 as U32;
            idx_0 = idx_0.wrapping_add(1)
        }
        n = n.wrapping_add(1)
    }
    return count[0usize][*ip.offset(-1i32 as isize) as usize] as libc::c_int;
}
/* U64 version */
unsafe extern "C" fn local_count8v2(mut dst: *mut libc::c_void,
                                    mut dstSize: size_t,
                                    mut src: *const libc::c_void,
                                    mut srcSize: size_t) -> libc::c_int {
    let mut count: [[U32; 272]; 8] = [[0; 272]; 8];
    let mut ptr: *const U64 = src as *const U64;
    let mut end: *const U64 = ptr.offset((srcSize >> 3i32) as isize);
    let fresh4 = ptr;
    ptr = ptr.offset(1);
    let mut next: U64 = *fresh4;
    memset(count.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[[U32; 272]; 8]>() as libc::c_ulong);
    while ptr != end {
        let mut bs: U64 = next;
        let fresh5 = ptr;
        ptr = ptr.offset(1);
        next = *fresh5;
        count[0usize][bs as BYTE as usize] =
            count[0usize][bs as BYTE as usize].wrapping_add(1);
        count[1usize][(bs >> 8i32) as BYTE as usize] =
            count[1usize][(bs >> 8i32) as BYTE as usize].wrapping_add(1);
        count[2usize][(bs >> 16i32) as BYTE as usize] =
            count[2usize][(bs >> 16i32) as BYTE as usize].wrapping_add(1);
        count[3usize][(bs >> 24i32) as BYTE as usize] =
            count[3usize][(bs >> 24i32) as BYTE as usize].wrapping_add(1);
        count[4usize][(bs >> 32i32) as BYTE as usize] =
            count[4usize][(bs >> 32i32) as BYTE as usize].wrapping_add(1);
        count[5usize][(bs >> 40i32) as BYTE as usize] =
            count[5usize][(bs >> 40i32) as BYTE as usize].wrapping_add(1);
        count[6usize][(bs >> 48i32) as BYTE as usize] =
            count[6usize][(bs >> 48i32) as BYTE as usize].wrapping_add(1);
        count[7usize][(bs >> 56i32) as BYTE as usize] =
            count[7usize][(bs >> 56i32) as BYTE as usize].wrapping_add(1)
    }
    let mut u: libc::c_uint = 0;
    u = 0i32 as libc::c_uint;
    while u < 256i32 as libc::c_uint {
        let mut idx: libc::c_uint = 0;
        idx = 1i32 as libc::c_uint;
        while idx < 8i32 as libc::c_uint {
            count[0usize][u as usize] =
                (count[0usize][u as usize] as
                     libc::c_uint).wrapping_add(count[idx as
                                                          usize][u as usize])
                    as U32 as U32;
            idx = idx.wrapping_add(1)
        }
        u = u.wrapping_add(1)
    }
    return count[0usize][0usize] as libc::c_int;
}
/* hist_X_Y function from https://github.com/powturbo/turbohist */
unsafe extern "C" fn local_hist_4_32(mut dst: *mut libc::c_void,
                                     mut dstSize: size_t,
                                     mut src: *const libc::c_void,
                                     mut srcSize: size_t) -> libc::c_int {
    //#define NU 8
    let mut i: libc::c_int = 0;
    let mut count: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut c0: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut c1: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut c2: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut c3: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut ip32: *const U32 = src as *const U32;
    let ip32end: *const U32 = ip32.offset((srcSize >> 2i32) as isize);
    let mut ip: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = ip.offset(srcSize as isize);
    let mut cp: U32 = *ip32;
    while ip32 != ip32end {
        let mut c: U32 = cp;
        ip32 = ip32.offset(1isize);
        cp = *ip32;
        c0[c as libc::c_uchar as usize] =
            c0[c as libc::c_uchar as usize].wrapping_add(1);
        c1[(c >> 8i32) as libc::c_uchar as usize] =
            c1[(c >> 8i32) as libc::c_uchar as usize].wrapping_add(1);
        c2[(c >> 16i32) as libc::c_uchar as usize] =
            c2[(c >> 16i32) as libc::c_uchar as usize].wrapping_add(1);
        c3[(c >> 24i32) as usize] = c3[(c >> 24i32) as usize].wrapping_add(1);
        c = cp;
        ip32 = ip32.offset(1isize);
        cp = *ip32;
        c0[c as libc::c_uchar as usize] =
            c0[c as libc::c_uchar as usize].wrapping_add(1);
        c1[(c >> 8i32) as libc::c_uchar as usize] =
            c1[(c >> 8i32) as libc::c_uchar as usize].wrapping_add(1);
        c2[(c >> 16i32) as libc::c_uchar as usize] =
            c2[(c >> 16i32) as libc::c_uchar as usize].wrapping_add(1);
        c3[(c >> 24i32) as usize] = c3[(c >> 24i32) as usize].wrapping_add(1);
        c = cp;
        ip32 = ip32.offset(1isize);
        cp = *ip32;
        c0[c as libc::c_uchar as usize] =
            c0[c as libc::c_uchar as usize].wrapping_add(1);
        c1[(c >> 8i32) as libc::c_uchar as usize] =
            c1[(c >> 8i32) as libc::c_uchar as usize].wrapping_add(1);
        c2[(c >> 16i32) as libc::c_uchar as usize] =
            c2[(c >> 16i32) as libc::c_uchar as usize].wrapping_add(1);
        c3[(c >> 24i32) as usize] = c3[(c >> 24i32) as usize].wrapping_add(1);
        c = cp;
        ip32 = ip32.offset(1isize);
        cp = *ip32;
        c0[c as libc::c_uchar as usize] =
            c0[c as libc::c_uchar as usize].wrapping_add(1);
        c1[(c >> 8i32) as libc::c_uchar as usize] =
            c1[(c >> 8i32) as libc::c_uchar as usize].wrapping_add(1);
        c2[(c >> 16i32) as libc::c_uchar as usize] =
            c2[(c >> 16i32) as libc::c_uchar as usize].wrapping_add(1);
        c3[(c >> 24i32) as usize] = c3[(c >> 24i32) as usize].wrapping_add(1)
    }
    ip = ip32 as *const BYTE;
    while ip < iend {
        let fresh6 = ip;
        ip = ip.offset(1);
        c0[*fresh6 as usize] = c0[*fresh6 as usize].wrapping_add(1)
    }
    i = 0i32;
    while i < 256i32 {
        count[i as usize] =
            c0[i as
                   usize].wrapping_add(c1[i as
                                              usize]).wrapping_add(c2[i as
                                                                          usize]).wrapping_add(c3[i
                                                                                                      as
                                                                                                      usize]);
        i += 1
    }
    return count[0usize] as libc::c_int;
}
unsafe extern "C" fn local_hist_4_32v2(mut dst: *mut libc::c_void,
                                       mut dstSize: size_t,
                                       mut src: *const libc::c_void,
                                       mut srcSize: size_t) -> libc::c_int {
    let mut c0: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut c1: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut c2: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut c3: [U32; 256] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0];
    let mut ip32: *const U32 = src as *const U32;
    let ip32end: *const U32 = ip32.offset((srcSize >> 2i32) as isize);
    let mut ip: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = ip.offset(srcSize as isize);
    let mut cp: U32 = *ip32;
    let mut i: libc::c_int = 0;
    while ip32 <= ip32end.offset(-4isize) {
        let mut c: U32 = cp;
        ip32 = ip32.offset(1isize);
        let mut d: U32 = *ip32;
        ip32 = ip32.offset(1isize);
        cp = *ip32;
        c0[c as BYTE as usize] = c0[c as BYTE as usize].wrapping_add(1);
        c1[d as BYTE as usize] = c1[d as BYTE as usize].wrapping_add(1);
        c2[(c >> 8i32) as BYTE as usize] =
            c2[(c >> 8i32) as BYTE as usize].wrapping_add(1);
        c >>= 16i32;
        c3[(d >> 8i32) as BYTE as usize] =
            c3[(d >> 8i32) as BYTE as usize].wrapping_add(1);
        d >>= 16i32;
        c0[c as BYTE as usize] = c0[c as BYTE as usize].wrapping_add(1);
        c1[d as BYTE as usize] = c1[d as BYTE as usize].wrapping_add(1);
        c2[(c >> 8i32) as usize] = c2[(c >> 8i32) as usize].wrapping_add(1);
        c3[(d >> 8i32) as usize] = c3[(d >> 8i32) as usize].wrapping_add(1);
        c = cp;
        ip32 = ip32.offset(1isize);
        d = *ip32;
        ip32 = ip32.offset(1isize);
        cp = *ip32;
        c0[c as BYTE as usize] = c0[c as BYTE as usize].wrapping_add(1);
        c1[d as BYTE as usize] = c1[d as BYTE as usize].wrapping_add(1);
        c2[(c >> 8i32) as BYTE as usize] =
            c2[(c >> 8i32) as BYTE as usize].wrapping_add(1);
        c >>= 16i32;
        c3[(d >> 8i32) as BYTE as usize] =
            c3[(d >> 8i32) as BYTE as usize].wrapping_add(1);
        d >>= 16i32;
        c0[c as BYTE as usize] = c0[c as BYTE as usize].wrapping_add(1);
        c1[d as BYTE as usize] = c1[d as BYTE as usize].wrapping_add(1);
        c2[(c >> 8i32) as usize] = c2[(c >> 8i32) as usize].wrapping_add(1);
        c3[(d >> 8i32) as usize] = c3[(d >> 8i32) as usize].wrapping_add(1)
    }
    ip = ip32 as *const BYTE;
    while ip < iend {
        let fresh7 = ip;
        ip = ip.offset(1);
        c0[*fresh7 as usize] = c0[*fresh7 as usize].wrapping_add(1)
    }
    i = 0i32;
    while i < 256i32 {
        c0[i as usize] =
            (c0[i as usize] as
                 libc::c_uint).wrapping_add(c1[i as
                                                   usize].wrapping_add(c2[i as
                                                                              usize]).wrapping_add(c3[i
                                                                                                          as
                                                                                                          usize]))
                as U32 as U32;
        i += 1
    }
    return c0[0usize] as libc::c_int;
}
unsafe extern "C" fn local_hist_8_32(mut dst: *mut libc::c_void,
                                     mut dstSize: size_t,
                                     mut src: *const libc::c_void,
                                     mut srcSize: size_t) -> libc::c_int {
    let mut c0: [U32; 264] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c1: [U32; 264] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c2: [U32; 264] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c3: [U32; 264] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c4: [U32; 264] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c5: [U32; 264] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c6: [U32; 264] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c7: [U32; 264] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut ip32: *const U32 = src as *const U32;
    let ip32end: *const U32 = ip32.offset((srcSize >> 2i32) as isize);
    let mut ip: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = (src as *const BYTE).offset(srcSize as isize);
    let mut cp: U32 = *(src as *const U32);
    let mut i: libc::c_int = 0;
    while ip32 <= ip32end.offset(-4isize) {
        let mut c: U32 = cp;
        ip32 = ip32.offset(1isize);
        let mut d: U32 = *ip32;
        ip32 = ip32.offset(1isize);
        cp = *ip32;
        c0[c as libc::c_uchar as usize] =
            c0[c as libc::c_uchar as usize].wrapping_add(1);
        c1[d as libc::c_uchar as usize] =
            c1[d as libc::c_uchar as usize].wrapping_add(1);
        c2[(c >> 8i32) as libc::c_uchar as usize] =
            c2[(c >> 8i32) as libc::c_uchar as usize].wrapping_add(1);
        c >>= 16i32;
        c3[(d >> 8i32) as libc::c_uchar as usize] =
            c3[(d >> 8i32) as libc::c_uchar as usize].wrapping_add(1);
        d >>= 16i32;
        c4[c as libc::c_uchar as usize] =
            c4[c as libc::c_uchar as usize].wrapping_add(1);
        c5[d as libc::c_uchar as usize] =
            c5[d as libc::c_uchar as usize].wrapping_add(1);
        c6[(c >> 8i32) as usize] = c6[(c >> 8i32) as usize].wrapping_add(1);
        c7[(d >> 8i32) as usize] = c7[(d >> 8i32) as usize].wrapping_add(1);
        c = cp;
        ip32 = ip32.offset(1isize);
        d = *ip32;
        ip32 = ip32.offset(1isize);
        cp = *ip32;
        c0[c as libc::c_uchar as usize] =
            c0[c as libc::c_uchar as usize].wrapping_add(1);
        c1[d as libc::c_uchar as usize] =
            c1[d as libc::c_uchar as usize].wrapping_add(1);
        c2[(c >> 8i32) as libc::c_uchar as usize] =
            c2[(c >> 8i32) as libc::c_uchar as usize].wrapping_add(1);
        c >>= 16i32;
        c3[(d >> 8i32) as libc::c_uchar as usize] =
            c3[(d >> 8i32) as libc::c_uchar as usize].wrapping_add(1);
        d >>= 16i32;
        c4[c as libc::c_uchar as usize] =
            c4[c as libc::c_uchar as usize].wrapping_add(1);
        c5[d as libc::c_uchar as usize] =
            c5[d as libc::c_uchar as usize].wrapping_add(1);
        c6[(c >> 8i32) as usize] = c6[(c >> 8i32) as usize].wrapping_add(1);
        c7[(d >> 8i32) as usize] = c7[(d >> 8i32) as usize].wrapping_add(1)
    }
    ip = ip32 as *const BYTE;
    while ip < iend {
        let fresh8 = ip;
        ip = ip.offset(1);
        c0[*fresh8 as usize] = c0[*fresh8 as usize].wrapping_add(1)
    }
    i = 0i32;
    while i < 256i32 {
        c0[i as usize] =
            (c0[i as usize] as
                 libc::c_uint).wrapping_add(c1[i as
                                                   usize].wrapping_add(c2[i as
                                                                              usize]).wrapping_add(c3[i
                                                                                                          as
                                                                                                          usize]).wrapping_add(c4[i
                                                                                                                                      as
                                                                                                                                      usize]).wrapping_add(c5[i
                                                                                                                                                                  as
                                                                                                                                                                  usize]).wrapping_add(c6[i
                                                                                                                                                                                              as
                                                                                                                                                                                              usize]).wrapping_add(c7[i
                                                                                                                                                                                                                          as
                                                                                                                                                                                                                          usize]))
                as U32 as U32;
        i += 1
    }
    return c0[0usize] as libc::c_int;
}
/* Modified version of count2x64 by Nathan Kurz, using C instead of assembler */
unsafe extern "C" fn local_count2x64v2(mut dst: *mut libc::c_void,
                                       mut dstSize: size_t,
                                       mut src0: *const libc::c_void,
                                       mut srcSize: size_t) -> libc::c_int {
    let mut src64: *const U64 = src0 as *const U64;
    let mut src64end: *const U64 = src64.offset((srcSize >> 3i32) as isize);
    let mut src: *const BYTE = src0 as *const BYTE;
    let mut remainder: U64 = srcSize as U64;
    let mut next0: U64 = 0;
    let mut next1: U64 = 0;
    let mut count: [[U32; 272]; 16] = [[0; 272]; 16];
    memset(count.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[[U32; 272]; 16]>() as libc::c_ulong);
    if !(srcSize < 32i32 as libc::c_ulong) {
        remainder = srcSize.wrapping_rem(16i32 as libc::c_ulong) as U64;
        next0 = *src64.offset(0isize);
        next1 = *src64.offset(1isize);
        while src64 != src64end {
            let mut data0: U64 = next0;
            let mut data1: U64 = next1;
            src64 = src64.offset(2isize);
            next0 = *src64.offset(0isize);
            next1 = *src64.offset(1isize);
            let mut byte0: U64 = data0 & 0xffi32 as libc::c_ulonglong;
            let mut byte1: U64 = data1 & 0xffi32 as libc::c_ulonglong;
            let mut byte2: U64 =
                (data0 & 0xff00i32 as libc::c_ulonglong) >> 8i32;
            let mut byte3: U64 =
                (data1 & 0xff00i32 as libc::c_ulonglong) >> 8i32;
            count[(0i32 + 0i32) as usize][byte0 as usize] =
                count[(0i32 + 0i32) as usize][byte0 as usize].wrapping_add(1);
            count[(0i32 + 1i32) as usize][byte1 as usize] =
                count[(0i32 + 1i32) as usize][byte1 as usize].wrapping_add(1);
            count[(0i32 + 2i32) as usize][byte2 as usize] =
                count[(0i32 + 2i32) as usize][byte2 as usize].wrapping_add(1);
            count[(0i32 + 3i32) as usize][byte3 as usize] =
                count[(0i32 + 3i32) as usize][byte3 as usize].wrapping_add(1);
            data0 >>= 16i32;
            data1 >>= 16i32;
            let mut byte0_0: U64 = data0 & 0xffi32 as libc::c_ulonglong;
            let mut byte1_0: U64 = data1 & 0xffi32 as libc::c_ulonglong;
            let mut byte2_0: U64 =
                (data0 & 0xff00i32 as libc::c_ulonglong) >> 8i32;
            let mut byte3_0: U64 =
                (data1 & 0xff00i32 as libc::c_ulonglong) >> 8i32;
            count[(0i32 + 0i32) as usize][byte0_0 as usize] =
                count[(0i32 + 0i32) as
                          usize][byte0_0 as usize].wrapping_add(1);
            count[(0i32 + 1i32) as usize][byte1_0 as usize] =
                count[(0i32 + 1i32) as
                          usize][byte1_0 as usize].wrapping_add(1);
            count[(0i32 + 2i32) as usize][byte2_0 as usize] =
                count[(0i32 + 2i32) as
                          usize][byte2_0 as usize].wrapping_add(1);
            count[(0i32 + 3i32) as usize][byte3_0 as usize] =
                count[(0i32 + 3i32) as
                          usize][byte3_0 as usize].wrapping_add(1);
            data0 >>= 16i32;
            data1 >>= 16i32;
            let mut byte0_1: U64 = data0 & 0xffi32 as libc::c_ulonglong;
            let mut byte1_1: U64 = data1 & 0xffi32 as libc::c_ulonglong;
            let mut byte2_1: U64 =
                (data0 & 0xff00i32 as libc::c_ulonglong) >> 8i32;
            let mut byte3_1: U64 =
                (data1 & 0xff00i32 as libc::c_ulonglong) >> 8i32;
            count[(0i32 + 0i32) as usize][byte0_1 as usize] =
                count[(0i32 + 0i32) as
                          usize][byte0_1 as usize].wrapping_add(1);
            count[(0i32 + 1i32) as usize][byte1_1 as usize] =
                count[(0i32 + 1i32) as
                          usize][byte1_1 as usize].wrapping_add(1);
            count[(0i32 + 2i32) as usize][byte2_1 as usize] =
                count[(0i32 + 2i32) as
                          usize][byte2_1 as usize].wrapping_add(1);
            count[(0i32 + 3i32) as usize][byte3_1 as usize] =
                count[(0i32 + 3i32) as
                          usize][byte3_1 as usize].wrapping_add(1);
            data0 >>= 16i32;
            data1 >>= 16i32;
            let mut byte0_2: U64 = data0 & 0xffi32 as libc::c_ulonglong;
            let mut byte1_2: U64 = data1 & 0xffi32 as libc::c_ulonglong;
            let mut byte2_2: U64 =
                (data0 & 0xff00i32 as libc::c_ulonglong) >> 8i32;
            let mut byte3_2: U64 =
                (data1 & 0xff00i32 as libc::c_ulonglong) >> 8i32;
            count[(0i32 + 0i32) as usize][byte0_2 as usize] =
                count[(0i32 + 0i32) as
                          usize][byte0_2 as usize].wrapping_add(1);
            count[(0i32 + 1i32) as usize][byte1_2 as usize] =
                count[(0i32 + 1i32) as
                          usize][byte1_2 as usize].wrapping_add(1);
            count[(0i32 + 2i32) as usize][byte2_2 as usize] =
                count[(0i32 + 2i32) as
                          usize][byte2_2 as usize].wrapping_add(1);
            count[(0i32 + 3i32) as usize][byte3_2 as usize] =
                count[(0i32 + 3i32) as
                          usize][byte3_2 as usize].wrapping_add(1)
        }
    }
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while (i as libc::c_ulonglong) < remainder {
        let mut byte: size_t = *src.offset(i as isize) as size_t;
        count[0usize][byte as usize] =
            count[0usize][byte as usize].wrapping_add(1);
        i = i.wrapping_add(1)
    }
    i = 0i32 as size_t;
    while i < 256i32 as libc::c_ulong {
        let mut idx: libc::c_int = 0;
        idx = 1i32;
        while idx < 16i32 {
            count[0usize][i as usize] =
                (count[0usize][i as usize] as
                     libc::c_uint).wrapping_add(count[idx as
                                                          usize][i as usize])
                    as U32 as U32;
            idx += 1
        }
        i = i.wrapping_add(1)
    }
    return count[0usize][0usize] as libc::c_int;
}
unsafe extern "C" fn histo_by8(mut counts: *mut U32,
                               mut rawArray: *const BYTE,
                               mut rawLen: size_t) {
    let mut countsArray: [[U32; 256]; 4] = [[0; 256]; 4];
    memset(countsArray.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[[U32; 256]; 4]>() as libc::c_ulong);
    let mut rawPtr: *const BYTE = rawArray;
    let rawEnd: *const BYTE = rawArray.offset(rawLen as isize);
    let mut rawEndMul4: *const BYTE =
        rawArray.offset((rawLen & !3i32 as libc::c_ulong) as isize);
    while rawPtr < rawEndMul4 {
        let mut x: U64 = MEM_read64(rawPtr as *const libc::c_void);
        countsArray[0usize][(x & 0xffi32 as libc::c_ulonglong) as usize] =
            countsArray[0usize][(x & 0xffi32 as libc::c_ulonglong) as
                                    usize].wrapping_add(1);
        x >>= 8i32;
        countsArray[1usize][(x & 0xffi32 as libc::c_ulonglong) as usize] =
            countsArray[1usize][(x & 0xffi32 as libc::c_ulonglong) as
                                    usize].wrapping_add(1);
        x >>= 8i32;
        countsArray[2usize][(x & 0xffi32 as libc::c_ulonglong) as usize] =
            countsArray[2usize][(x & 0xffi32 as libc::c_ulonglong) as
                                    usize].wrapping_add(1);
        x >>= 8i32;
        countsArray[3usize][(x & 0xffi32 as libc::c_ulonglong) as usize] =
            countsArray[3usize][(x & 0xffi32 as libc::c_ulonglong) as
                                    usize].wrapping_add(1);
        x >>= 8i32;
        countsArray[0usize][(x & 0xffi32 as libc::c_ulonglong) as usize] =
            countsArray[0usize][(x & 0xffi32 as libc::c_ulonglong) as
                                    usize].wrapping_add(1);
        x >>= 8i32;
        countsArray[1usize][(x & 0xffi32 as libc::c_ulonglong) as usize] =
            countsArray[1usize][(x & 0xffi32 as libc::c_ulonglong) as
                                    usize].wrapping_add(1);
        x >>= 8i32;
        countsArray[2usize][(x & 0xffi32 as libc::c_ulonglong) as usize] =
            countsArray[2usize][(x & 0xffi32 as libc::c_ulonglong) as
                                    usize].wrapping_add(1);
        x >>= 8i32;
        countsArray[3usize][x as usize] =
            countsArray[3usize][x as usize].wrapping_add(1);
        rawPtr = rawPtr.offset(8isize)
    }
    while rawPtr < rawEnd {
        let fresh9 = rawPtr;
        rawPtr = rawPtr.offset(1);
        countsArray[0usize][*fresh9 as usize] =
            countsArray[0usize][*fresh9 as usize].wrapping_add(1)
    }
    let mut s: U32 = 0;
    s = 0i32 as U32;
    while s < 256i32 as libc::c_uint {
        *counts.offset(s as isize) =
            countsArray[0usize][s as
                                    usize].wrapping_add(countsArray[1usize][s
                                                                                as
                                                                                usize]).wrapping_add(countsArray[2usize][s
                                                                                                                             as
                                                                                                                             usize]).wrapping_add(countsArray[3usize][s
                                                                                                                                                                          as
                                                                                                                                                                          usize]);
        s = s.wrapping_add(1)
    };
}
unsafe extern "C" fn local_histo_by8(mut dst: *mut libc::c_void,
                                     mut dstSize: size_t,
                                     mut src: *const libc::c_void,
                                     mut srcSize: size_t) -> libc::c_int {
    let mut count: [U32; 256] = [0; 256];
    histo_by8(count.as_mut_ptr(), src as *const BYTE, srcSize);
    return count[0usize] as libc::c_int;
}
unsafe extern "C" fn local_FSE_count255(mut dst: *mut libc::c_void,
                                        mut dstSize: size_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> libc::c_int {
    let mut count: [U32; 256] = [0; 256];
    let mut max: U32 = 255i32 as U32;
    return HIST_count(count.as_mut_ptr(), &mut max,
                      src as *const BYTE as *const libc::c_void,
                      srcSize as U32 as size_t) as libc::c_int;
}
unsafe extern "C" fn local_FSE_count254(mut dst: *mut libc::c_void,
                                        mut dstSize: size_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> libc::c_int {
    let mut count: [U32; 256] = [0; 256];
    let mut max: U32 = 254i32 as U32;
    return HIST_count(count.as_mut_ptr(), &mut max,
                      src as *const BYTE as *const libc::c_void,
                      srcSize as U32 as size_t) as libc::c_int;
}
unsafe extern "C" fn local_FSE_countFast254(mut dst: *mut libc::c_void,
                                            mut dstSize: size_t,
                                            mut src: *const libc::c_void,
                                            mut srcSize: size_t)
 -> libc::c_int {
    let mut count: [U32; 256] = [0; 256];
    let mut max: U32 = 254i32 as U32;
    return HIST_countFast(count.as_mut_ptr(), &mut max,
                          src as *const libc::c_uchar as *const libc::c_void,
                          srcSize) as libc::c_int;
}
unsafe extern "C" fn local_FSE_compress(mut dst: *mut libc::c_void,
                                        mut dstSize: size_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> libc::c_int {
    return FSE_compress(dst, dstSize, src, srcSize) as libc::c_int;
}
unsafe extern "C" fn local_HUF_compress(mut dst: *mut libc::c_void,
                                        mut dstSize: size_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> libc::c_int {
    return HUF_compress(dst, dstSize, src, srcSize) as libc::c_int;
}
static mut fakeTree: [U32; 256] = [0; 256];
static mut g_treeVoidPtr: *mut libc::c_void =
    unsafe { fakeTree.as_ptr() as *mut libc::c_void };
static mut g_tree: *mut HUF_CElt = 0 as *const HUF_CElt as *mut HUF_CElt;
static mut g_normTable: [libc::c_short; 256] = [0; 256];
static mut g_countTable: [U32; 256] = [0; 256];
static mut g_tableLog: U32 = 0;
static mut g_CTable: [U32; 2350] = [0; 2350];
static mut g_DTable: [U32; 4097] = [0; 4097];
static mut g_max: U32 = 0;
static mut g_bmi2: U32 = 0i32 as U32;
static mut g_skip: size_t = 0;
static mut g_cSize: size_t = 0;
static mut g_oSize: size_t = 0;
// Initialized in run_static_initializers
#[no_mangle]
pub static mut g_huff_dtable: [HUF_DTable; 4097] = [0; 4097];
unsafe extern "C" fn BMK_init() {
    g_tree = g_treeVoidPtr as *mut HUF_CElt;
    g_bmi2 = ZSTD_cpuid_bmi2(ZSTD_cpuid()) as U32;
}
unsafe extern "C" fn local_HUF_buildCTable(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t)
 -> libc::c_int {
    return HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                           0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn local_HUF_writeCTable(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t)
 -> libc::c_int {
    return HUF_writeCTable(dst, dstSize, g_tree, g_max, g_tableLog) as
               libc::c_int;
}
unsafe extern "C" fn local_HUF_compress4x_usingCTable(mut dst:
                                                          *mut libc::c_void,
                                                      mut dstSize: size_t,
                                                      mut src:
                                                          *const libc::c_void,
                                                      mut srcSize: size_t)
 -> libc::c_int {
    return HUF_compress4X_usingCTable(dst, dstSize, src, srcSize, g_tree) as
               libc::c_int;
}
static mut huf4x_wksp: [libc::c_uint; 1536] = [0; 1536];
unsafe extern "C" fn local_HUF_compress4x_usingCTable_bmi2(mut dst:
                                                               *mut libc::c_void,
                                                           mut dstSize:
                                                               size_t,
                                                           mut src:
                                                               *const libc::c_void,
                                                           mut srcSize:
                                                               size_t)
 -> libc::c_int {
    let mut repeat: HUF_repeat = HUF_repeat_valid;
    return HUF_compress4X_repeat(dst, dstSize, src, srcSize, g_max,
                                 g_tableLog,
                                 huf4x_wksp.as_mut_ptr() as *mut libc::c_void,
                                 ::std::mem::size_of::<[libc::c_uint; 1536]>()
                                     as libc::c_ulong, g_tree, &mut repeat,
                                 1i32, g_bmi2 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn local_FSE_normalizeCount(mut dst: *mut libc::c_void,
                                              mut dstSize: size_t,
                                              mut src: *const libc::c_void,
                                              mut srcSize: size_t)
 -> libc::c_int {
    return FSE_normalizeCount(g_normTable.as_mut_ptr(), 0i32 as libc::c_uint,
                              g_countTable.as_mut_ptr(), srcSize, g_max) as
               libc::c_int;
}
unsafe extern "C" fn local_FSE_writeNCount(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t)
 -> libc::c_int {
    return FSE_writeNCount(dst, dstSize, g_normTable.as_mut_ptr(), g_max,
                           g_tableLog) as libc::c_int;
}
/*
static int local_FSE_writeHeader_small(void* dst, size_t dstSize, const void* src, size_t srcSize)
{
    (void)src; (void)srcSize; (void)dstSize;
    return FSE_writeHeader(dst, 500, g_normTable, 255, g_tableLog);
}
*/
unsafe extern "C" fn local_FSE_buildCTable(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t)
 -> libc::c_int {
    return FSE_buildCTable(g_CTable.as_mut_ptr(), g_normTable.as_mut_ptr(),
                           g_max, g_tableLog) as libc::c_int;
}
unsafe extern "C" fn local_FSE_buildCTable_raw(mut dst: *mut libc::c_void,
                                               mut dstSize: size_t,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t)
 -> libc::c_int {
    return FSE_buildCTable_raw(g_CTable.as_mut_ptr(), 6i32 as libc::c_uint) as
               libc::c_int;
}
unsafe extern "C" fn local_FSE_compress_usingCTable(mut dst:
                                                        *mut libc::c_void,
                                                    mut dstSize: size_t,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t)
 -> libc::c_int {
    return FSE_compress_usingCTable(dst, dstSize, src, srcSize,
                                    g_CTable.as_mut_ptr()) as libc::c_int;
}
unsafe extern "C" fn local_FSE_compress_usingCTable_tooSmall(mut dst:
                                                                 *mut libc::c_void,
                                                             mut dstSize:
                                                                 size_t,
                                                             mut src:
                                                                 *const libc::c_void,
                                                             mut srcSize:
                                                                 size_t)
 -> libc::c_int {
    return FSE_compress_usingCTable(dst,
                                    srcSize.wrapping_add(srcSize >>
                                                             7i32).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong),
                                    src, srcSize, g_CTable.as_mut_ptr()) as
               libc::c_int;
}
unsafe extern "C" fn local_FSE_readNCount(mut src: *mut libc::c_void,
                                          mut srcSize: size_t,
                                          mut initialBuffer:
                                              *const libc::c_void,
                                          mut initialBufferSize: size_t)
 -> libc::c_int {
    let mut norm: [libc::c_short; 256] = [0; 256];
    return FSE_readNCount(norm.as_mut_ptr(), &mut g_max, &mut g_tableLog, src,
                          srcSize) as libc::c_int;
}
unsafe extern "C" fn local_FSE_buildDTable(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t)
 -> libc::c_int {
    return FSE_buildDTable(g_DTable.as_mut_ptr(), g_normTable.as_mut_ptr(),
                           g_max, g_tableLog) as libc::c_int;
}
unsafe extern "C" fn local_FSE_buildDTable_raw(mut dst: *mut libc::c_void,
                                               mut dstSize: size_t,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t)
 -> libc::c_int {
    return FSE_buildDTable_raw(g_DTable.as_mut_ptr(), 6i32 as libc::c_uint) as
               libc::c_int;
}
unsafe extern "C" fn local_FSE_decompress_usingDTable(mut dst:
                                                          *mut libc::c_void,
                                                      mut maxDstSize: size_t,
                                                      mut src:
                                                          *const libc::c_void,
                                                      mut srcSize: size_t)
 -> libc::c_int {
    return FSE_decompress_usingDTable(dst, maxDstSize,
                                      (src as
                                           *const BYTE).offset(g_skip as
                                                                   isize) as
                                          *const libc::c_void, g_cSize,
                                      g_DTable.as_mut_ptr()) as libc::c_int;
}
unsafe extern "C" fn local_FSE_decompress(mut dst: *mut libc::c_void,
                                          mut maxDstSize: size_t,
                                          mut src: *const libc::c_void,
                                          mut srcSize: size_t)
 -> libc::c_int {
    return FSE_decompress(dst, maxDstSize, src, g_cSize) as libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress(mut dst: *mut libc::c_void,
                                          mut maxDstSize: size_t,
                                          mut src: *const libc::c_void,
                                          mut srcSize: size_t)
 -> libc::c_int {
    return HUF_decompress(dst, g_oSize, src, g_cSize) as libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress4X1(mut dst: *mut libc::c_void,
                                             mut maxDstSize: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t)
 -> libc::c_int {
    return HUF_decompress4X1(dst, g_oSize, src, g_cSize) as libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress4X2(mut dst: *mut libc::c_void,
                                             mut maxDstSize: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t)
 -> libc::c_int {
    return HUF_decompress4X2(dst, g_oSize, src, g_cSize) as libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress1X1(mut dst: *mut libc::c_void,
                                             mut maxDstSize: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t)
 -> libc::c_int {
    return HUF_decompress1X1(dst, g_oSize, src, g_cSize) as libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress1X2(mut dst: *mut libc::c_void,
                                             mut maxDstSize: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t)
 -> libc::c_int {
    return HUF_decompress1X2(dst, g_oSize, src, g_cSize) as libc::c_int;
}
unsafe extern "C" fn local_HUF_readStats(mut dst: *mut libc::c_void,
                                         mut maxDstSize: size_t,
                                         mut src: *const libc::c_void,
                                         mut srcSize: size_t) -> libc::c_int {
    let mut weights: [BYTE; 256] = [0; 256];
    let mut ranks: [U32; 16] = [0; 16];
    let mut nbSymbols: U32 = 0i32 as U32;
    let mut tableLog: U32 = 0i32 as U32;
    return HUF_readStats(weights.as_mut_ptr(), (255i32 + 1i32) as size_t,
                         ranks.as_mut_ptr(), &mut nbSymbols, &mut tableLog,
                         src, g_cSize) as libc::c_int;
}
unsafe extern "C" fn local_HUF_readDTableX2(mut dst: *mut libc::c_void,
                                            mut maxDstSize: size_t,
                                            mut src: *const libc::c_void,
                                            mut srcSize: size_t)
 -> libc::c_int {
    return HUF_readDTableX2(g_huff_dtable.as_mut_ptr(), src, g_cSize) as
               libc::c_int;
}
unsafe extern "C" fn local_HUF_readDTable(mut dst: *mut libc::c_void,
                                          mut maxDstSize: size_t,
                                          mut src: *const libc::c_void,
                                          mut srcSize: size_t)
 -> libc::c_int {
    return local_HUF_readDTableX2(dst, maxDstSize, src, srcSize);
}
unsafe extern "C" fn local_HUF_readDTableX1(mut dst: *mut libc::c_void,
                                            mut maxDstSize: size_t,
                                            mut src: *const libc::c_void,
                                            mut srcSize: size_t)
 -> libc::c_int {
    return HUF_readDTableX1(g_huff_dtable.as_mut_ptr(), src, g_cSize) as
               libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress4X1_usingDTable(mut dst:
                                                             *mut libc::c_void,
                                                         mut maxDstSize:
                                                             size_t,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t)
 -> libc::c_int {
    return HUF_decompress4X1_usingDTable(dst, g_oSize, src, g_cSize,
                                         g_huff_dtable.as_mut_ptr()) as
               libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress4X2_usingDTable(mut dst:
                                                             *mut libc::c_void,
                                                         mut maxDstSize:
                                                             size_t,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t)
 -> libc::c_int {
    return HUF_decompress4X2_usingDTable(dst, g_oSize, src, g_cSize,
                                         g_huff_dtable.as_mut_ptr()) as
               libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress_usingDTable(mut dst:
                                                          *mut libc::c_void,
                                                      mut maxDstSize: size_t,
                                                      mut src:
                                                          *const libc::c_void,
                                                      mut srcSize: size_t)
 -> libc::c_int {
    return local_HUF_decompress4X2_usingDTable(dst, maxDstSize, src, srcSize);
}
unsafe extern "C" fn local_HUF_decompress4X_usingDTable_bmi2(mut dst:
                                                                 *mut libc::c_void,
                                                             mut maxDstSize:
                                                                 size_t,
                                                             mut src:
                                                                 *const libc::c_void,
                                                             mut srcSize:
                                                                 size_t)
 -> libc::c_int {
    return HUF_decompress4X_usingDTable_bmi2(dst, g_oSize, src, g_cSize,
                                             g_huff_dtable.as_mut_ptr(),
                                             g_bmi2 as libc::c_int) as
               libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress1X1_usingDTable(mut dst:
                                                             *mut libc::c_void,
                                                         mut maxDstSize:
                                                             size_t,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t)
 -> libc::c_int {
    return HUF_decompress1X1_usingDTable(dst, g_oSize, src, g_cSize,
                                         g_huff_dtable.as_mut_ptr()) as
               libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress1X2_usingDTable(mut dst:
                                                             *mut libc::c_void,
                                                         mut maxDstSize:
                                                             size_t,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t)
 -> libc::c_int {
    return HUF_decompress1X2_usingDTable(dst, g_oSize, src, g_cSize,
                                         g_huff_dtable.as_mut_ptr()) as
               libc::c_int;
}
unsafe extern "C" fn local_HUF_decompress1X_usingDTable_bmi2(mut dst:
                                                                 *mut libc::c_void,
                                                             mut maxDstSize:
                                                                 size_t,
                                                             mut src:
                                                                 *const libc::c_void,
                                                             mut srcSize:
                                                                 size_t)
 -> libc::c_int {
    return HUF_decompress1X_usingDTable_bmi2(dst, g_oSize, src, g_cSize,
                                             g_huff_dtable.as_mut_ptr(),
                                             g_bmi2 as libc::c_int) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn runBench(mut buffer: *const libc::c_void,
                                  mut blockSize: size_t, mut algNb: U32,
                                  mut nbBenchs: U32) -> libc::c_int {
    let mut current_block: u64;
    let mut benchedSize: size_t = blockSize;
    let mut cBuffSize: size_t =
        FSE_compressBound(benchedSize as libc::c_uint as size_t);
    let mut oBuffer: *mut libc::c_void = malloc(blockSize);
    let mut cBuffer: *mut libc::c_void = malloc(cBuffSize);
    let mut funcName: *const libc::c_char = 0 as *const libc::c_char;
    let mut func:
            Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t,
                                        _: *const libc::c_void, _: size_t)
                       -> libc::c_int> = None;
    memcpy(oBuffer, buffer, blockSize);
    /* Bench selection */
    match algNb {
        1 => {
            funcName =
                b"HIST_count(255)\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_count255);
            current_block = 14908777651318078790;
        }
        2 => {
            funcName =
                b"HIST_count(254)\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_count254);
            current_block = 14908777651318078790;
        }
        3 => {
            funcName =
                b"HIST_countFast(254)\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_FSE_countFast254);
            current_block = 14908777651318078790;
        }
        4 => {
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog = FSE_optimalTableLog(g_tableLog, benchedSize, g_max);
            funcName =
                b"FSE_normalizeCount\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_normalizeCount);
            current_block = 14908777651318078790;
        }
        5 => {
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog = FSE_optimalTableLog(g_tableLog, benchedSize, g_max);
            FSE_normalizeCount(g_normTable.as_mut_ptr(), g_tableLog,
                               g_countTable.as_mut_ptr(), benchedSize, g_max);
            funcName =
                b"FSE_writeNCount\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_writeNCount);
            current_block = 14908777651318078790;
        }
        6 => {
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog = FSE_optimalTableLog(g_tableLog, benchedSize, g_max);
            FSE_normalizeCount(g_normTable.as_mut_ptr(), g_tableLog,
                               g_countTable.as_mut_ptr(), benchedSize, g_max);
            funcName =
                b"FSE_buildCTable\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_buildCTable);
            current_block = 14908777651318078790;
        }
        7 => {
            let mut max: U32 = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                FSE_normalizeCount(g_normTable.as_mut_ptr(), g_tableLog,
                                   g_countTable.as_mut_ptr(), benchedSize,
                                   max) as U32;
            FSE_buildCTable(g_CTable.as_mut_ptr(), g_normTable.as_mut_ptr(),
                            max, g_tableLog);
            funcName =
                b"FSE_compress_usingCTable\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_FSE_compress_usingCTable);
            current_block = 14908777651318078790;
        }
        8 => {
            let mut max_0: U32 = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut max_0,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                FSE_normalizeCount(g_normTable.as_mut_ptr(), g_tableLog,
                                   g_countTable.as_mut_ptr(), benchedSize,
                                   max_0) as U32;
            FSE_buildCTable(g_CTable.as_mut_ptr(), g_normTable.as_mut_ptr(),
                            max_0, g_tableLog);
            funcName =
                b"FSE_compress_usingCTable_smallDst\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_FSE_compress_usingCTable_tooSmall);
            current_block = 14908777651318078790;
        }
        9 => {
            funcName =
                b"FSE_compress\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_compress);
            current_block = 14908777651318078790;
        }
        11 => {
            FSE_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            g_max = 255i32 as U32;
            funcName =
                b"FSE_readNCount\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_readNCount);
            current_block = 14908777651318078790;
        }
        12 => {
            FSE_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            g_max = 255i32 as U32;
            FSE_readNCount(g_normTable.as_mut_ptr(), &mut g_max,
                           &mut g_tableLog, cBuffer, benchedSize);
            funcName =
                b"FSE_buildDTable\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_buildDTable);
            current_block = 14908777651318078790;
        }
        13 => {
            g_cSize = FSE_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            memcpy(oBuffer, cBuffer, g_cSize);
            g_max = 255i32 as U32;
            g_skip =
                FSE_readNCount(g_normTable.as_mut_ptr(), &mut g_max,
                               &mut g_tableLog, oBuffer, g_cSize);
            g_cSize =
                (g_cSize as libc::c_ulong).wrapping_sub(g_skip) as size_t as
                    size_t;
            FSE_buildDTable(g_DTable.as_mut_ptr(), g_normTable.as_mut_ptr(),
                            g_max, g_tableLog);
            funcName =
                b"FSE_decompress_usingDTable\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_FSE_decompress_usingDTable);
            current_block = 14908777651318078790;
        }
        14 => {
            g_cSize = FSE_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"FSE_decompress\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_decompress);
            current_block = 14908777651318078790;
        }
        20 => {
            funcName =
                b"HUF_compress\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_compress);
            current_block = 14908777651318078790;
        }
        21 => {
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            funcName =
                b"HUF_buildCTable\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_buildCTable);
            current_block = 14908777651318078790;
        }
        22 => {
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                                0i32 as libc::c_uint) as U32;
            funcName =
                b"HUF_writeCTable\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_writeCTable);
            current_block = 14908777651318078790;
        }
        23 => {
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                                0i32 as libc::c_uint) as U32;
            funcName =
                b"HUF_compress4x_usingCTable\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_compress4x_usingCTable);
            current_block = 14908777651318078790;
        }
        24 => {
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                                0i32 as libc::c_uint) as U32;
            funcName =
                b"HUF_compress4x_usingCTable_bmi2\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_compress4x_usingCTable_bmi2);
            current_block = 14908777651318078790;
        }
        30 => {
            g_oSize = benchedSize;
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"HUF_decompress\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_decompress);
            current_block = 14908777651318078790;
        }
        31 => {
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"HUF_readStats\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_readStats);
            current_block = 14908777651318078790;
        }
        32 => {
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"HUF_readDTable\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_readDTable);
            current_block = 14908777651318078790;
        }
        33 => {
            let mut hSize: size_t = 0;
            g_oSize = benchedSize;
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            hSize =
                HUF_readDTableX2(g_huff_dtable.as_mut_ptr(), cBuffer,
                                 g_cSize);
            g_cSize =
                (g_cSize as libc::c_ulong).wrapping_sub(hSize) as size_t as
                    size_t;
            memcpy(oBuffer,
                   (cBuffer as *mut libc::c_char).offset(hSize as isize) as
                       *const libc::c_void, g_cSize);
            funcName =
                b"HUF_decompress_usingDTable\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_decompress_usingDTable);
            current_block = 14908777651318078790;
        }
        40 => {
            g_oSize = benchedSize;
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"HUF_decompress4X1\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_decompress4X1);
            current_block = 14908777651318078790;
        }
        41 => {
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"HUF_readDTableX1\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_readDTableX1);
            current_block = 14908777651318078790;
        }
        42 => {
            let mut hSize_0: size_t = 0;
            g_oSize = benchedSize;
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            hSize_0 =
                HUF_readDTableX1(g_huff_dtable.as_mut_ptr(), cBuffer,
                                 g_cSize);
            g_cSize =
                (g_cSize as libc::c_ulong).wrapping_sub(hSize_0) as size_t as
                    size_t;
            memcpy(oBuffer,
                   (cBuffer as *mut libc::c_char).offset(hSize_0 as isize) as
                       *const libc::c_void, g_cSize);
            funcName =
                b"HUF_decompress4X1_usingDTable\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_decompress4X1_usingDTable);
            current_block = 14908777651318078790;
        }
        43 => {
            let mut hSize_1: size_t = 0;
            g_oSize = benchedSize;
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            hSize_1 =
                HUF_readDTableX1(g_huff_dtable.as_mut_ptr(), cBuffer,
                                 g_cSize);
            g_cSize =
                (g_cSize as libc::c_ulong).wrapping_sub(hSize_1) as size_t as
                    size_t;
            memcpy(oBuffer,
                   (cBuffer as *mut libc::c_char).offset(hSize_1 as isize) as
                       *const libc::c_void, g_cSize);
            funcName =
                b"HUF_decompress4X1_usingDTable_bmi2\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_decompress4X_usingDTable_bmi2);
            current_block = 14908777651318078790;
        }
        45 => {
            g_oSize = benchedSize;
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                                0i32 as libc::c_uint) as U32;
            g_cSize =
                HUF_writeCTable(cBuffer, cBuffSize, g_tree, g_max,
                                g_tableLog);
            g_cSize =
                (g_cSize as
                     libc::c_ulong).wrapping_add(HUF_compress1X_usingCTable((cBuffer
                                                                                 as
                                                                                 *mut BYTE).offset(g_cSize
                                                                                                       as
                                                                                                       isize)
                                                                                as
                                                                                *mut libc::c_void,
                                                                            cBuffSize,
                                                                            oBuffer,
                                                                            benchedSize,
                                                                            g_tree))
                    as size_t as size_t;
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"HUF_decompress1X1\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_decompress1X1);
            current_block = 14908777651318078790;
        }
        46 => {
            let mut hSize_2: size_t = 0;
            g_oSize = benchedSize;
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                                0i32 as libc::c_uint) as U32;
            hSize_2 =
                HUF_writeCTable(cBuffer, cBuffSize, g_tree, g_max,
                                g_tableLog);
            g_cSize =
                HUF_compress1X_usingCTable((cBuffer as
                                                *mut BYTE).offset(hSize_2 as
                                                                      isize)
                                               as *mut libc::c_void,
                                           cBuffSize, oBuffer, benchedSize,
                                           g_tree);
            hSize_2 =
                HUF_readDTableX1(g_huff_dtable.as_mut_ptr(), cBuffer,
                                 g_cSize);
            memcpy(oBuffer,
                   (cBuffer as *mut libc::c_char).offset(hSize_2 as isize) as
                       *const libc::c_void, g_cSize);
            funcName =
                b"HUF_decompress1X1_usingDTable\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_decompress1X1_usingDTable);
            current_block = 14908777651318078790;
        }
        47 => {
            let mut hSize_3: size_t = 0;
            g_oSize = benchedSize;
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                                0i32 as libc::c_uint) as U32;
            hSize_3 =
                HUF_writeCTable(cBuffer, cBuffSize, g_tree, g_max,
                                g_tableLog);
            g_cSize =
                HUF_compress1X_usingCTable((cBuffer as
                                                *mut BYTE).offset(hSize_3 as
                                                                      isize)
                                               as *mut libc::c_void,
                                           cBuffSize, oBuffer, benchedSize,
                                           g_tree);
            hSize_3 =
                HUF_readDTableX1(g_huff_dtable.as_mut_ptr(), cBuffer,
                                 g_cSize);
            memcpy(oBuffer,
                   (cBuffer as *mut libc::c_char).offset(hSize_3 as isize) as
                       *const libc::c_void, g_cSize);
            funcName =
                b"HUF_decompress1X1_usingDTable_bmi2\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_decompress1X_usingDTable_bmi2);
            current_block = 14908777651318078790;
        }
        50 => {
            g_oSize = benchedSize;
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"HUF_decompress4X2\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_decompress4X2);
            current_block = 14908777651318078790;
        }
        51 => {
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"HUF_readDTableX2\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_readDTableX2);
            current_block = 14908777651318078790;
        }
        52 => {
            let mut hSize_4: size_t = 0;
            g_oSize = benchedSize;
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            hSize_4 =
                HUF_readDTableX2(g_huff_dtable.as_mut_ptr(), cBuffer,
                                 g_cSize);
            g_cSize =
                (g_cSize as libc::c_ulong).wrapping_sub(hSize_4) as size_t as
                    size_t;
            memcpy(oBuffer,
                   (cBuffer as *mut libc::c_char).offset(hSize_4 as isize) as
                       *const libc::c_void, g_cSize);
            funcName =
                b"HUF_decompress4X2_usingDTable\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_decompress4X2_usingDTable);
            current_block = 14908777651318078790;
        }
        53 => {
            let mut hSize_5: size_t = 0;
            g_oSize = benchedSize;
            g_cSize = HUF_compress(cBuffer, cBuffSize, oBuffer, benchedSize);
            hSize_5 =
                HUF_readDTableX2(g_huff_dtable.as_mut_ptr(), cBuffer,
                                 g_cSize);
            g_cSize =
                (g_cSize as libc::c_ulong).wrapping_sub(hSize_5) as size_t as
                    size_t;
            memcpy(oBuffer,
                   (cBuffer as *mut libc::c_char).offset(hSize_5 as isize) as
                       *const libc::c_void, g_cSize);
            funcName =
                b"HUF_decompress4X2_usingDTable_bmi2\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_decompress4X_usingDTable_bmi2);
            current_block = 14908777651318078790;
        }
        55 => {
            g_oSize = benchedSize;
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                                0i32 as libc::c_uint) as U32;
            g_cSize =
                HUF_writeCTable(cBuffer, cBuffSize, g_tree, g_max,
                                g_tableLog);
            g_cSize =
                (g_cSize as
                     libc::c_ulong).wrapping_add(HUF_compress1X_usingCTable((cBuffer
                                                                                 as
                                                                                 *mut BYTE).offset(g_cSize
                                                                                                       as
                                                                                                       isize)
                                                                                as
                                                                                *mut libc::c_void,
                                                                            cBuffSize,
                                                                            oBuffer,
                                                                            benchedSize,
                                                                            g_tree))
                    as size_t as size_t;
            memcpy(oBuffer, cBuffer, g_cSize);
            funcName =
                b"HUF_decompress1X2\x00" as *const u8 as *const libc::c_char;
            func = Some(local_HUF_decompress1X2);
            current_block = 14908777651318078790;
        }
        56 => {
            let mut hSize_6: size_t = 0;
            g_oSize = benchedSize;
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                                0i32 as libc::c_uint) as U32;
            hSize_6 =
                HUF_writeCTable(cBuffer, cBuffSize, g_tree, g_max,
                                g_tableLog);
            g_cSize =
                HUF_compress1X_usingCTable((cBuffer as
                                                *mut BYTE).offset(hSize_6 as
                                                                      isize)
                                               as *mut libc::c_void,
                                           cBuffSize, oBuffer, benchedSize,
                                           g_tree);
            hSize_6 =
                HUF_readDTableX2(g_huff_dtable.as_mut_ptr(), cBuffer,
                                 g_cSize);
            memcpy(oBuffer,
                   (cBuffer as *mut libc::c_char).offset(hSize_6 as isize) as
                       *const libc::c_void, g_cSize);
            funcName =
                b"HUF_decompress1X2_usingDTable\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_decompress1X2_usingDTable);
            current_block = 14908777651318078790;
        }
        57 => {
            let mut hSize_7: size_t = 0;
            g_oSize = benchedSize;
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max,
                       oBuffer as *const libc::c_uchar as *const libc::c_void,
                       benchedSize);
            g_tableLog =
                HUF_buildCTable(g_tree, g_countTable.as_mut_ptr(), g_max,
                                0i32 as libc::c_uint) as U32;
            hSize_7 =
                HUF_writeCTable(cBuffer, cBuffSize, g_tree, g_max,
                                g_tableLog);
            g_cSize =
                HUF_compress1X_usingCTable((cBuffer as
                                                *mut BYTE).offset(hSize_7 as
                                                                      isize)
                                               as *mut libc::c_void,
                                           cBuffSize, oBuffer, benchedSize,
                                           g_tree);
            hSize_7 =
                HUF_readDTableX2(g_huff_dtable.as_mut_ptr(), cBuffer,
                                 g_cSize);
            memcpy(oBuffer,
                   (cBuffer as *mut libc::c_char).offset(hSize_7 as isize) as
                       *const libc::c_void, g_cSize);
            funcName =
                b"HUF_decompress1X2_usingDTable_bmi2\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_HUF_decompress1X_usingDTable_bmi2);
            current_block = 14908777651318078790;
        }
        70 => {
            funcName =
                b"FSE_buildCTable_raw(6)\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_FSE_buildCTable_raw);
            current_block = 14908777651318078790;
        }
        80 => {
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max, oBuffer,
                       benchedSize);
            g_tableLog =
                FSE_optimalTableLog(10i32 as libc::c_uint, benchedSize,
                                    g_max);
            FSE_normalizeCount(g_normTable.as_mut_ptr(), g_tableLog,
                               g_countTable.as_mut_ptr(), benchedSize, g_max);
            funcName =
                b"FSE_buildDTable(10)\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_FSE_buildDTable);
            current_block = 14908777651318078790;
        }
        81 => {
            g_max = 255i32 as U32;
            HIST_count(g_countTable.as_mut_ptr(), &mut g_max, oBuffer,
                       benchedSize);
            g_tableLog =
                FSE_optimalTableLog(9i32 as libc::c_uint, benchedSize, g_max);
            FSE_normalizeCount(g_normTable.as_mut_ptr(), g_tableLog,
                               g_countTable.as_mut_ptr(), benchedSize, g_max);
            funcName =
                b"FSE_buildDTable(9)\x00" as *const u8 as *const libc::c_char;
            func = Some(local_FSE_buildDTable);
            current_block = 14908777651318078790;
        }
        82 => {
            funcName =
                b"FSE_buildDTable_raw(6)\x00" as *const u8 as
                    *const libc::c_char;
            func = Some(local_FSE_buildDTable_raw);
            current_block = 14908777651318078790;
        }
        100 => {
            funcName =
                b"trivialCount\x00" as *const u8 as *const libc::c_char;
            func = Some(local_trivialCount);
            current_block = 14908777651318078790;
        }
        101 => {
            funcName = b"count8\x00" as *const u8 as *const libc::c_char;
            func = Some(local_count8);
            current_block = 14908777651318078790;
        }
        102 => {
            funcName = b"count8v2\x00" as *const u8 as *const libc::c_char;
            func = Some(local_count8v2);
            current_block = 14908777651318078790;
        }
        103 => {
            funcName =
                b"local_hist_4_32\x00" as *const u8 as *const libc::c_char;
            func = Some(local_hist_4_32);
            current_block = 14908777651318078790;
        }
        104 => {
            funcName =
                b"local_hist_4_32v2\x00" as *const u8 as *const libc::c_char;
            func = Some(local_hist_4_32v2);
            current_block = 14908777651318078790;
        }
        105 => {
            funcName =
                b"local_hist_8_32\x00" as *const u8 as *const libc::c_char;
            func = Some(local_hist_8_32);
            current_block = 14908777651318078790;
        }
        106 => {
            funcName =
                b"local_count2x64v2\x00" as *const u8 as *const libc::c_char;
            func = Some(local_count2x64v2);
            current_block = 14908777651318078790;
        }
        107 => {
            funcName =
                b"local_histo_by8\x00" as *const u8 as *const libc::c_char;
            func = Some(local_histo_by8);
            current_block = 14908777651318078790;
        }
        _ => { current_block = 12601936440572221197; }
    }
    match current_block {
        14908777651318078790 => {
            fprintf(__stderrp,
                    b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                    b"\x00" as *const u8 as *const libc::c_char);
            let mut nbLoops: libc::c_int =
                ((100i32 * (1i32 << 20i32)) as
                     libc::c_ulong).wrapping_div(benchedSize.wrapping_add(1i32
                                                                              as
                                                                              libc::c_ulong)).wrapping_add(1i32
                                                                                                               as
                                                                                                               libc::c_ulong)
                    as libc::c_int;
            let mut bestTimeS: libc::c_double = 999.0f64;
            let mut benchNb: U32 = 1i32 as U32;
            fprintf(__stderrp,
                    b"%2u-%-34.34s : \r\x00" as *const u8 as
                        *const libc::c_char, benchNb, funcName);
            benchNb = 1i32 as U32;
            while benchNb <= nbBenchs {
                let mut resultCode: size_t = 0i32 as size_t;
                let mut clockStart: clock_t = clock();
                while clock() == clockStart { }
                clockStart = clock();
                let mut loopNb: libc::c_int = 0;
                loopNb = 0i32;
                while loopNb < nbLoops {
                    resultCode =
                        func.expect("non-null function pointer")(cBuffer,
                                                                 cBuffSize,
                                                                 oBuffer,
                                                                 benchedSize)
                            as size_t;
                    loopNb += 1
                }
                let benchClock: clock_t = BMK_clockSpan(clockStart);
                let averageTimeS: libc::c_double =
                    benchClock as libc::c_double / nbLoops as libc::c_double /
                        1000000i32 as libc::c_double;
                if benchClock > 0i32 as libc::c_ulong {
                    nbLoops =
                        ((1.0f64 / averageTimeS) as
                             U32).wrapping_add(1i32 as libc::c_uint) as
                            libc::c_int
                } else { nbLoops *= 100i32 }
                if benchClock < (1000000i32 / 4i32) as libc::c_ulong {
                    benchNb = benchNb.wrapping_sub(1)
                } else {
                    if averageTimeS < bestTimeS { bestTimeS = averageTimeS }
                    fprintf(__stderrp,
                            b"%2u-%-34.34s : %8.1f MB/s  (%6u) \r\x00" as
                                *const u8 as *const libc::c_char,
                            benchNb.wrapping_add(1i32 as libc::c_uint),
                            funcName,
                            benchedSize as libc::c_double /
                                (1i32 * (1i32 << 20i32)) as libc::c_double /
                                bestTimeS, resultCode as U32);
                }
                benchNb = benchNb.wrapping_add(1)
            }
            fprintf(__stderrp,
                    b"%2u#\n\x00" as *const u8 as *const libc::c_char, algNb);
        }
        _ => { }
    }
    free(oBuffer);
    free(cBuffer);
    return 0i32;
}
unsafe extern "C" fn fullbench(mut filename: *const libc::c_char,
                               mut p: libc::c_double, mut blockSize: size_t,
                               mut algNb: U32, mut nbLoops: U32)
 -> libc::c_int {
    let mut result: libc::c_int = 0i32;
    let mut buffer: *mut libc::c_void = malloc(blockSize);
    if filename.is_null() {
        BMK_genData(buffer, blockSize, p);
    } else {
        let mut f: *mut FILE =
            fopen(filename, b"rb\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            fprintf(__stderrp,
                    b"Pb opening %s\n\x00" as *const u8 as
                        *const libc::c_char, filename);
            return 11i32
        }
        blockSize = fread(buffer, 1i32 as libc::c_ulong, blockSize, f);
        fprintf(__stderrp,
                b"Loading %u bytes from %s \n\x00" as *const u8 as
                    *const libc::c_char, blockSize as U32, filename);
        fclose(f);
    }
    if algNb == 0i32 as libc::c_uint {
        let mut u: U32 = 0;
        u = 1i32 as U32;
        while u <= 99i32 as libc::c_uint {
            result += runBench(buffer, blockSize, u, nbLoops);
            u = u.wrapping_add(1)
        }
    } else { result = runBench(buffer, blockSize, algNb, nbLoops) }
    free(buffer);
    return result;
}
unsafe extern "C" fn benchMultipleFiles(mut fnTable: *mut *const libc::c_char,
                                        mut nbFn: libc::c_int,
                                        mut startFn: libc::c_int,
                                        mut p: libc::c_double,
                                        mut blockSize: size_t, mut algNb: U32,
                                        mut nbLoops: U32) -> libc::c_int {
    if startFn == 0i32 {
        return fullbench(0 as *const libc::c_char, p, blockSize, algNb,
                         nbLoops)
    }
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0i32;
    i = startFn;
    while i < nbFn {
        result +=
            fullbench(*fnTable.offset(i as isize), p, blockSize, algNb,
                      nbLoops);
        i += 1
    }
    return result;
}
unsafe extern "C" fn usage(mut exename: *const libc::c_char) -> libc::c_int {
    fprintf(__stderrp, b"Usage :\n\x00" as *const u8 as *const libc::c_char);
    fprintf(__stderrp,
            b"      %s [arg] [filename]\n\x00" as *const u8 as
                *const libc::c_char, exename);
    fprintf(__stderrp,
            b"Arguments :\n\x00" as *const u8 as *const libc::c_char);
    fprintf(__stderrp,
            b" -b#    : select function to benchmark (default : 0 ==  all)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(__stderrp,
            b" -H/-h  : Help (this text + advanced options)\n\x00" as
                *const u8 as *const libc::c_char);
    return 0i32;
}
unsafe extern "C" fn usage_advanced(mut exename: *const libc::c_char)
 -> libc::c_int {
    usage(exename);
    fprintf(__stderrp,
            b"\nAdvanced options :\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(__stderrp,
            b" -i#    : iteration loops [1-9] (default : %i)\n\x00" as
                *const u8 as *const libc::c_char, 6i32);
    fprintf(__stderrp,
            b" -B#    : block size, in bytes (default : %i)\n\x00" as
                *const u8 as *const libc::c_char, 32i32 * (1i32 << 10i32));
    fprintf(__stderrp,
            b" -P#    : probability curve, in %% (default : %i%%)\n\x00" as
                *const u8 as *const libc::c_char, 20i32);
    return 0i32;
}
unsafe extern "C" fn badusage(mut exename: *const libc::c_char)
 -> libc::c_int {
    fprintf(__stderrp,
            b"Wrong parameters\n\x00" as *const u8 as *const libc::c_char);
    usage(exename);
    return 1i32;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    let mut exename: *const libc::c_char = *argv.offset(0isize);
    let mut proba: U32 = 20i32 as U32;
    let mut nbLoops: U32 = 6i32 as U32;
    let mut pause: U32 = 0i32 as U32;
    let mut algNb: U32 = 0i32 as U32;
    let mut blockSize: U32 = (32i32 * (1i32 << 10i32)) as U32;
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut fnStart: libc::c_int = 0i32;
    BMK_init();
    fprintf(__stderrp,
            b"*** %s %s %i-bits, by %s (%s) ***\n\x00" as *const u8 as
                *const libc::c_char,
            b"FSE speed analyzer\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<*mut libc::c_void>() as
                 libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) as
                libc::c_int,
            b"Yann Collet\x00" as *const u8 as *const libc::c_char,
            b"Mar 29 2019\x00" as *const u8 as *const libc::c_char);
    if argc < 1i32 { return badusage(exename) }
    i = 1i32;
    while i < argc {
        let mut argument: *const libc::c_char = *argv.offset(i as isize);
        if !argument.is_null() {
            // Protection if argument empty
            if 0 ==
                   strcmp(argument,
                          b"--no-prompt\x00" as *const u8 as
                              *const libc::c_char) {
                no_prompt = 1i32 as U32
            } else if *argument as libc::c_int == '-' as i32 {
                argument = argument.offset(1isize);
                while *argument as libc::c_int != 0i32 {
                    match *argument as libc::c_int {
                        45 => { argument = argument.offset(1isize) }
                        104 | 72 => { return usage_advanced(exename) }
                        98 => {
                            argument = argument.offset(1isize);
                            algNb = 0i32 as U32;
                            while *argument as libc::c_int >= '0' as i32 &&
                                      *argument as libc::c_int <= '9' as i32 {
                                algNb =
                                    (algNb as
                                         libc::c_uint).wrapping_mul(10i32 as
                                                                        libc::c_uint)
                                        as U32 as U32;
                                let fresh10 = argument;
                                argument = argument.offset(1);
                                algNb =
                                    (algNb as
                                         libc::c_uint).wrapping_add((*fresh10
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         '0'
                                                                             as
                                                                             i32)
                                                                        as
                                                                        libc::c_uint)
                                        as U32 as U32
                            }
                        }
                        105 => {
                            argument = argument.offset(1isize);
                            nbLoops = 0i32 as U32;
                            while *argument as libc::c_int >= '0' as i32 &&
                                      *argument as libc::c_int <= '9' as i32 {
                                nbLoops =
                                    (nbLoops as
                                         libc::c_uint).wrapping_mul(10i32 as
                                                                        libc::c_uint)
                                        as U32 as U32;
                                let fresh11 = argument;
                                argument = argument.offset(1);
                                nbLoops =
                                    (nbLoops as
                                         libc::c_uint).wrapping_add((*fresh11
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         '0'
                                                                             as
                                                                             i32)
                                                                        as
                                                                        libc::c_uint)
                                        as U32 as U32
                            }
                        }
                        80 => {
                            argument = argument.offset(1isize);
                            proba = 0i32 as U32;
                            while *argument as libc::c_int >= '0' as i32 &&
                                      *argument as libc::c_int <= '9' as i32 {
                                proba =
                                    (proba as
                                         libc::c_uint).wrapping_mul(10i32 as
                                                                        libc::c_uint)
                                        as U32 as U32;
                                let fresh12 = argument;
                                argument = argument.offset(1);
                                proba =
                                    (proba as
                                         libc::c_uint).wrapping_add((*fresh12
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         '0'
                                                                             as
                                                                             i32)
                                                                        as
                                                                        libc::c_uint)
                                        as U32 as U32
                            }
                        }
                        66 => {
                            argument = argument.offset(1isize);
                            blockSize = 0i32 as U32;
                            while *argument as libc::c_int >= '0' as i32 &&
                                      *argument as libc::c_int <= '9' as i32 {
                                blockSize =
                                    (blockSize as
                                         libc::c_uint).wrapping_mul(10i32 as
                                                                        libc::c_uint)
                                        as U32 as U32;
                                let fresh13 = argument;
                                argument = argument.offset(1);
                                blockSize =
                                    (blockSize as
                                         libc::c_uint).wrapping_add((*fresh13
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         '0'
                                                                             as
                                                                             i32)
                                                                        as
                                                                        libc::c_uint)
                                        as U32 as U32
                            }
                            if *argument.offset(0isize) as libc::c_int ==
                                   'K' as i32 {
                                blockSize <<= 10i32;
                                argument = argument.offset(1isize)
                            }
                            if *argument.offset(0isize) as libc::c_int ==
                                   'M' as i32 {
                                blockSize <<= 20i32;
                                argument = argument.offset(1isize)
                            }
                            if *argument.offset(0isize) as libc::c_int ==
                                   'B' as i32 {
                                argument = argument.offset(1isize)
                            }
                        }
                        112 => {
                            pause = 1i32 as U32;
                            argument = argument.offset(1isize)
                        }
                        _ => { return badusage(exename) }
                    }
                }
            } else if fnStart == 0i32 { fnStart = i }
        }
        i += 1
    }
    result =
        benchMultipleFiles(argv, argc, fnStart,
                           proba as libc::c_double / 100i32 as libc::c_double,
                           blockSize as size_t, algNb, nbLoops);
    if 0 != pause {
        fprintf(__stderrp,
                b"press enter...\n\x00" as *const u8 as *const libc::c_char);
        getchar();
    }
    return result;
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
unsafe extern "C" fn run_static_initializers() {
    g_huff_dtable =
        {
            [(12i32 as U32).wrapping_mul(0x1000001i32 as libc::c_uint), 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0]
        }
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];