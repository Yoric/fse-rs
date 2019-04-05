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
    #[repr(C)]
    pub union unnamed {
        pub u: U32,
        pub c: [BYTE; 4],
    }
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
    pub type FSE_ErrorCode = libc::c_uint;
    pub const FSE_error_maxCode: FSE_ErrorCode = 9;
    pub const FSE_error_workSpace_tooSmall: FSE_ErrorCode = 8;
    pub const FSE_error_maxSymbolValue_tooSmall: FSE_ErrorCode = 7;
    pub const FSE_error_maxSymbolValue_tooLarge: FSE_ErrorCode = 6;
    pub const FSE_error_tableLog_tooLarge: FSE_ErrorCode = 5;
    pub const FSE_error_corruption_detected: FSE_ErrorCode = 4;
    pub const FSE_error_srcSize_wrong: FSE_ErrorCode = 3;
    pub const FSE_error_dstSize_tooSmall: FSE_ErrorCode = 2;
    pub const FSE_error_GENERIC: FSE_ErrorCode = 1;
    pub const FSE_error_no_error: FSE_ErrorCode = 0;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/error_private.h"]
pub mod error_private_h {
    /* ******************************************************************
       Error codes and messages
       Copyright (C) 2013-2016, Yann Collet

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
       - Homepage : http://www.zstd.net
    ****************************************************************** */
    /* Note : this module is expected to remain private, do not expose it */
    /* ****************************************
    *  Dependencies
    ******************************************/
    /* size_t */
    /* ****************************************
    *  Compiler-specific
    ******************************************/
    /* C99 */
    /*-****************************************
    *  Customization (error_public.h)
    ******************************************/
    pub type ERR_enum = FSE_ErrorCode;
    use super::error_public_h::FSE_ErrorCode;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/fse.h"]
pub mod fse_h {
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
    /* don't allocate that. It's just a way to be more restrictive than void* */
    pub type FSE_DTable = libc::c_uint;
    use super::stddef_h::size_t;
    extern "C" {
        /* *< build a fake FSE_DTable, designed to always generate the same symbolValue */
        #[no_mangle]
        pub fn FSE_decompress_wksp(
            dst: *mut libc::c_void,
            dstCapacity: size_t,
            cSrc: *const libc::c_void,
            cSrcSize: size_t,
            workSpace: *mut FSE_DTable,
            maxLog: libc::c_uint,
        ) -> size_t;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
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
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/bitstream.h"]
pub mod bitstream_h {}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/huf.h"]
pub mod huf_h {
    use super::mem_h::{BYTE, U32};
    use super::stddef_h::size_t;
}
use self::_uint32_t_h::uint32_t;
use self::_uint8_t_h::uint8_t;
use self::assert_h::__assert_rtn;
use self::error_private_h::ERR_enum;
use self::error_public_h::{
    FSE_ErrorCode, FSE_error_GENERIC, FSE_error_corruption_detected, FSE_error_dstSize_tooSmall,
    FSE_error_maxCode, FSE_error_maxSymbolValue_tooLarge, FSE_error_maxSymbolValue_tooSmall,
    FSE_error_no_error, FSE_error_srcSize_wrong, FSE_error_tableLog_tooLarge,
    FSE_error_workSpace_tooSmall,
};
use self::fse_h::{FSE_DTable, FSE_decompress_wksp};
use self::mem_h::{unalign32, unnamed, BYTE, U32};
use self::stddef_h::size_t;
use self::string_h::{memcpy, memset};
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    /* don't use static : performance detrimental  */
    let one: unnamed = unnamed { u: 1i32 as U32 };
    return one.c[0usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign32)).v;
}
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0 << 24i32 & 0xff000000u32
        | in_0 << 8i32 & 0xff0000i32 as libc::c_uint
        | in_0 >> 8i32 & 0xff00i32 as libc::c_uint
        | in_0 >> 24i32 & 0xffi32 as libc::c_uint;
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read32(memPtr);
    } else {
        return MEM_swap32(MEM_read32(memPtr));
    };
}
/*-****************************************
*  Error codes handling
******************************************/
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(FSE_error_maxCode as libc::c_int) as size_t) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if 0 == ERR_isError(code) {
        return FSE_error_no_error;
    }
    return (0i32 as libc::c_ulong).wrapping_sub(code) as ERR_enum;
}
/*-****************************************
*  Error Strings
******************************************/
unsafe extern "C" fn ERR_getErrorString(mut code: ERR_enum) -> *const libc::c_char {
    static mut notErrorCode: *const libc::c_char =
        b"Unspecified error code\x00" as *const u8 as *const libc::c_char;
    match code as libc::c_uint {
        0 => return b"No error detected\x00" as *const u8 as *const libc::c_char,
        1 => return b"Error (generic)\x00" as *const u8 as *const libc::c_char,
        2 => return b"Destination buffer is too small\x00" as *const u8 as *const libc::c_char,
        3 => return b"Src size is incorrect\x00" as *const u8 as *const libc::c_char,
        4 => return b"Corrupted block detected\x00" as *const u8 as *const libc::c_char,
        5 => {
            return b"tableLog requires too much memory : unsupported\x00" as *const u8
                as *const libc::c_char;
        }
        6 => {
            return b"Unsupported max Symbol Value : too large\x00" as *const u8
                as *const libc::c_char;
        }
        7 => {
            return b"Specified maxSymbolValue is too small\x00" as *const u8 as *const libc::c_char;
        }
        8 => return b"workspace buffer is too small\x00" as *const u8 as *const libc::c_char,
        9 | _ => return notErrorCode,
    };
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t) -> *const libc::c_char {
    return ERR_getErrorString(ERR_getErrorCode(code));
}
/* ******************************************************************
   FSE : Finite State Entropy codec
   Public Prototypes declaration
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
   - Source repository : https://github.com/Cyan4973/FiniteStateEntropy
****************************************************************** */
/*-*****************************************
*  Dependencies
******************************************/
/* size_t, ptrdiff_t */
/*-*****************************************
*  FSE_PUBLIC_API : control library symbols visibility
******************************************/
/* Visual expected */
/*------   Version   ------*/
/* *< library version number; to be used when checking dll version */
#[no_mangle]
pub unsafe extern "C" fn FSE_versionNumber() -> libc::c_uint {
    return (0i32 * 100i32 * 100i32 + 9i32 * 100i32 + 0i32) as libc::c_uint;
}
/* Error Management */
/* tells if a return value is an error code */
#[no_mangle]
pub unsafe extern "C" fn FSE_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
/* provides error code string (useful for debugging) */
#[no_mangle]
pub unsafe extern "C" fn FSE_getErrorName(mut code: size_t) -> *const libc::c_char {
    return ERR_getErrorName(code);
}
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
pub unsafe extern "C" fn FSE_readNCount(
    mut normalizedCounter: *mut libc::c_short,
    mut maxSVPtr: *mut libc::c_uint,
    mut tableLogPtr: *mut libc::c_uint,
    mut headerBuffer: *const libc::c_void,
    mut hbSize: size_t,
) -> size_t {
    let istart: *const BYTE = headerBuffer as *const BYTE;
    let iend: *const BYTE = istart.offset(hbSize as isize);
    let mut ip: *const BYTE = istart;
    let mut nbBits: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut bitStream: U32 = 0;
    let mut bitCount: libc::c_int = 0;
    let mut charnum: libc::c_uint = 0i32 as libc::c_uint;
    let mut previous0: libc::c_int = 0i32;
    if hbSize < 4i32 as libc::c_ulong {
        let mut buffer: [libc::c_char; 4] = [0; 4];
        memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
        );
        memcpy(
            buffer.as_mut_ptr() as *mut libc::c_void,
            headerBuffer,
            hbSize,
        );
        let countSize: size_t = FSE_readNCount(
            normalizedCounter,
            maxSVPtr,
            tableLogPtr,
            buffer.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
        );
        if 0 != FSE_isError(countSize) {
            return countSize;
        }
        if countSize > hbSize {
            return -(FSE_error_corruption_detected as libc::c_int) as size_t;
        }
        return countSize;
    }
    if 0 != !(hbSize >= 4i32 as libc::c_ulong) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"FSE_readNCount\x00"))
                .as_ptr(),
            b"../lib/entropy_common.c\x00" as *const u8 as *const libc::c_char,
            87i32,
            b"hbSize >= 4\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    bitStream = MEM_readLE32(ip as *const libc::c_void);
    nbBits = (bitStream & 0xfi32 as libc::c_uint).wrapping_add(5i32 as libc::c_uint) as libc::c_int;
    if nbBits > 15i32 {
        return -(FSE_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    bitStream >>= 4i32;
    bitCount = 4i32;
    *tableLogPtr = nbBits as libc::c_uint;
    remaining = (1i32 << nbBits) + 1i32;
    threshold = 1i32 << nbBits;
    nbBits += 1;
    while 0 != (remaining > 1i32) as libc::c_int & (charnum <= *maxSVPtr) as libc::c_int {
        if 0 != previous0 {
            let mut n0: libc::c_uint = charnum;
            while bitStream & 0xffffi32 as libc::c_uint == 0xffffi32 as libc::c_uint {
                n0 = n0.wrapping_add(24i32 as libc::c_uint);
                if ip < iend.offset(-5isize) {
                    ip = ip.offset(2isize);
                    bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount
                } else {
                    bitStream >>= 16i32;
                    bitCount += 16i32
                }
            }
            while bitStream & 3i32 as libc::c_uint == 3i32 as libc::c_uint {
                n0 = n0.wrapping_add(3i32 as libc::c_uint);
                bitStream >>= 2i32;
                bitCount += 2i32
            }
            n0 = n0.wrapping_add(bitStream & 3i32 as libc::c_uint);
            bitCount += 2i32;
            if n0 > *maxSVPtr {
                return -(FSE_error_maxSymbolValue_tooSmall as libc::c_int) as size_t;
            }
            while charnum < n0 {
                let fresh0 = charnum;
                charnum = charnum.wrapping_add(1);
                *normalizedCounter.offset(fresh0 as isize) = 0i32 as libc::c_short
            }
            if ip <= iend.offset(-7isize)
                || ip.offset((bitCount >> 3i32) as isize) <= iend.offset(-4isize)
            {
                if 0 != !(bitCount >> 3i32 <= 3i32) as libc::c_int as libc::c_long {
                    __assert_rtn(
                        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"FSE_readNCount\x00",
                        ))
                        .as_ptr(),
                        b"../lib/entropy_common.c\x00" as *const u8 as *const libc::c_char,
                        121i32,
                        b"(bitCount >> 3) <= 3\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                };
                ip = ip.offset((bitCount >> 3i32) as isize);
                bitCount &= 7i32;
                bitStream = MEM_readLE32(ip as *const libc::c_void) >> bitCount
            } else {
                bitStream >>= 2i32
            }
        }
        let max: libc::c_int = 2i32 * threshold - 1i32 - remaining;
        let mut count: libc::c_int = 0;
        if (bitStream & (threshold - 1i32) as libc::c_uint) < max as U32 {
            count = (bitStream & (threshold - 1i32) as libc::c_uint) as libc::c_int;
            bitCount += nbBits - 1i32
        } else {
            count = (bitStream & (2i32 * threshold - 1i32) as libc::c_uint) as libc::c_int;
            if count >= threshold {
                count -= max
            }
            bitCount += nbBits
        }
        count -= 1;
        remaining -= if count < 0i32 { -count } else { count };
        let fresh1 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh1 as isize) = count as libc::c_short;
        previous0 = (0 == count) as libc::c_int;
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1i32
        }
        if ip <= iend.offset(-7isize)
            || ip.offset((bitCount >> 3i32) as isize) <= iend.offset(-4isize)
        {
            ip = ip.offset((bitCount >> 3i32) as isize);
            bitCount &= 7i32
        } else {
            bitCount -= (8i32 as libc::c_long
                * iend.offset(-4isize).wrapping_offset_from(ip) as libc::c_long)
                as libc::c_int;
            ip = iend.offset(-4isize)
        }
        bitStream = MEM_readLE32(ip as *const libc::c_void) >> (bitCount & 31i32)
    }
    if remaining != 1i32 {
        return -(FSE_error_corruption_detected as libc::c_int) as size_t;
    }
    if bitCount > 32i32 {
        return -(FSE_error_corruption_detected as libc::c_int) as size_t;
    }
    let mut symbNb: libc::c_uint = charnum;
    symbNb = charnum;
    while symbNb <= *maxSVPtr {
        *normalizedCounter.offset(symbNb as isize) = 0i32 as libc::c_short;
        symbNb = symbNb.wrapping_add(1)
    }
    *maxSVPtr = charnum.wrapping_sub(1i32 as libc::c_uint);
    ip = ip.offset((bitCount + 7i32 >> 3i32) as isize);
    return ip.wrapping_offset_from(istart) as libc::c_long as size_t;
}
/* faster, but works only if nbBits >= 1 */
/*-**************************************************************
*  Internal functions
****************************************************************/
unsafe extern "C" fn BIT_highbit32(mut val: U32) -> libc::c_uint {
    if 0 != !(val != 0i32 as libc::c_uint) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"BIT_highbit32\x00"))
                .as_ptr(),
            b"../lib/bitstream.h\x00" as *const u8 as *const libc::c_char,
            158i32,
            b"val != 0\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    return (31i32 - val.leading_zeros() as i32) as libc::c_uint;
}
/* Error Management */
/* *< tells if a return value is an error code */
#[no_mangle]
pub unsafe extern "C" fn HUF_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
/* *< provides error code string (useful for debugging) */
#[no_mangle]
pub unsafe extern "C" fn HUF_getErrorName(mut code: size_t) -> *const libc::c_char {
    return ERR_getErrorName(code);
}
/* ! HUF_readStats() :
 *  Read compact Huffman tree, saved by HUF_writeCTable().
 * `huffWeight` is destination buffer.
 * @return : size read from `src` , or an error Code .
 *  Note : Needed by HUF_readCTable() and HUF_readDTableXn() . */
