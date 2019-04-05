use libc;
#[header_src = "/usr/local/Cellar/llvm/7.0.1/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type ptrdiff_t = libc::c_long;
    pub type size_t = libc::c_ulong;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/mem.h"]
pub mod mem_h {
    /*-**************************************************************
     *  Basic Types
     *****************************************************************/
    /* C99 */
    pub type BYTE = uint8_t;
    pub type U16 = uint16_t;
    pub type U64 = uint64_t;
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    pub struct unalign64 {
        pub v: U64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union unnamed {
        pub u: U32,
        pub c: [BYTE; 4],
    }
    pub type U32 = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    pub struct unalign32 {
        pub v: U32,
    }
    /* __pack instructions are safer, but compiler specific, hence potentially problematic for some compilers */
    /* currently only defined for gcc and icc */
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    pub struct unalign16 {
        /* ! Constructor and Destructor of FSE_CTable.
        Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
        /* don't allocate that. It's only meant to be more restrictive than void* */
        /* *****************************************
         *  FSE symbol compression API
         *******************************************/
        /*
           This API consists of small unitary functions, which highly benefit from being inlined.
           Hence their body are included in next section.
        */
        /* faster, but works only if nbBits is always >= 1 (otherwise, result will be corrupted) */
        /* *****************************************
         *  Implementation of inlined functions
         *******************************************/
        /* ! Constructor and Destructor of FSE_DTable.
        Note that its size depends on 'tableLog' */
        /* don't allocate that. It's just a way to be more restrictive than void* */
        pub v: U16,
    }
    pub type S16 = int16_t;
    use super::_int16_t_h::int16_t;
    use super::_uint16_t_h::uint16_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
}
#[header_src = "/usr/include/_types/_uint8_t.h"]
pub mod _uint8_t_h {
    pub type uint8_t = libc::c_uchar;
}
#[header_src = "/usr/include/_types/_uint16_t.h"]
pub mod _uint16_t_h {
    pub type uint16_t = libc::c_ushort;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/fse.h"]
pub mod fse_h {
    pub type FSE_CTable = libc::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct FSE_CState_t {
        pub value: ptrdiff_t,
        pub stateTable: *const libc::c_void,
        pub symbolTT: *const libc::c_void,
        pub stateLog: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct FSE_symbolCompressionTransform {
        pub deltaFindState: libc::c_int,
        pub deltaNbBits: U32,
    }
    pub type FSE_DTable = libc::c_uint;
    /* *<
    These functions are inner components of FSE_compress_usingCTable().
    They allow the creation of custom streams, mixing multiple tables and bit sources.

    A key property to keep in mind is that encoding and decoding are done **in reverse direction**.
    So the first symbol you will encode is the last you will decode, like a LIFO stack.

    You will need a few variables to track your CStream. They are :

    FSE_CTable    ct;         // Provided by FSE_buildCTable()
    BIT_CStream_t bitStream;  // bitStream tracking structure
    FSE_CState_t  state;      // State tracking structure (can have several)


    The first thing to do is to init bitStream and state.
        size_t errorCode = BIT_initCStream(&bitStream, dstBuffer, maxDstSize);
        FSE_initCState(&state, ct);

    Note that BIT_initCStream() can produce an error code, so its result should be tested, using FSE_isError();
    You can then encode your input data, byte after byte.
    FSE_encodeSymbol() outputs a maximum of 'tableLog' bits at a time.
    Remember decoding will be done in reverse direction.
        FSE_encodeByte(&bitStream, &state, symbol);

    At any time, you can also add any bit sequence.
    Note : maximum allowed nbBits is 25, for compatibility with 32-bits decoders
        BIT_addBits(&bitStream, bitField, nbBits);

    The above methods don't commit data to memory, they just store it into local register, for speed.
    Local register size is 64-bits on 64-bits systems, 32-bits on 32-bits systems (size_t).
    Writing data to memory is a manual operation, performed by the flushBits function.
        BIT_flushBits(&bitStream);

    Your last FSE encoding operation shall be to flush your last state value(s).
        FSE_flushState(&bitStream, &state);

    Finally, you must close the bitStream.
    The function returns the size of CStream in bytes.
    If data couldn't fit into dstBuffer, it will return a 0 ( == not compressible)
    If there is an error, it returns an errorCode (which can be tested using FSE_isError()).
        size_t size = BIT_closeCStream(&bitStream);
    */
    /* *****************************************
     *  FSE symbol decompression API
     *******************************************/
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct FSE_DState_t {
        /* ======    Decompression    ====== */
        pub state: size_t,
        pub table: *const libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct FSE_DTableHeader {
        /* ! FSE_writeNCount():
        Compactly save 'normalizedCounter' into 'buffer'.
        @return : size of the compressed table,
                  or an errorCode, which can be tested using FSE_isError(). */
        /* ! FSE_normalizeCount():
        normalize counts so that sum(count[]) == Power_of_2 (2^tableLog)
        'normalizedCounter' is a table of short, of minimum size (maxSymbolValue+1).
        @return : tableLog,
                  or an errorCode, which can be tested using FSE_isError() */
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
        pub tableLog: U16,
        pub fastMode: U16,
    }
    use super::mem_h::{U16, U32};
    use super::stddef_h::{ptrdiff_t, size_t};
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/bitstream.h"]
pub mod bitstream_h {
    /* ******************************************************************
       bitstream
       Part of FSE library
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
    /*
     *  This API consists of small unitary functions, which must be inlined for best performance.
     *  Since link-time-optimization is not available for all compilers,
     *  these functions are defined into a .h to be included.
     */
    /*-****************************************
     *  Dependencies
     ******************************************/
    /* unaligned access routines */
    /* assert(), DEBUGLOG(), RAWLOG() */
    /*=========================================
    *  Target specific
    =========================================*/
    /*-******************************************
     *  bitStream encoding API (write forward)
     ********************************************/
    /* bitStream can mix input from multiple sources.
     * A critical property of these streams is that they encode and decode in **reverse** direction.
     * So the first bit sequence you add will be the last to be read, like a LIFO stack.
     */
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct BIT_CStream_t {
        pub bitContainer: size_t,
        pub bitPos: libc::c_uint,
        pub startPtr: *mut libc::c_char,
        pub ptr: *mut libc::c_char,
        pub endPtr: *mut libc::c_char,
    }
    pub type unnamed_0 = libc::c_uint;
    pub const MEM_static_assert: unnamed_0 = 1;
    /* Start with initCStream, providing the size of buffer to write into.
     *  bitStream will never write outside of this buffer.
     *  `dstCapacity` must be >= sizeof(bitD->bitContainer), otherwise @return will be an error code.
     *
     *  bits are first added to a local register.
     *  Local register is size_t, hence 64-bits on 64-bits systems, or 32-bits on 32-bits systems.
     *  Writing data into memory is an explicit operation, performed by the flushBits function.
     *  Hence keep track how many bits are potentially stored into local register to avoid register overflow.
     *  After a flushBits, a maximum of 7 bits might still be stored into local register.
     *
     *  Avoid storing elements of more than 24 bits if you want compatibility with 32-bits bitstream readers.
     *
     *  Last operation is to close the bitStream.
     *  The function returns the final size of CStream in bytes.
     *  If data couldn't fit into `dstBuffer`, it will return a 0 ( == not storable)
     */
    /*-********************************************
     *  bitStream decoding API (read backward)
     **********************************************/
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct BIT_DStream_t {
        pub bitContainer: size_t,
        pub bitsConsumed: libc::c_uint,
        pub ptr: *const libc::c_char,
        pub start: *const libc::c_char,
        pub limitPtr: *const libc::c_char,
    }
    pub const BIT_DStream_completed: BIT_DStream_status = 2;
    pub type BIT_DStream_status = libc::c_uint;
    pub const BIT_DStream_overflow: BIT_DStream_status = 3;
    pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
    pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
    use super::stddef_h::size_t;
}
#[header_src = "/usr/include/_types/_uint64_t.h"]
pub mod _uint64_t_h {
    pub type uint64_t = libc::c_ulonglong;
}
#[header_src = "/usr/include/_types/_uint32_t.h"]
pub mod _uint32_t_h {
    pub type uint32_t = libc::c_uint;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/error_public.h"]
pub mod error_public_h {
    pub const FSE_error_dstSize_tooSmall: unnamed_1 = 2;
    pub const FSE_error_GENERIC: unnamed_1 = 1;
    pub const FSE_error_tableLog_tooLarge: unnamed_1 = 5;
    pub const FSE_error_maxCode: unnamed_1 = 9;
    pub const FSE_error_maxSymbolValue_tooSmall: unnamed_1 = 7;
    pub const FSE_error_maxSymbolValue_tooLarge: unnamed_1 = 6;
    pub const FSE_error_corruption_detected: unnamed_1 = 4;
    pub const FSE_error_srcSize_wrong: unnamed_1 = 3;
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
    pub type unnamed_1 = libc::c_uint;
    pub const FSE_error_workSpace_tooSmall: unnamed_1 = 8;
    pub const FSE_error_no_error: unnamed_1 = 0;
}
#[header_src = "/usr/include/sys/_types/_int16_t.h"]
pub mod _int16_t_h {
    pub type int16_t = libc::c_short;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/fseU16.c"]
pub mod fseU16_c {
    pub type DTable_max_t = [FSE_DTable; 8193];
    /* ******************************************************************
       FSEU16 : Finite State Entropy coder for 16-bits input
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
       - Public forum : https://groups.google.com/forum/#!forum/lz4c
    ****************************************************************** */
    /* *************************************************************
     *  Tuning parameters
     *****************************************************************/
    /* MEMORY_USAGE :
     *  Memory usage formula : N->2^N Bytes (examples : 10 -> 1KB; 12 -> 4KB ; 16 -> 64KB; 20 -> 1MB; etc.)
     *  Increasing memory usage improves compression ratio
     *  Reduced memory usage can improve speed, due to cache effect
     *  Recommended max value is 14, for 16KB, which nicely fits into Intel x86 L1 cache */
    /* **************************************************************
     *  Includes
     *****************************************************************/
    /* **************************************************************
     *  Compiler specifics
     *****************************************************************/
    /* Visual Studio */
    /* **************************************************************
     *  Local type
     ****************************************************************/
    #[derive(BitfieldStruct, Clone, Copy)]
    #[repr(C)]
    pub struct FSE_decode_tU16 {
        pub newState: libc::c_ushort,
        #[bitfield(name = "nbBits", ty = "libc::c_uint", bits = "0..=3")]
        #[bitfield(name = "symbol", ty = "libc::c_uint", bits = "4..=15")]
        pub nbBits_symbol: [u8; 2],
    }
    use super::bitstream_h::BIT_DStream_t;
    use super::fse_h::{FSE_CTable, FSE_DState_t, FSE_DTable};
    use super::mem_h::U16;
    use super::stddef_h::size_t;
}
#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/fseU16.h"]
pub mod fseU16_h {
    use super::stddef_h::size_t;
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
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[header_src = "/usr/include/malloc/_malloc.h"]
pub mod _malloc_h {
    extern "C" {
        #[no_mangle]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn free(_: *mut libc::c_void);
    }
}
use self::_int16_t_h::int16_t;
use self::_malloc_h::{free, malloc};
use self::_uint16_t_h::uint16_t;
use self::_uint32_t_h::uint32_t;
use self::_uint64_t_h::uint64_t;
use self::_uint8_t_h::uint8_t;
use self::assert_h::__assert_rtn;
use self::bitstream_h::{
    unnamed_0, BIT_CStream_t, BIT_DStream_completed, BIT_DStream_endOfBuffer, BIT_DStream_overflow,
    BIT_DStream_status, BIT_DStream_t, BIT_DStream_unfinished, MEM_static_assert,
};
use self::error_public_h::{
    unnamed_1, FSE_error_GENERIC, FSE_error_corruption_detected, FSE_error_dstSize_tooSmall,
    FSE_error_maxCode, FSE_error_maxSymbolValue_tooLarge, FSE_error_maxSymbolValue_tooSmall,
    FSE_error_no_error, FSE_error_srcSize_wrong, FSE_error_tableLog_tooLarge,
    FSE_error_workSpace_tooSmall,
};
use self::fseU16_c::{DTable_max_t, FSE_decode_tU16};
use self::fse_h::{
    FSE_CState_t, FSE_CTable, FSE_DState_t, FSE_DTable, FSE_DTableHeader,
    FSE_symbolCompressionTransform,
};
use self::mem_h::{unalign16, unalign32, unalign64, unnamed, BYTE, S16, U16, U32, U64};
use self::stddef_h::{ptrdiff_t, size_t};
use self::string_h::{memcpy, memset};
use super::entropy_common::FSE_readNCount;
use super::fse_compress::{FSE_normalizeCount, FSE_optimalTableLog, FSE_writeNCount};
use c2rust_bitfields::BitfieldStruct;
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
pub unsafe extern "C" fn FSE_compressU16(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const libc::c_ushort,
    mut srcSize: size_t,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
) -> size_t {
    let istart: *const U16 = src;
    let mut ip: *const U16 = istart;
    let ostart: *mut BYTE = dst as *mut BYTE;
    let omax: *mut BYTE = ostart.offset(maxDstSize as isize);
    let mut op: *mut BYTE = ostart;
    let mut counting: [U32; 287] = [
        0i32 as U32,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut norm: [S16; 287] = [0; 287];
    if srcSize <= 1i32 as libc::c_ulong {
        return srcSize;
    }
    if 0 == maxSymbolValue {
        maxSymbolValue = 286i32 as libc::c_uint
    }
    if 0 == tableLog {
        tableLog = (14i32 - 2i32) as libc::c_uint
    }
    if maxSymbolValue > 286i32 as libc::c_uint {
        return -(FSE_error_maxSymbolValue_tooLarge as libc::c_int) as size_t;
    }
    if tableLog > (15i32 - 2i32) as libc::c_uint {
        return -(FSE_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    let maxCount: size_t = FSE_countU16(counting.as_mut_ptr(), &mut maxSymbolValue, ip, srcSize);
    if 0 != ERR_isError(maxCount) {
        return maxCount;
    }
    if maxCount == srcSize {
        return 1i32 as size_t;
    }
    tableLog = FSE_optimalTableLog(tableLog, srcSize, maxSymbolValue);
    let errorCode: size_t = FSE_normalizeCount(
        norm.as_mut_ptr(),
        tableLog,
        counting.as_mut_ptr(),
        srcSize,
        maxSymbolValue,
    );
    if 0 != ERR_isError(errorCode) {
        return errorCode;
    }
    let NSize: size_t = FSE_writeNCount(
        op as *mut libc::c_void,
        omax.wrapping_offset_from(op) as libc::c_long as size_t,
        norm.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if 0 != ERR_isError(NSize) {
        return NSize;
    }
    op = op.offset(NSize as isize);
    let mut CTable: [FSE_CTable; 4671] = [0; 4671];
    let errorCode_0: size_t = FSE_buildCTableU16(
        CTable.as_mut_ptr(),
        norm.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if 0 != ERR_isError(errorCode_0) {
        return errorCode_0;
    }
    op = op.offset(FSE_compressU16_usingCTable(
        op as *mut libc::c_void,
        omax.wrapping_offset_from(op) as libc::c_long as size_t,
        ip,
        srcSize,
        CTable.as_mut_ptr(),
    ) as isize);
    if op.wrapping_offset_from(ostart) as libc::c_long as size_t
        >= srcSize
            .wrapping_sub(1i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<U16>() as libc::c_ulong)
    {
        return 0i32 as size_t;
    }
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
}
/* *******************************************************
*  U16 Compression functions
*********************************************************/
#[no_mangle]
pub unsafe extern "C" fn FSE_compressU16_usingCTable(
    mut dst: *mut libc::c_void,
    mut maxDstSize: size_t,
    mut src: *const U16,
    mut srcSize: size_t,
    mut ct: *const FSE_CTable,
) -> size_t {
    let istart: *const U16 = src;
    let iend: *const U16 = istart.offset(srcSize as isize);
    let mut ip: *const U16 = 0 as *const U16;
    let mut op: *mut BYTE = dst as *mut BYTE;
    let mut bitC: BIT_CStream_t = BIT_CStream_t {
        bitContainer: 0,
        bitPos: 0,
        startPtr: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        endPtr: 0 as *mut libc::c_char,
    };
    let mut CState: FSE_CState_t = FSE_CState_t {
        value: 0,
        stateTable: 0 as *const libc::c_void,
        symbolTT: 0 as *const libc::c_void,
        stateLog: 0,
    };
    BIT_initCStream(&mut bitC, op as *mut libc::c_void, maxDstSize);
    FSE_initCState(&mut CState, ct);
    ip = iend;
    if 0 != srcSize & 1i32 as libc::c_ulong {
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState, *ip as libc::c_uint);
        BIT_flushBits(&mut bitC);
    }
    if 0 != srcSize & 2i32 as libc::c_ulong {
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState, *ip as libc::c_uint);
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState, *ip as libc::c_uint);
        BIT_flushBits(&mut bitC);
    }
    while ip > istart {
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState, *ip as libc::c_uint);
        if (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong)
            < ((15i32 - 2i32) * 2i32 + 7i32) as libc::c_ulong
        {
            BIT_flushBits(&mut bitC);
        }
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState, *ip as libc::c_uint);
        if (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong)
            > ((15i32 - 2i32) * 4i32 + 7i32) as libc::c_ulong
        {
            ip = ip.offset(-1isize);
            FSE_encodeSymbol(&mut bitC, &mut CState, *ip as libc::c_uint);
            ip = ip.offset(-1isize);
            FSE_encodeSymbol(&mut bitC, &mut CState, *ip as libc::c_uint);
        }
        BIT_flushBits(&mut bitC);
    }
    FSE_flushCState(&mut bitC, &mut CState);
    return BIT_closeCStream(&mut bitC);
}
unsafe extern "C" fn BIT_closeCStream(mut bitC: *mut BIT_CStream_t) -> size_t {
    BIT_addBitsFast(bitC, 1i32 as size_t, 1i32 as libc::c_uint);
    BIT_flushBits(bitC);
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0i32 as size_t;
    }
    return ((*bitC).ptr.wrapping_offset_from((*bitC).startPtr) as libc::c_long
        + ((*bitC).bitPos > 0i32 as libc::c_uint) as libc::c_int as libc::c_long)
        as size_t;
}
unsafe extern "C" fn BIT_flushBits(mut bitC: *mut BIT_CStream_t) {
    let nbBytes: size_t = ((*bitC).bitPos >> 3i32) as size_t;
    if 0 != !(((*bitC).bitPos as libc::c_ulong)
        < (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong))
        as libc::c_int as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"BIT_flushBits\x00"))
                .as_ptr(),
            b"../lib/bitstream.h\x00" as *const u8 as *const libc::c_char,
            258i32,
            b"bitC->bitPos < sizeof(bitC->bitContainer) * 8\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
    };
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, (*bitC).bitContainer);
    (*bitC).ptr = (*bitC).ptr.offset(nbBytes as isize);
    if (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr
    }
    (*bitC).bitPos &= 7i32 as libc::c_uint;
    (*bitC).bitContainer >>= nbBytes.wrapping_mul(8i32 as libc::c_ulong);
}
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut libc::c_void, mut val: size_t) {
    if 0 != MEM_32bits() {
        MEM_writeLE32(memPtr, val as U32);
    } else {
        MEM_writeLE64(memPtr, val as U64);
    };
}
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut libc::c_void, mut val64: U64) {
    if 0 != MEM_isLittleEndian() {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return in_0 << 56i32 & 0xff00000000000000u64
        | in_0 << 40i32 & 0xff000000000000u64
        | in_0 << 24i32 & 0xff0000000000u64
        | in_0 << 8i32 & 0xff00000000u64
        | in_0 >> 8i32 & 0xff000000u64
        | in_0 >> 24i32 & 0xff0000u64
        | in_0 >> 40i32 & 0xff00u64
        | in_0 >> 56i32 & 0xffu64;
}
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void, mut value: U64) {
    (*(memPtr as *mut unalign64)).v = value;
}
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    /* don't use static : performance detrimental  */
    let one: unnamed = unnamed { u: 1i32 as U32 };
    return one.c[0usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void, mut val32: U32) {
    if 0 != MEM_isLittleEndian() {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0 << 24i32 & 0xff000000u32
        | in_0 << 8i32 & 0xff0000i32 as libc::c_uint
        | in_0 >> 8i32 & 0xff00i32 as libc::c_uint
        | in_0 >> 24i32 & 0xffi32 as libc::c_uint;
}
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void, mut value: U32) {
    (*(memPtr as *mut unalign32)).v = value;
}
/*-**************************************************************
*  Memory I/O
*****************************************************************/
/* MEM_FORCE_MEMORY_ACCESS :
 * By default, access to unaligned memory is controlled by `memcpy()`, which is safe and portable.
 * Unfortunately, on some target/compiler combinations, the generated assembly is sub-optimal.
 * The below switch allow to select different access method for improved performance.
 * Method 0 (default) : use `memcpy()`. Safe and portable.
 * Method 1 : `__packed` statement. It depends on compiler extension (i.e., not portable).
 *            This method is safe if your compiler supports it, and *generally* as fast or faster than `memcpy`.
 * Method 2 : direct access. This method is portable but violate C standard.
 *            It can generate buggy code on targets depending on alignment.
 *            In some circumstances, it's the only known way to get the most performance (i.e. GCC + ARMv6)
 * See http://fastcompression.blogspot.fr/2015/08/accessing-unaligned-memory.html for details.
 * Prefer these methods in priority order (0 > 1 > 2)
 */
/* can be defined externally, on command line for example */
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::std::mem::size_of::<size_t>() as libc::c_ulong == 4i32 as libc::c_ulong)
        as libc::c_int as libc::c_uint;
}
/* Start by invoking BIT_initDStream().
*  A chunk of the bitStream is then stored into a local register.
*  Local register size is 64-bits on 64-bits systems, 32-bits on 32-bits systems (size_t).
*  You can then retrieve bitFields stored into the local register, **in reverse order**.
*  Local register is explicitly reloaded from memory by the BIT_reloadDStream() method.
*  A reload guarantee a minimum of ((8*sizeof(bitD->bitContainer))-7) bits when its result is BIT_DStream_unfinished.
*  Otherwise, it can be less than that, so proceed accordingly.
*  Checking if DStream has reached its end can be performed with BIT_endOfDStream().
*/
/*-****************************************
*  unsafe API
******************************************/
unsafe extern "C" fn BIT_addBitsFast(
    mut bitC: *mut BIT_CStream_t,
    mut value: size_t,
    mut nbBits: libc::c_uint,
) {
    if 0 != !(value >> nbBits == 0i32 as libc::c_ulong) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"BIT_addBitsFast\x00"))
                .as_ptr(),
            b"../lib/bitstream.h\x00" as *const u8 as *const libc::c_char,
            230i32,
            b"(value>>nbBits) == 0\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if 0 != !((nbBits.wrapping_add((*bitC).bitPos) as libc::c_ulong)
        < (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong))
        as libc::c_int as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"BIT_addBitsFast\x00"))
                .as_ptr(),
            b"../lib/bitstream.h\x00" as *const u8 as *const libc::c_char,
            231i32,
            b"nbBits + bitC->bitPos < sizeof(bitC->bitContainer) * 8\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
    };
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
}
unsafe extern "C" fn FSE_flushCState(
    mut bitC: *mut BIT_CStream_t,
    mut statePtr: *const FSE_CState_t,
) {
    BIT_addBits(bitC, (*statePtr).value as size_t, (*statePtr).stateLog);
    BIT_flushBits(bitC);
}
unsafe extern "C" fn BIT_addBits(
    mut bitC: *mut BIT_CStream_t,
    mut value: size_t,
    mut nbBits: libc::c_uint,
) {
    if 0 != !((nbBits as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_uint; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong))
        as libc::c_int as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"BIT_addBits\x00")).as_ptr(),
            b"../lib/bitstream.h\x00" as *const u8 as *const libc::c_char,
            218i32,
            b"nbBits < BIT_MASK_SIZE\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    if 0 != !((nbBits.wrapping_add((*bitC).bitPos) as libc::c_ulong)
        < (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong))
        as libc::c_int as libc::c_long
    {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"BIT_addBits\x00")).as_ptr(),
            b"../lib/bitstream.h\x00" as *const u8 as *const libc::c_char,
            219i32,
            b"nbBits + bitC->bitPos < sizeof(bitC->bitContainer) * 8\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
    };
    (*bitC).bitContainer |= (value & BIT_mask[nbBits as usize] as libc::c_ulong) << (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
}
/* Software version */
/*=====    Local Constants   =====*/
static mut BIT_mask: [libc::c_uint; 32] = [
    0i32 as libc::c_uint,
    1i32 as libc::c_uint,
    3i32 as libc::c_uint,
    7i32 as libc::c_uint,
    0xfi32 as libc::c_uint,
    0x1fi32 as libc::c_uint,
    0x3fi32 as libc::c_uint,
    0x7fi32 as libc::c_uint,
    0xffi32 as libc::c_uint,
    0x1ffi32 as libc::c_uint,
    0x3ffi32 as libc::c_uint,
    0x7ffi32 as libc::c_uint,
    0xfffi32 as libc::c_uint,
    0x1fffi32 as libc::c_uint,
    0x3fffi32 as libc::c_uint,
    0x7fffi32 as libc::c_uint,
    0xffffi32 as libc::c_uint,
    0x1ffffi32 as libc::c_uint,
    0x3ffffi32 as libc::c_uint,
    0x7ffffi32 as libc::c_uint,
    0xfffffi32 as libc::c_uint,
    0x1fffffi32 as libc::c_uint,
    0x3fffffi32 as libc::c_uint,
    0x7fffffi32 as libc::c_uint,
    0xffffffi32 as libc::c_uint,
    0x1ffffffi32 as libc::c_uint,
    0x3ffffffi32 as libc::c_uint,
    0x7ffffffi32 as libc::c_uint,
    0xfffffffi32 as libc::c_uint,
    0x1fffffffi32 as libc::c_uint,
    0x3fffffffi32 as libc::c_uint,
    0x7fffffffi32 as libc::c_uint,
];
unsafe extern "C" fn FSE_encodeSymbol(
    mut bitC: *mut BIT_CStream_t,
    mut statePtr: *mut FSE_CState_t,
    mut symbol: U32,
) {
    let symbolTT: FSE_symbolCompressionTransform =
        *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let stateTable: *const U16 = (*statePtr).stateTable as *const U16;
    let nbBitsOut: U32 = ((*statePtr).value + symbolTT.deltaNbBits as libc::c_long >> 16i32) as U32;
    BIT_addBits(bitC, (*statePtr).value as size_t, nbBitsOut);
    (*statePtr).value = *stateTable.offset(
        (((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as libc::c_long) as isize,
    ) as ptrdiff_t;
}
unsafe extern "C" fn FSE_initCState(mut statePtr: *mut FSE_CState_t, mut ct: *const FSE_CTable) {
    let mut ptr: *const libc::c_void = ct as *const libc::c_void;
    let mut u16ptr: *const U16 = ptr as *const U16;
    let tableLog: U32 = MEM_read16(ptr) as U32;
    (*statePtr).value = (1i32 as ptrdiff_t) << tableLog;
    (*statePtr).stateTable = u16ptr.offset(2isize) as *const libc::c_void;
    (*statePtr).symbolTT = ct.offset(1isize).offset(
        (if 0 != tableLog {
            1i32 << tableLog.wrapping_sub(1i32 as libc::c_uint)
        } else {
            1i32
        }) as isize,
    ) as *const libc::c_void;
    (*statePtr).stateLog = tableLog;
}
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> U16 {
    return (*(ptr as *const unalign16)).v;
}
unsafe extern "C" fn BIT_initCStream(
    mut bitC: *mut BIT_CStream_t,
    mut startPtr: *mut libc::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    (*bitC).bitContainer = 0i32 as size_t;
    (*bitC).bitPos = 0i32 as libc::c_uint;
    (*bitC).startPtr = startPtr as *mut libc::c_char;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr = (*bitC)
        .startPtr
        .offset(dstCapacity as isize)
        .offset(-(::std::mem::size_of::<size_t>() as libc::c_ulong as isize));
    if dstCapacity <= ::std::mem::size_of::<size_t>() as libc::c_ulong {
        return -(FSE_error_dstSize_tooSmall as libc::c_int) as size_t;
    }
    return 0i32 as size_t;
}
/* ! FSE_buildCTable():
Builds `ct`, which must be already allocated, using FSE_createCTable().
@return : 0, or an errorCode, which can be tested using FSE_isError() */
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTableU16(
    mut ct: *mut FSE_CTable,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
) -> size_t {
    /* memset() is not necessary, even if static analyzer complain about it */
    let mut tableSymbol: [U16; 8192] = [0; 8192];
    return FSE_buildCTable_wksp_U16(
        ct,
        normalizedCounter,
        maxSymbolValue,
        tableLog,
        tableSymbol.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[U16; 8192]>() as libc::c_ulong,
    );
}
/* *< build a fake FSE_CTable, designed to compress always the same symbolValue */
/* FSE_buildCTable_wksp() :
 * Same as FSE_buildCTable(), but using an externally allocated scratch buffer (`workSpace`).
 * `wkspSize` must be >= `(1<<tableLog)`.
 */
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTable_wksp_U16(
    mut ct: *mut FSE_CTable,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
    mut workSpace: *mut libc::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let tableSize: U32 = (1i32 << tableLog) as U32;
    let tableMask: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    let ptr: *mut libc::c_void = ct as *mut libc::c_void;
    let tableU16: *mut U16 = (ptr as *mut U16).offset(2isize);
    /* header */
    let FSCT: *mut libc::c_void = (ptr as *mut U32).offset(1isize).offset(
        (if 0 != tableLog {
            tableSize >> 1i32
        } else {
            1i32 as libc::c_uint
        }) as isize,
    ) as *mut libc::c_void;
    let symbolTT: *mut FSE_symbolCompressionTransform = FSCT as *mut FSE_symbolCompressionTransform;
    let step: U32 = (tableSize >> 1i32)
        .wrapping_add(tableSize >> 3i32)
        .wrapping_add(3i32 as libc::c_uint);
    let mut cumul: [U32; 288] = [0; 288];
    let tableSymbol: *mut U16 = workSpace as *mut U16;
    let mut highThreshold: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    if ((1i32 as size_t) << tableLog).wrapping_mul(::std::mem::size_of::<U16>() as libc::c_ulong)
        > wkspSize
    {
        return -(FSE_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    *tableU16.offset(-2i32 as isize) = tableLog as U16;
    *tableU16.offset(-1i32 as isize) = maxSymbolValue as U16;
    if 0 != !(tableLog < 16i32 as libc::c_uint) as libc::c_int as libc::c_long {
        __assert_rtn(
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"FSE_buildCTable_wksp_U16\x00",
            ))
            .as_ptr(),
            b"../lib/fse_compress.c\x00" as *const u8 as *const libc::c_char,
            104i32,
            b"tableLog < 16\x00" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    let mut u: U32 = 0;
    cumul[0usize] = 0i32 as U32;
    u = 1i32 as U32;
    while u <= maxSymbolValue.wrapping_add(1i32 as libc::c_uint) {
        if *normalizedCounter.offset(u.wrapping_sub(1i32 as libc::c_uint) as isize) as libc::c_int
            == -1i32
        {
            cumul[u as usize] = cumul[u.wrapping_sub(1i32 as libc::c_uint) as usize]
                .wrapping_add(1i32 as libc::c_uint);
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            *tableSymbol.offset(fresh0 as isize) = u.wrapping_sub(1i32 as libc::c_uint) as U16
        } else {
            cumul[u as usize] = cumul[u.wrapping_sub(1i32 as libc::c_uint) as usize].wrapping_add(
                *normalizedCounter.offset(u.wrapping_sub(1i32 as libc::c_uint) as isize)
                    as libc::c_uint,
            )
        }
        u = u.wrapping_add(1)
    }
    cumul[maxSymbolValue.wrapping_add(1i32 as libc::c_uint) as usize] =
        tableSize.wrapping_add(1i32 as libc::c_uint);
    let mut position: U32 = 0i32 as U32;
    let mut symbol: U32 = 0;
    symbol = 0i32 as U32;
    while symbol <= maxSymbolValue {
        let mut nbOccurences: libc::c_int = 0;
        nbOccurences = 0i32;
        while nbOccurences < *normalizedCounter.offset(symbol as isize) as libc::c_int {
            *tableSymbol.offset(position as isize) = symbol as U16;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask
            }
            nbOccurences += 1
        }
        symbol = symbol.wrapping_add(1)
    }
    if position != 0i32 as libc::c_uint {
        return -(FSE_error_GENERIC as libc::c_int) as size_t;
    }
    let mut u_0: U32 = 0;
    u_0 = 0i32 as U32;
    while u_0 < tableSize {
        let mut s: U16 = *tableSymbol.offset(u_0 as isize);
        let fresh1 = cumul[s as usize];
        cumul[s as usize] = cumul[s as usize].wrapping_add(1);
        *tableU16.offset(fresh1 as isize) = tableSize.wrapping_add(u_0) as U16;
        u_0 = u_0.wrapping_add(1)
    }
    let mut total: libc::c_uint = 0i32 as libc::c_uint;
    let mut s_0: libc::c_uint = 0;
    s_0 = 0i32 as libc::c_uint;
    while s_0 <= maxSymbolValue {
        match *normalizedCounter.offset(s_0 as isize) as libc::c_int {
            0 => {
                (*symbolTT.offset(s_0 as isize)).deltaNbBits =
                    (tableLog.wrapping_add(1i32 as libc::c_uint) << 16i32)
                        .wrapping_sub((1i32 << tableLog) as libc::c_uint)
            }
            -1 | 1 => {
                (*symbolTT.offset(s_0 as isize)).deltaNbBits =
                    (tableLog << 16i32).wrapping_sub((1i32 << tableLog) as libc::c_uint);
                (*symbolTT.offset(s_0 as isize)).deltaFindState =
                    total.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
                total = total.wrapping_add(1)
            }
            _ => {
                let maxBitsOut: U32 = tableLog.wrapping_sub(BIT_highbit32(
                    (*normalizedCounter.offset(s_0 as isize) as libc::c_int - 1i32) as U32,
                ));
                let minStatePlus: U32 =
                    ((*normalizedCounter.offset(s_0 as isize) as libc::c_int) << maxBitsOut) as U32;
                (*symbolTT.offset(s_0 as isize)).deltaNbBits =
                    (maxBitsOut << 16i32).wrapping_sub(minStatePlus);
                (*symbolTT.offset(s_0 as isize)).deltaFindState = total
                    .wrapping_sub(*normalizedCounter.offset(s_0 as isize) as libc::c_uint)
                    as libc::c_int;
                total = total.wrapping_add(*normalizedCounter.offset(s_0 as isize) as libc::c_uint)
            }
        }
        s_0 = s_0.wrapping_add(1)
    }
    return 0i32 as size_t;
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
/*-****************************************
*  Error codes handling
******************************************/
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(FSE_error_maxCode as libc::c_int) as size_t) as libc::c_int as libc::c_uint;
}
/* *******************************************************************
*  Include type-specific functions from fse.c (C template emulation)
*********************************************************************/
/* ! FSE_countU16() :
    This function counts U16 values stored in `src`,
    and push the histogram into `count`.
   @return : count of most common element
   *maxSymbolValuePtr : will be updated with value of highest symbol.
*/
#[no_mangle]
pub unsafe extern "C" fn FSE_countU16(
    mut count: *mut libc::c_uint,
    mut maxSymbolValuePtr: *mut libc::c_uint,
    mut src: *const U16,
    mut srcSize: size_t,
) -> size_t {
    let mut ip16: *const U16 = src;
    let end: *const U16 = src.offset(srcSize as isize);
    let mut maxSymbolValue: libc::c_uint = *maxSymbolValuePtr;
    memset(
        count as *mut libc::c_void,
        0i32,
        (maxSymbolValue.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    if srcSize == 0i32 as libc::c_ulong {
        *maxSymbolValuePtr = 0i32 as libc::c_uint;
        return 0i32 as size_t;
    }
    while ip16 < end {
        if *ip16 as libc::c_uint > maxSymbolValue {
            return -(FSE_error_maxSymbolValue_tooSmall as libc::c_int) as size_t;
        }
        let fresh2 = ip16;
        ip16 = ip16.offset(1);
        let ref mut fresh3 = *count.offset(*fresh2 as isize);
        *fresh3 = (*fresh3).wrapping_add(1)
    }
    while 0 == *count.offset(maxSymbolValue as isize) {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1)
    }
    *maxSymbolValuePtr = maxSymbolValue;
    let mut s: U32 = 0;
    let mut max: U32 = 0i32 as U32;
    s = 0i32 as U32;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) > max {
            max = *count.offset(s as isize)
        }
        s = s.wrapping_add(1)
    }
    return max as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_decompressU16(
    mut dst: *mut U16,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let istart: *const BYTE = cSrc as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut NCount: [libc::c_short; 287] = [0; 287];
    let mut dt: DTable_max_t = [0; 8193];
    let mut maxSymbolValue: libc::c_uint = 286i32 as libc::c_uint;
    let mut tableLog: libc::c_uint = 0;
    if cSrcSize < 2i32 as libc::c_ulong {
        return -(FSE_error_srcSize_wrong as libc::c_int) as size_t;
    }
    let NSize: size_t = FSE_readNCount(
        NCount.as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const libc::c_void,
        cSrcSize,
    );
    if 0 != ERR_isError(NSize) {
        return NSize;
    }
    ip = ip.offset(NSize as isize);
    cSrcSize = (cSrcSize as libc::c_ulong).wrapping_sub(NSize) as size_t as size_t;
    let errorCode: size_t = FSE_buildDTableU16(
        dt.as_mut_ptr(),
        NCount.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if 0 != ERR_isError(errorCode) {
        return errorCode;
    }
    return FSE_decompressU16_usingDTable(
        dst,
        maxDstSize,
        ip as *const libc::c_void,
        cSrcSize,
        dt.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn FSE_decompressU16_usingDTable(
    mut dst: *mut U16,
    mut maxDstSize: size_t,
    mut cSrc: *const libc::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSE_DTable,
) -> size_t {
    let ostart: *mut U16 = dst;
    let mut op: *mut U16 = ostart;
    let oend: *mut U16 = ostart.offset(maxDstSize as isize);
    let mut bitD: BIT_DStream_t = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: 0 as *const libc::c_char,
        start: 0 as *const libc::c_char,
        limitPtr: 0 as *const libc::c_char,
    };
    let mut state: FSE_DState_t = FSE_DState_t {
        state: 0,
        table: 0 as *const libc::c_void,
    };
    memset(
        &mut bitD as *mut BIT_DStream_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<BIT_DStream_t>() as libc::c_ulong,
    );
    BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    FSE_initDState(&mut state, &mut bitD, dt);
    while (BIT_reloadDStream(&mut bitD) as libc::c_uint)
        < BIT_DStream_completed as libc::c_int as libc::c_uint
        && op < oend
    {
        let fresh4 = op;
        op = op.offset(1);
        *fresh4 = FSE_decodeSymbolU16(&mut state, &mut bitD)
    }
    if 0 == BIT_endOfDStream(&mut bitD) {
        return -(FSE_error_corruption_detected as libc::c_int) as size_t;
    }
    while 0 != state.state && op < oend {
        let fresh5 = op;
        op = op.offset(1);
        *fresh5 = FSE_decodeSymbolU16(&mut state, &mut bitD)
    }
    if 0 != state.state {
        return -(FSE_error_corruption_detected as libc::c_int) as size_t;
    }
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
}
/* *******************************************************
*  U16 Decompression functions
*********************************************************/
#[no_mangle]
pub unsafe extern "C" fn FSE_decodeSymbolU16(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> U16 {
    let DInfo: FSE_decode_tU16 =
        *((*DStatePtr).table as *const FSE_decode_tU16).offset((*DStatePtr).state as isize);
    let mut symbol: U16 = 0;
    let mut lowBits: size_t = 0;
    let nbBits: U32 = DInfo.nbBits();
    symbol = DInfo.symbol() as U16;
    lowBits = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn BIT_readBits(mut bitD: *mut BIT_DStream_t, mut nbBits: U32) -> size_t {
    let value: size_t = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
/* ! BIT_lookBits() :
 *  Provides next n bits from local register.
 *  local register is not modified.
 *  On 32-bits, maxNbBits==24.
 *  On 64-bits, maxNbBits==56.
 * @return : value extracted */
unsafe extern "C" fn BIT_lookBits(mut bitD: *const BIT_DStream_t, mut nbBits: U32) -> size_t {
    /* experimental; fails if bitD->bitsConsumed + nbBits > sizeof(bitD->bitContainer)*8 */
    let regMask: U32 = (::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8i32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as U32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)
        >> 1i32
        >> (regMask.wrapping_sub(nbBits) & regMask);
}
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: U32) {
    (*bitD).bitsConsumed = (*bitD).bitsConsumed.wrapping_add(nbBits);
}
unsafe extern "C" fn BIT_endOfDStream(mut DStream: *const BIT_DStream_t) -> libc::c_uint {
    return ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as libc::c_ulong
            == (::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8i32 as libc::c_ulong)) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn BIT_reloadDStream(mut bitD: *mut BIT_DStream_t) -> BIT_DStream_status {
    if (*bitD).bitsConsumed as libc::c_ulong
        > (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong)
    {
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        (*bitD).ptr = (*bitD)
            .ptr
            .offset(-(((*bitD).bitsConsumed >> 3i32) as isize));
        (*bitD).bitsConsumed &= 7i32 as libc::c_uint;
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
        return BIT_DStream_unfinished;
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as libc::c_ulong)
            < (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong)
        {
            return BIT_DStream_endOfBuffer;
        }
        return BIT_DStream_completed;
    }
    let mut nbBytes: U32 = (*bitD).bitsConsumed >> 3i32;
    let mut result: BIT_DStream_status = BIT_DStream_unfinished;
    if (*bitD).ptr.offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = (*bitD).ptr.wrapping_offset_from((*bitD).start) as libc::c_long as U32;
        result = BIT_DStream_endOfBuffer
    }
    (*bitD).ptr = (*bitD).ptr.offset(-(nbBytes as isize));
    (*bitD).bitsConsumed = (*bitD)
        .bitsConsumed
        .wrapping_sub(nbBytes.wrapping_mul(8i32 as libc::c_uint));
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
    return result;
}
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> size_t {
    if 0 != MEM_32bits() {
        return MEM_readLE32(memPtr) as size_t;
    } else {
        return MEM_readLE64(memPtr) as size_t;
    };
}
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read64(memPtr);
    } else {
        return MEM_swap64(MEM_read64(memPtr));
    };
}
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> U64 {
    return (*(ptr as *const unalign64)).v;
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read32(memPtr);
    } else {
        return MEM_swap32(MEM_read32(memPtr));
    };
}
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign32)).v;
}
unsafe extern "C" fn FSE_initDState(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
    mut dt: *const FSE_DTable,
) {
    let mut ptr: *const libc::c_void = dt as *const libc::c_void;
    let DTableH: *const FSE_DTableHeader = ptr as *const FSE_DTableHeader;
    (*DStatePtr).state = BIT_readBits(bitD, (*DTableH).tableLog as libc::c_uint);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1isize) as *const libc::c_void;
}
/* 1,2,4,8 would be better for bitmap combinations, but slows down performance a bit ... :( */
unsafe extern "C" fn BIT_initDStream(
    mut bitD: *mut BIT_DStream_t,
    mut srcBuffer: *const libc::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1i32 as libc::c_ulong {
        memset(
            bitD as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<BIT_DStream_t>() as libc::c_ulong,
        );
        return -(FSE_error_srcSize_wrong as libc::c_int) as size_t;
    }
    (*bitD).start = srcBuffer as *const libc::c_char;
    (*bitD).limitPtr = (*bitD)
        .start
        .offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize);
    if srcSize >= ::std::mem::size_of::<size_t>() as libc::c_ulong {
        (*bitD).ptr = (srcBuffer as *const libc::c_char)
            .offset(srcSize as isize)
            .offset(-(::std::mem::size_of::<size_t>() as libc::c_ulong as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
        let lastByte: BYTE = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1i32 as libc::c_ulong) as isize);
        (*bitD).bitsConsumed = if 0 != lastByte as libc::c_int {
            (8i32 as libc::c_uint).wrapping_sub(BIT_highbit32(lastByte as U32))
        } else {
            0i32 as libc::c_uint
        };
        if lastByte as libc::c_int == 0i32 {
            return -(FSE_error_GENERIC as libc::c_int) as size_t;
        }
    } else {
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const BYTE) as size_t;
        let mut current_block_20: u64;
        match srcSize {
            7 => {
                (*bitD).bitContainer = ((*bitD).bitContainer as libc::c_ulong).wrapping_add(
                    (*(srcBuffer as *const BYTE).offset(6isize) as size_t)
                        << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_mul(8i32 as libc::c_ulong)
                            .wrapping_sub(16i32 as libc::c_ulong),
                ) as size_t as size_t;
                /* fall-through */
                current_block_20 = 13578171414943539863;
            }
            6 => {
                current_block_20 = 13578171414943539863;
            }
            5 => {
                current_block_20 = 9718663512059496072;
            }
            4 => {
                current_block_20 = 13850764817919632987;
            }
            3 => {
                current_block_20 = 5572819222429982616;
            }
            2 => {
                current_block_20 = 5413951294637847147;
            }
            _ => {
                current_block_20 = 14576567515993809846;
            }
        }
        match current_block_20 {
            13578171414943539863 => {
                (*bitD).bitContainer = ((*bitD).bitContainer as libc::c_ulong).wrapping_add(
                    (*(srcBuffer as *const BYTE).offset(5isize) as size_t)
                        << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_mul(8i32 as libc::c_ulong)
                            .wrapping_sub(24i32 as libc::c_ulong),
                ) as size_t as size_t;
                /* fall-through */
                current_block_20 = 9718663512059496072;
            }
            _ => {}
        }
        match current_block_20 {
            9718663512059496072 => {
                (*bitD).bitContainer = ((*bitD).bitContainer as libc::c_ulong).wrapping_add(
                    (*(srcBuffer as *const BYTE).offset(4isize) as size_t)
                        << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_mul(8i32 as libc::c_ulong)
                            .wrapping_sub(32i32 as libc::c_ulong),
                ) as size_t as size_t;
                /* fall-through */
                current_block_20 = 13850764817919632987;
            }
            _ => {}
        }
        match current_block_20 {
            13850764817919632987 => {
                (*bitD).bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add((*(srcBuffer as *const BYTE).offset(3isize) as size_t) << 24i32)
                    as size_t as size_t;
                /* fall-through */
                current_block_20 = 5572819222429982616;
            }
            _ => {}
        }
        match current_block_20 {
            5572819222429982616 => {
                (*bitD).bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add((*(srcBuffer as *const BYTE).offset(2isize) as size_t) << 16i32)
                    as size_t as size_t;
                /* fall-through */
                current_block_20 = 5413951294637847147;
            }
            _ => {}
        }
        match current_block_20 {
            5413951294637847147 => {
                (*bitD).bitContainer = ((*bitD).bitContainer as libc::c_ulong)
                    .wrapping_add((*(srcBuffer as *const BYTE).offset(1isize) as size_t) << 8i32)
                    as size_t as size_t
            }
            _ => {}
        }
        let lastByte_0: BYTE = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1i32 as libc::c_ulong) as isize);
        (*bitD).bitsConsumed = if 0 != lastByte_0 as libc::c_int {
            (8i32 as libc::c_uint).wrapping_sub(BIT_highbit32(lastByte_0 as U32))
        } else {
            0i32 as libc::c_uint
        };
        if lastByte_0 as libc::c_int == 0i32 {
            return -(FSE_error_corruption_detected as libc::c_int) as size_t;
        }
        (*bitD).bitsConsumed = (*bitD).bitsConsumed.wrapping_add(
            ((::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_sub(srcSize) as U32)
                .wrapping_mul(8i32 as libc::c_uint),
        )
    }
    return srcSize;
}
/* ! FSE_buildDTable():
Builds 'dt', which must be already allocated, using FSE_createDTable().
return : 0, or an errorCode, which can be tested using FSE_isError() */
#[no_mangle]
pub unsafe extern "C" fn FSE_buildDTableU16(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const libc::c_short,
    mut maxSymbolValue: libc::c_uint,
    mut tableLog: libc::c_uint,
) -> size_t {
    /* because *dt is unsigned, 32-bits aligned on 32-bits */
    let tdPtr: *mut libc::c_void = dt.offset(1isize) as *mut libc::c_void;
    let tableDecode: *mut FSE_decode_tU16 = tdPtr as *mut FSE_decode_tU16;
    let mut symbolNext: [U16; 287] = [0; 287];
    let maxSV1: U32 = maxSymbolValue.wrapping_add(1i32 as libc::c_uint);
    let tableSize: U32 = (1i32 << tableLog) as U32;
    let mut highThreshold: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    if maxSymbolValue > 286i32 as libc::c_uint {
        return -(FSE_error_maxSymbolValue_tooLarge as libc::c_int) as size_t;
    }
    if tableLog > (15i32 - 2i32) as libc::c_uint {
        return -(FSE_error_tableLog_tooLarge as libc::c_int) as size_t;
    }
    let mut DTableH: FSE_DTableHeader = FSE_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    DTableH.tableLog = tableLog as U16;
    DTableH.fastMode = 1i32 as U16;
    let largeLimit: S16 = (1i32 << tableLog.wrapping_sub(1i32 as libc::c_uint)) as S16;
    let mut s: U32 = 0;
    s = 0i32 as U32;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as libc::c_int == -1i32 {
            let fresh6 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            let ref mut fresh7 = *tableDecode.offset(fresh6 as isize);
            (*fresh7).set_symbol(s as U16 as libc::c_uint);
            symbolNext[s as usize] = 1i32 as U16
        } else {
            if *normalizedCounter.offset(s as isize) as libc::c_int >= largeLimit as libc::c_int {
                DTableH.fastMode = 0i32 as U16
            }
            symbolNext[s as usize] = *normalizedCounter.offset(s as isize) as U16
        }
        s = s.wrapping_add(1)
    }
    memcpy(
        dt as *mut libc::c_void,
        &mut DTableH as *mut FSE_DTableHeader as *const libc::c_void,
        ::std::mem::size_of::<FSE_DTableHeader>() as libc::c_ulong,
    );
    let tableMask: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    let step: U32 = (tableSize >> 1i32)
        .wrapping_add(tableSize >> 3i32)
        .wrapping_add(3i32 as libc::c_uint);
    let mut s_0: U32 = 0;
    let mut position: U32 = 0i32 as U32;
    s_0 = 0i32 as U32;
    while s_0 < maxSV1 {
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < *normalizedCounter.offset(s_0 as isize) as libc::c_int {
            let ref mut fresh8 = *tableDecode.offset(position as isize);
            (*fresh8).set_symbol(s_0 as U16 as libc::c_uint);
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask
            }
            i += 1
        }
        s_0 = s_0.wrapping_add(1)
    }
    if position != 0i32 as libc::c_uint {
        return -(FSE_error_GENERIC as libc::c_int) as size_t;
    }
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < tableSize {
        let symbol: U16 = (*tableDecode.offset(u as isize)).symbol() as U16;
        let fresh9 = symbolNext[symbol as usize];
        symbolNext[symbol as usize] = symbolNext[symbol as usize].wrapping_add(1);
        let nextState: U32 = fresh9 as U32;
        let ref mut fresh10 = *tableDecode.offset(u as isize);
        (*fresh10)
            .set_nbBits(tableLog.wrapping_sub(BIT_highbit32(nextState)) as BYTE as libc::c_uint);
        (*tableDecode.offset(u as isize)).newState = (nextState
            << (*tableDecode.offset(u as isize)).nbBits() as libc::c_int)
            .wrapping_sub(tableSize) as U16;
        u = u.wrapping_add(1)
    }
    return 0i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_createDTableU16(mut tableLog: libc::c_uint) -> *mut FSE_DTable {
    if tableLog > 15i32 as libc::c_uint {
        tableLog = 15i32 as libc::c_uint
    }
    return malloc(
        ((1i32 + (1i32 << tableLog)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<U32>() as libc::c_ulong),
    ) as *mut FSE_DTable;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_freeDTableU16(mut dt: *mut FSE_DTable) {
    free(dt as *mut libc::c_void);
}
