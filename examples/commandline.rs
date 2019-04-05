#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
extern crate fse_rs;
extern crate libc;

use fse_rs::converted::*;

extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut __stdinp: *mut FILE;
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn getchar() -> libc::c_int;
    #[no_mangle]
    fn fileno(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn BMK_benchFiles(
        fileNamesTable: *mut *const libc::c_char,
        nbFiles: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn BMK_benchCore_Files(
        fileNamesTable: *mut *const libc::c_char,
        nbFiles: libc::c_int,
    ) -> libc::c_int;
    // Parameters
    #[no_mangle]
    fn BMK_SetBlocksize(bsize: libc::c_uint);
    #[no_mangle]
    fn BMK_SetNbIterations(nbLoops: libc::c_int);
    #[no_mangle]
    fn BMK_SetByteCompressor(id: libc::c_int);
    #[no_mangle]
    fn BMK_SetTableLog(tableLog: libc::c_int);
    #[no_mangle]
    fn FIO_setCompressor(c: FIO_compressor_t);
    #[no_mangle]
    fn FIO_setDisplayLevel(dlevel: libc::c_int);
    #[no_mangle]
    fn FIO_overwriteMode();
    /* *************************************
     *  Stream/File functions
     **************************************/
    #[no_mangle]
    fn FIO_compressFilename(
        outfilename: *const libc::c_char,
        infilename: *const libc::c_char,
    ) -> libc::c_ulonglong;
    #[no_mangle]
    fn FIO_decompressFilename(
        outfilename: *const libc::c_char,
        infilename: *const libc::c_char,
    ) -> libc::c_ulonglong;
    #[no_mangle]
    fn isatty(_: libc::c_int) -> libc::c_int;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
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
/* *************************************
*  Parameters
**************************************/
pub type FIO_compressor_t = libc::c_uint;
pub const FIO_zlibh: FIO_compressor_t = 2;
pub const FIO_huf: FIO_compressor_t = 1;
pub const FIO_fse: FIO_compressor_t = 0;
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
/*
  commandline.c - simple command line interface for FSE
  Copyright (C) Yann Collet 2013-2017

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
  - Source repository : https://github.com/Cyan4973/FiniteStateEntropy
*/
/*
  Note : this is stand-alone program.
  It is not part of FSE compression library, just a user program of the FSE library.
  The license of FSE library is BSD.
  The license of this program is GPLv2.
*/
/*-*************************************************
*  Compiler instructions
****************************************************/
/* Remove warning under visual studio */
/* get fileno() within <stdio.h> for Unix */
/*-*************************************************
*  Includes
***************************************************/
/* exit */
/* fprintf */
/* strcmp, strcat */
/*-*************************************************
*  OS-specific Includes
***************************************************/
/* isatty */
/*-*************************************************
*  Constants
***************************************************/
/*-*************************************************
*  Macros
***************************************************/
/*-*************************************************
*  Local variables
***************************************************/
// 0 : no display  // 1: errors  // 2 : + result + interaction + warnings ;  // 3 : + progression;  // 4 : + information
static mut displayLevel: libc::c_int = 2i32;
static mut fse_pause: libc::c_int = 0i32;
/*-*************************************************
*  Functions
***************************************************/
unsafe extern "C" fn usage(mut programName: *const libc::c_char) -> libc::c_int {
    fprintf(
        __stderrp,
        b"Usage :\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b"%s [arg] inputFilename [outputFilename]\n\x00" as *const u8 as *const libc::c_char,
        programName,
    );
    fprintf(
        __stderrp,
        b"Arguments :\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b"(default): fse core loop timing tests\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b" -e : use FSE (default)\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b" -h : use HUF\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b" -z : use zlib\'s huffman\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b" -d : decompression (default for %s extension)\n\x00" as *const u8 as *const libc::c_char,
        b".fse\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b" -b : benchmark mode\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b" -i#: iteration loops [1-9](default : 4), benchmark mode only\n\x00" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b" -B#: block size (default : 32768), benchmark mode only\n\x00" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        __stderrp,
        b" -H : display help and exit\n\x00" as *const u8 as *const libc::c_char,
    );
    return 0i32;
}
unsafe extern "C" fn badusage(mut programName: *const libc::c_char) -> libc::c_int {
    if displayLevel >= 1i32 {
        fprintf(
            __stderrp,
            b"Incorrect parameters\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if displayLevel >= 1i32 {
        usage(programName);
    }
    exit(1i32);
}
unsafe extern "C" fn waitEnter() {
    let mut unused: libc::c_int = 0;
    fprintf(
        __stderrp,
        b"Press enter to continue...\n\x00" as *const u8 as *const libc::c_char,
    );
    unused = getchar();
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* default action if no argument */
    let mut forceCompress: libc::c_int = 1i32;
    let mut decode: libc::c_int = 0i32;
    let mut bench: libc::c_int = 0i32;
    let mut indexFileNames: libc::c_int = 0i32;
    let mut input_filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut output_filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmpFilenameBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpFilenameSize: size_t = 0i32 as size_t;
    let extension: [libc::c_char; 5] =
        *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".fse\x00");
    let programName: *const libc::c_char = *argv.offset(0isize);
    let mut compressor: FIO_compressor_t = FIO_fse;
    fprintf(
        __stderrp,
        b"%s, %i-bits demo by %s (%s)\n\x00" as *const u8 as *const libc::c_char,
        b"FSE : Finite State Entropy\x00" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_int * 8i32,
        b"Yann Collet\x00" as *const u8 as *const libc::c_char,
        b"Mar 29 2019\x00" as *const u8 as *const libc::c_char,
    );
    if argc < 2i32 {
        badusage(programName);
    }
    i = 1i32;
    while i <= argc {
        let mut argument: *const libc::c_char = *argv.offset(i as isize);
        if !argument.is_null() {
            /* Protection if argument empty */
            // Decode command (note : aggregated commands are allowed)
            if *argument.offset(0isize) as libc::c_int == '-' as i32 {
                if *argument.offset(1isize) as libc::c_int == 0i32 {
                    if input_filename.is_null() {
                        input_filename = stdinmark.as_ptr()
                    } else {
                        output_filename = stdoutmark.as_ptr()
                    }
                }
                while *argument.offset(1isize) as libc::c_int != 0i32 {
                    argument = argument.offset(1isize);
                    match *argument.offset(0isize) as libc::c_int {
                        86 => {
                            fprintf(
                                __stderrp,
                                b"%s, %i-bits demo by %s (%s)\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"FSE : Finite State Entropy\x00" as *const u8
                                    as *const libc::c_char,
                                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                                    as libc::c_int
                                    * 8i32,
                                b"Yann Collet\x00" as *const u8 as *const libc::c_char,
                                b"Mar 29 2019\x00" as *const u8 as *const libc::c_char,
                            );
                            return 0i32;
                        }
                        72 => {
                            usage(programName);
                            return 0i32;
                        }
                        100 => {
                            decode = 1i32;
                            bench = 0i32
                        }
                        98 => bench = 1i32,
                        101 => {
                            BMK_SetByteCompressor(1i32);
                            compressor = FIO_fse
                        }
                        104 => {
                            BMK_SetByteCompressor(2i32);
                            compressor = FIO_huf
                        }
                        122 => {
                            BMK_SetByteCompressor(3i32);
                            compressor = FIO_zlibh
                        }
                        116 => {
                            decode = 1i32;
                            output_filename = b"/dev/null\x00" as *const u8 as *const libc::c_char
                        }
                        102 => {
                            FIO_overwriteMode();
                        }
                        118 => displayLevel += 1,
                        113 => displayLevel -= 1,
                        107 => {}
                        66 => {
                            let mut bSize: libc::c_uint = 0i32 as libc::c_uint;
                            while *argument.offset(1isize) as libc::c_int >= '0' as i32
                                && *argument.offset(1isize) as libc::c_int <= '9' as i32
                            {
                                let mut digit: libc::c_uint =
                                    (*argument.offset(1isize) as libc::c_int - '0' as i32)
                                        as libc::c_uint;
                                bSize = bSize.wrapping_mul(10i32 as libc::c_uint);
                                bSize = bSize.wrapping_add(digit);
                                argument = argument.offset(1isize)
                            }
                            if *argument.offset(1isize) as libc::c_int == 'K' as i32 {
                                bSize <<= 10i32;
                                argument = argument.offset(1isize)
                            }
                            if *argument.offset(1isize) as libc::c_int == 'M' as i32 {
                                bSize <<= 20i32;
                                argument = argument.offset(1isize)
                            }
                            if *argument.offset(1isize) as libc::c_int == 'B' as i32 {
                                argument = argument.offset(1isize)
                            }
                            BMK_SetBlocksize(bSize);
                        }
                        83 => {}
                        105 => {
                            // to be completed later
                            if *argument.offset(1isize) as libc::c_int >= '1' as i32
                                && *argument.offset(1isize) as libc::c_int <= '9' as i32
                            {
                                let mut iters: libc::c_int =
                                    *argument.offset(1isize) as libc::c_int - '0' as i32;
                                BMK_SetNbIterations(iters);
                                argument = argument.offset(1isize)
                            }
                        }
                        112 => fse_pause = 1i32,
                        77 => {
                            if *argument.offset(1isize) as libc::c_int >= '1' as i32
                                && *argument.offset(1isize) as libc::c_int <= '9' as i32
                            {
                                let mut tableLog: libc::c_int =
                                    *argument.offset(1isize) as libc::c_int - '0' as i32;
                                BMK_SetTableLog(tableLog);
                                argument = argument.offset(1isize)
                            }
                        }
                        _ => {
                            badusage(programName);
                        }
                    }
                }
            } else if input_filename.is_null() {
                input_filename = argument;
                indexFileNames = i
            } else if output_filename.is_null() {
                output_filename = argument
            }
        }
        i += 1
    }
    if input_filename.is_null() {
        input_filename = stdinmark.as_ptr()
    }
    if 0 == strcmp(input_filename, stdinmark.as_ptr()) && 0 != isatty(fileno(__stdinp)) {
        badusage(programName);
    }
    /* Check if benchmark is selected */
    if bench == 1i32 {
        BMK_benchFiles(argv.offset(indexFileNames as isize), argc - indexFileNames);
    } else if bench == 3i32 {
        BMK_benchCore_Files(argv.offset(indexFileNames as isize), argc - indexFileNames);
    } else {
        /* no longer possible */
        while output_filename.is_null() {
            if 0 == isatty(fileno(__stdoutp)) {
                output_filename = stdoutmark.as_ptr();
                // Default to stdout whenever possible (i.e. not a console)
                break;
            } else {
                if 0 == decode && 0 == forceCompress {
                    let l: size_t = strlen(input_filename);
                    if 0 == strcmp(
                        input_filename.offset(l.wrapping_sub(4i32 as libc::c_ulong) as isize),
                        b".fse\x00" as *const u8 as *const libc::c_char,
                    ) {
                        decode = 1i32
                    }
                }
                if 0 == decode {
                    /* compression to file */
                    let l_0: size_t = strlen(input_filename);
                    if tmpFilenameSize < l_0.wrapping_add(6i32 as libc::c_ulong) {
                        tmpFilenameSize = l_0.wrapping_add(6i32 as libc::c_ulong)
                    }
                    tmpFilenameBuffer =
                        calloc(1i32 as libc::c_ulong, tmpFilenameSize) as *mut libc::c_char;
                    if tmpFilenameBuffer.is_null() {
                        fprintf(
                            __stderrp,
                            b"Not enough memory, exiting ... \n\x00" as *const u8
                                as *const libc::c_char,
                        );
                        exit(1i32);
                    }
                    strcpy(tmpFilenameBuffer, input_filename);
                    strcpy(
                        tmpFilenameBuffer.offset(l_0 as isize),
                        b".fse\x00" as *const u8 as *const libc::c_char,
                    );
                    output_filename = tmpFilenameBuffer;
                    if displayLevel >= 2i32 {
                        fprintf(
                            __stderrp,
                            b"Compressed filename will be : %s \n\x00" as *const u8
                                as *const libc::c_char,
                            output_filename,
                        );
                    }
                    break;
                } else {
                    let mut outl: size_t = 0;
                    let inl: size_t = strlen(input_filename);
                    if tmpFilenameSize < inl.wrapping_add(2i32 as libc::c_ulong) {
                        tmpFilenameSize = inl.wrapping_add(2i32 as libc::c_ulong)
                    }
                    tmpFilenameBuffer =
                        calloc(1i32 as libc::c_ulong, tmpFilenameSize) as *mut libc::c_char;
                    strcpy(tmpFilenameBuffer, input_filename);
                    outl = inl;
                    if inl > 4i32 as libc::c_ulong {
                        while outl >= inl.wrapping_sub(4i32 as libc::c_ulong)
                            && *input_filename.offset(outl as isize) as libc::c_int
                                == extension[outl
                                    .wrapping_sub(inl)
                                    .wrapping_add(4i32 as libc::c_ulong)
                                    as usize] as libc::c_int
                        {
                            let fresh0 = outl;
                            outl = outl.wrapping_sub(1);
                            *tmpFilenameBuffer.offset(fresh0 as isize) = 0i32 as libc::c_char
                        }
                    }
                    if outl != inl.wrapping_sub(5i32 as libc::c_ulong) {
                        if displayLevel >= 1i32 {
                            fprintf(
                                __stderrp,
                                b"Cannot determine an output filename\n\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        badusage(programName);
                    }
                    output_filename = tmpFilenameBuffer;
                    if displayLevel >= 2i32 {
                        fprintf(
                            __stderrp,
                            b"Decoding into filename : %s \n\x00" as *const u8
                                as *const libc::c_char,
                            output_filename,
                        );
                    }
                }
            }
        }
        if 0 == strcmp(input_filename, stdinmark.as_ptr())
            && 0 == strcmp(output_filename, stdoutmark.as_ptr())
            && displayLevel == 2i32
        {
            displayLevel = 1i32
        }
        if 0 == strcmp(input_filename, stdinmark.as_ptr()) && 0 != isatty(fileno(__stdinp)) {
            badusage(programName);
        }
        if 0 == strcmp(output_filename, stdoutmark.as_ptr()) && 0 != isatty(fileno(__stdoutp)) {
            badusage(programName);
        }
        FIO_setDisplayLevel(displayLevel);
        if 0 != decode {
            FIO_decompressFilename(output_filename, input_filename);
        } else {
            FIO_setCompressor(compressor);
            FIO_compressFilename(output_filename, input_filename);
        }
    }
    if 0 != fse_pause {
        waitEnter();
    }
    free(tmpFilenameBuffer as *mut libc::c_void);
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
            args.as_mut_ptr() as *mut *const libc::c_char,
        ) as i32)
    }
}

mod bench {
    #![allow(
        dead_code,
        mutable_transmutes,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused_mut
    )]
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
        fn fread(
            _: *mut libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
        #[no_mangle]
        fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[no_mangle]
        fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        fn stat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
        #[no_mangle]
        fn clock() -> clock_t;
        #[no_mangle]
        fn __assert_rtn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_int,
            _: *const libc::c_char,
        ) -> !;
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
    pub type off_t = __darwin_off_t;
    pub type blkcnt_t = __darwin_blkcnt_t;
    pub type blksize_t = __darwin_blksize_t;
    pub type gid_t = __darwin_gid_t;
    pub type nlink_t = __uint16_t;
    pub type clock_t = __darwin_clock_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: __darwin_time_t,
        pub tv_nsec: libc::c_long,
    }
    #[derive(Copy, Clone)]
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
    #[derive(Copy, Clone)]
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
    pub unsafe extern "C" fn BMK_benchFiles(
        mut fileNamesTable: *mut *const libc::c_char,
        mut nbFiles: libc::c_int,
    ) -> libc::c_int {
        use fse_rs::converted::*;
        let mut fileIdx: libc::c_int = 0i32;
        let mut totalSourceSize: U64 = 0i32 as U64;
        let mut totalCompressedSize: U64 = 0i32 as U64;
        let mut totalc: libc::c_double = 0.0f64;
        let mut totald: libc::c_double = 0.0f64;
        while fileIdx < nbFiles {
            let fresh0 = fileIdx;
            fileIdx = fileIdx + 1;
            let inFileName: *const libc::c_char = *fileNamesTable.offset(fresh0 as isize);
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
                fprintf(
                    __stderrp,
                    b"Pb opening %s\n\x00" as *const u8 as *const libc::c_char,
                    inFileName,
                );
                return 11i32;
            }
            inFileSize = BMK_GetFileSize(inFileName);
            if inFileSize == 0i32 as libc::c_ulonglong {
                fprintf(
                    __stderrp,
                    b"file is empty\n\x00" as *const u8 as *const libc::c_char,
                );
                fclose(inFile);
                return 11i32;
            }
            benchedSize = BMK_findMaxMem(inFileSize.wrapping_mul(3i32 as libc::c_ulonglong))
                .wrapping_div(3i32 as libc::c_ulong);
            if benchedSize as U64 > inFileSize {
                benchedSize = inFileSize as size_t
            }
            if (benchedSize as libc::c_ulonglong) < inFileSize {
                fprintf(
                    __stderrp,
                    b"Not enough memory for \'%s\' full size; testing %i MB only...\n\x00"
                        as *const u8 as *const libc::c_char,
                    inFileName,
                    (benchedSize >> 20i32) as libc::c_int,
                );
            }
            chunkP = malloc(
                benchedSize
                    .wrapping_div(chunkSize as libc::c_ulong)
                    .wrapping_add(1i32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<chunkParameters_t>() as libc::c_ulong),
            ) as *mut chunkParameters_t;
            orig_buff = malloc(benchedSize) as *mut libc::c_char;
            nbChunks = benchedSize.wrapping_div(chunkSize as libc::c_ulong) as libc::c_int + 1i32;
            maxCompressedChunkSize =
                fse_rs::converted::FSE_compressBound(chunkSize as size_t) as libc::c_int;
            compressedBuffSize = nbChunks * maxCompressedChunkSize;
            compressedBuffer = malloc(compressedBuffSize as size_t) as *mut libc::c_char;
            destBuffer = malloc(benchedSize) as *mut libc::c_char;
            if orig_buff.is_null()
                || compressedBuffer.is_null()
                || destBuffer.is_null()
                || chunkP.is_null()
            {
                fprintf(
                    __stderrp,
                    b"\nError: not enough memory!\n\x00" as *const u8 as *const libc::c_char,
                );
                free(orig_buff as *mut libc::c_void);
                free(compressedBuffer as *mut libc::c_void);
                free(destBuffer as *mut libc::c_void);
                free(chunkP as *mut libc::c_void);
                fclose(inFile);
                return 12i32;
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
                    remaining = (remaining as libc::c_ulong)
                        .wrapping_sub(chunkSize as libc::c_ulong)
                        as size_t as size_t
                } else {
                    (*chunkP.offset(i as isize)).origSize = remaining as libc::c_int as size_t;
                    remaining = 0i32 as size_t
                }
                let ref mut fresh2 = (*chunkP.offset(i as isize)).compressedBuffer;
                *fresh2 = out;
                out = out.offset(maxCompressedChunkSize as isize);
                (*chunkP.offset(i as isize)).compressedSize = 0i32 as size_t;
                let ref mut fresh3 = (*chunkP.offset(i as isize)).destBuffer;
                *fresh3 = dst;
                dst = dst.offset(chunkSize as isize);
                i += 1
            }
            fprintf(
                __stderrp,
                b"Loading %s...       \r\x00" as *const u8 as *const libc::c_char,
                inFileName,
            );
            readSize = fread(
                orig_buff as *mut libc::c_void,
                1i32 as libc::c_ulong,
                benchedSize,
                inFile,
            );
            fclose(inFile);
            if readSize != benchedSize {
                fprintf(
                    __stderrp,
                    b"\nError: problem reading file \'%s\' (%i read, should be %i) !!    \n\x00"
                        as *const u8 as *const libc::c_char,
                    inFileName,
                    readSize as libc::c_int,
                    benchedSize as libc::c_int,
                );
                free(orig_buff as *mut libc::c_void);
                free(compressedBuffer as *mut libc::c_void);
                free(destBuffer as *mut libc::c_void);
                free(chunkP as *mut libc::c_void);
                return 13i32;
            }
            BMK_benchMem(
                chunkP,
                nbChunks,
                inFileName,
                benchedSize as libc::c_int,
                &mut totalCompressedSize,
                &mut totalc,
                &mut totald,
                255i32,
                BMK_tableLog,
            );
            totalSourceSize = (totalSourceSize as libc::c_ulonglong)
                .wrapping_add(benchedSize as libc::c_ulonglong) as U64
                as U64;
            free(orig_buff as *mut libc::c_void);
            free(compressedBuffer as *mut libc::c_void);
            free(destBuffer as *mut libc::c_void);
            free(chunkP as *mut libc::c_void);
        }
        if nbFiles > 1i32 {
            fprintf(
                __stderrp,
                b"%-17.17s :%10llu ->%10llu (%5.2f%%), %6.1f MB/s , %6.1f MB/s\n\x00" as *const u8
                    as *const libc::c_char,
                b"  TOTAL\x00" as *const u8 as *const libc::c_char,
                totalSourceSize,
                totalCompressedSize,
                totalCompressedSize as libc::c_double / totalSourceSize as libc::c_double
                    * 100.0f64,
                totalSourceSize as libc::c_double / totalc / 1000000i32 as libc::c_double,
                totalSourceSize as libc::c_double / totald / 1000000i32 as libc::c_double,
            );
        }
        return 0i32;
    }
    static mut BMK_tableLog: libc::c_int = 12i32;
    /* BMK_benchMem() :
     * chunkP is expected to be correctly filled */
    #[no_mangle]
    pub unsafe extern "C" fn BMK_benchMem(
        mut chunkP: *mut chunkParameters_t,
        mut nbChunks: libc::c_int,
        mut inFileName: *const libc::c_char,
        mut benchedSize: libc::c_int,
        mut totalCompressedSize: *mut U64,
        mut totalCompressionTime: *mut libc::c_double,
        mut totalDecompressionTime: *mut libc::c_double,
        mut nbSymbols: libc::c_int,
        mut memLog: libc::c_int,
    ) {
        let mut trial: libc::c_int = 0;
        let mut chunkNb: libc::c_int = 0;
        let mut cSize: size_t = 0i32 as size_t;
        let mut fastestC: libc::c_double = 100000000.0f64;
        let mut fastestD: libc::c_double = 100000000.0f64;
        let mut ratio: libc::c_double = 0.0f64;
        let mut crcCheck: U32 = 0i32 as U32;
        use fse_rs::converted::*; // FIXME: Why do I need to reimport it here?
        let mut nbDecodeLoops: libc::c_int = (100i32 as libc::c_uint)
            .wrapping_mul(1u32 << 20i32)
            .wrapping_div((benchedSize + 1i32) as libc::c_uint)
            .wrapping_add(1i32 as libc::c_uint)
            as libc::c_int;
        let crcOrig: U32 = XXH32(
            (*chunkP.offset(0isize)).origBuffer as *const libc::c_void,
            benchedSize as size_t,
            0i32 as libc::c_uint,
        );
        let mut compressor: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: size_t,
                _: *const libc::c_void,
                _: size_t,
                _: libc::c_uint,
                _: libc::c_uint,
            ) -> size_t,
        > = None;
        let mut decompressor: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: size_t,
                _: *const libc::c_void,
                _: size_t,
            ) -> size_t,
        > = None;
        let nameLength: size_t = strlen(inFileName);
        if nameLength > 17i32 as libc::c_ulong {
            inFileName = inFileName.offset(nameLength.wrapping_sub(17i32 as libc::c_ulong) as isize)
        }
        if nbSymbols == 3i32 {
            BMK_benchMem285(
                chunkP,
                nbChunks,
                inFileName,
                benchedSize,
                totalCompressedSize,
                totalCompressionTime,
                totalDecompressionTime,
                memLog,
            );
            return;
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
        fprintf(
            __stderrp,
            b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
        trial = 1i32;
        while trial <= nbIterations {
            let mut nbLoops: libc::c_int = 0i32;
            let mut clockStart: clock_t = 0;
            let mut clockDuration: clock_t = 0;
            fprintf(
                __stderrp,
                b"%1i-%-15.15s : %9i ->\r\x00" as *const u8 as *const libc::c_char,
                trial,
                inFileName,
                benchedSize,
            );
            let mut i: libc::c_int = 0;
            i = 0i32;
            while i < benchedSize {
                *(*chunkP.offset(0isize)).compressedBuffer.offset(i as isize) = i as libc::c_char;
                i += 1
            }
            clockStart = clock();
            while clock() == clockStart {}
            clockStart = clock();
            while BMK_clockSpan(clockStart) < (1000000i32 * 2i32) as libc::c_ulong {
                chunkNb = 0i32;
                while chunkNb < nbChunks {
                    let cBSize: size_t = compressor.expect("non-null function pointer")(
                        (*chunkP.offset(chunkNb as isize)).compressedBuffer as *mut libc::c_void,
                        FSE_compressBound((*chunkP.offset(chunkNb as isize)).origSize),
                        (*chunkP.offset(chunkNb as isize)).origBuffer as *const libc::c_void,
                        (*chunkP.offset(chunkNb as isize)).origSize,
                        nbSymbols as libc::c_uint,
                        memLog as libc::c_uint,
                    );
                    if 0 != FSE_isError(cBSize) {
                        fprintf(
                            __stderrp,
                            b"!!! Error compressing block %i  !!!!  => %s   \n\x00" as *const u8
                                as *const libc::c_char,
                            chunkNb,
                            FSE_getErrorName(cBSize),
                        );
                        return;
                    }
                    (*chunkP.offset(chunkNb as isize)).compressedSize = cBSize;
                    chunkNb += 1
                }
                nbLoops += 1
            }
            clockDuration = BMK_clockSpan(clockStart);
            clockDuration = (clockDuration as libc::c_ulong)
                .wrapping_add((0 == clockDuration) as libc::c_int as libc::c_ulong)
                as clock_t as clock_t;
            if (clockDuration as libc::c_double)
                < fastestC * nbLoops as libc::c_double * 1000000i32 as libc::c_double
            {
                fastestC = clockDuration as libc::c_double
                    / 1000000i32 as libc::c_double
                    / nbLoops as libc::c_double
            }
            cSize = 0i32 as size_t;
            chunkNb = 0i32;
            while chunkNb < nbChunks {
                cSize = (cSize as libc::c_ulong).wrapping_add(
                    if 0 != (*chunkP.offset(chunkNb as isize)).compressedSize {
                        (*chunkP.offset(chunkNb as isize)).compressedSize
                    } else {
                        (*chunkP.offset(chunkNb as isize)).origSize
                    },
                ) as size_t as size_t;
                chunkNb += 1
            }
            ratio = cSize as libc::c_double / benchedSize as libc::c_double * 100.0f64;
            fprintf(
                __stderrp,
                b"%1i-%-15.15s : %9i -> %9i (%5.2f%%),%7.1f MB/s\r\x00" as *const u8
                    as *const libc::c_char,
                trial,
                inFileName,
                benchedSize,
                cSize as libc::c_int,
                ratio,
                benchedSize as libc::c_double
                    / (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as libc::c_double
                    / fastestC,
            );
            let mut i_0: libc::c_int = 0;
            i_0 = 0i32;
            while i_0 < benchedSize {
                *(*chunkP.offset(0isize)).destBuffer.offset(i_0 as isize) = 0i32 as libc::c_char;
                i_0 += 1
            }
            clockStart = clock();
            while clock() == clockStart {}
            clockStart = clock();
            nbLoops = 0i32;
            while nbLoops < nbDecodeLoops {
                chunkNb = 0i32;
                while chunkNb < nbChunks {
                    let mut regenSize: size_t = 0;
                    match (*chunkP.offset(chunkNb as isize)).compressedSize {
                        0 => {
                            regenSize = (*chunkP.offset(chunkNb as isize)).origSize;
                            memcpy(
                                (*chunkP.offset(chunkNb as isize)).destBuffer as *mut libc::c_void,
                                (*chunkP.offset(chunkNb as isize)).origBuffer
                                    as *const libc::c_void,
                                regenSize,
                            );
                        }
                        1 => {
                            regenSize = (*chunkP.offset(chunkNb as isize)).origSize;
                            memset(
                                (*chunkP.offset(chunkNb as isize)).destBuffer as *mut libc::c_void,
                                *(*chunkP.offset(chunkNb as isize)).origBuffer.offset(0isize)
                                    as libc::c_int,
                                (*chunkP.offset(chunkNb as isize)).origSize,
                            );
                        }
                        _ => {
                            regenSize = decompressor.expect("non-null function pointer")(
                                (*chunkP.offset(chunkNb as isize)).destBuffer as *mut libc::c_void,
                                (*chunkP.offset(chunkNb as isize)).origSize,
                                (*chunkP.offset(chunkNb as isize)).compressedBuffer
                                    as *const libc::c_void,
                                (*chunkP.offset(chunkNb as isize)).compressedSize,
                            )
                        }
                    }
                    if regenSize != (*chunkP.offset(chunkNb as isize)).origSize {
                        fprintf(
                            __stderrp,
                            b"!! Error decompressing block %i of cSize %u !! => (%s)  \n\x00"
                                as *const u8 as *const libc::c_char,
                            chunkNb,
                            (*chunkP.offset(chunkNb as isize)).compressedSize as U32,
                            FSE_getErrorName(regenSize),
                        );
                        return;
                    }
                    chunkNb += 1
                }
                nbLoops += 1
            }
            clockDuration = BMK_clockSpan(clockStart);
            if clockDuration > 0i32 as libc::c_ulong {
                if (clockDuration as libc::c_double)
                    < fastestD * nbDecodeLoops as libc::c_double * 1000000i32 as libc::c_double
                {
                    fastestD = clockDuration as libc::c_double
                        / 1000000i32 as libc::c_double
                        / nbDecodeLoops as libc::c_double
                }
                if 0 != !(fastestD > 1.0f64 / 1000000000i32 as libc::c_double) as libc::c_int
                    as libc::c_long
                {
                    __assert_rtn(
                        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(
                            b"BMK_benchMem\x00",
                        ))
                        .as_ptr(),
                        b"bench.c\x00" as *const u8 as *const libc::c_char,
                        431i32,
                        b"fastestD > 1./1000000000\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                };
                nbDecodeLoops =
                    ((1.0f64 / fastestD) as U32).wrapping_add(1i32 as libc::c_uint) as libc::c_int
            } else {
                if 0 != !(nbDecodeLoops < 20000000i32) as libc::c_int as libc::c_long {
                    __assert_rtn(
                        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(
                            b"BMK_benchMem\x00",
                        ))
                        .as_ptr(),
                        b"bench.c\x00" as *const u8 as *const libc::c_char,
                        434i32,
                        b"nbDecodeLoops < 20000000\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                };
                nbDecodeLoops *= 100i32
            }
            fprintf(
                __stderrp,
                b"%1i-%-15.15s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\r\x00" as *const u8
                    as *const libc::c_char,
                trial,
                inFileName,
                benchedSize,
                cSize as libc::c_int,
                ratio,
                benchedSize as libc::c_double
                    / (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as libc::c_double
                    / fastestC,
                benchedSize as libc::c_double
                    / (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as libc::c_double
                    / fastestD,
            );
            crcCheck = XXH32(
                (*chunkP.offset(0isize)).destBuffer as *const libc::c_void,
                benchedSize as size_t,
                0i32 as libc::c_uint,
            );
            if crcOrig != crcCheck {
                let mut src_0: *const libc::c_char = (*chunkP.offset(0isize)).origBuffer;
                let mut fin: *const libc::c_char = (*chunkP.offset(0isize)).destBuffer;
                let srcStart: *const libc::c_char = src_0;
                while *src_0 as libc::c_int == *fin as libc::c_int {
                    src_0 = src_0.offset(1isize);
                    fin = fin.offset(1isize)
                }
                fprintf(
                    __stderrp,
                    b"\n!!! %15s : Invalid Checksum !!! pos %i/%i\n\x00" as *const u8
                        as *const libc::c_char,
                    inFileName,
                    src_0.wrapping_offset_from(srcStart) as libc::c_long as libc::c_int,
                    benchedSize,
                );
                break;
            } else {
                trial += 1
            }
        }
        if crcOrig == crcCheck {
            if ratio < 100.0f64 {
                fprintf(
                    __stderrp,
                    b"%-17.17s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\n\x00" as *const u8
                        as *const libc::c_char,
                    inFileName,
                    benchedSize,
                    cSize as libc::c_int,
                    ratio,
                    benchedSize as libc::c_double
                        / (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as libc::c_double
                        / fastestC,
                    benchedSize as libc::c_double
                        / (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as libc::c_double
                        / fastestD,
                );
            } else {
                fprintf(
                    __stderrp,
                    b"%-17.17s : %9i -> %9i (%5.1f%%),%7.1f MB/s ,%7.1f MB/s \n\x00" as *const u8
                        as *const libc::c_char,
                    inFileName,
                    benchedSize,
                    cSize as libc::c_int,
                    ratio,
                    benchedSize as libc::c_double
                        / (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as libc::c_double
                        / fastestC,
                    benchedSize as libc::c_double
                        / (1i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as libc::c_double
                        / fastestD,
                );
            }
        } else {
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        *totalCompressedSize = (*totalCompressedSize as libc::c_ulonglong)
            .wrapping_add(cSize as libc::c_ulonglong) as U64 as U64;
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
    pub unsafe extern "C" fn BMK_ZLIBH_decompress(
        mut dest: *mut libc::c_void,
        mut originalSize: size_t,
        mut compressed: *const libc::c_void,
        mut cSize: size_t,
    ) -> size_t {
        use fse_rs::converted::*; // FIXME: Why do I need to reimport it here?
        ZLIBH_decompress(dest as *mut libc::c_char, compressed as *const libc::c_char);
        return originalSize;
    }
    #[no_mangle]
    pub unsafe extern "C" fn BMK_ZLIBH_compress(
        mut dst: *mut libc::c_void,
        mut dstSize: size_t,
        mut src: *const libc::c_void,
        mut srcSize: size_t,
        mut nbSymbols: libc::c_uint,
        mut tableLog: libc::c_uint,
    ) -> size_t {
        use fse_rs::converted::*; // FIXME: Why do I need to reimport it here?
        return ZLIBH_compress(
            dst as *mut libc::c_char,
            src as *const libc::c_char,
            srcSize as libc::c_int,
        ) as size_t;
    }
    static mut BMK_byteCompressor: libc::c_int = 1i32;
    /*-*******************************************************
    *  Public function
    *********************************************************/
    #[no_mangle]
    pub unsafe fn BMK_benchMem285(
        mut chunkP: *mut chunkParameters_t,
        mut nbChunks: libc::c_int,
        mut inFileName: *const libc::c_char,
        mut benchedSize: libc::c_int,
        mut totalCompressedSize: *mut U64,
        mut totalCompressionTime: *mut libc::c_double,
        mut totalDecompressionTime: *mut libc::c_double,
        mut memLog: libc::c_int,
    ) {
        use fse_rs::converted::*; // FIXME: Why do I need to reimport it here?
        let mut loopNb: libc::c_int = 0;
        let mut chunkNb: libc::c_int = 0;
        let mut cSize: size_t = 0i32 as size_t;
        let mut fastestC: libc::c_double = 100000000.0f64;
        let mut fastestD: libc::c_double = 100000000.0f64;
        let mut ratio: libc::c_double = 0.0f64;
        let mut crcCheck: U32 = 0i32 as U32;
        let mut crcOrig: U32 = 0;
        crcOrig = XXH32(
            (*chunkP.offset(0isize)).origBuffer as *const libc::c_void,
            benchedSize as size_t,
            0i32 as libc::c_uint,
        );
        fprintf(
            __stderrp,
            b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
        loopNb = 1i32;
        while loopNb <= nbIterations {
            let mut nbLoops: libc::c_int = 0;
            let mut clockStart: clock_t = 0;
            let mut clockDuration: clock_t = 0;
            fprintf(
                __stderrp,
                b"%1i-%-14.14s : %9i ->\r\x00" as *const u8 as *const libc::c_char,
                loopNb,
                inFileName,
                benchedSize,
            );
            let mut i: libc::c_int = 0;
            i = 0i32;
            while i < benchedSize {
                *(*chunkP.offset(0isize)).compressedBuffer.offset(i as isize) = i as libc::c_char;
                i += 1
            }
            nbLoops = 0i32;
            clockStart = clock();
            while clock() == clockStart {}
            clockStart = clock();
            while BMK_clockSpan(clockStart) < (1000000i32 * 2i32) as libc::c_ulong {
                chunkNb = 0i32;
                while chunkNb < nbChunks {
                    let mut rawPtr: *const libc::c_void =
                        (*chunkP.offset(chunkNb as isize)).origBuffer as *const libc::c_void;
                    let mut U16chunkPtr: *const U16 = rawPtr as *const U16;
                    (*chunkP.offset(chunkNb as isize)).compressedSize = FSE_compressU16(
                        (*chunkP.offset(chunkNb as isize)).compressedBuffer as *mut libc::c_void,
                        (*chunkP.offset(chunkNb as isize)).origSize,
                        U16chunkPtr,
                        (*chunkP.offset(chunkNb as isize))
                            .origSize
                            .wrapping_div(2i32 as libc::c_ulong),
                        0i32 as libc::c_uint,
                        memLog as libc::c_uint,
                    );
                    chunkNb += 1
                }
                nbLoops += 1
            }
            clockDuration = BMK_clockSpan(clockStart);
            if (clockDuration as libc::c_double) < fastestC * nbLoops as libc::c_double {
                fastestC = clockDuration as libc::c_double / nbLoops as libc::c_double
            }
            cSize = 0i32 as size_t;
            chunkNb = 0i32;
            while chunkNb < nbChunks {
                cSize = (cSize as libc::c_ulong)
                    .wrapping_add((*chunkP.offset(chunkNb as isize)).compressedSize)
                    as size_t as size_t;
                chunkNb += 1
            }
            ratio = cSize as libc::c_double / benchedSize as libc::c_double * 100.0f64;
            fprintf(
                __stderrp,
                b"%1i-%-14.14s : %9i -> %9i (%5.2f%%),%7.1f MB/s\r\x00" as *const u8
                    as *const libc::c_char,
                loopNb,
                inFileName,
                benchedSize,
                cSize as libc::c_int,
                ratio,
                benchedSize as libc::c_double / fastestC / 1000.0f64,
            );
            nbLoops = 0i32;
            clockStart = clock();
            while clock() == clockStart {}
            clockStart = clock();
            while BMK_clockSpan(clockStart) < (1000000i32 * 2i32) as libc::c_ulong {
                chunkNb = 0i32;
                while chunkNb < nbChunks {
                    let mut rawPtr_0: *mut libc::c_void =
                        (*chunkP.offset(chunkNb as isize)).destBuffer as *mut libc::c_void;
                    let mut U16dstPtr: *mut U16 = rawPtr_0 as *mut U16;
                    (*chunkP.offset(chunkNb as isize)).compressedSize = FSE_decompressU16(
                        U16dstPtr,
                        (*chunkP.offset(chunkNb as isize))
                            .origSize
                            .wrapping_div(2i32 as libc::c_ulong),
                        (*chunkP.offset(chunkNb as isize)).compressedBuffer as *const libc::c_void,
                        (*chunkP.offset(chunkNb as isize)).compressedSize,
                    );
                    chunkNb += 1
                }
                nbLoops += 1
            }
            clockDuration = BMK_clockSpan(clockStart);
            if (clockDuration as libc::c_double) < fastestC * nbLoops as libc::c_double {
                fastestC = clockDuration as libc::c_double / nbLoops as libc::c_double
            }
            fprintf(
                __stderrp,
                b"%1i-%-14.14s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\r\x00" as *const u8
                    as *const libc::c_char,
                loopNb,
                inFileName,
                benchedSize,
                cSize as libc::c_int,
                ratio,
                benchedSize as libc::c_double / fastestC / 1000.0f64,
                benchedSize as libc::c_double / fastestD / 1000.0f64,
            );
            crcCheck = XXH32(
                (*chunkP.offset(0isize)).destBuffer as *const libc::c_void,
                benchedSize as size_t,
                0i32 as libc::c_uint,
            );
            if crcOrig != crcCheck {
                let mut src: *const libc::c_char = (*chunkP.offset(0isize)).origBuffer;
                let mut fin: *const libc::c_char = (*chunkP.offset(0isize)).destBuffer;
                let srcStart: *const libc::c_char = src;
                while *src as libc::c_int == *fin as libc::c_int {
                    src = src.offset(1isize);
                    fin = fin.offset(1isize)
                }
                fprintf(
                    __stderrp,
                    b"\n!!! %14s : Invalid Checksum !!! pos %i/%i\n\x00" as *const u8
                        as *const libc::c_char,
                    inFileName,
                    src.wrapping_offset_from(srcStart) as libc::c_long as libc::c_int,
                    benchedSize,
                );
                break;
            } else {
                loopNb += 1
            }
        }
        if crcOrig == crcCheck {
            if ratio < 100.0f64 {
                fprintf(
                    __stderrp,
                    b"%-16.16s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\n\x00" as *const u8
                        as *const libc::c_char,
                    inFileName,
                    benchedSize,
                    cSize as libc::c_int,
                    ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64,
                    benchedSize as libc::c_double / fastestD / 1000.0f64,
                );
            } else {
                fprintf(
                    __stderrp,
                    b"%-16.16s : %9i -> %9i (%5.1f%%),%7.1f MB/s ,%7.1f MB/s \n\x00" as *const u8
                        as *const libc::c_char,
                    inFileName,
                    benchedSize,
                    cSize as libc::c_int,
                    ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64,
                    benchedSize as libc::c_double / fastestD / 1000.0f64,
                );
            }
        }
        *totalCompressedSize = (*totalCompressedSize as libc::c_ulonglong)
            .wrapping_add(cSize as libc::c_ulonglong) as U64 as U64;
        *totalCompressionTime += fastestC;
        *totalDecompressionTime += fastestD;
    }
    // Initialized in run_static_initializers
    static mut chunkSize: U32 = 0;
    unsafe extern "C" fn BMK_findMaxMem(mut requiredMem: U64) -> size_t {
        let mut step: size_t = (64i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as size_t;
        let mut testmem: *mut BYTE = 0 as *mut BYTE;
        requiredMem = (requiredMem >> 26i32).wrapping_add(1i32 as libc::c_ulonglong) << 26i32;
        requiredMem = (requiredMem as libc::c_ulonglong)
            .wrapping_add((2i32 as libc::c_ulong).wrapping_mul(step) as libc::c_ulonglong)
            as U64 as U64;
        if requiredMem
            > if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                == 4i32 as libc::c_ulong
            {
                (2i32 as libc::c_uint)
                    .wrapping_mul(1u32 << 30i32)
                    .wrapping_sub((64i32 as libc::c_uint).wrapping_mul(1u32 << 20i32))
                    as libc::c_ulonglong
            } else {
                9u64.wrapping_mul((1u32 << 30i32) as libc::c_ulonglong)
            }
        {
            requiredMem = if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                == 4i32 as libc::c_ulong
            {
                (2i32 as libc::c_uint)
                    .wrapping_mul(1u32 << 30i32)
                    .wrapping_sub((64i32 as libc::c_uint).wrapping_mul(1u32 << 20i32))
                    as libc::c_ulonglong
            } else {
                9u64.wrapping_mul((1u32 << 30i32) as libc::c_ulonglong)
            }
        }
        while testmem.is_null() {
            requiredMem = (requiredMem as libc::c_ulonglong).wrapping_sub(step as libc::c_ulonglong)
                as U64 as U64;
            if requiredMem <= step as libc::c_ulonglong {
                requiredMem = step.wrapping_add(64i32 as libc::c_ulong) as U64;
                break;
            } else {
                testmem = malloc(requiredMem as size_t) as *mut BYTE
            }
        }
        free(testmem as *mut libc::c_void);
        return requiredMem.wrapping_sub(step as libc::c_ulonglong) as size_t;
    }
    unsafe extern "C" fn BMK_GetFileSize(mut infilename: *const libc::c_char) -> U64 {
        let mut r: libc::c_int = 0;
        let mut statbuf: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_birthtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        r = stat(infilename, &mut statbuf);
        if 0 != r || !(statbuf.st_mode as libc::c_int & 0o170000i32 == 0o100000i32) {
            return 0i32 as U64;
        }
        return statbuf.st_size as U64;
    }
    #[no_mangle]
    pub unsafe extern "C" fn BMK_benchCore_Files(
        mut fileNamesTable: *mut *const libc::c_char,
        mut nbFiles: libc::c_int,
    ) -> libc::c_int {
        use fse_rs::converted::*; // FIXME: Why do I need to reimport it here?
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
            inFile = fopen(inFileName, b"rb\x00" as *const u8 as *const libc::c_char);
            if inFile.is_null() {
                fprintf(
                    __stderrp,
                    b"Pb opening %s\n\x00" as *const u8 as *const libc::c_char,
                    inFileName,
                );
                return 11i32;
            }
            inFileSize = BMK_GetFileSize(inFileName);
            if inFileSize == 0i32 as libc::c_ulonglong {
                fprintf(
                    __stderrp,
                    b"%s is empty\n\x00" as *const u8 as *const libc::c_char,
                    inFileName,
                );
                return 11i32;
            }
            benchedSize = (256i32 as libc::c_uint).wrapping_mul(1u32 << 20i32) as size_t;
            if benchedSize as U64 > inFileSize {
                benchedSize = inFileSize as size_t
            } else {
                fprintf(
                    __stderrp,
                    b"FSE Core Loop speed evaluation, testing %i KB ...\n\x00" as *const u8
                        as *const libc::c_char,
                    (benchedSize >> 10i32) as libc::c_int,
                );
            }
            orig_buff = malloc(benchedSize) as *mut libc::c_char;
            nbChunks = 1i32;
            maxCompressedChunkSize = FSE_compressBound(benchedSize as libc::c_int as size_t);
            compressedBuffSize = (nbChunks as libc::c_ulong).wrapping_mul(maxCompressedChunkSize);
            compressedBuffer = malloc(compressedBuffSize) as *mut libc::c_char;
            if orig_buff.is_null() || compressedBuffer.is_null() {
                fprintf(
                    __stderrp,
                    b"\nError: not enough memory!\n\x00" as *const u8 as *const libc::c_char,
                );
                free(orig_buff as *mut libc::c_void);
                free(compressedBuffer as *mut libc::c_void);
                fclose(inFile);
                return 12i32;
            }
            fprintf(
                __stderrp,
                b"Loading %s...       \r\x00" as *const u8 as *const libc::c_char,
                inFileName,
            );
            readSize = fread(
                orig_buff as *mut libc::c_void,
                1i32 as libc::c_ulong,
                benchedSize,
                inFile,
            );
            fclose(inFile);
            if readSize != benchedSize {
                fprintf(
                    __stderrp,
                    b"\nError: problem reading file \'%s\' (%i read, should be %i) !!    \n\x00"
                        as *const u8 as *const libc::c_char,
                    inFileName,
                    readSize as libc::c_int,
                    benchedSize as libc::c_int,
                );
                free(orig_buff as *mut libc::c_void);
                free(compressedBuffer as *mut libc::c_void);
                return 13i32;
            }
            BMK_benchCore_Mem(
                compressedBuffer,
                orig_buff,
                benchedSize as libc::c_int as libc::c_uint,
                255i32 as libc::c_uint,
                BMK_tableLog as libc::c_uint,
                inFileName,
                &mut totalz,
                &mut totalc,
                &mut totald,
            );
            totals = (totals as libc::c_ulonglong).wrapping_add(benchedSize as libc::c_ulonglong)
                as U64 as U64;
            free(orig_buff as *mut libc::c_void);
            free(compressedBuffer as *mut libc::c_void);
        }
        if nbFiles > 1i32 {
            fprintf(
                __stderrp,
                b"%-16.16s :%10llu ->%10llu (%5.2f%%), %6.1f MB/s , %6.1f MB/s\n\x00" as *const u8
                    as *const libc::c_char,
                b"  TOTAL\x00" as *const u8 as *const libc::c_char,
                totals,
                totalz,
                totalz as libc::c_double / totals as libc::c_double * 100.0f64,
                totals as libc::c_double / totalc / 1000.0f64,
                totals as libc::c_double / totald / 1000.0f64,
            );
        }
        return 0i32;
    }
    /*-********************************************************************
    *  BenchCore
    **********************************************************************/
    unsafe extern "C" fn BMK_benchCore_Mem(
        mut dst: *mut libc::c_char,
        mut src: *mut libc::c_char,
        mut benchedSize: libc::c_uint,
        mut nbSymbols: libc::c_uint,
        mut tableLog: libc::c_uint,
        mut inFileName: *const libc::c_char,
        mut totalCompressedSize: *mut U64,
        mut totalCompressionTime: *mut libc::c_double,
        mut totalDecompressionTime: *mut libc::c_double,
    ) {
        use fse_rs::converted::*; // FIXME: Why do I need to reimport it here?
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
        crcOrig = XXH64(
            src as *const libc::c_void,
            benchedSize as size_t,
            0i32 as libc::c_ulonglong,
        );
        HIST_count(
            count.as_mut_ptr(),
            &mut nbSymbols,
            src as *mut BYTE as *const libc::c_void,
            benchedSize as size_t,
        );
        tableLog = FSE_normalizeCount(
            norm.as_mut_ptr(),
            tableLog,
            count.as_mut_ptr(),
            benchedSize as size_t,
            nbSymbols,
        ) as U32;
        ct = FSE_createCTable(tableLog, nbSymbols);
        FSE_buildCTable(ct, norm.as_mut_ptr(), nbSymbols, tableLog);
        dt = FSE_createDTable(tableLog);
        FSE_buildDTable(dt, norm.as_mut_ptr(), nbSymbols, tableLog);
        fprintf(
            __stderrp,
            b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
        loopNb = 1i32;
        while loopNb <= nbIterations {
            let mut nbLoops: libc::c_int = 0;
            let mut clockStart: clock_t = 0;
            let mut clockDuration: clock_t = 0;
            fprintf(
                __stderrp,
                b"%1i-%-14.14s : %9u ->\r\x00" as *const u8 as *const libc::c_char,
                loopNb,
                inFileName,
                benchedSize,
            );
            let mut i: libc::c_uint = 0;
            i = 0i32 as libc::c_uint;
            while i < benchedSize {
                *dst.offset(i as isize) = i as libc::c_char;
                i = i.wrapping_add(1)
            }
            nbLoops = 0i32;
            clockStart = clock();
            while clock() == clockStart {}
            clockStart = clock();
            while BMK_clockSpan(clockStart) < (1000000i32 * 2i32) as libc::c_ulong {
                cSize = FSE_compress_usingCTable(
                    dst as *mut libc::c_void,
                    FSE_compressBound(benchedSize as size_t),
                    src as *const libc::c_void,
                    benchedSize as size_t,
                    ct,
                );
                nbLoops += 1
            }
            clockDuration = BMK_clockSpan(clockStart);
            if 0 != FSE_isError(cSize) {
                fprintf(
                    __stderrp,
                    b"!!! Error compressing file %s !!!!    \n\x00" as *const u8
                        as *const libc::c_char,
                    inFileName,
                );
                break;
            } else {
                if (clockDuration as libc::c_double) < fastestC * nbLoops as libc::c_double {
                    fastestC = clockDuration as libc::c_double / nbLoops as libc::c_double
                }
                ratio = cSize as libc::c_double / benchedSize as libc::c_double * 100.0f64;
                fprintf(
                    __stderrp,
                    b"%1i-%-14.14s : %9i -> %9i (%5.2f%%),%7.1f MB/s\r\x00" as *const u8
                        as *const libc::c_char,
                    loopNb,
                    inFileName,
                    benchedSize as libc::c_int,
                    cSize as libc::c_int,
                    ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64,
                );
                let mut i_0: libc::c_uint = 0;
                i_0 = 0i32 as libc::c_uint;
                while i_0 < benchedSize {
                    *src.offset(i_0 as isize) = 0i32 as libc::c_char;
                    i_0 = i_0.wrapping_add(1)
                }
                nbLoops = 0i32;
                clockStart = clock();
                while clock() == clockStart {}
                clockStart = clock();
                while BMK_clockSpan(clockStart) < (1000000i32 * 2i32) as libc::c_ulong {
                    dSize = FSE_decompress_usingDTable(
                        src as *mut libc::c_void,
                        benchedSize as size_t,
                        dst as *const libc::c_void,
                        cSize,
                        dt,
                    );
                    nbLoops += 1
                }
                clockDuration = BMK_clockSpan(clockStart);
                if 0 != FSE_isError(dSize) {
                    fprintf(
                        __stderrp,
                        b"\n!!! Error decompressing file %s !!!!    \n\x00" as *const u8
                            as *const libc::c_char,
                        inFileName,
                    );
                    break;
                } else if dSize != benchedSize as libc::c_ulong {
                    fprintf(
                        __stderrp,
                        b"\n!!! Error decompressing file %s !!!!    \n\x00" as *const u8
                            as *const libc::c_char,
                        inFileName,
                    );
                    break;
                } else {
                    if (clockDuration as libc::c_double) < fastestD * nbLoops as libc::c_double {
                        fastestD = clockDuration as libc::c_double / nbLoops as libc::c_double
                    }
                    fprintf(
                        __stderrp,
                        b"%1i-%-14.14s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\r\x00"
                            as *const u8 as *const libc::c_char,
                        loopNb,
                        inFileName,
                        benchedSize as libc::c_int,
                        cSize as libc::c_int,
                        ratio,
                        benchedSize as libc::c_double / fastestC / 1000.0f64,
                        benchedSize as libc::c_double / fastestD / 1000.0f64,
                    );
                    crcCheck = XXH64(
                        src as *const libc::c_void,
                        benchedSize as size_t,
                        0i32 as libc::c_ulonglong,
                    );
                    if crcOrig != crcCheck {
                        fprintf(
                            __stderrp,
                            b"\n!!! WARNING !!! %14s : Invalid Checksum : %x != %x\n\x00"
                                as *const u8 as *const libc::c_char,
                            inFileName,
                            crcOrig as libc::c_uint,
                            crcCheck as libc::c_uint,
                        );
                        break;
                    } else {
                        loopNb += 1
                    }
                }
            }
        }
        if crcOrig == crcCheck {
            if ratio < 100.0f64 {
                fprintf(
                    __stderrp,
                    b"%-16.16s : %9i -> %9i (%5.2f%%),%7.1f MB/s ,%7.1f MB/s\n\x00" as *const u8
                        as *const libc::c_char,
                    inFileName,
                    benchedSize as libc::c_int,
                    cSize as libc::c_int,
                    ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64,
                    benchedSize as libc::c_double / fastestD / 1000.0f64,
                );
            } else {
                fprintf(
                    __stderrp,
                    b"%-16.16s : %9i -> %9i (%5.1f%%),%7.1f MB/s ,%7.1f MB/s \n\x00" as *const u8
                        as *const libc::c_char,
                    inFileName,
                    benchedSize as libc::c_int,
                    cSize as libc::c_int,
                    ratio,
                    benchedSize as libc::c_double / fastestC / 1000.0f64,
                    benchedSize as libc::c_double / fastestD / 1000.0f64,
                );
            }
        }
        *totalCompressedSize = (*totalCompressedSize as libc::c_ulonglong)
            .wrapping_add(cSize as libc::c_ulonglong) as U64 as U64;
        *totalCompressionTime += fastestC;
        *totalDecompressionTime += fastestD;
        free(ct as *mut libc::c_void);
        free(dt as *mut libc::c_void);
    }
    // Parameters
    #[no_mangle]
    pub unsafe extern "C" fn BMK_SetBlocksize(mut bsize: U32) {
        chunkSize = bsize;
        fprintf(
            __stderrp,
            b"- Blocks %u KB -\n\x00" as *const u8 as *const libc::c_char,
            chunkSize >> 10i32,
        );
    }
    #[no_mangle]
    pub unsafe extern "C" fn BMK_SetNbIterations(mut nbLoops: libc::c_int) {
        nbIterations = nbLoops;
        fprintf(
            __stderrp,
            b"- %i iterations -\n\x00" as *const u8 as *const libc::c_char,
            nbIterations,
        );
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
    #[cfg_attr(target_os = "linux", link_section = ".init_array")]
    #[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
    #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
    static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

}

mod io {
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
        fn ZLIBH_decompress(
            dest: *mut libc::c_char,
            compressed: *const libc::c_char,
        ) -> libc::c_int;
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
        let inputBlockSize: size_t =
            FIO_blockID_to_blockSize(g_blockSizeId as libc::c_int) as size_t;
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
            filesize = (filesize as libc::c_ulonglong).wrapping_add(inSize as libc::c_ulonglong)
                as U64 as U64;
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
                            b"packing compressed block, of size %zu, into %zu bytes \n\x00"
                                as *const u8 as *const libc::c_char,
                            inSize,
                            cSize,
                        );
                    }
                    if inSize == inputBlockSize {
                        *out_buff.offset(2isize) = (((bt_compressed as libc::c_int) << 6i32)
                            + 0x20i32) as BYTE
                            as libc::c_char;
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
                            (FIO_maxBlockHeaderSize as libc::c_ulong).wrapping_sub(headerSize)
                                as isize,
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
                            b"Warning : %s already exists\n\x00" as *const u8
                                as *const libc::c_char,
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
                        b"compressed with zlib\'s huffman \n\x00" as *const u8
                            as *const libc::c_char,
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
                        b"Wrong file type : unknown header\n\x00" as *const u8
                            as *const libc::c_char,
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
                    b"Wrong version : unknown header flags\n\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            if g_displayLevel >= 1i32 {
                fprintf(__stderrp, b"\n\x00" as *const u8 as *const libc::c_char);
            }
            exit(32i32);
        }
        blockSize = FIO_blockID_to_blockSize(blockSizeId as libc::c_int) as U32;
        in_buff =
            malloc(blockSize.wrapping_add(FIO_maxBlockHeaderSize) as libc::c_ulong) as *mut BYTE;
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
            let bType: libc::c_int =
                (*ip.offset(0isize) as libc::c_int & 0x80i32 + 0x40i32) >> 6i32;
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
                        + *in_buff.offset(1isize) as libc::c_int)
                        as size_t
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
            + ((*ip.offset(0isize) as libc::c_int & 0x3fi32) << 16i32))
            as U32;
        let CRCcalculated: U32 = XXH32_digest(&mut xxhState) >> 5i32
            & (1u32 << 22i32).wrapping_sub(1i32 as libc::c_uint);
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
}