#[no_mangle]
pub unsafe extern "C" fn HUF_readStats(
    mut huffWeight: *mut BYTE,
    mut hwSize: size_t,
    mut rankStats: *mut U32,
    mut nbSymbolsPtr: *mut U32,
    mut tableLogPtr: *mut U32,
    mut src: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut weightTotal: U32 = 0;
    let mut ip: *const BYTE = src as *const BYTE;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    if 0 == srcSize {
        return -(FSE_error_srcSize_wrong as libc::c_int) as size_t;
    }
    iSize = *ip.offset(0isize) as size_t;
    if iSize >= 128i32 as libc::c_ulong {
        oSize = iSize.wrapping_sub(127i32 as libc::c_ulong);
        iSize = oSize
            .wrapping_add(1i32 as libc::c_ulong)
            .wrapping_div(2i32 as libc::c_ulong);
        if iSize.wrapping_add(1i32 as libc::c_ulong) > srcSize {
            return -(FSE_error_srcSize_wrong as libc::c_int) as size_t;
        }
        if oSize >= hwSize {
            return -(FSE_error_corruption_detected as libc::c_int) as size_t;
        }
        ip = ip.offset(1isize);
        let mut n: U32 = 0;
        n = 0i32 as U32;
        while (n as libc::c_ulong) < oSize {
            *huffWeight.offset(n as isize) =
                (*ip.offset(n.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int >> 4i32)
                    as BYTE;
            *huffWeight.offset(n.wrapping_add(1i32 as libc::c_uint) as isize) =
                (*ip.offset(n.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int & 15i32)
                    as BYTE;
            n = (n as libc::c_uint).wrapping_add(2i32 as libc::c_uint) as U32 as U32
        }
    } else {
        let mut fseWorkspace: [FSE_DTable; 65] = [0; 65];
        if iSize.wrapping_add(1i32 as libc::c_ulong) > srcSize {
            return -(FSE_error_srcSize_wrong as libc::c_int) as size_t;
        }
        oSize = FSE_decompress_wksp(
            huffWeight as *mut libc::c_void,
            hwSize.wrapping_sub(1i32 as libc::c_ulong),
            ip.offset(1isize) as *const libc::c_void,
            iSize,
            fseWorkspace.as_mut_ptr(),
            6i32 as libc::c_uint,
        );
        if 0 != FSE_isError(oSize) {
            return oSize;
        }
    }
    memset(
        rankStats as *mut libc::c_void,
        0i32,
        ((12i32 + 1i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<U32>() as libc::c_ulong),
    );
    weightTotal = 0i32 as U32;
    let mut n_0: U32 = 0;
    n_0 = 0i32 as U32;
    while (n_0 as libc::c_ulong) < oSize {
        if *huffWeight.offset(n_0 as isize) as libc::c_int >= 12i32 {
            return -(FSE_error_corruption_detected as libc::c_int) as size_t;
        }
        let ref mut fresh2 = *rankStats.offset(*huffWeight.offset(n_0 as isize) as isize);
        *fresh2 = (*fresh2).wrapping_add(1);
        weightTotal = (weightTotal as libc::c_uint).wrapping_add(
            (1i32 << *huffWeight.offset(n_0 as isize) as libc::c_int >> 1i32) as libc::c_uint,
        ) as U32 as U32;
        n_0 = n_0.wrapping_add(1)
    }
    if weightTotal == 0i32 as libc::c_uint {
        return -(FSE_error_corruption_detected as libc::c_int) as size_t;
    }
    let tableLog: U32 = BIT_highbit32(weightTotal).wrapping_add(1i32 as libc::c_uint);
    if tableLog > 12i32 as libc::c_uint {
        return -(FSE_error_corruption_detected as libc::c_int) as size_t;
    }
    *tableLogPtr = tableLog;
    let total: U32 = (1i32 << tableLog) as U32;
    let rest: U32 = total.wrapping_sub(weightTotal);
    let verif: U32 = (1i32 << BIT_highbit32(rest)) as U32;
    let lastWeight: U32 = BIT_highbit32(rest).wrapping_add(1i32 as libc::c_uint);
    if verif != rest {
        return -(FSE_error_corruption_detected as libc::c_int) as size_t;
    }
    *huffWeight.offset(oSize as isize) = lastWeight as BYTE;
    let ref mut fresh3 = *rankStats.offset(lastWeight as isize);
    *fresh3 = (*fresh3).wrapping_add(1);
    if *rankStats.offset(1isize) < 2i32 as libc::c_uint
        || 0 != *rankStats.offset(1isize) & 1i32 as libc::c_uint
    {
        return -(FSE_error_corruption_detected as libc::c_int) as size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1i32 as libc::c_ulong) as U32;
    return iSize.wrapping_add(1i32 as libc::c_ulong);
}
