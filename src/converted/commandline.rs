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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
    fn BMK_benchFiles(fileNamesTable: *mut *const libc::c_char,
                      nbFiles: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn BMK_benchCore_Files(fileNamesTable: *mut *const libc::c_char,
                           nbFiles: libc::c_int) -> libc::c_int;
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
    fn FIO_compressFilename(outfilename: *const libc::c_char,
                            infilename: *const libc::c_char)
     -> libc::c_ulonglong;
    #[no_mangle]
    fn FIO_decompressFilename(outfilename: *const libc::c_char,
                              infilename: *const libc::c_char)
     -> libc::c_ulonglong;
    #[no_mangle]
    fn isatty(_: libc::c_int) -> libc::c_int;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
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
unsafe extern "C" fn usage(mut programName: *const libc::c_char)
 -> libc::c_int {
    fprintf(__stderrp, b"Usage :\n\x00" as *const u8 as *const libc::c_char);
    fprintf(__stderrp,
            b"%s [arg] inputFilename [outputFilename]\n\x00" as *const u8 as
                *const libc::c_char, programName);
    fprintf(__stderrp,
            b"Arguments :\n\x00" as *const u8 as *const libc::c_char);
    fprintf(__stderrp,
            b"(default): fse core loop timing tests\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(__stderrp,
            b" -e : use FSE (default)\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(__stderrp,
            b" -h : use HUF\n\x00" as *const u8 as *const libc::c_char);
    fprintf(__stderrp,
            b" -z : use zlib\'s huffman\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(__stderrp,
            b" -d : decompression (default for %s extension)\n\x00" as
                *const u8 as *const libc::c_char,
            b".fse\x00" as *const u8 as *const libc::c_char);
    fprintf(__stderrp,
            b" -b : benchmark mode\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(__stderrp,
            b" -i#: iteration loops [1-9](default : 4), benchmark mode only\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(__stderrp,
            b" -B#: block size (default : 32768), benchmark mode only\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(__stderrp,
            b" -H : display help and exit\n\x00" as *const u8 as
                *const libc::c_char);
    return 0i32;
}
unsafe extern "C" fn badusage(mut programName: *const libc::c_char)
 -> libc::c_int {
    if displayLevel >= 1i32 {
        fprintf(__stderrp,
                b"Incorrect parameters\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if displayLevel >= 1i32 { usage(programName); }
    exit(1i32);
}
unsafe extern "C" fn waitEnter() {
    let mut unused: libc::c_int = 0;
    fprintf(__stderrp,
            b"Press enter to continue...\n\x00" as *const u8 as
                *const libc::c_char);
    unused = getchar();
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char)
 -> libc::c_int {
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
    fprintf(__stderrp,
            b"%s, %i-bits demo by %s (%s)\n\x00" as *const u8 as
                *const libc::c_char,
            b"FSE : Finite State Entropy\x00" as *const u8 as
                *const libc::c_char,
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as
                libc::c_int * 8i32,
            b"Yann Collet\x00" as *const u8 as *const libc::c_char,
            b"Mar 29 2019\x00" as *const u8 as *const libc::c_char);
    if argc < 2i32 { badusage(programName); }
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
                    } else { output_filename = stdoutmark.as_ptr() }
                }
                while *argument.offset(1isize) as libc::c_int != 0i32 {
                    argument = argument.offset(1isize);
                    match *argument.offset(0isize) as libc::c_int {
                        86 => {
                            fprintf(__stderrp,
                                    b"%s, %i-bits demo by %s (%s)\n\x00" as
                                        *const u8 as *const libc::c_char,
                                    b"FSE : Finite State Entropy\x00" as
                                        *const u8 as *const libc::c_char,
                                    ::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong as libc::c_int *
                                        8i32,
                                    b"Yann Collet\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Mar 29 2019\x00" as *const u8 as
                                        *const libc::c_char);
                            return 0i32
                        }
                        72 => { usage(programName); return 0i32 }
                        100 => { decode = 1i32; bench = 0i32 }
                        98 => { bench = 1i32 }
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
                            output_filename =
                                b"/dev/null\x00" as *const u8 as
                                    *const libc::c_char
                        }
                        102 => { FIO_overwriteMode(); }
                        118 => { displayLevel += 1 }
                        113 => { displayLevel -= 1 }
                        107 => { }
                        66 => {
                            let mut bSize: libc::c_uint =
                                0i32 as libc::c_uint;
                            while *argument.offset(1isize) as libc::c_int >=
                                      '0' as i32 &&
                                      *argument.offset(1isize) as libc::c_int
                                          <= '9' as i32 {
                                let mut digit: libc::c_uint =
                                    (*argument.offset(1isize) as libc::c_int -
                                         '0' as i32) as libc::c_uint;
                                bSize =
                                    bSize.wrapping_mul(10i32 as libc::c_uint);
                                bSize = bSize.wrapping_add(digit);
                                argument = argument.offset(1isize)
                            }
                            if *argument.offset(1isize) as libc::c_int ==
                                   'K' as i32 {
                                bSize <<= 10i32;
                                argument = argument.offset(1isize)
                            }
                            if *argument.offset(1isize) as libc::c_int ==
                                   'M' as i32 {
                                bSize <<= 20i32;
                                argument = argument.offset(1isize)
                            }
                            if *argument.offset(1isize) as libc::c_int ==
                                   'B' as i32 {
                                argument = argument.offset(1isize)
                            }
                            BMK_SetBlocksize(bSize);
                        }
                        83 => { }
                        105 => {
                            // to be completed later
                            if *argument.offset(1isize) as libc::c_int >=
                                   '1' as i32 &&
                                   *argument.offset(1isize) as libc::c_int <=
                                       '9' as i32 {
                                let mut iters: libc::c_int =
                                    *argument.offset(1isize) as libc::c_int -
                                        '0' as i32;
                                BMK_SetNbIterations(iters);
                                argument = argument.offset(1isize)
                            }
                        }
                        112 => { fse_pause = 1i32 }
                        77 => {
                            if *argument.offset(1isize) as libc::c_int >=
                                   '1' as i32 &&
                                   *argument.offset(1isize) as libc::c_int <=
                                       '9' as i32 {
                                let mut tableLog: libc::c_int =
                                    *argument.offset(1isize) as libc::c_int -
                                        '0' as i32;
                                BMK_SetTableLog(tableLog);
                                argument = argument.offset(1isize)
                            }
                        }
                        _ => { badusage(programName); }
                    }
                }
            } else if input_filename.is_null() {
                input_filename = argument;
                indexFileNames = i
            } else if output_filename.is_null() { output_filename = argument }
        }
        i += 1
    }
    if input_filename.is_null() { input_filename = stdinmark.as_ptr() }
    if 0 == strcmp(input_filename, stdinmark.as_ptr()) &&
           0 != isatty(fileno(__stdinp)) {
        badusage(programName);
    }
    /* Check if benchmark is selected */
    if bench == 1i32 {
        BMK_benchFiles(argv.offset(indexFileNames as isize),
                       argc - indexFileNames);
    } else if bench == 3i32 {
        BMK_benchCore_Files(argv.offset(indexFileNames as isize),
                            argc - indexFileNames);
    } else {
        /* no longer possible */
        while output_filename.is_null() {
            if 0 == isatty(fileno(__stdoutp)) {
                output_filename = stdoutmark.as_ptr();
                // Default to stdout whenever possible (i.e. not a console)
                break ;
            } else {
                if 0 == decode && 0 == forceCompress {
                    let l: size_t = strlen(input_filename);
                    if 0 ==
                           strcmp(input_filename.offset(l.wrapping_sub(4i32 as
                                                                           libc::c_ulong)
                                                            as isize),
                                  b".fse\x00" as *const u8 as
                                      *const libc::c_char) {
                        decode = 1i32
                    }
                }
                if 0 == decode {
                    /* compression to file */
                    let l_0: size_t = strlen(input_filename);
                    if tmpFilenameSize <
                           l_0.wrapping_add(6i32 as libc::c_ulong) {
                        tmpFilenameSize =
                            l_0.wrapping_add(6i32 as libc::c_ulong)
                    }
                    tmpFilenameBuffer =
                        calloc(1i32 as libc::c_ulong, tmpFilenameSize) as
                            *mut libc::c_char;
                    if tmpFilenameBuffer.is_null() {
                        fprintf(__stderrp,
                                b"Not enough memory, exiting ... \n\x00" as
                                    *const u8 as *const libc::c_char);
                        exit(1i32);
                    }
                    strcpy(tmpFilenameBuffer, input_filename);
                    strcpy(tmpFilenameBuffer.offset(l_0 as isize),
                           b".fse\x00" as *const u8 as *const libc::c_char);
                    output_filename = tmpFilenameBuffer;
                    if displayLevel >= 2i32 {
                        fprintf(__stderrp,
                                b"Compressed filename will be : %s \n\x00" as
                                    *const u8 as *const libc::c_char,
                                output_filename);
                    }
                    break ;
                } else {
                    let mut outl: size_t = 0;
                    let inl: size_t = strlen(input_filename);
                    if tmpFilenameSize <
                           inl.wrapping_add(2i32 as libc::c_ulong) {
                        tmpFilenameSize =
                            inl.wrapping_add(2i32 as libc::c_ulong)
                    }
                    tmpFilenameBuffer =
                        calloc(1i32 as libc::c_ulong, tmpFilenameSize) as
                            *mut libc::c_char;
                    strcpy(tmpFilenameBuffer, input_filename);
                    outl = inl;
                    if inl > 4i32 as libc::c_ulong {
                        while outl >= inl.wrapping_sub(4i32 as libc::c_ulong)
                                  &&
                                  *input_filename.offset(outl as isize) as
                                      libc::c_int ==
                                      extension[outl.wrapping_sub(inl).wrapping_add(4i32
                                                                                        as
                                                                                        libc::c_ulong)
                                                    as usize] as libc::c_int {
                            let fresh0 = outl;
                            outl = outl.wrapping_sub(1);
                            *tmpFilenameBuffer.offset(fresh0 as isize) =
                                0i32 as libc::c_char
                        }
                    }
                    if outl != inl.wrapping_sub(5i32 as libc::c_ulong) {
                        if displayLevel >= 1i32 {
                            fprintf(__stderrp,
                                    b"Cannot determine an output filename\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                        badusage(programName);
                    }
                    output_filename = tmpFilenameBuffer;
                    if displayLevel >= 2i32 {
                        fprintf(__stderrp,
                                b"Decoding into filename : %s \n\x00" as
                                    *const u8 as *const libc::c_char,
                                output_filename);
                    }
                }
            }
        }
        if 0 == strcmp(input_filename, stdinmark.as_ptr()) &&
               0 == strcmp(output_filename, stdoutmark.as_ptr()) &&
               displayLevel == 2i32 {
            displayLevel = 1i32
        }
        if 0 == strcmp(input_filename, stdinmark.as_ptr()) &&
               0 != isatty(fileno(__stdinp)) {
            badusage(programName);
        }
        if 0 == strcmp(output_filename, stdoutmark.as_ptr()) &&
               0 != isatty(fileno(__stdoutp)) {
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
    if 0 != fse_pause { waitEnter(); }
    free(tmpFilenameBuffer as *mut libc::c_void);
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