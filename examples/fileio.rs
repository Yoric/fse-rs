#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    static mut __stdinp: *mut FILE;
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fclose(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn getchar() -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    /*-*****************************************
    *  Tool functions
    ******************************************/
    #[no_mangle]
    fn FSE_compressBound(size: size_t) -> size_t;
    #[no_mangle]
    fn XXH32_digest(statePtr: *const XXH32_state_t) -> XXH32_hash_t;
    #[no_mangle]
    fn FSE_getErrorName(code: size_t) -> *const libc::c_char;
    /* Error Management */
    #[no_mangle]
    fn FSE_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn XXH32_update(
        statePtr: *mut XXH32_state_t,
        input: *const libc::c_void,
        length: size_t,
    ) -> XXH_errorcode;
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
    fn ZLIBH_compress(
        dest: *mut libc::c_char,
        source: *const libc::c_char,
        inputSize: libc::c_int,
    ) -> libc::c_int;
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
    fn HUF_compress(
        dst: *mut libc::c_void,
        dstCapacity: size_t,
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
    /* hash streaming */
    #[no_mangle]
    fn XXH32_reset(statePtr: *mut XXH32_state_t, seed: libc::c_uint) -> XXH_errorcode;
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
    #[no_mangle]
    fn ZLIBH_decompress(dest: *mut libc::c_char, compressed: *const libc::c_char) -> libc::c_int;
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
    fn HUF_decompress(
        dst: *mut libc::c_void,
        originalSize: size_t,
        cSrc: *const libc::c_void,
        cSrcSize: size_t,
    ) -> size_t;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_clock_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
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
pub type clock_t = __darwin_clock_t;
/* *************************************
*  Parameters
**************************************/
pub type FIO_compressor_t = libc::c_uint;
pub const FIO_zlibh: FIO_compressor_t = 2;
pub const FIO_huf: FIO_compressor_t = 1;
pub const FIO_fse: FIO_compressor_t = 0;
pub type U32 = uint32_t;
pub type uint32_t = libc::c_uint;
pub type U64 = uint64_t;
pub type uint64_t = libc::c_ulonglong;
/*
  fileio.c - simple generic file i/o handler
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
/*
  Note : this is stand-alone program.
  It is not part of FSE compression library, it is a user program of the FSE library.
  The license of FSE library is BSD.
  The license of this library is GPLv2.
*/
/*-************************************
*  Compiler Options
**************************************/
/* Disable some Visual warning messages */
/* Large file support on 32-bits unix */
/* enable fileno() within <stdio.h> on unix */
/*-************************************
*  Includes
**************************************/
/* fprintf, fopen, fread, _fileno, stdin, stdout */
/* malloc, free */
/* strcmp, strlen */
/* clock */
/* assert */
/*-************************************
*  OS-specific Includes
**************************************/
// isatty
/*-************************************
*  Basic Types
**************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type uint8_t = libc::c_uchar;
pub const bt_crc: unnamed = 3;
/* ****************************
*  Simple Hash Functions
******************************/
pub type XXH32_hash_t = libc::c_uint;
/* !
XXH32() :
    Calculate the 32-bits hash of sequence "length" bytes stored at memory address "input".
    The memory between input & input+length must be valid (allocated and read-accessible).
    "seed" can be used to alter the result predictably.
    Speed on Core 2 Duo @ 3 GHz (single thread, SMHasher benchmark) : 5.4 GB/s
XXH64() :
    Calculate the 64-bits hash of sequence of length "len" stored at memory address "input".
    "seed" can be used to alter the result predictably.
    This function runs 2x faster on 64-bits systems, but slower on 32-bits systems (see benchmark).
*/
/* ****************************
*  Streaming Hash Functions
******************************/
/* incomplete type */
pub type XXH32_state_t = XXH32_state_s;
/* Default result type for XXH functions are primitive unsigned 32 and 64 bits.
*  The canonical representation uses human-readable write convention, aka big-endian (large digits first).
*  These functions allow transformation of hash result into and from its canonical format.
*  This way, hash values can be written into a file / memory, and remain comparable on different systems and programs.
*/
/* ================================================================================================
   This section contains definitions which are not guaranteed to remain stable.
   They could change in a future version, becoming incompatible with a different version of the library.
   They shall only be used with static linking.
=================================================================================================== */
/* These definitions allow allocating XXH state statically (on stack) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_state_s {
    pub total_len: libc::c_ulonglong,
    pub seed: libc::c_uint,
    pub v1: libc::c_uint,
    pub v2: libc::c_uint,
    pub v3: libc::c_uint,
    pub v4: libc::c_uint,
    pub mem32: [libc::c_uint; 4],
    pub memsize: libc::c_uint,
}
pub const bt_compressed: unnamed = 0;
pub const bt_rle: unnamed = 2;
pub const bt_raw: unnamed = 1;
pub type compressor_t = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: size_t,
        _: *const libc::c_void,
        _: size_t,
    ) -> size_t,
>;
/*
   xxHash - Extremely Fast Hash algorithm
   Header File
   Copyright (C) 2012-2016, Yann Collet.

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
   - xxHash source repository : https://github.com/Cyan4973/xxHash
*/
/* Notice extracted from xxHash homepage :

xxHash is an extremely fast Hash algorithm, running at RAM speed limits.
It also successfully passes all tests from the SMHasher suite.

Comparison (single thread, Windows Seven 32 bits, using SMHasher on a Core 2 Duo @3GHz)

Name            Speed       Q.Score   Author
xxHash          5.4 GB/s     10
CrapWow         3.2 GB/s      2       Andrew
MumurHash 3a    2.7 GB/s     10       Austin Appleby
SpookyHash      2.0 GB/s     10       Bob Jenkins
SBox            1.4 GB/s      9       Bret Mulvey
Lookup3         1.2 GB/s      9       Bob Jenkins
SuperFastHash   1.2 GB/s      1       Paul Hsieh
CityHash64      1.05 GB/s    10       Pike & Alakuijala
FNV             0.55 GB/s     5       Fowler, Noll, Vo
CRC32           0.43 GB/s     9
MD5-32          0.33 GB/s    10       Ronald L. Rivest
SHA1-32         0.28 GB/s    10

Q.Score is a measure of quality of the hash function.
It depends on successfully passing SMHasher test set.
10 is a perfect score.

A 64-bits version, named XXH64, is available since r35.
It offers much better speed, but for 64-bits applications only.
Name     Speed on 64 bits    Speed on 32 bits
XXH64       13.8 GB/s            1.9 GB/s
XXH32        6.8 GB/s            6.0 GB/s
*/
/* ****************************
*  Definitions
******************************/
/* size_t */
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type decompressor_t = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: size_t,
        _: *const libc::c_void,
        _: size_t,
    ) -> size_t,
>;
/* as a define, because needed to allocated table on stack */
/* as a define, because needed to init static g_blockSizeId */
/*-************************************
*  Complex types
**************************************/
pub type unnamed = libc::c_uint;
/*
  fileio.h - simple generic file i/o handler
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
/* *************************************
*  Special i/o constants
**************************************/
static mut stdinmark: [libc::c_char; 6] = [115, 116, 100, 105, 110, 0];
static mut stdoutmark: [libc::c_char; 7] = [115, 116, 100, 111, 117, 116, 0];
#[no_mangle]
pub unsafe extern "C" fn FIO_setCompressor(mut c: FIO_compressor_t) {
    g_compressor = c;
}
#[no_mangle]
pub static mut g_compressor: FIO_compressor_t = FIO_fse;
#[no_mangle]
pub unsafe extern "C" fn FIO_setDisplayLevel(mut dlevel: libc::c_int) {
    g_displayLevel = dlevel;
}
/*-************************************
*  Macros
**************************************/
/* 0 : no display;   1: errors;   2 : + result + interaction + warnings;   3 : + progression;   4 : + information */
static mut g_displayLevel: libc::c_int = 2i32;
#[no_mangle]
pub unsafe extern "C" fn FIO_overwriteMode() {
    g_overwrite = 1i32 as U32;
}
/*-************************************
*  Local Parameters
**************************************/
static mut g_overwrite: U32 = 0i32 as U32;
/* *************************************
*  Stream/File functions
**************************************/
#[no_mangle]
pub unsafe extern "C" fn FIO_compressFilename(
    mut output_filename: *const libc::c_char,
    mut input_filename: *const libc::c_char,
) -> libc::c_ulonglong {
    let mut filesize: U64 = 0i32 as U64;
    let mut compressedfilesize: U64 = 0i32 as U64;
    let mut finput: *mut FILE = 0 as *mut FILE;
    let mut foutput: *mut FILE = 0 as *mut FILE;
    let inputBlockSize: size_t = FIO_blockID_to_blockSize(g_blockSizeId as libc::c_int) as size_t;
    let in_buff: *mut libc::c_char = malloc(inputBlockSize) as *mut libc::c_char;
    let out_buff: *mut libc::c_char =
        malloc(FSE_compressBound(inputBlockSize).wrapping_add(5i32 as libc::c_ulong))
            as *mut libc::c_char;
    let mut xxhState: XXH32_state_t = XXH32_state_s {
        total_len: 0,
        seed: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        v4: 0,
        mem32: [0; 4],
        memsize: 0,
    };
    let mut compressor: compressor_t = None;
    let mut magicNumber: libc::c_uint = 0;
    if in_buff.is_null() || out_buff.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                21i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Allocation error : not enough memory\x00" as *const u8 as *const libc::c_char,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(21i32);
    }
    XXH32_reset(&mut xxhState, 0i32 as libc::c_uint);
    get_fileHandle(input_filename, output_filename, &mut finput, &mut foutput);
    match g_compressor as libc::c_uint {
        0 => {
            compressor = Some(FSE_compress);
            magicNumber = 0x183e2309i32 as libc::c_uint
        }
        1 => {
            compressor = Some(HUF_compress);
            magicNumber = 0x183e3309i32 as libc::c_uint
        }
        2 => {
            compressor = Some(FIO_ZLIBH_compress);
            magicNumber = 0x183e4309i32 as libc::c_uint
        }
        _ => {
            if g_displayLevel >= 1i32 {
                fprintf(
                    __stderrp,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    20i32,
                );
            }
            if g_displayLevel >= 1i32 {
                fprintf(
                    __stderrp,
                    b"unknown compressor selection\x00" as *const u8 as *const libc::c_char,
                );
            }
            if g_displayLevel >= 1i32 {
                fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
            }
            exit(20i32);
        }
    }
    FIO_writeLE32(out_buff as *mut libc::c_void, magicNumber);
    *out_buff.offset(4isize) = g_blockSizeId as libc::c_char;
    let sizeCheck: size_t = fwrite(
        out_buff as *const libc::c_void,
        1i32 as libc::c_ulong,
        5i32 as libc::c_ulong,
        foutput,
    );
    if sizeCheck != 5i32 as libc::c_ulong {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                22i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Write error : cannot write header\x00" as *const u8 as *const libc::c_char,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(22i32);
    }
    compressedfilesize = (compressedfilesize as libc::c_ulonglong)
        .wrapping_add(5i32 as libc::c_ulonglong) as U64 as U64;
    loop {
        /* Fill input Buffer */
        let mut cSize: size_t = 0;
        let inSize: size_t = fread(
            in_buff as *mut libc::c_void,
            1i32 as size_t,
            inputBlockSize,
            finput,
        );
        if g_displayLevel >= 6i32 {
            fprintf(
                __stderrp,
                b"reading %zu bytes from input (%s)\n\x00" as *const u8 as *const libc::c_char,
                inSize,
                input_filename,
            );
        }
        if inSize == 0i32 as libc::c_ulong {
            break;
        }
        filesize =
            (filesize as libc::c_ulonglong).wrapping_add(inSize as libc::c_ulonglong) as U64 as U64;
        XXH32_update(&mut xxhState, in_buff as *const libc::c_void, inSize);
        if g_displayLevel >= 2i32 {
            if FIO_GetMilliSpan(g_time) > refreshRate || g_displayLevel >= 4i32 {
                g_time = clock();
                fprintf(
                    __stderrp,
                    b"\rRead : %u MB   \x00" as *const u8 as *const libc::c_char,
                    (filesize >> 20i32) as U32,
                );
                if g_displayLevel >= 4i32 {
                    fflush(__stdoutp);
                }
            }
        }
        cSize = compressor.expect("non-null function pointer")(
            out_buff.offset(FIO_maxBlockHeaderSize as isize) as *mut libc::c_void,
            FSE_compressBound(inputBlockSize),
            in_buff as *const libc::c_void,
            inSize,
        );
        if 0 != FSE_isError(cSize) {
            if g_displayLevel >= 1i32 {
                fprintf(
                    __stderrp,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    23i32,
                );
            }
            if g_displayLevel >= 1i32 {
                fprintf(
                    __stderrp,
                    b"Compression error : %s \x00" as *const u8 as *const libc::c_char,
                    FSE_getErrorName(cSize),
                );
            }
            if g_displayLevel >= 1i32 {
                fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
            }
            exit(23i32);
        }
        let mut headerSize: size_t = 0;
        match cSize {
            0 => {
                if g_displayLevel >= 6i32 {
                    fprintf(
                        __stderrp,
                        b"packing uncompressed block, of size %zu \n\x00" as *const u8
                            as *const libc::c_char,
                        inSize,
                    );
                }
                if inSize == inputBlockSize {
                    *out_buff.offset(0isize) =
                        (((bt_raw as libc::c_int) << 6i32) + 0x20i32) as BYTE as libc::c_char;
                    headerSize = 1i32 as size_t
                } else {
                    *out_buff.offset(0isize) =
                        ((bt_raw as libc::c_int) << 6i32) as BYTE as libc::c_char;
                    *out_buff.offset(1isize) = (inSize >> 8i32) as BYTE as libc::c_char;
                    *out_buff.offset(2isize) = inSize as BYTE as libc::c_char;
                    headerSize = 3i32 as size_t
                }
                let sizeCheck_0: size_t = fwrite(
                    out_buff as *const libc::c_void,
                    1i32 as libc::c_ulong,
                    headerSize,
                    foutput,
                );
                if sizeCheck_0 != headerSize {
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Error %i : \x00" as *const u8 as *const libc::c_char,
                            24i32,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Write error : cannot write block header\x00" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    exit(24i32);
                }
                let sizeCheck_1: size_t = fwrite(
                    in_buff as *const libc::c_void,
                    1i32 as libc::c_ulong,
                    inSize,
                    foutput,
                );
                if sizeCheck_1 != inSize {
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Error %i : \x00" as *const u8 as *const libc::c_char,
                            25i32,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Write error : cannot write block\x00" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    exit(25i32);
                }
                compressedfilesize = (compressedfilesize as libc::c_ulonglong)
                    .wrapping_add(inSize.wrapping_add(headerSize) as libc::c_ulonglong)
                    as U64 as U64
            }
            1 => {
                if g_displayLevel >= 6i32 {
                    fprintf(
                        __stderrp,
                        b"packing RLE block, of size %zu \n\x00" as *const u8
                            as *const libc::c_char,
                        inSize,
                    );
                }
                if inSize == inputBlockSize {
                    *out_buff.offset(0isize) =
                        (((bt_rle as libc::c_int) << 6i32) + 0x20i32) as BYTE as libc::c_char;
                    headerSize = 1i32 as size_t
                } else {
                    *out_buff.offset(0isize) =
                        ((bt_rle as libc::c_int) << 6i32) as BYTE as libc::c_char;
                    *out_buff.offset(1isize) = (inSize >> 8i32) as BYTE as libc::c_char;
                    *out_buff.offset(2isize) = inSize as BYTE as libc::c_char;
                    headerSize = 3i32 as size_t
                }
                *out_buff.offset(headerSize as isize) = *in_buff.offset(0isize);
                let sizeCheck_2: size_t = fwrite(
                    out_buff as *const libc::c_void,
                    1i32 as libc::c_ulong,
                    headerSize.wrapping_add(1i32 as libc::c_ulong),
                    foutput,
                );
                if sizeCheck_2 != headerSize.wrapping_add(1i32 as libc::c_ulong) {
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Error %i : \x00" as *const u8 as *const libc::c_char,
                            26i32,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Write error : cannot write rle block\x00" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    exit(26i32);
                }
                compressedfilesize = (compressedfilesize as libc::c_ulonglong).wrapping_add(
                    headerSize.wrapping_add(1i32 as libc::c_ulong) as libc::c_ulonglong,
                ) as U64 as U64
            }
            _ => {
                if g_displayLevel >= 6i32 {
                    fprintf(
                        __stderrp,
                        b"packing compressed block, of size %zu, into %zu bytes \n\x00" as *const u8
                            as *const libc::c_char,
                        inSize,
                        cSize,
                    );
                }
                if inSize == inputBlockSize {
                    *out_buff.offset(2isize) = (((bt_compressed as libc::c_int) << 6i32) + 0x20i32)
                        as BYTE as libc::c_char;
                    if g_displayLevel >= 7i32 {
                        fprintf(
                            __stderrp,
                            b"generated block descriptor : %u \n\x00" as *const u8
                                as *const libc::c_char,
                            *out_buff.offset(2isize) as libc::c_int,
                        );
                    }
                    *out_buff.offset(3isize) = (cSize >> 8i32) as BYTE as libc::c_char;
                    *out_buff.offset(4isize) = cSize as BYTE as libc::c_char;
                    headerSize = 3i32 as size_t
                } else {
                    *out_buff.offset(0isize) =
                        ((bt_compressed as libc::c_int) << 6i32) as BYTE as libc::c_char;
                    *out_buff.offset(1isize) = (inSize >> 8i32) as BYTE as libc::c_char;
                    *out_buff.offset(2isize) = inSize as BYTE as libc::c_char;
                    *out_buff.offset(3isize) = (cSize >> 8i32) as BYTE as libc::c_char;
                    *out_buff.offset(4isize) = cSize as BYTE as libc::c_char;
                    headerSize = FIO_maxBlockHeaderSize as size_t
                }
                let sizeCheck_3: size_t = fwrite(
                    out_buff.offset(
                        (FIO_maxBlockHeaderSize as libc::c_ulong).wrapping_sub(headerSize) as isize,
                    ) as *const libc::c_void,
                    1i32 as libc::c_ulong,
                    headerSize.wrapping_add(cSize),
                    foutput,
                );
                if sizeCheck_3 != headerSize.wrapping_add(cSize) {
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Error %i : \x00" as *const u8 as *const libc::c_char,
                            27i32,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Write error : cannot write rle block\x00" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    exit(27i32);
                }
                compressedfilesize = (compressedfilesize as libc::c_ulonglong)
                    .wrapping_add(headerSize.wrapping_add(cSize) as libc::c_ulonglong)
                    as U64 as U64
            }
        }
        if g_displayLevel >= 2i32 {
            if FIO_GetMilliSpan(g_time) > refreshRate || g_displayLevel >= 4i32 {
                g_time = clock();
                fprintf(
                    __stderrp,
                    b"\rRead : %u MB  ==> %.2f%%   \x00" as *const u8 as *const libc::c_char,
                    (filesize >> 20i32) as U32,
                    compressedfilesize as libc::c_double / filesize as libc::c_double
                        * 100i32 as libc::c_double,
                );
                if g_displayLevel >= 4i32 {
                    fflush(__stdoutp);
                }
            }
        }
    }
    let mut checksum: U32 = XXH32_digest(&mut xxhState);
    checksum = checksum >> 5i32 & (1u32 << 22i32).wrapping_sub(1i32 as libc::c_uint);
    *out_buff.offset(2isize) = checksum as BYTE as libc::c_char;
    *out_buff.offset(1isize) = (checksum >> 8i32) as BYTE as libc::c_char;
    *out_buff.offset(0isize) = (checksum >> 16i32)
        .wrapping_add(((bt_crc as libc::c_int) << 6i32) as libc::c_uint)
        as BYTE as libc::c_char;
    let sizeCheck_4: size_t = fwrite(
        out_buff as *const libc::c_void,
        1i32 as libc::c_ulong,
        3i32 as libc::c_ulong,
        foutput,
    );
    if sizeCheck_4 != 3i32 as libc::c_ulong {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                28i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Write error : cannot write checksum\x00" as *const u8 as *const libc::c_char,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(28i32);
    }
    compressedfilesize = (compressedfilesize as libc::c_ulonglong)
        .wrapping_add(3i32 as libc::c_ulonglong) as U64 as U64;
    if g_displayLevel >= 2i32 {
        fprintf(
            __stderrp,
            b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    if g_displayLevel >= 2i32 {
        fprintf(
            __stderrp,
            b"Compressed %llu bytes into %llu bytes ==> %.2f%%\n\x00" as *const u8
                as *const libc::c_char,
            filesize,
            compressedfilesize,
            compressedfilesize as libc::c_double / filesize as libc::c_double
                * 100i32 as libc::c_double,
        );
    }
    free(in_buff as *mut libc::c_void);
    free(out_buff as *mut libc::c_void);
    fclose(finput);
    fclose(foutput);
    return compressedfilesize;
}
static mut g_blockSizeId: U32 = 5i32 as U32;
unsafe extern "C" fn FIO_blockID_to_blockSize(mut id: libc::c_int) -> libc::c_int {
    return ((1i32 << id) as libc::c_uint).wrapping_mul(1u32 << 10i32) as libc::c_int;
}
static mut g_time: clock_t = 0i32 as clock_t;
static mut refreshRate: libc::c_uint = 150i32 as libc::c_uint;
/*-************************************
*  Exceptions
**************************************/
/*-************************************
*  Version modifiers
**************************************/
/*-************************************
*  Functions
**************************************/
unsafe extern "C" fn FIO_GetMilliSpan(mut nPrevious: clock_t) -> libc::c_uint {
    let mut nCurrent: clock_t = clock();
    let mut nSpan: libc::c_uint = nCurrent
        .wrapping_sub(nPrevious)
        .wrapping_mul(1000i32 as libc::c_ulong)
        .wrapping_div(1000000i32 as libc::c_ulong)
        as libc::c_uint;
    return nSpan;
}
static mut FIO_maxBlockHeaderSize: libc::c_uint = 5i32 as libc::c_uint;
/*-************************************
*  Memory operations
**************************************/
unsafe extern "C" fn FIO_writeLE32(mut memPtr: *mut libc::c_void, mut val32: U32) {
    let mut p: *mut BYTE = memPtr as *mut BYTE;
    *p.offset(0isize) = val32 as BYTE;
    *p.offset(1isize) = (val32 >> 8i32) as BYTE;
    *p.offset(2isize) = (val32 >> 16i32) as BYTE;
    *p.offset(3isize) = (val32 >> 24i32) as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_ZLIBH_compress(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZLIBH_compress(
        dst as *mut libc::c_char,
        src as *const libc::c_char,
        srcSize as libc::c_int,
    ) as size_t;
}
unsafe extern "C" fn get_fileHandle(
    mut input_filename: *const libc::c_char,
    mut output_filename: *const libc::c_char,
    mut pfinput: *mut *mut FILE,
    mut pfoutput: *mut *mut FILE,
) {
    if 0 == strcmp(input_filename, stdinmark.as_ptr()) {
        if g_displayLevel >= 4i32 {
            fprintf(
                __stderrp,
                b"Using stdin for input\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        *pfinput = __stdinp
    } else {
        *pfinput = fopen(
            input_filename,
            b"rb\x00" as *const u8 as *const libc::c_char,
        )
    }
    if (*pfinput).is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                12i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Pb opening %s\x00" as *const u8 as *const libc::c_char,
                input_filename,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(12i32);
    }
    if 0 == strcmp(output_filename, stdoutmark.as_ptr()) {
        if g_displayLevel >= 4i32 {
            fprintf(
                __stderrp,
                b"Using stdout for output\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        *pfoutput = __stdoutp
    } else {
        *pfoutput = 0 as *mut FILE;
        if 0 != strcmp(
            output_filename,
            b"/dev/null\x00" as *const u8 as *const libc::c_char,
        ) {
            *pfoutput = fopen(
                output_filename,
                b"rb\x00" as *const u8 as *const libc::c_char,
            )
        }
        if !(*pfoutput).is_null() {
            fclose(*pfoutput);
            if 0 == g_overwrite {
                let mut ch: libc::c_char = 0;
                if g_displayLevel <= 1i32 {
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Error %i : \x00" as *const u8 as *const libc::c_char,
                            11i32,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Operation aborted : %s already exists\x00" as *const u8
                                as *const libc::c_char,
                            output_filename,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    exit(11i32);
                }
                if g_displayLevel >= 2i32 {
                    fprintf(
                        __stderrp,
                        b"Warning : %s already exists\n\x00" as *const u8 as *const libc::c_char,
                        output_filename,
                    );
                }
                if g_displayLevel >= 2i32 {
                    fprintf(
                        __stderrp,
                        b"Overwrite ? (Y/N) : \x00" as *const u8 as *const libc::c_char,
                    );
                }
                ch = getchar() as libc::c_char;
                if ch as libc::c_int != 'Y' as i32 && ch as libc::c_int != 'y' as i32 {
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Error %i : \x00" as *const u8 as *const libc::c_char,
                            11i32,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Operation aborted : %s already exists\x00" as *const u8
                                as *const libc::c_char,
                            output_filename,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    exit(11i32);
                }
            }
        }
        *pfoutput = fopen(
            output_filename,
            b"wb\x00" as *const u8 as *const libc::c_char,
        )
    }
    if (*pfoutput).is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                13i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Pb opening %s\x00" as *const u8 as *const libc::c_char,
                output_filename,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(13i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressFilename(
    mut output_filename: *const libc::c_char,
    mut input_filename: *const libc::c_char,
) -> libc::c_ulonglong {
    let mut finput: *mut FILE = 0 as *mut FILE;
    let mut foutput: *mut FILE = 0 as *mut FILE;
    let mut filesize: U64 = 0i32 as U64;
    let mut in_buff: *mut BYTE = 0 as *mut BYTE;
    let mut out_buff: *mut BYTE = 0 as *mut BYTE;
    let mut ip: *mut BYTE = 0 as *mut BYTE;
    let mut blockSize: U32 = 0;
    let mut xxhState: XXH32_state_t = XXH32_state_s {
        total_len: 0,
        seed: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        v4: 0,
        mem32: [0; 4],
        memsize: 0,
    };
    let mut decompressor: decompressor_t = Some(FSE_decompress);
    XXH32_reset(&mut xxhState, 0i32 as libc::c_uint);
    get_fileHandle(input_filename, output_filename, &mut finput, &mut foutput);
    let mut header: [BYTE; 5] = [0; 5];
    let sizeCheck: size_t = fread(
        header.as_mut_ptr() as *mut libc::c_void,
        1i32 as size_t,
        5i32 as libc::c_ulong,
        finput,
    );
    if sizeCheck != 5i32 as libc::c_ulong {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                30i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Read error : cannot read header\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(30i32);
    }
    match FIO_readLE32(header.as_mut_ptr() as *const libc::c_void) {
        406725385 => {
            if g_displayLevel >= 5i32 {
                fprintf(
                    __stderrp,
                    b"compressed with fse \n\x00" as *const u8 as *const libc::c_char,
                );
            }
            decompressor = Some(FSE_decompress)
        }
        406729481 => {
            if g_displayLevel >= 5i32 {
                fprintf(
                    __stderrp,
                    b"compressed with huff0 \n\x00" as *const u8 as *const libc::c_char,
                );
            }
            decompressor = Some(HUF_decompress)
        }
        406733577 => {
            if g_displayLevel >= 5i32 {
                fprintf(
                    __stderrp,
                    b"compressed with zlib\'s huffman \n\x00" as *const u8 as *const libc::c_char,
                );
            }
            decompressor = Some(FIO_ZLIBH_decompress)
        }
        _ => {
            if g_displayLevel >= 1i32 {
                fprintf(
                    __stderrp,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    31i32,
                );
            }
            if g_displayLevel >= 1i32 {
                fprintf(
                    __stderrp,
                    b"Wrong file type : unknown header\n\x00" as *const u8 as *const libc::c_char,
                );
            }
            if g_displayLevel >= 1i32 {
                fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
            }
            exit(31i32);
        }
    }
    let blockSizeId: U32 = header[4usize] as U32;
    if blockSizeId > FIO_maxBlockSizeID {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                32i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Wrong version : unknown header flags\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(32i32);
    }
    blockSize = FIO_blockID_to_blockSize(blockSizeId as libc::c_int) as U32;
    in_buff = malloc(blockSize.wrapping_add(FIO_maxBlockHeaderSize) as libc::c_ulong) as *mut BYTE;
    out_buff = malloc(blockSize as libc::c_ulong) as *mut BYTE;
    if in_buff.is_null() || out_buff.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                33i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Allocation error : not enough memory\x00" as *const u8 as *const libc::c_char,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(33i32);
    }
    let sizeCheck_0: size_t = fread(
        in_buff as *mut libc::c_void,
        1i32 as libc::c_ulong,
        1i32 as libc::c_ulong,
        finput,
    );
    if sizeCheck_0 != 1i32 as libc::c_ulong {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                34i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Read error : cannot read header\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(34i32);
    }
    ip = in_buff;
    loop {
        let mut rSize: size_t = blockSize as size_t;
        let mut cSize: size_t = 0;
        /* Decode header */
        let bType: libc::c_int = (*ip.offset(0isize) as libc::c_int & 0x80i32 + 0x40i32) >> 6i32;
        if g_displayLevel >= 6i32 {
            fprintf(
                __stderrp,
                b"next block type == %i \n\x00" as *const u8 as *const libc::c_char,
                bType,
            );
        }
        if g_displayLevel >= 7i32 {
            fprintf(
                __stderrp,
                b"read block descriptor : %u \n\x00" as *const u8 as *const libc::c_char,
                *ip.offset(0isize) as libc::c_int,
            );
        }
        if bType == bt_crc as libc::c_int {
            /* end - frame content CRC */
            break;
        } else {
            let fullBlock: libc::c_int = *ip.offset(0isize) as libc::c_int & 0x20i32;
            if g_displayLevel >= 6i32 {
                fprintf(
                    __stderrp,
                    b"next block is full ? ==> %i \n\x00" as *const u8 as *const libc::c_char,
                    (0 != fullBlock) as libc::c_int,
                );
            }
            if 0 == fullBlock {
                let sizeCheck_1: size_t = fread(
                    in_buff as *mut libc::c_void,
                    1i32 as libc::c_ulong,
                    2i32 as libc::c_ulong,
                    finput,
                );
                if sizeCheck_1 != 2i32 as libc::c_ulong {
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Error %i : \x00" as *const u8 as *const libc::c_char,
                            35i32,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(
                            __stderrp,
                            b"Read error : cannot read header\n\x00" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    exit(35i32);
                }
                rSize = (((*in_buff.offset(0isize) as libc::c_int) << 8i32)
                    + *in_buff.offset(1isize) as libc::c_int) as size_t
            }
            match bType {
                0 => {
                    let sizeCheck_2: size_t = fread(
                        in_buff as *mut libc::c_void,
                        1i32 as libc::c_ulong,
                        2i32 as libc::c_ulong,
                        finput,
                    );
                    if sizeCheck_2 != 2i32 as libc::c_ulong {
                        if g_displayLevel >= 1i32 {
                            fprintf(
                                __stderrp,
                                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                                36i32,
                            );
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(
                                __stderrp,
                                b"Read error : cannot read header\n\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                        }
                        exit(36i32);
                    }
                    cSize = (((*in_buff.offset(0isize) as libc::c_int) << 8i32)
                        + *in_buff.offset(1isize) as libc::c_int)
                        as size_t
                }
                1 => cSize = rSize,
                2 => cSize = 1i32 as size_t,
                3 | _ => {
                    cSize = 0i32 as size_t;
                    if 0 != (0 == 0i32) as libc::c_int as libc::c_long {
                        __assert_rtn(
                            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                                b"FIO_decompressFilename\x00",
                            ))
                            .as_ptr(),
                            b"fileio.c\x00" as *const u8 as *const libc::c_char,
                            552i32,
                            b"0\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                    };
                }
            }
            if g_displayLevel >= 6i32 {
                fprintf(
                    __stderrp,
                    b"next block has a compressed size of %zu, and an original size of %zu \n\x00"
                        as *const u8 as *const libc::c_char,
                    cSize,
                    rSize,
                );
            }
            let toReadSize: size_t = cSize.wrapping_add(1i32 as libc::c_ulong);
            let readSize: size_t = fread(
                in_buff as *mut libc::c_void,
                1i32 as libc::c_ulong,
                toReadSize,
                finput,
            );
            if readSize != toReadSize {
                if g_displayLevel >= 1i32 {
                    fprintf(
                        __stderrp,
                        b"Error %i : \x00" as *const u8 as *const libc::c_char,
                        38i32,
                    );
                }
                if g_displayLevel >= 1i32 {
                    fprintf(
                        __stderrp,
                        b"Read error\x00" as *const u8 as *const libc::c_char,
                    );
                }
                if g_displayLevel >= 1i32 {
                    fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                }
                exit(38i32);
            }
            ip = in_buff.offset(cSize as isize);
            match bType {
                0 => {
                    rSize = decompressor.expect("non-null function pointer")(
                        out_buff as *mut libc::c_void,
                        rSize,
                        in_buff as *const libc::c_void,
                        cSize,
                    );
                    if 0 != FSE_isError(rSize) {
                        if g_displayLevel >= 1i32 {
                            fprintf(
                                __stderrp,
                                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                                39i32,
                            );
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(
                                __stderrp,
                                b"Decoding error : %s\x00" as *const u8 as *const libc::c_char,
                                FSE_getErrorName(rSize),
                            );
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                        }
                        exit(39i32);
                    }
                }
                1 => {}
                2 => {
                    /* will read directly from in_buff, so no need to memcpy */
                    memset(
                        out_buff as *mut libc::c_void,
                        *in_buff.offset(0isize) as libc::c_int,
                        rSize,
                    );
                }
                3 | _ => {
                    if 0 != (0 == 0i32) as libc::c_int as libc::c_long {
                        __assert_rtn(
                            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                                b"FIO_decompressFilename\x00",
                            ))
                            .as_ptr(),
                            b"fileio.c\x00" as *const u8 as *const libc::c_char,
                            581i32,
                            b"0\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                    };
                }
            }
            match bType {
                0 | 2 => {
                    let writeSizeCheck: size_t = fwrite(
                        out_buff as *const libc::c_void,
                        1i32 as libc::c_ulong,
                        rSize,
                        foutput,
                    );
                    if writeSizeCheck != rSize {
                        if g_displayLevel >= 1i32 {
                            fprintf(
                                __stderrp,
                                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                                41i32,
                            );
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(
                                __stderrp,
                                b"Write error : unable to write data block to destination file\x00"
                                    as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                        }
                        exit(41i32);
                    }
                    XXH32_update(&mut xxhState, out_buff as *const libc::c_void, rSize);
                    filesize = (filesize as libc::c_ulonglong)
                        .wrapping_add(rSize as libc::c_ulonglong)
                        as U64 as U64
                }
                1 => {
                    let writeSizeCheck_0: size_t = fwrite(
                        in_buff as *const libc::c_void,
                        1i32 as libc::c_ulong,
                        cSize,
                        foutput,
                    );
                    if writeSizeCheck_0 != cSize {
                        if g_displayLevel >= 1i32 {
                            fprintf(
                                __stderrp,
                                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                                42i32,
                            );
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(
                                __stderrp,
                                b"Write error : unable to write data block to destination file\x00"
                                    as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
                        }
                        exit(42i32);
                    }
                    XXH32_update(&mut xxhState, in_buff as *const libc::c_void, cSize);
                    filesize = (filesize as libc::c_ulonglong)
                        .wrapping_add(cSize as libc::c_ulonglong)
                        as U64 as U64
                }
                3 | _ => {
                    if 0 != (0 == 0i32) as libc::c_int as libc::c_long {
                        __assert_rtn(
                            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                                b"FIO_decompressFilename\x00",
                            ))
                            .as_ptr(),
                            b"fileio.c\x00" as *const u8 as *const libc::c_char,
                            602i32,
                            b"0\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                    };
                }
            }
        }
    }
    let sizeCheck_3: size_t = fread(
        ip.offset(1isize) as *mut libc::c_void,
        1i32 as libc::c_ulong,
        2i32 as libc::c_ulong,
        finput,
    );
    if sizeCheck_3 != 2i32 as libc::c_ulong {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                43i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Read error\x00" as *const u8 as *const libc::c_char,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(43i32);
    }
    let CRCsaved: U32 = (*ip.offset(2isize) as libc::c_int
        + ((*ip.offset(1isize) as libc::c_int) << 8i32)
        + ((*ip.offset(0isize) as libc::c_int & 0x3fi32) << 16i32)) as U32;
    let CRCcalculated: U32 =
        XXH32_digest(&mut xxhState) >> 5i32 & (1u32 << 22i32).wrapping_sub(1i32 as libc::c_uint);
    if CRCsaved != CRCcalculated {
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                44i32,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(
                __stderrp,
                b"CRC error : wrong checksum, corrupted data\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        if g_displayLevel >= 1i32 {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(44i32);
    }
    if g_displayLevel >= 2i32 {
        fprintf(
            __stderrp,
            b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    if g_displayLevel >= 2i32 {
        fprintf(
            __stderrp,
            b"Decoded %llu bytes\n\x00" as *const u8 as *const libc::c_char,
            filesize,
        );
    }
    free(in_buff as *mut libc::c_void);
    free(out_buff as *mut libc::c_void);
    fclose(finput);
    fclose(foutput);
    return filesize;
}
/*-************************************
*  Constants
**************************************/
/* => 64 KB block */
static mut FIO_maxBlockSizeID: libc::c_uint = 6i32 as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn FIO_ZLIBH_decompress(
    mut dst: *mut libc::c_void,
    mut dstSize: size_t,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    return ZLIBH_decompress(dst as *mut libc::c_char, src as *const libc::c_char) as size_t;
}
unsafe extern "C" fn FIO_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    let mut p: *const BYTE = memPtr as *const BYTE;
    return (*p.offset(0isize) as U32)
        .wrapping_add((*p.offset(1isize) as U32) << 8i32)
        .wrapping_add((*p.offset(2isize) as U32) << 16i32)
        .wrapping_add((*p.offset(3isize) as U32) << 24i32);
}
