#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_slice_as_ptr, libc, ptr_wrapping_offset_from)]
extern crate libc;
pub type ct_data = ct_data_s;
/* ******************************************************************
ZLIBH : Zlib based Huffman coder
Copyright (C) 1995-2012 Jean-loup Gailly
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
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF Sunsigned char DAMAGE.

You can contact the author at :
- Public forum : https://groups.google.com/forum/#!forum/lz4c
****************************************************************** */
/* ***************************************************************
*  Tuning parameters
****************************************************************/
// MEMORY_USAGE :
// Memory usage formula : N->2^N Bytes (examples : 10 -> 1KB; 12 -> 4KB ; 16 -> 64KB; 20 -> 1MB; etc.)
// Increasing memory usage improves compression ratio
// Reduced memory usage can improve speed, due to cache effect
// Default value is 14, for 16KB, which nicely fits into Intel x86 L1 cache
// ZLIBH_ILP
// Instruction Level Parallelism : improve performance on modern CPU featuring multiple ALU and OoO capabilities
// ZLIBH_DEBUG
// Enable verification code, which checks table construction and state values (munsigned char slower, for debug purpose only)
/* ***************************************************************
*  Includes
****************************************************************/
// memcpy, memset
// printf (debug)
// Visual Studio
// GCC
/*
* Maximums for allocations and loops.  It is not useful to change these --
* they are fixed by the deflate format.
*/
/* maximum bits in a code */
/* maximum bits in a length code code */
/* maximum codes lengths to read */
/* number of fixed literal/length codes */
/* number of literal bytes 0..255 */
/* End of Block code */
/* number of literal including the END_BLOCK code */
/* number of codes used to transfer the bit lengths */
/* maximum heap size */
/* repeat previous bit length 3-6 times (2 bits of repeat count) */
/* repeat a zero length 3-10 times  (3 bits of repeat count) */
/* repeat a zero length 11-138 times  (7 bits of repeat count) */
/* Data structure describing a single value and its code string. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ct_data_s {
    pub fc: unnamed_0,
    pub dl: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    pub dad: libc::c_ushort,
    pub len: libc::c_ushort,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    pub freq: libc::c_ushort,
    pub code: libc::c_ushort,
}
pub type tree_desc = tree_desc_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tree_desc_s {
    pub dyn_tree: *mut ct_data,
    pub max_code: libc::c_int,
    pub comp_size: *mut libc::c_ulong,
    pub stat_desc: *mut static_tree_desc,
}
pub type static_tree_desc = static_tree_desc_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct static_tree_desc_s {
    pub static_tree: *const ct_data,
    pub extra_bits: *const libc::c_int,
    pub extra_base: libc::c_int,
    pub elems: libc::c_int,
    pub max_length: libc::c_int,
}
pub type U32 = uint32_t;
pub type uint32_t = libc::c_uint;
/* got a data error -- remain here until reset */
pub const BAD: inflate_mode = 4;
/* finished check, done -- remain here until reset */
pub const DONE: inflate_mode = 3;
//****************************
// Decompression CODE
//****************************
/* Possible inflate modes between inflate() calls */
pub type inflate_mode = libc::c_uint;
/* i: waiting for length/lit/eob code */
pub const LEN: inflate_mode = 2;
/* i: waiting for dynamic block table lengths */
pub const TABLE: inflate_mode = 1;
/* i: same, but skip check to exit inflate on new block */
pub const TYPEDO: inflate_mode = 0;
/* state maintained between inflate() calls.  Approximately 10K bytes. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct inflate_state {
    pub mode: inflate_mode,
    pub last: libc::c_int,
    pub wrap: libc::c_int,
    pub havedict: libc::c_int,
    pub flags: libc::c_int,
    pub dmax: libc::c_uint,
    pub check: libc::c_ulong,
    pub total: libc::c_ulong,
    pub wbits: libc::c_uint,
    pub wsize: libc::c_uint,
    pub whave: libc::c_uint,
    pub wnext: libc::c_uint,
    pub window: *mut libc::c_uchar,
    pub hold: libc::c_ulong,
    pub bits: libc::c_uint,
    pub length: libc::c_uint,
    pub offset: libc::c_uint,
    pub extra: libc::c_uint,
    pub lencode: *const code,
    pub distcode: *const code,
    pub lenbits: libc::c_uint,
    pub distbits: libc::c_uint,
    pub ncode: libc::c_uint,
    pub nlen: libc::c_uint,
    pub ndist: libc::c_uint,
    pub have: libc::c_uint,
    pub next: *mut code,
    pub lens: [libc::c_ushort; 320],
    pub work: [libc::c_ushort; 288],
    pub codes: [code; 1444],
    pub sane: libc::c_int,
    pub back: libc::c_int,
    pub was: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct code {
    pub op: libc::c_uchar,
    pub bits: libc::c_uchar,
    pub val: libc::c_ushort,
}
/* op values as set by inflate_table():
00000000 - literal
0000tttt - table link, tttt != 0 is the number of table index bits
0001eeee - length or distance, eeee is the number of extra bits
01100000 - end of block
01000000 - invalid code
*/
/* Maximum size of the dynamic table.  The maximum number of code structures is
1444, which is the sum of 852 for literal/length codes and 592 for distance
codes.  These values were found by exhaustive searches using the program
examples/enough.c found in the zlib distribtution.  The arguments to that
program are the number of symbols, the initial root table size, and the
maximum bit length of a code.  "enough 286 9 15" for literal/length codes
returns returns 852, and "enough 30 6 15" for distance codes returns 592.
The initial root table size (9 or 6) is found in the fifth argument of the
inflate_table() calls in inflate.c and infback.c.  If the root table size is
changed, then these maximum sizes would be need to be recalculated and
updated. */
/* Type of code to build for inflate_table() */
pub type codetype = libc::c_uint;
pub const DISTS: codetype = 2;
pub const LENS: codetype = 1;
pub const CODES: codetype = 0;
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
pub unsafe extern "C" fn ZLIBH_compress(mut dest: *mut libc::c_char,
                                        mut source: *const libc::c_char,
                                        mut inputSize: libc::c_int)
 -> libc::c_int {
    let mut ip: *const libc::c_uchar = source as *const libc::c_uchar;
    let bsourceend: *const libc::c_uchar = ip.offset(inputSize as isize);
    let mut bsource: *const libc::c_uchar = source as *const libc::c_uchar;
    let mut op: *mut libc::c_uchar = dest as *mut libc::c_uchar;
    let mut ltree: tree_desc =
        tree_desc_s{dyn_tree: 0 as *mut ct_data,
                    max_code: 0,
                    comp_size: 0 as *mut libc::c_ulong,
                    stat_desc: 0 as *mut static_tree_desc,};
    let mut dyn_ltree: [ct_data; 515] =
        [ct_data_s{fc: unnamed_0{freq: 0,}, dl: unnamed{dad: 0,},}; 515];
    let mut ldata_compsize: [libc::c_ulong; 2] = [0i32 as libc::c_ulong, 0];
    let mut bltree: tree_desc =
        tree_desc_s{dyn_tree: 0 as *mut ct_data,
                    max_code: 0,
                    comp_size: 0 as *mut libc::c_ulong,
                    stat_desc: 0 as *mut static_tree_desc,};
    let mut dyn_bltree: [ct_data; 39] =
        [ct_data_s{fc: unnamed_0{freq: 0,}, dl: unnamed{dad: 0,},}; 39];
    let mut bldata_compsize: [libc::c_ulong; 2] = [0i32 as libc::c_ulong, 0];
    let mut symbol: libc::c_int = 0;
    let mut max_blindex: libc::c_int = 0;
    let mut compressed_size: libc::c_int = 0;
    let mut freq_l: [U32; 257] =
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
         0, 0, 0, 0, 0, 0, 0];
    ltree.dyn_tree = dyn_ltree.as_mut_ptr();
    ltree.comp_size = ldata_compsize.as_mut_ptr();
    ltree.max_code = 257i32;
    ltree.stat_desc = &mut static_l_desc;
    bltree.dyn_tree = dyn_bltree.as_mut_ptr();
    bltree.comp_size = bldata_compsize.as_mut_ptr();
    bltree.max_code = 19i32;
    bltree.stat_desc = &mut static_bl_desc;
    while bsource < bsourceend {
        let fresh0 = bsource;
        bsource = bsource.offset(1);
        freq_l[*fresh0 as usize] = freq_l[*fresh0 as usize].wrapping_add(1)
    }
    freq_l[256usize] = 1i32 as U32;
    symbol = 0i32;
    loop  {
        dyn_ltree[symbol as usize].fc.freq =
            freq_l[symbol as usize] as libc::c_ushort;
        symbol += 1;
        if !(symbol < 257i32) { break ; }
    }
    build_tree(&mut ltree);
    feed_bltree(&mut ltree, &mut bltree);
    build_tree(&mut bltree);
    max_blindex = 19i32 - 1i32;
    while max_blindex >= 3i32 {
        if dyn_bltree[bl_order[max_blindex as usize] as usize].dl.len as
               libc::c_int != 0i32 {
            break ;
        }
        max_blindex -= 1
    }
    bldata_compsize[0usize] =
        bldata_compsize[0usize].wrapping_add((3i32 * (max_blindex + 1i32) +
                                                  4i32) as libc::c_ulong);
    if bldata_compsize[0usize].wrapping_add(ldata_compsize[0usize]) <
           ldata_compsize[1usize] {
        *op = (max_blindex + 1i32) as libc::c_uchar;
        ZLIBH_compress_block(ip, op, dyn_ltree.as_mut_ptr(),
                             dyn_bltree.as_mut_ptr(),
                             inputSize as libc::c_uint);
        compressed_size =
            (bldata_compsize[0usize].wrapping_add(ldata_compsize[0usize]).wrapping_add(8i32
                                                                                           as
                                                                                           libc::c_ulong)
                 >> 3i32) as libc::c_int
    } else {
        ZLIBH_compress_block(ip, op, static_ltree.as_ptr(),
                             dyn_bltree.as_mut_ptr(),
                             inputSize as libc::c_uint);
        compressed_size =
            (ldata_compsize[1usize].wrapping_add(8i32 as libc::c_ulong) >>
                 3i32) as libc::c_int
    }
    return compressed_size;
}
static mut static_ltree: [ct_data; 288] =
    [ct_data_s{fc: unnamed_0{freq: 12i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 140i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 76i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 204i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 44i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 172i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 108i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 236i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 28i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 156i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 92i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 220i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 60i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 188i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 124i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 252i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 2i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 130i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 66i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 194i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 34i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 162i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 98i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 226i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 18i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 146i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 82i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 210i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 50i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 178i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 114i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 242i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 10i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 138i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 74i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 202i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 42i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 170i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 106i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 234i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 26i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 154i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 90i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 218i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 58i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 186i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 122i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 250i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 6i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 134i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 70i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 198i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 38i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 166i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 102i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 230i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 22i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 150i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 86i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 214i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 54i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 182i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 118i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 246i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 14i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 142i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 78i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 206i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 46i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 174i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 110i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 238i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 30i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 158i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 94i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 222i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 62i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 190i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 126i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 254i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 1i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 129i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 65i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 193i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 33i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 161i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 97i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 225i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 17i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 145i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 81i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 209i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 49i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 177i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 113i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 241i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 9i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 137i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 73i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 201i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 41i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 169i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 105i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 233i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 25i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 153i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 89i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 217i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 57i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 185i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 121i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 249i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 5i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 133i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 69i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 197i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 37i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 165i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 101i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 229i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 21i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 149i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 85i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 213i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 53i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 181i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 117i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 245i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 13i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 141i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 77i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 205i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 45i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 173i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 109i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 237i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 29i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 157i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 93i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 221i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 61i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 189i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 125i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 253i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 19i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 275i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 147i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 403i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 83i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 339i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 211i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 467i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 51i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 307i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 179i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 435i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 115i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 371i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 243i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 499i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 11i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 267i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 139i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 395i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 75i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 331i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 203i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 459i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 43i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 299i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 171i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 427i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 107i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 363i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 235i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 491i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 27i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 283i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 155i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 411i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 91i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 347i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 219i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 475i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 59i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 315i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 187i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 443i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 123i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 379i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 251i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 507i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 7i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 263i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 135i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 391i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 71i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 327i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 199i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 455i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 39i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 295i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 167i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 423i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 103i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 359i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 231i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 487i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 23i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 279i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 151i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 407i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 87i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 343i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 215i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 471i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 55i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 311i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 183i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 439i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 119i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 375i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 247i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 503i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 15i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 271i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 143i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 399i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 79i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 335i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 207i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 463i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 47i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 303i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 175i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 431i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 111i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 367i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 239i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 495i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 31i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 287i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 159i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 415i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 95i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 351i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 223i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 479i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 63i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 319i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 191i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 447i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 127i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 383i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 255i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 511i32 as libc::c_ushort,},
               dl: unnamed{dad: 9i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 0i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 64i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 32i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 96i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 16i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 80i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 48i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 112i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 8i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 72i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 40i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 104i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 24i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 88i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 56i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 120i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 4i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 68i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 36i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 100i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 20i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 84i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 52i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 116i32 as libc::c_ushort,},
               dl: unnamed{dad: 7i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 3i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 131i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 67i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 195i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 35i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 163i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 99i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},},
     ct_data_s{fc: unnamed_0{freq: 227i32 as libc::c_ushort,},
               dl: unnamed{dad: 8i32 as libc::c_ushort,},}];
/* ***************************************************************
*  Constants
****************************************************************/
/* ***************************************************************
*  Compiler specifics
****************************************************************/
// Visual Studio
/* ***************************
*  ZLIBH Compression Code
****************************/
/* If not enough room in bi_buf, use (valid) bits from bi_buf and
* (16 - bi_valid) bits from value, leaving (width - (16-bi_valid))
* unused bits in value.
*/
/* ===========================================================================
* Send the block data compressed using the given Huffman trees
*/
unsafe extern "C" fn ZLIBH_compress_block(mut ip: *const libc::c_uchar,
                                          mut op: *mut libc::c_uchar,
                                          mut ltree: *const ct_data,
                                          mut bltree: *const ct_data,
                                          mut ip_len: libc::c_uint) {
    /* bit buffer */
    let mut bi_buf: libc::c_uint = 0;
    /* bits used in bit_buf */
    let mut bi_valid: libc::c_uint = 0;
    /* running index in l_buf */
    let mut lx: libc::c_uint = 0i32 as libc::c_uint;
    /* value to send */
    let mut value: libc::c_uint = 0;
    /* number of bits to send */
    let mut length: libc::c_ushort = 0;
    if ltree != static_ltree.as_ptr() {
        let mut prevlen: libc::c_int = -1i32;
        let mut nextlen: libc::c_int =
            (*ltree.offset(0isize)).dl.len as libc::c_int;
        let mut count: libc::c_int = 0i32;
        let mut max_count: libc::c_int = 7i32;
        let mut min_count: libc::c_int = 4i32;
        let mut max_code: libc::c_uint = 256i32 as libc::c_uint;
        let mut blcodes: libc::c_uint = 0;
        let mut n: libc::c_uint = 0;
        bi_valid = 5i32 as libc::c_uint;
        blcodes = *op.offset(0isize) as libc::c_uint;
        bi_buf = blcodes.wrapping_sub(4i32 as libc::c_uint) << 1i32;
        length = 3i32 as libc::c_ushort;
        n = 0i32 as libc::c_uint;
        while n < blcodes {
            value =
                (*bltree.offset(bl_order[n as usize] as isize)).dl.len as
                    libc::c_uint;
            if bi_valid > 16u32.wrapping_sub(length as libc::c_uint) {
                bi_buf |= value << bi_valid;
                let fresh1 = op;
                op = op.offset(1);
                *fresh1 = (bi_buf & 0xffi32 as libc::c_uint) as libc::c_uchar;
                let fresh2 = op;
                op = op.offset(1);
                *fresh2 = (bi_buf >> 8i32) as libc::c_uchar;
                bi_buf =
                    value >> (16i32 as libc::c_uint).wrapping_sub(bi_valid);
                bi_valid =
                    bi_valid.wrapping_add((length as libc::c_int - 16i32) as
                                              libc::c_uint)
            } else {
                bi_buf |= value << bi_valid;
                bi_valid = bi_valid.wrapping_add(length as libc::c_uint)
            }
            n = n.wrapping_add(1)
        }
        if nextlen == 0i32 { max_count = 138i32; min_count = 3i32 }
        n = 0i32 as libc::c_uint;
        while n <= max_code {
            /* length of current code */
            let mut curlen: libc::c_int = 0;
            curlen = nextlen;
            nextlen =
                (*ltree.offset(n.wrapping_add(1i32 as libc::c_uint) as
                                   isize)).dl.len as libc::c_int;
            count += 1;
            if !(count < max_count && curlen == nextlen) {
                if count < min_count {
                    loop  {
                        value =
                            (*bltree.offset(curlen as isize)).fc.code as
                                libc::c_uint;
                        length = (*bltree.offset(curlen as isize)).dl.len;
                        if bi_valid >
                               16u32.wrapping_sub(length as libc::c_uint) {
                            bi_buf |= value << bi_valid;
                            let fresh3 = op;
                            op = op.offset(1);
                            *fresh3 =
                                (bi_buf & 0xffi32 as libc::c_uint) as
                                    libc::c_uchar;
                            let fresh4 = op;
                            op = op.offset(1);
                            *fresh4 = (bi_buf >> 8i32) as libc::c_uchar;
                            bi_buf =
                                value >>
                                    (16i32 as
                                         libc::c_uint).wrapping_sub(bi_valid);
                            bi_valid =
                                bi_valid.wrapping_add((length as libc::c_int -
                                                           16i32) as
                                                          libc::c_uint)
                        } else {
                            bi_buf |= value << bi_valid;
                            bi_valid =
                                bi_valid.wrapping_add(length as libc::c_uint)
                        }
                        count -= 1;
                        if !(count != 0i32) { break ; }
                    }
                } else if curlen != 0i32 {
                    if curlen != prevlen {
                        value =
                            (*bltree.offset(curlen as isize)).fc.code as
                                libc::c_uint;
                        length = (*bltree.offset(curlen as isize)).dl.len;
                        if bi_valid >
                               16u32.wrapping_sub(length as libc::c_uint) {
                            bi_buf |= value << bi_valid;
                            let fresh5 = op;
                            op = op.offset(1);
                            *fresh5 =
                                (bi_buf & 0xffi32 as libc::c_uint) as
                                    libc::c_uchar;
                            let fresh6 = op;
                            op = op.offset(1);
                            *fresh6 = (bi_buf >> 8i32) as libc::c_uchar;
                            bi_buf =
                                value >>
                                    (16i32 as
                                         libc::c_uint).wrapping_sub(bi_valid);
                            bi_valid =
                                bi_valid.wrapping_add((length as libc::c_int -
                                                           16i32) as
                                                          libc::c_uint)
                        } else {
                            bi_buf |= value << bi_valid;
                            bi_valid =
                                bi_valid.wrapping_add(length as libc::c_uint)
                        }
                        count -= 1
                    }
                    value = (*bltree.offset(16isize)).fc.code as libc::c_uint;
                    length = (*bltree.offset(16isize)).dl.len;
                    if bi_valid > 16u32.wrapping_sub(length as libc::c_uint) {
                        bi_buf |= value << bi_valid;
                        let fresh7 = op;
                        op = op.offset(1);
                        *fresh7 =
                            (bi_buf & 0xffi32 as libc::c_uint) as
                                libc::c_uchar;
                        let fresh8 = op;
                        op = op.offset(1);
                        *fresh8 = (bi_buf >> 8i32) as libc::c_uchar;
                        bi_buf =
                            value >>
                                (16i32 as
                                     libc::c_uint).wrapping_sub(bi_valid);
                        bi_valid =
                            bi_valid.wrapping_add((length as libc::c_int -
                                                       16i32) as libc::c_uint)
                    } else {
                        bi_buf |= value << bi_valid;
                        bi_valid =
                            bi_valid.wrapping_add(length as libc::c_uint)
                    }
                    length = 2i32 as libc::c_ushort;
                    value = (count - 3i32) as libc::c_uint;
                    if bi_valid > 16u32.wrapping_sub(length as libc::c_uint) {
                        bi_buf |= value << bi_valid;
                        let fresh9 = op;
                        op = op.offset(1);
                        *fresh9 =
                            (bi_buf & 0xffi32 as libc::c_uint) as
                                libc::c_uchar;
                        let fresh10 = op;
                        op = op.offset(1);
                        *fresh10 = (bi_buf >> 8i32) as libc::c_uchar;
                        bi_buf =
                            value >>
                                (16i32 as
                                     libc::c_uint).wrapping_sub(bi_valid);
                        bi_valid =
                            bi_valid.wrapping_add((length as libc::c_int -
                                                       16i32) as libc::c_uint)
                    } else {
                        bi_buf |= value << bi_valid;
                        bi_valid =
                            bi_valid.wrapping_add(length as libc::c_uint)
                    }
                } else if count < 11i32 {
                    value = (*bltree.offset(17isize)).fc.code as libc::c_uint;
                    length = (*bltree.offset(17isize)).dl.len;
                    if bi_valid > 16u32.wrapping_sub(length as libc::c_uint) {
                        bi_buf |= value << bi_valid;
                        let fresh11 = op;
                        op = op.offset(1);
                        *fresh11 =
                            (bi_buf & 0xffi32 as libc::c_uint) as
                                libc::c_uchar;
                        let fresh12 = op;
                        op = op.offset(1);
                        *fresh12 = (bi_buf >> 8i32) as libc::c_uchar;
                        bi_buf =
                            value >>
                                (16i32 as
                                     libc::c_uint).wrapping_sub(bi_valid);
                        bi_valid =
                            bi_valid.wrapping_add((length as libc::c_int -
                                                       16i32) as libc::c_uint)
                    } else {
                        bi_buf |= value << bi_valid;
                        bi_valid =
                            bi_valid.wrapping_add(length as libc::c_uint)
                    }
                    length = 3i32 as libc::c_ushort;
                    value = (count - 3i32) as libc::c_uint;
                    if bi_valid > 16u32.wrapping_sub(length as libc::c_uint) {
                        bi_buf |= value << bi_valid;
                        let fresh13 = op;
                        op = op.offset(1);
                        *fresh13 =
                            (bi_buf & 0xffi32 as libc::c_uint) as
                                libc::c_uchar;
                        let fresh14 = op;
                        op = op.offset(1);
                        *fresh14 = (bi_buf >> 8i32) as libc::c_uchar;
                        bi_buf =
                            value >>
                                (16i32 as
                                     libc::c_uint).wrapping_sub(bi_valid);
                        bi_valid =
                            bi_valid.wrapping_add((length as libc::c_int -
                                                       16i32) as libc::c_uint)
                    } else {
                        bi_buf |= value << bi_valid;
                        bi_valid =
                            bi_valid.wrapping_add(length as libc::c_uint)
                    }
                } else {
                    value = (*bltree.offset(18isize)).fc.code as libc::c_uint;
                    length = (*bltree.offset(18isize)).dl.len;
                    if bi_valid > 16u32.wrapping_sub(length as libc::c_uint) {
                        bi_buf |= value << bi_valid;
                        let fresh15 = op;
                        op = op.offset(1);
                        *fresh15 =
                            (bi_buf & 0xffi32 as libc::c_uint) as
                                libc::c_uchar;
                        let fresh16 = op;
                        op = op.offset(1);
                        *fresh16 = (bi_buf >> 8i32) as libc::c_uchar;
                        bi_buf =
                            value >>
                                (16i32 as
                                     libc::c_uint).wrapping_sub(bi_valid);
                        bi_valid =
                            bi_valid.wrapping_add((length as libc::c_int -
                                                       16i32) as libc::c_uint)
                    } else {
                        bi_buf |= value << bi_valid;
                        bi_valid =
                            bi_valid.wrapping_add(length as libc::c_uint)
                    }
                    length = 7i32 as libc::c_ushort;
                    value = (count - 11i32) as libc::c_uint;
                    if bi_valid > 16u32.wrapping_sub(length as libc::c_uint) {
                        bi_buf |= value << bi_valid;
                        let fresh17 = op;
                        op = op.offset(1);
                        *fresh17 =
                            (bi_buf & 0xffi32 as libc::c_uint) as
                                libc::c_uchar;
                        let fresh18 = op;
                        op = op.offset(1);
                        *fresh18 = (bi_buf >> 8i32) as libc::c_uchar;
                        bi_buf =
                            value >>
                                (16i32 as
                                     libc::c_uint).wrapping_sub(bi_valid);
                        bi_valid =
                            bi_valid.wrapping_add((length as libc::c_int -
                                                       16i32) as libc::c_uint)
                    } else {
                        bi_buf |= value << bi_valid;
                        bi_valid =
                            bi_valid.wrapping_add(length as libc::c_uint)
                    }
                }
                count = 0i32;
                prevlen = curlen;
                if nextlen == 0i32 {
                    max_count = 138i32;
                    min_count = 3i32
                } else if curlen == nextlen {
                    max_count = 6i32;
                    min_count = 3i32
                } else { max_count = 7i32; min_count = 4i32 }
            }
            n = n.wrapping_add(1)
        }
    } else { bi_valid = 1i32 as libc::c_uint; bi_buf = 1i32 as libc::c_uint }
    lx = 1i32 as libc::c_uint;
    loop  {
        let mut t_index: libc::c_uint = 0;
        let fresh19 = ip;
        ip = ip.offset(1);
        t_index = *fresh19 as libc::c_uint;
        value = (*ltree.offset(t_index as isize)).fc.code as libc::c_uint;
        length = (*ltree.offset(t_index as isize)).dl.len;
        if bi_valid > 16u32.wrapping_sub(length as libc::c_uint) {
            bi_buf |= value << bi_valid;
            let fresh20 = op;
            op = op.offset(1);
            *fresh20 = (bi_buf & 0xffi32 as libc::c_uint) as libc::c_uchar;
            let fresh21 = op;
            op = op.offset(1);
            *fresh21 = (bi_buf >> 8i32) as libc::c_uchar;
            bi_buf = value >> (16i32 as libc::c_uint).wrapping_sub(bi_valid);
            bi_valid =
                bi_valid.wrapping_add((length as libc::c_int - 16i32) as
                                          libc::c_uint)
        } else {
            bi_buf |= value << bi_valid;
            bi_valid = bi_valid.wrapping_add(length as libc::c_uint)
        }
        let fresh22 = lx;
        lx = lx.wrapping_add(1);
        if !(fresh22 < ip_len) { break ; }
    }
    value = (*ltree.offset(256isize)).fc.code as libc::c_uint;
    length = (*ltree.offset(256isize)).dl.len;
    if bi_valid > 16u32.wrapping_sub(length as libc::c_uint) {
        bi_buf |= value << bi_valid;
        let fresh23 = op;
        op = op.offset(1);
        *fresh23 = (bi_buf & 0xffi32 as libc::c_uint) as libc::c_uchar;
        let fresh24 = op;
        op = op.offset(1);
        *fresh24 = (bi_buf >> 8i32) as libc::c_uchar;
        bi_buf = value >> (16i32 as libc::c_uint).wrapping_sub(bi_valid);
        bi_valid =
            bi_valid.wrapping_add((length as libc::c_int - 16i32) as
                                      libc::c_uint)
    } else {
        bi_buf |= value << bi_valid;
        bi_valid = bi_valid.wrapping_add(length as libc::c_uint)
    }
    if bi_valid > 8i32 as libc::c_uint {
        let fresh25 = op;
        op = op.offset(1);
        *fresh25 = (bi_buf & 0xffi32 as libc::c_uint) as libc::c_uchar;
        let fresh26 = op;
        op = op.offset(1);
        *fresh26 = (bi_buf >> 8i32) as libc::c_uchar
    } else if bi_valid > 0i32 as libc::c_uint {
        let fresh27 = op;
        op = op.offset(1);
        *fresh27 = bi_buf as libc::c_uchar
    };
}
static mut bl_order: [libc::c_uchar; 19] =
    [16i32 as libc::c_uchar, 17i32 as libc::c_uchar, 18i32 as libc::c_uchar,
     0i32 as libc::c_uchar, 8i32 as libc::c_uchar, 7i32 as libc::c_uchar,
     9i32 as libc::c_uchar, 6i32 as libc::c_uchar, 10i32 as libc::c_uchar,
     5i32 as libc::c_uchar, 11i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     12i32 as libc::c_uchar, 3i32 as libc::c_uchar, 13i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 14i32 as libc::c_uchar, 1i32 as libc::c_uchar,
     15i32 as libc::c_uchar];
/* ===========================================================================
* Construct one Huffman tree and assigns the code bit strings and lengths.
* Update the total bit length for the current block.
* IN assertion: the field freq is set for all tree elements.
* OUT assertions: the fields len and code are set to the optimal bit length
*     and corresponding code. The length os_len[0] is updated; os_len[1] is
*     also updated if stree is not null. The field max_code is set.
*/
unsafe extern "C" fn build_tree(mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut elems: libc::c_int = (*desc).max_code;
    let mut csize: *mut libc::c_ulong = (*desc).comp_size;
    /* iterate over heap elements */
    let mut n: libc::c_int = 0;
    /* largest code with non zero frequency */
    let mut max_code: libc::c_int = -1i32;
    /* heap used to build the Huffman trees */
    let mut huf_heap: [libc::c_int; 515] = [0; 515];
    /* element of largest frequency */
    let mut heap_max: libc::c_int = 0;
    let mut depth: [libc::c_uchar; 515] = [0; 515];
    huf_heap[0usize] = 0i32;
    heap_max = 2i32 * 257i32 + 1i32;
    n = 0i32;
    while n < 2i32 * 257i32 + 1i32 {
        depth[n as usize] = 0i32 as libc::c_uchar;
        n += 1
    }
    n = 0i32;
    while n < elems {
        if (*tree.offset(n as isize)).fc.freq as libc::c_int != 0i32 {
            huf_heap[0usize] += 1;
            max_code = n;
            huf_heap[huf_heap[0usize] as usize] = max_code;
            depth[n as usize] = 0i32 as libc::c_uchar
        } else { (*tree.offset(n as isize)).dl.len = 0i32 as libc::c_ushort }
        n += 1
    }
    if huf_heap[0usize] > 1i32 {
        let mut node: libc::c_int = 0;
        let mut bl_count: [libc::c_ushort; 16] = [0; 16];
        (*desc).max_code = max_code;
        n = huf_heap[0usize] / 2i32;
        while n >= 1i32 {
            pqdownheap(tree, huf_heap.as_mut_ptr(), depth.as_mut_ptr(), n);
            n -= 1
        }
        node = elems;
        loop  {
            let mut m: libc::c_int = 0;
            n = huf_heap[1usize];
            let fresh28 = huf_heap[0usize];
            huf_heap[0usize] = huf_heap[0usize] - 1;
            huf_heap[1usize] = huf_heap[fresh28 as usize];
            pqdownheap(tree, huf_heap.as_mut_ptr(), depth.as_mut_ptr(), 1i32);
            m = huf_heap[1usize];
            heap_max -= 1;
            huf_heap[heap_max as usize] = n;
            heap_max -= 1;
            huf_heap[heap_max as usize] = m;
            (*tree.offset(node as isize)).fc.freq =
                ((*tree.offset(n as isize)).fc.freq as libc::c_int +
                     (*tree.offset(m as isize)).fc.freq as libc::c_int) as
                    libc::c_ushort;
            depth[node as usize] =
                (if depth[n as usize] as libc::c_int >=
                        depth[m as usize] as libc::c_int {
                     depth[n as usize] as libc::c_int
                 } else { depth[m as usize] as libc::c_int } + 1i32) as
                    libc::c_uchar;
            let ref mut fresh29 = (*tree.offset(m as isize)).dl.dad;
            *fresh29 = node as libc::c_ushort;
            (*tree.offset(n as isize)).dl.dad = *fresh29;
            let fresh30 = node;
            node = node + 1;
            huf_heap[1usize] = fresh30;
            pqdownheap(tree, huf_heap.as_mut_ptr(), depth.as_mut_ptr(), 1i32);
            if !(huf_heap[0usize] >= 2i32) { break ; }
        }
        heap_max -= 1;
        huf_heap[heap_max as usize] = huf_heap[1usize];
        gen_bitlen(desc, huf_heap.as_mut_ptr(), heap_max,
                   bl_count.as_mut_ptr());
        gen_codes(tree, max_code as libc::c_uint, bl_count.as_mut_ptr());
    } else if huf_heap[0usize] == 0i32 {
        (*desc).max_code = 0i32;
        (*tree.offset(0isize)).dl.len = 0i32 as libc::c_ushort;
        (*tree.offset(0isize)).fc.code = 0i32 as libc::c_ushort
    } else {
        let mut extra: *const libc::c_int = (*(*desc).stat_desc).extra_bits;
        let mut base: libc::c_int = (*(*desc).stat_desc).extra_base;
        let mut xbits: libc::c_int = 0;
        let mut f: libc::c_ushort = 0;
        (*desc).max_code = max_code;
        n = 0i32;
        while n < max_code {
            (*tree.offset(n as isize)).dl.len = 0i32 as libc::c_ushort;
            n += 1
        }
        (*tree.offset(max_code as isize)).dl.len = 1i32 as libc::c_ushort;
        xbits = 0i32;
        if max_code >= base {
            xbits = *extra.offset((max_code - base) as isize)
        }
        f = (*tree.offset(max_code as isize)).fc.freq;
        let ref mut fresh31 = *csize.offset(0isize);
        *fresh31 =
            (*fresh31).wrapping_add((f as
                                         libc::c_ulong).wrapping_mul((1i32 +
                                                                          xbits)
                                                                         as
                                                                         libc::c_ulong));
        if !stree.is_null() {
            let ref mut fresh32 = *csize.offset(1isize);
            *fresh32 =
                (*fresh32).wrapping_add((f as
                                             libc::c_ulong).wrapping_mul(((*stree.offset(max_code
                                                                                             as
                                                                                             isize)).dl.len
                                                                              as
                                                                              libc::c_int
                                                                              +
                                                                              xbits)
                                                                             as
                                                                             libc::c_ulong))
        }
        (*tree.offset(max_code as isize)).fc.code = 0i32 as libc::c_ushort
    };
}
/* ===========================================================================
* Generate the codes for a given tree and bit counts (which need not be
* optimal).
* IN assertion: the array bl_count contains the bit length statistics for
* the given tree and the field len is set for all tree elements.
* OUT assertion: the field code is set for all tree elements of non
*     zero code length.
*/
unsafe extern "C" fn gen_codes(mut tree: *mut ct_data,
                               mut max_code: libc::c_uint,
                               mut bl_count: *mut libc::c_ushort) {
    /* next code value for each bit length */
    let mut next_code: [libc::c_ushort; 16] = [0; 16];
    /* running code value */
    let mut code: libc::c_ushort = 0i32 as libc::c_ushort;
    /* bit index */
    let mut bits: libc::c_uint = 0;
    /* code index */
    let mut n: libc::c_uint = 0;
    bits = 1i32 as libc::c_uint;
    while bits <= 15i32 as libc::c_uint {
        code =
            ((code as libc::c_int +
                  *bl_count.offset(bits.wrapping_sub(1i32 as libc::c_uint) as
                                       isize) as libc::c_int) << 1i32) as
                libc::c_ushort;
        next_code[bits as usize] = code;
        bits = bits.wrapping_add(1)
    }
    n = 0i32 as libc::c_uint;
    while n <= max_code {
        let mut len: libc::c_int =
            (*tree.offset(n as isize)).dl.len as libc::c_int;
        if !(len == 0i32) {
            let fresh33 = next_code[len as usize];
            next_code[len as usize] = next_code[len as usize].wrapping_add(1);
            (*tree.offset(n as isize)).fc.code =
                bi_reverse(fresh33 as libc::c_uint, len) as libc::c_ushort
        }
        n = n.wrapping_add(1)
    };
}
/* ===========================================================================
* Reverse the first len bits of a code, using straightforward code (a faster
* method would use a table)
* IN assertion: 1 <= len <= 15
*/
unsafe extern "C" fn bi_reverse(mut code: libc::c_uint, mut len: libc::c_int)
 -> libc::c_uint {
    let mut res: libc::c_uint = 0i32 as libc::c_uint;
    loop  {
        res |= code & 1i32 as libc::c_uint;
        code >>= 1i32;
        res <<= 1i32;
        len -= 1;
        if !(len > 0i32) { break ; }
    }
    return res >> 1i32;
}
/* ===========================================================================
* Compute the optimal bit lengths for a tree and update the total bit length
* for the current block.
* IN assertion: the fields .Freq and dad are set, heap[heap_max] and
*    above are the tree nodes sorted by increasing frequency.
* OUT assertions: the field .Len is set to the optimal bit length, the
*     array bl_count contains the frequencies for each bit length.
*     The length os_len[0] is updated; os_len[1] is also updated if stree is
*     not null.
*/
unsafe extern "C" fn gen_bitlen(mut desc: *mut tree_desc,
                                mut huf_heap: *mut libc::c_int,
                                mut heap_max: libc::c_int,
                                mut bl_count: *mut libc::c_ushort) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut max_code: libc::c_int = (*desc).max_code;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut extra: *const libc::c_int = (*(*desc).stat_desc).extra_bits;
    let mut base: libc::c_int = (*(*desc).stat_desc).extra_base;
    let mut max_length: libc::c_int = (*(*desc).stat_desc).max_length;
    let mut csize: *mut libc::c_ulong = (*desc).comp_size;
    /* heap index */
    let mut h: libc::c_int = 0;
    /* iterate over the tree elements */
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    /* bit length */
    let mut bits: libc::c_int = 0;
    /* extra bits */
    let mut xbits: libc::c_int = 0;
    /* frequency */
    let mut f: libc::c_ushort = 0;
    /* number of elements with bit length too large */
    let mut overflow: libc::c_int = 0i32;
    bits = 0i32;
    while bits <= 15i32 {
        *bl_count.offset(bits as isize) = 0i32 as libc::c_ushort;
        bits += 1
    }
    (*tree.offset(*huf_heap.offset(heap_max as isize) as isize)).dl.len =
        0i32 as libc::c_ushort;
    h = heap_max + 1i32;
    while h < 2i32 * 257i32 + 1i32 {
        n = *huf_heap.offset(h as isize);
        bits =
            (*tree.offset((*tree.offset(n as isize)).dl.dad as isize)).dl.len
                as libc::c_int + 1i32;
        if bits > max_length { bits = max_length; overflow += 1 }
        (*tree.offset(n as isize)).dl.len = bits as libc::c_ushort;
        /* We overwrite tree[n].Dad which is no longer needed */
        if !(n > max_code) {
            /* not a leaf node */
            let ref mut fresh34 = *bl_count.offset(bits as isize);
            *fresh34 = (*fresh34).wrapping_add(1);
            xbits = 0i32;
            if n >= base { xbits = *extra.offset((n - base) as isize) }
            f = (*tree.offset(n as isize)).fc.freq;
            let ref mut fresh35 = *csize.offset(0isize);
            *fresh35 =
                (*fresh35).wrapping_add((f as
                                             libc::c_ulong).wrapping_mul((bits
                                                                              +
                                                                              xbits)
                                                                             as
                                                                             libc::c_ulong));
            if !stree.is_null() {
                let ref mut fresh36 = *csize.offset(1isize);
                *fresh36 =
                    (*fresh36).wrapping_add((f as
                                                 libc::c_ulong).wrapping_mul(((*stree.offset(n
                                                                                                 as
                                                                                                 isize)).dl.len
                                                                                  as
                                                                                  libc::c_int
                                                                                  +
                                                                                  xbits)
                                                                                 as
                                                                                 libc::c_ulong))
            }
        }
        h += 1
    }
    if overflow == 0i32 { return }
    loop  {
        bits = max_length - 1i32;
        while *bl_count.offset(bits as isize) as libc::c_int == 0i32 {
            bits -= 1
        }
        let ref mut fresh37 = *bl_count.offset(bits as isize);
        *fresh37 = (*fresh37).wrapping_sub(1);
        let ref mut fresh38 = *bl_count.offset((bits + 1i32) as isize);
        *fresh38 = (*fresh38 as libc::c_int + 2i32) as libc::c_ushort;
        let ref mut fresh39 = *bl_count.offset(max_length as isize);
        *fresh39 = (*fresh39).wrapping_sub(1);
        overflow -= 2i32;
        if !(overflow > 0i32) { break ; }
    }
    bits = max_length;
    while bits != 0i32 {
        n = *bl_count.offset(bits as isize) as libc::c_int;
        while n != 0i32 {
            h -= 1;
            m = *huf_heap.offset(h as isize);
            if m > max_code { continue ; }
            if (*tree.offset(m as isize)).dl.len as libc::c_uint !=
                   bits as libc::c_uint {
                let ref mut fresh40 = *csize.offset(0isize);
                *fresh40 =
                    (*fresh40).wrapping_add(((bits as libc::c_long -
                                                  (*tree.offset(m as
                                                                    isize)).dl.len
                                                      as libc::c_long) *
                                                 (*tree.offset(m as
                                                                   isize)).fc.freq
                                                     as libc::c_long) as
                                                libc::c_ulong);
                (*tree.offset(m as isize)).dl.len = bits as libc::c_ushort
            }
            n -= 1
        }
        bits -= 1
    };
}
/* Index within the heap array of least frequent node in the Huffman tree */
/* ===========================================================================
* Remove the smallest element from the heap and recreate the heap with
* one less element. Updates heap and huf_heap[0].
*/
/* ===========================================================================
* Compares to subtrees, using the tree depth as tie breaker when
* the subtrees have equal frequency. This minimizes the worst case length.
*/
/* ===========================================================================
* Restore the heap property by moving down the tree starting at node k,
* exchanging a node with the smallest of its two sons if necessary, stopping
* when the heap property is re-established (each father smaller than its
* two sons).
*/
unsafe extern "C" fn pqdownheap(mut tree: *mut ct_data,
                                mut huf_heap: *mut libc::c_int,
                                mut depth: *mut libc::c_uchar,
                                mut k: libc::c_int) {
    let mut v: libc::c_int = *huf_heap.offset(k as isize);
    /* left son of k */
    let mut j: libc::c_int = k << 1i32;
    while j <= *huf_heap.offset(0isize) {
        if j < *huf_heap.offset(0isize) &&
               (((*tree.offset(*huf_heap.offset((j + 1i32) as isize) as
                                   isize)).fc.freq as libc::c_int) <
                    (*tree.offset(*huf_heap.offset(j as isize) as
                                      isize)).fc.freq as libc::c_int ||
                    (*tree.offset(*huf_heap.offset((j + 1i32) as isize) as
                                      isize)).fc.freq as libc::c_int ==
                        (*tree.offset(*huf_heap.offset(j as isize) as
                                          isize)).fc.freq as libc::c_int &&
                        *depth.offset(*huf_heap.offset((j + 1i32) as isize) as
                                          isize) as libc::c_int <=
                            *depth.offset(*huf_heap.offset(j as isize) as
                                              isize) as libc::c_int) {
            j += 1
        }
        /* Exit if v is smaller than both sons */
        if ((*tree.offset(v as isize)).fc.freq as libc::c_int) <
               (*tree.offset(*huf_heap.offset(j as isize) as isize)).fc.freq
                   as libc::c_int ||
               (*tree.offset(v as isize)).fc.freq as libc::c_int ==
                   (*tree.offset(*huf_heap.offset(j as isize) as
                                     isize)).fc.freq as libc::c_int &&
                   *depth.offset(v as isize) as libc::c_int <=
                       *depth.offset(*huf_heap.offset(j as isize) as isize) as
                           libc::c_int {
            break ;
        }
        *huf_heap.offset(k as isize) = *huf_heap.offset(j as isize);
        k = j;
        j <<= 1i32
    }
    *huf_heap.offset(k as isize) = v;
}
/* ===========================================================================
* Merge the literal and distance tree and scan the resulting tree to determine
* the frequencies of the codes in the bit length tree.
*/
unsafe extern "C" fn feed_bltree(mut ltree_desc: *mut tree_desc,
                                 mut bltree_desc: *mut tree_desc) {
    let mut ltree: *mut ct_data = (*ltree_desc).dyn_tree;
    let mut bltree: *mut ct_data = (*bltree_desc).dyn_tree;
    let mut lmax_code: libc::c_int = (*ltree_desc).max_code;
    /* iterates over all tree elements */
    let mut n: libc::c_int = 0i32;
    /* last emitted length */
    let mut prevlen: libc::c_int = -1i32;
    /* length of next code */
    let mut nextlen: libc::c_int =
        (*ltree.offset(0isize)).dl.len as libc::c_int;
    /* repeat count of the current code */
    let mut count: libc::c_int = 0i32;
    /* max repeat count */
    let mut max_count: libc::c_int = 7i32;
    /* min repeat count */
    let mut min_count: libc::c_int = 4i32;
    if nextlen == 0i32 { max_count = 138i32; min_count = 3i32 }
    (*ltree.offset((lmax_code + 1i32) as isize)).dl.len =
        0xffffi32 as libc::c_ushort;
    n = 0i32;
    loop  {
        let fresh41 = n;
        n = n + 1;
        (*bltree.offset(fresh41 as isize)).fc.freq = 0i32 as libc::c_ushort;
        if !(n < 19i32) { break ; }
    }
    n = 0i32;
    while n <= lmax_code {
        /* length of current code */
        let mut curlen: libc::c_int = 0;
        curlen = nextlen;
        nextlen = (*ltree.offset((n + 1i32) as isize)).dl.len as libc::c_int;
        count += 1;
        if !(count < max_count && curlen == nextlen) {
            if count < min_count {
                let ref mut fresh42 =
                    (*bltree.offset(curlen as isize)).fc.freq;
                *fresh42 =
                    (*fresh42 as libc::c_int +
                         count as libc::c_ushort as libc::c_int) as
                        libc::c_ushort
            } else if curlen != 0i32 {
                if curlen != prevlen {
                    let ref mut fresh43 =
                        (*bltree.offset(curlen as isize)).fc.freq;
                    *fresh43 = (*fresh43).wrapping_add(1)
                }
                let ref mut fresh44 = (*bltree.offset(16isize)).fc.freq;
                *fresh44 = (*fresh44).wrapping_add(1)
            } else if count <= 10i32 {
                let ref mut fresh45 = (*bltree.offset(17isize)).fc.freq;
                *fresh45 = (*fresh45).wrapping_add(1)
            } else {
                let ref mut fresh46 = (*bltree.offset(18isize)).fc.freq;
                *fresh46 = (*fresh46).wrapping_add(1)
            }
            count = 0i32;
            prevlen = curlen;
            if nextlen == 0i32 {
                max_count = 138i32;
                min_count = 3i32
            } else if curlen == nextlen {
                max_count = 6i32;
                min_count = 3i32
            } else { max_count = 7i32; min_count = 4i32 }
        }
        n += 1
    };
}
static mut static_bl_desc: static_tree_desc =
    unsafe {
        static_tree_desc_s{static_tree: 0 as *const ct_data,
                           extra_bits: extra_blbits.as_ptr(),
                           extra_base: 0i32,
                           elems: 19i32,
                           max_length: 7i32,}
    };
/* extra bits for each bit length code */
static mut extra_blbits: [libc::c_int; 19] =
    [0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32,
     0i32, 0i32, 0i32, 0i32, 2i32, 3i32, 7i32];
static mut static_l_desc: static_tree_desc =
    unsafe {
        static_tree_desc_s{static_tree: static_ltree.as_ptr(),
                           extra_bits: extra_lbits.as_ptr(),
                           extra_base: 256i32 + 1i32,
                           elems: 257i32,
                           max_length: 15i32,}
    };
/* The lengths of the bit length codes are sent in order of decreasing
* probability, to avoid transmitting the lengths for unused bit length codes.
*/
/* extra bits for each length code */
static mut extra_lbits: [libc::c_int; 29] =
    [0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 1i32,
     2i32, 2i32, 2i32, 2i32, 3i32, 3i32, 3i32, 3i32, 4i32, 4i32, 4i32, 4i32,
     5i32, 5i32, 5i32, 5i32, 0i32];
#[no_mangle]
pub unsafe extern "C" fn ZLIBH_decompress(mut dest: *mut libc::c_char,
                                          mut compressed: *const libc::c_char)
 -> libc::c_int {
    let mut ip: *const libc::c_uchar = compressed as *const libc::c_uchar;
    let mut op: *mut libc::c_uchar = dest as *mut libc::c_uchar;
    return ZLIBH_inflate(op, ip);
}
/*
inflate() uses a state machine to process as much input data and generate as
much output data as possible before returning.  The state machine is
structured roughly as follows:

for (;;) switch (state) {
...
case STATEn:
if (not enough input data or output space to make progress)
return;
... make progress ...
state = STATEm;
break;
...
}

so when inflate() is called again, the same case is attempted again, and
if the appropriate resources are provided, the machine proceeds to the
next state.  The NEEDBITS() macro is usually the way the state evaluates
whether it can proceed or should return.  NEEDBITS() does the return if
the requested bits are not available.  The typical use of the BITS macros
is:

NEEDBITS(n);
... do something with BITS(n) ...
DROPBITS(n);

where NEEDBITS(n) either returns from inflate() if there isn't enough
input left to load n bits into the accumulator, or it continues.  BITS(n)
gives the low n bits in the accumulator.  When done, DROPBITS(n) drops
the low n bits off the accumulator.  INITBITS() clears the accumulator
and sets the number of available bits to zero.  BYTEBITS() discards just
enough bits to put the accumulator on a byte boundary.  After BYTEBITS()
and a NEEDBITS(8), then BITS(8) would return the next byte in the stream.

NEEDBITS(n) uses PULLBYTE() to get an available byte of input, or to return
if there is no input available.  The decoding of variable length codes uses
PULLBYTE() directly in order to pull just enough bytes to decode the next
code, and no more.

Some states loop until they get enough input, making sure that enough
state information is maintained to continue the loop where it left off
if NEEDBITS() returns in the loop.  For example, want, need, and keep
would all have to actually be part of the saved state in case NEEDBITS()
returns:

case STATEw:
while (want < need) {
NEEDBITS(n);
keep[want++] = BITS(n);
DROPBITS(n);
}
state = STATEx;
case STATEx:

As shown above, if the next state is also the next case, then the break
is omitted.

A state may also return if there is not enough output space available to
complete that state.  Those states are copying stored data, writing a
literal byte, and copying a matching string.

When returning, a "goto inf_leave" is used to update the total counters,
update the check value, and determine whether any progress has been made
during that inflate() call in order to return the proper return code.
Progress is defined as a change in either strm->avail_in or strm->avail_out.
When there is a window, goto inf_leave will update the window with the last
output written.  If a goto inf_leave occurs in the middle of decompression
and there is no window currently, goto inf_leave will create one and copy
output to the window for the next call of inflate().

In this implementation, the flush parameter of inflate() only affects the
return code (per zlib.h).  inflate() always writes as much as possible to
strm->next_out, given the space available and the provided input--the effect
documented in zlib.h of Z_SYNC_FLUSH.  Furthermore, inflate() always defers
the allocation of and copying into a sliding window until necessary, which
provides the effect documented in zlib.h for Z_FINISH when the entire input
stream available.  So the only thing the flush parameter actually does is:
when flush is set to Z_FINISH, inflate() cannot return Z_OK.  Instead it
will return Z_BUF_ERROR if it has not reached the end of the stream.
*/
#[no_mangle]
pub unsafe extern "C" fn ZLIBH_inflate(mut dest: *mut libc::c_uchar,
                                       mut compressed: *const libc::c_uchar)
 -> libc::c_int {
    /* next input */
    let mut next: *const libc::c_uchar = compressed;
    /* next output */
    let mut put: *mut libc::c_uchar = dest;
    /* bit buffer */
    let mut hold: libc::c_uint = 0;
    /* bits in bit buffer */
    let mut bits: libc::c_uint = 0;
    /* number of stored or match bytes to copy */
    let mut copy: libc::c_uint = 0;
    /* current decoding table entry */
    let mut here: code = code{op: 0, bits: 0, val: 0,};
    /* length to copy for repeats, bits to drop */
    let mut len: libc::c_uint = 0;
    /* return code */
    let mut ret: libc::c_int = 0;
    let mut state: inflate_state =
        inflate_state{mode: TYPEDO,
                      last: 0,
                      wrap: 0,
                      havedict: 0,
                      flags: 0,
                      dmax: 0,
                      check: 0,
                      total: 0,
                      wbits: 0,
                      wsize: 0,
                      whave: 0,
                      wnext: 0,
                      window: 0 as *mut libc::c_uchar,
                      hold: 0,
                      bits: 0,
                      length: 0,
                      offset: 0,
                      extra: 0,
                      lencode: 0 as *const code,
                      distcode: 0 as *const code,
                      lenbits: 0,
                      distbits: 0,
                      ncode: 0,
                      nlen: 0,
                      ndist: 0,
                      have: 0,
                      next: 0 as *mut code,
                      lens: [0; 320],
                      work: [0; 288],
                      codes: [code{op: 0, bits: 0, val: 0,}; 1444],
                      sane: 0,
                      back: 0,
                      was: 0,};
    /* permutation of code lengths */
    static mut order: [libc::c_ushort; 19] =
        [16i32 as libc::c_ushort, 17i32 as libc::c_ushort,
         18i32 as libc::c_ushort, 0i32 as libc::c_ushort,
         8i32 as libc::c_ushort, 7i32 as libc::c_ushort,
         9i32 as libc::c_ushort, 6i32 as libc::c_ushort,
         10i32 as libc::c_ushort, 5i32 as libc::c_ushort,
         11i32 as libc::c_ushort, 4i32 as libc::c_ushort,
         12i32 as libc::c_ushort, 3i32 as libc::c_ushort,
         13i32 as libc::c_ushort, 2i32 as libc::c_ushort,
         14i32 as libc::c_ushort, 1i32 as libc::c_ushort,
         15i32 as libc::c_ushort];
    let mut lcode: *const code = 0 as *const code;
    /* mask for first level of length codes */
    let mut lmask: libc::c_uint = 0;
    /* code bits, operation, was op */
    let mut codebits: libc::c_uint = 0;
    state.mode = TYPEDO;
    hold = 0i32 as libc::c_uint;
    bits = 0i32 as libc::c_uint;
    loop  {
        match state.mode as libc::c_uint {
            0 => {
                while bits < 1i32 as libc::c_uint {
                    let fresh47 = next;
                    next = next.offset(1);
                    hold =
                        (hold as
                             libc::c_ulong).wrapping_add((*fresh47 as
                                                              libc::c_ulong)
                                                             << bits) as
                            libc::c_uint as libc::c_uint;
                    bits = bits.wrapping_add(8i32 as libc::c_uint)
                }
                match hold & (1u32 << 1i32).wrapping_sub(1i32 as libc::c_uint)
                    {
                    0 => { state.mode = TABLE }
                    1 => { fixedtables(&mut state); state.mode = LEN }
                    _ => { }
                }
                hold >>= 1i32;
                bits = bits.wrapping_sub(1i32 as libc::c_uint);
                continue ;
            }
            1 => {
                while bits < 4i32 as libc::c_uint {
                    let fresh48 = next;
                    next = next.offset(1);
                    hold =
                        (hold as
                             libc::c_ulong).wrapping_add((*fresh48 as
                                                              libc::c_ulong)
                                                             << bits) as
                            libc::c_uint as libc::c_uint;
                    bits = bits.wrapping_add(8i32 as libc::c_uint)
                }
                state.nlen = 257i32 as libc::c_uint;
                state.ndist = 0i32 as libc::c_uint;
                state.ncode =
                    (hold &
                         (1u32 <<
                              4i32).wrapping_sub(1i32 as
                                                     libc::c_uint)).wrapping_add(4i32
                                                                                     as
                                                                                     libc::c_uint);
                hold >>= 4i32;
                bits = bits.wrapping_sub(4i32 as libc::c_uint);
                state.have = 0i32 as libc::c_uint;
                while state.have < state.ncode {
                    while bits < 3i32 as libc::c_uint {
                        let fresh49 = next;
                        next = next.offset(1);
                        hold =
                            (hold as
                                 libc::c_ulong).wrapping_add((*fresh49 as
                                                                  libc::c_ulong)
                                                                 << bits) as
                                libc::c_uint as libc::c_uint;
                        bits = bits.wrapping_add(8i32 as libc::c_uint)
                    }
                    let fresh50 = state.have;
                    state.have = state.have.wrapping_add(1);
                    state.lens[order[fresh50 as usize] as usize] =
                        (hold &
                             (1u32 <<
                                  3i32).wrapping_sub(1i32 as libc::c_uint)) as
                            libc::c_ushort;
                    hold >>= 3i32;
                    bits = bits.wrapping_sub(3i32 as libc::c_uint)
                }
                while state.have < 19i32 as libc::c_uint {
                    let fresh51 = state.have;
                    state.have = state.have.wrapping_add(1);
                    state.lens[order[fresh51 as usize] as usize] =
                        0i32 as libc::c_ushort
                }
                state.next = state.codes.as_mut_ptr();
                state.lencode = state.next as *const code;
                state.lenbits = 7i32 as libc::c_uint;
                ret =
                    inflate_table(CODES, state.lens.as_mut_ptr(),
                                  19i32 as libc::c_uint, &mut state.next,
                                  &mut state.lenbits,
                                  state.work.as_mut_ptr());
                if 0 != ret {
                    state.mode = BAD;
                    continue ;
                } else {
                    state.have = 0i32 as libc::c_uint;
                    while state.have < state.nlen {
                        loop  {
                            here =
                                *state.lencode.offset((hold &
                                                           (1u32 <<
                                                                state.lenbits).wrapping_sub(1i32
                                                                                                as
                                                                                                libc::c_uint))
                                                          as isize);
                            if here.bits as libc::c_uint <= bits { break ; }
                            let fresh52 = next;
                            next = next.offset(1);
                            hold =
                                (hold as
                                     libc::c_ulong).wrapping_add((*fresh52 as
                                                                      libc::c_ulong)
                                                                     << bits)
                                    as libc::c_uint as libc::c_uint;
                            bits = bits.wrapping_add(8i32 as libc::c_uint)
                        }
                        if (here.val as libc::c_int) < 16i32 {
                            hold >>= here.bits as libc::c_int;
                            bits =
                                bits.wrapping_sub(here.bits as libc::c_uint);
                            let fresh53 = state.have;
                            state.have = state.have.wrapping_add(1);
                            state.lens[fresh53 as usize] = here.val
                        } else {
                            if here.val as libc::c_int == 16i32 {
                                while bits <
                                          (here.bits as libc::c_int + 2i32) as
                                              libc::c_uint {
                                    let fresh54 = next;
                                    next = next.offset(1);
                                    hold =
                                        (hold as
                                             libc::c_ulong).wrapping_add((*fresh54
                                                                              as
                                                                              libc::c_ulong)
                                                                             <<
                                                                             bits)
                                            as libc::c_uint as libc::c_uint;
                                    bits =
                                        bits.wrapping_add(8i32 as
                                                              libc::c_uint)
                                }
                                hold >>= here.bits as libc::c_int;
                                bits =
                                    bits.wrapping_sub(here.bits as
                                                          libc::c_uint);
                                if state.have == 0i32 as libc::c_uint {
                                    state.mode = BAD;
                                    break ;
                                } else {
                                    len =
                                        state.lens[state.have.wrapping_sub(1i32
                                                                               as
                                                                               libc::c_uint)
                                                       as usize] as
                                            libc::c_uint;
                                    copy =
                                        (3i32 as
                                             libc::c_uint).wrapping_add(hold &
                                                                            (1u32
                                                                                 <<
                                                                                 2i32).wrapping_sub(1i32
                                                                                                        as
                                                                                                        libc::c_uint));
                                    hold >>= 2i32;
                                    bits =
                                        bits.wrapping_sub(2i32 as
                                                              libc::c_uint)
                                }
                            } else if here.val as libc::c_int == 17i32 {
                                while bits <
                                          (here.bits as libc::c_int + 3i32) as
                                              libc::c_uint {
                                    let fresh55 = next;
                                    next = next.offset(1);
                                    hold =
                                        (hold as
                                             libc::c_ulong).wrapping_add((*fresh55
                                                                              as
                                                                              libc::c_ulong)
                                                                             <<
                                                                             bits)
                                            as libc::c_uint as libc::c_uint;
                                    bits =
                                        bits.wrapping_add(8i32 as
                                                              libc::c_uint)
                                }
                                hold >>= here.bits as libc::c_int;
                                bits =
                                    bits.wrapping_sub(here.bits as
                                                          libc::c_uint);
                                len = 0i32 as libc::c_uint;
                                copy =
                                    (3i32 as
                                         libc::c_uint).wrapping_add(hold &
                                                                        (1u32
                                                                             <<
                                                                             3i32).wrapping_sub(1i32
                                                                                                    as
                                                                                                    libc::c_uint));
                                hold >>= 3i32;
                                bits = bits.wrapping_sub(3i32 as libc::c_uint)
                            } else {
                                while bits <
                                          (here.bits as libc::c_int + 7i32) as
                                              libc::c_uint {
                                    let fresh56 = next;
                                    next = next.offset(1);
                                    hold =
                                        (hold as
                                             libc::c_ulong).wrapping_add((*fresh56
                                                                              as
                                                                              libc::c_ulong)
                                                                             <<
                                                                             bits)
                                            as libc::c_uint as libc::c_uint;
                                    bits =
                                        bits.wrapping_add(8i32 as
                                                              libc::c_uint)
                                }
                                hold >>= here.bits as libc::c_int;
                                bits =
                                    bits.wrapping_sub(here.bits as
                                                          libc::c_uint);
                                len = 0i32 as libc::c_uint;
                                copy =
                                    (11i32 as
                                         libc::c_uint).wrapping_add(hold &
                                                                        (1u32
                                                                             <<
                                                                             7i32).wrapping_sub(1i32
                                                                                                    as
                                                                                                    libc::c_uint));
                                hold >>= 7i32;
                                bits = bits.wrapping_sub(7i32 as libc::c_uint)
                            }
                            if state.have.wrapping_add(copy) > state.nlen {
                                state.mode = BAD;
                                break ;
                            } else {
                                loop  {
                                    let fresh57 = copy;
                                    copy = copy.wrapping_sub(1);
                                    if !(0 != fresh57) { break ; }
                                    let fresh58 = state.have;
                                    state.have = state.have.wrapping_add(1);
                                    state.lens[fresh58 as usize] =
                                        len as libc::c_ushort
                                }
                            }
                        }
                    }
                    /* handle error breaks in while */
                    if state.mode as libc::c_uint ==
                           BAD as libc::c_int as libc::c_uint {
                        continue ;
                    }
                    /* check for end-of-block code (better have one) */
                    if state.lens[256usize] as libc::c_int == 0i32 {
                        state.mode = BAD;
                        continue ;
                    } else {
                        state.next = state.codes.as_mut_ptr();
                        state.lencode = state.next as *const code;
                        state.lenbits = 9i32 as libc::c_uint;
                        ret =
                            inflate_table(LENS, state.lens.as_mut_ptr(),
                                          state.nlen, &mut state.next,
                                          &mut state.lenbits,
                                          state.work.as_mut_ptr());
                        if 0 != ret {
                            state.mode = BAD;
                            continue ;
                        } else { state.mode = LEN }
                    }
                }
            }
            2 => { }
            3 => {
                /* fallthrough */
                break ;
            }
            4 => { return 0i32 }
            _ => { continue ; }
        }
        lcode = state.lencode;
        lmask = (1u32 << state.lenbits).wrapping_sub(1i32 as libc::c_uint);
        's_792:
            loop  {
                if bits < 15i32 as libc::c_uint {
                    let fresh59 = next;
                    next = next.offset(1);
                    hold =
                        (hold as
                             libc::c_ulong).wrapping_add((*fresh59 as
                                                              libc::c_ulong)
                                                             << bits) as
                            libc::c_uint as libc::c_uint;
                    bits = bits.wrapping_add(8i32 as libc::c_uint);
                    let fresh60 = next;
                    next = next.offset(1);
                    hold =
                        (hold as
                             libc::c_ulong).wrapping_add((*fresh60 as
                                                              libc::c_ulong)
                                                             << bits) as
                            libc::c_uint as libc::c_uint;
                    bits = bits.wrapping_add(8i32 as libc::c_uint)
                }
                here = *lcode.offset((hold & lmask) as isize);
                loop  {
                    codebits = here.bits as libc::c_uint;
                    hold >>= codebits;
                    bits = bits.wrapping_sub(codebits);
                    codebits = here.op as libc::c_uint;
                    if codebits == 0i32 as libc::c_uint {
                        let fresh61 = put;
                        put = put.offset(1);
                        *fresh61 = here.val as libc::c_uchar;
                        break ;
                    } else if codebits & 64i32 as libc::c_uint ==
                                  0i32 as libc::c_uint {
                        /* 2nd level length code */
                        here =
                            *lcode.offset((here.val as
                                               libc::c_uint).wrapping_add(hold
                                                                              &
                                                                              (1u32
                                                                                   <<
                                                                                   codebits).wrapping_sub(1i32
                                                                                                              as
                                                                                                              libc::c_uint))
                                              as isize)
                    } else {
                        if !(0 != codebits & 32i32 as libc::c_uint) {
                            break ;
                        }
                        /* end-of-block */
                        //len = bits >> 3;                        /* restitute unused bytes */
                    //next -= len;
                        break 's_792 ;
                    }
                }
            }
        state.mode = DONE;
        break ;
    }
    return put.wrapping_offset_from(dest) as libc::c_long as libc::c_int;
}
/* Macros for inflate(): */
/* check function to use adler32() for zlib or crc32() for gzip */
/* check macros for header crc */
/* Load registers with state in inflate() for speed */
/* Restore state from registers in inflate() */
/* Get a byte of input into the bit accumulator, or return from inflate()
if there is no input available. */
/* Assure that there are at least n bits in the bit accumulator.  If there is
not enough available input to do that, then return from inflate(). */
/* Return the low n bits of the bit accumulator (n < 16) */
/* Remove n bits from the bit accumulator */
/* Remove zero to seven bits as needed to go to a byte boundary */
/*
Build a set of tables to decode the provided canonical Huffman code.
The code lengths are lens[0..codes-1].  The result starts at *table,
whose indices are 0..2^bits-1.  work is a writable array of at least
lens shorts, which is used as a work area.  type is the type of code
to be generated, CODES, LENS, or DISTS.  On return, zero is success,
-1 is an invalid code, and +1 means that ENOUGH isn't enough.  table
on return points to the next available entry's address.  bits is the
requested root table index bits, and on return it is the actual root
table index bits.  It will differ if the request is greater than the
longest code or if it is less than the shortest code.
*/
#[no_mangle]
pub unsafe extern "C" fn inflate_table(mut type_0: codetype,
                                       mut lens: *mut libc::c_ushort,
                                       mut codes: libc::c_uint,
                                       mut table: *mut *mut code,
                                       mut bits: *mut libc::c_uint,
                                       mut work: *mut libc::c_ushort)
 -> libc::c_int {
    /* a code's length in bits */
    let mut len: libc::c_uint = 0;
    /* index of code symbols */
    let mut sym: libc::c_uint = 0;
    /* minimum and maximum code lengths */
    let mut min: libc::c_uint = 0;
    let mut max: libc::c_uint = 0;
    /* number of index bits for root table */
    let mut root: libc::c_uint = 0;
    /* number of index bits for current table */
    let mut curr: libc::c_uint = 0;
    /* code bits to drop for sub-table */
    let mut drop_0: libc::c_uint = 0;
    /* number of prefix codes available */
    let mut left: libc::c_int = 0;
    /* code entries in table used */
    let mut used: libc::c_uint = 0;
    /* Huffman code */
    let mut huff: libc::c_uint = 0;
    /* for incrementing code, index */
    let mut incr: libc::c_uint = 0;
    /* low bits for current root entry */
    let mut low: libc::c_uint = 0;
    /* mask for low root bits */
    let mut mask: libc::c_uint = 0;
    /* table entry for duplication */
    let mut here: code = code{op: 0, bits: 0, val: 0,};
    /* next available space in table */
    let mut next: *mut code = 0 as *mut code;
    /* base value table to use */
    let mut base: *const libc::c_ushort = 0 as *const libc::c_ushort;
    /* extra bits table to use */
    let mut extra: *const libc::c_ushort = 0 as *const libc::c_ushort;
    /* use base and extra for symbol > end */
    let mut end: libc::c_int = 0;
    /* number of codes of each length */
    let mut count: [libc::c_ushort; 16] = [0; 16];
    /* offsets in table for each length */
    let mut offs: [libc::c_ushort; 16] = [0; 16];
    /* Length codes 257..285 base */
    static mut lbase: [libc::c_ushort; 31] =
        [3i32 as libc::c_ushort, 4i32 as libc::c_ushort,
         5i32 as libc::c_ushort, 6i32 as libc::c_ushort,
         7i32 as libc::c_ushort, 8i32 as libc::c_ushort,
         9i32 as libc::c_ushort, 10i32 as libc::c_ushort,
         11i32 as libc::c_ushort, 13i32 as libc::c_ushort,
         15i32 as libc::c_ushort, 17i32 as libc::c_ushort,
         19i32 as libc::c_ushort, 23i32 as libc::c_ushort,
         27i32 as libc::c_ushort, 31i32 as libc::c_ushort,
         35i32 as libc::c_ushort, 43i32 as libc::c_ushort,
         51i32 as libc::c_ushort, 59i32 as libc::c_ushort,
         67i32 as libc::c_ushort, 83i32 as libc::c_ushort,
         99i32 as libc::c_ushort, 115i32 as libc::c_ushort,
         131i32 as libc::c_ushort, 163i32 as libc::c_ushort,
         195i32 as libc::c_ushort, 227i32 as libc::c_ushort,
         258i32 as libc::c_ushort, 0i32 as libc::c_ushort,
         0i32 as libc::c_ushort];
    /* Length codes 257..285 extra */
    static mut lext: [libc::c_ushort; 31] =
        [16i32 as libc::c_ushort, 16i32 as libc::c_ushort,
         16i32 as libc::c_ushort, 16i32 as libc::c_ushort,
         16i32 as libc::c_ushort, 16i32 as libc::c_ushort,
         16i32 as libc::c_ushort, 16i32 as libc::c_ushort,
         17i32 as libc::c_ushort, 17i32 as libc::c_ushort,
         17i32 as libc::c_ushort, 17i32 as libc::c_ushort,
         18i32 as libc::c_ushort, 18i32 as libc::c_ushort,
         18i32 as libc::c_ushort, 18i32 as libc::c_ushort,
         19i32 as libc::c_ushort, 19i32 as libc::c_ushort,
         19i32 as libc::c_ushort, 19i32 as libc::c_ushort,
         20i32 as libc::c_ushort, 20i32 as libc::c_ushort,
         20i32 as libc::c_ushort, 20i32 as libc::c_ushort,
         21i32 as libc::c_ushort, 21i32 as libc::c_ushort,
         21i32 as libc::c_ushort, 21i32 as libc::c_ushort,
         16i32 as libc::c_ushort, 72i32 as libc::c_ushort,
         78i32 as libc::c_ushort];
    /* Distance codes 0..29 base */
    static mut dbase: [libc::c_ushort; 32] =
        [1i32 as libc::c_ushort, 2i32 as libc::c_ushort,
         3i32 as libc::c_ushort, 4i32 as libc::c_ushort,
         5i32 as libc::c_ushort, 7i32 as libc::c_ushort,
         9i32 as libc::c_ushort, 13i32 as libc::c_ushort,
         17i32 as libc::c_ushort, 25i32 as libc::c_ushort,
         33i32 as libc::c_ushort, 49i32 as libc::c_ushort,
         65i32 as libc::c_ushort, 97i32 as libc::c_ushort,
         129i32 as libc::c_ushort, 193i32 as libc::c_ushort,
         257i32 as libc::c_ushort, 385i32 as libc::c_ushort,
         513i32 as libc::c_ushort, 769i32 as libc::c_ushort,
         1025i32 as libc::c_ushort, 1537i32 as libc::c_ushort,
         2049i32 as libc::c_ushort, 3073i32 as libc::c_ushort,
         4097i32 as libc::c_ushort, 6145i32 as libc::c_ushort,
         8193i32 as libc::c_ushort, 12289i32 as libc::c_ushort,
         16385i32 as libc::c_ushort, 24577i32 as libc::c_ushort,
         0i32 as libc::c_ushort, 0i32 as libc::c_ushort];
    /* Distance codes 0..29 extra */
    static mut dext: [libc::c_ushort; 32] =
        [16i32 as libc::c_ushort, 16i32 as libc::c_ushort,
         16i32 as libc::c_ushort, 16i32 as libc::c_ushort,
         17i32 as libc::c_ushort, 17i32 as libc::c_ushort,
         18i32 as libc::c_ushort, 18i32 as libc::c_ushort,
         19i32 as libc::c_ushort, 19i32 as libc::c_ushort,
         20i32 as libc::c_ushort, 20i32 as libc::c_ushort,
         21i32 as libc::c_ushort, 21i32 as libc::c_ushort,
         22i32 as libc::c_ushort, 22i32 as libc::c_ushort,
         23i32 as libc::c_ushort, 23i32 as libc::c_ushort,
         24i32 as libc::c_ushort, 24i32 as libc::c_ushort,
         25i32 as libc::c_ushort, 25i32 as libc::c_ushort,
         26i32 as libc::c_ushort, 26i32 as libc::c_ushort,
         27i32 as libc::c_ushort, 27i32 as libc::c_ushort,
         28i32 as libc::c_ushort, 28i32 as libc::c_ushort,
         29i32 as libc::c_ushort, 29i32 as libc::c_ushort,
         64i32 as libc::c_ushort, 64i32 as libc::c_ushort];
    len = 0i32 as libc::c_uint;
    while len <= 15i32 as libc::c_uint {
        count[len as usize] = 0i32 as libc::c_ushort;
        len = len.wrapping_add(1)
    }
    sym = 0i32 as libc::c_uint;
    while sym < codes {
        count[*lens.offset(sym as isize) as usize] =
            count[*lens.offset(sym as isize) as usize].wrapping_add(1);
        sym = sym.wrapping_add(1)
    }
    root = *bits;
    max = 15i32 as libc::c_uint;
    while max >= 1i32 as libc::c_uint {
        if count[max as usize] as libc::c_int != 0i32 { break ; }
        max = max.wrapping_sub(1)
    }
    if root > max { root = max }
    if max == 0i32 as libc::c_uint {
        here.op = 64i32 as libc::c_uchar;
        here.bits = 1i32 as libc::c_uchar;
        here.val = 0i32 as libc::c_ushort;
        let fresh62 = *table;
        *table = (*table).offset(1);
        *fresh62 = here;
        let fresh63 = *table;
        *table = (*table).offset(1);
        *fresh63 = here;
        *bits = 1i32 as libc::c_uint;
        return 0i32
    }
    min = 1i32 as libc::c_uint;
    while min < max {
        if count[min as usize] as libc::c_int != 0i32 { break ; }
        min = min.wrapping_add(1)
    }
    if root < min { root = min }
    left = 1i32;
    len = 1i32 as libc::c_uint;
    while len <= 15i32 as libc::c_uint {
        left <<= 1i32;
        left -= count[len as usize] as libc::c_int;
        if left < 0i32 { return -1i32 }
        len = len.wrapping_add(1)
    }
    if left > 0i32 &&
           (type_0 as libc::c_uint == CODES as libc::c_int as libc::c_uint ||
                max != 1i32 as libc::c_uint) {
        return -1i32
    }
    offs[1usize] = 0i32 as libc::c_ushort;
    len = 1i32 as libc::c_uint;
    while len < 15i32 as libc::c_uint {
        offs[len.wrapping_add(1i32 as libc::c_uint) as usize] =
            (offs[len as usize] as libc::c_int +
                 count[len as usize] as libc::c_int) as libc::c_ushort;
        len = len.wrapping_add(1)
    }
    sym = 0i32 as libc::c_uint;
    while sym < codes {
        if *lens.offset(sym as isize) as libc::c_int != 0i32 {
            let fresh64 = offs[*lens.offset(sym as isize) as usize];
            offs[*lens.offset(sym as isize) as usize] =
                offs[*lens.offset(sym as isize) as usize].wrapping_add(1);
            *work.offset(fresh64 as isize) = sym as libc::c_ushort
        }
        sym = sym.wrapping_add(1)
    }
    let mut current_block_51: u64;
    match type_0 as libc::c_uint {
        0 => {
            extra = work;
            base = extra;
            end = 19i32;
            current_block_51 = 8869332144787829186;
        }
        1 => {
            base = lbase.as_ptr();
            base = base.offset(-257isize);
            extra = lext.as_ptr();
            extra = extra.offset(-257isize);
            end = 256i32;
            current_block_51 = 8869332144787829186;
        }
        2 => {
            /* DISTS */
            current_block_51 = 3657175711928608847;
        }
        _ => { current_block_51 = 3657175711928608847; }
    }
    match current_block_51 {
        3657175711928608847 => {
            base = dbase.as_ptr();
            extra = dext.as_ptr();
            end = -1i32
        }
        _ => { }
    }
    huff = 0i32 as libc::c_uint;
    sym = 0i32 as libc::c_uint;
    len = min;
    next = *table;
    curr = root;
    drop_0 = 0i32 as libc::c_uint;
    low = -1i32 as libc::c_uint;
    used = 1u32 << root;
    mask = used.wrapping_sub(1i32 as libc::c_uint);
    if type_0 as libc::c_uint == LENS as libc::c_int as libc::c_uint &&
           used > 852i32 as libc::c_uint ||
           type_0 as libc::c_uint == DISTS as libc::c_int as libc::c_uint &&
               used > 592i32 as libc::c_uint {
        return 1i32
    }
    loop  {
        /* index for replicating entries */
        let mut fill: libc::c_uint = 0;
        here.bits = len.wrapping_sub(drop_0) as libc::c_uchar;
        if (*work.offset(sym as isize) as libc::c_int) < end {
            here.op = 0i32 as libc::c_uchar;
            here.val = *work.offset(sym as isize)
        } else if *work.offset(sym as isize) as libc::c_int > end {
            here.op =
                *extra.offset(*work.offset(sym as isize) as isize) as
                    libc::c_uchar;
            here.val = *base.offset(*work.offset(sym as isize) as isize)
        } else {
            here.op = (32i32 + 64i32) as libc::c_uchar;
            here.val = 0i32 as libc::c_ushort
        }
        incr = 1u32 << len.wrapping_sub(drop_0);
        fill = 1u32 << curr;
        min = fill;
        loop  {
            fill = fill.wrapping_sub(incr);
            *next.offset((huff >> drop_0).wrapping_add(fill) as isize) = here;
            if !(fill != 0i32 as libc::c_uint) { break ; }
        }
        incr = 1u32 << len.wrapping_sub(1i32 as libc::c_uint);
        while 0 != huff & incr { incr >>= 1i32 }
        if incr != 0i32 as libc::c_uint {
            huff &= incr.wrapping_sub(1i32 as libc::c_uint);
            huff = huff.wrapping_add(incr)
        } else { huff = 0i32 as libc::c_uint }
        sym = sym.wrapping_add(1);
        count[len as usize] = count[len as usize].wrapping_sub(1);
        if count[len as usize] as libc::c_int == 0i32 {
            if len == max { break ; }
            len =
                *lens.offset(*work.offset(sym as isize) as isize) as
                    libc::c_uint
        }
        if len > root && huff & mask != low {
            if drop_0 == 0i32 as libc::c_uint { drop_0 = root }
            next = next.offset(min as isize);
            curr = len.wrapping_sub(drop_0);
            left = 1i32 << curr;
            while curr.wrapping_add(drop_0) < max {
                left -=
                    count[curr.wrapping_add(drop_0) as usize] as libc::c_int;
                if left <= 0i32 { break ; }
                curr = curr.wrapping_add(1);
                left <<= 1i32
            }
            used = used.wrapping_add(1u32 << curr);
            if type_0 as libc::c_uint == LENS as libc::c_int as libc::c_uint
                   && used > 852i32 as libc::c_uint ||
                   type_0 as libc::c_uint ==
                       DISTS as libc::c_int as libc::c_uint &&
                       used > 592i32 as libc::c_uint {
                return 1i32
            }
            low = huff & mask;
            (*(*table).offset(low as isize)).op = curr as libc::c_uchar;
            (*(*table).offset(low as isize)).bits = root as libc::c_uchar;
            (*(*table).offset(low as isize)).val =
                next.wrapping_offset_from(*table) as libc::c_long as
                    libc::c_ushort
        }
    }
    if huff != 0i32 as libc::c_uint {
        here.op = 64i32 as libc::c_uchar;
        here.bits = len.wrapping_sub(drop_0) as libc::c_uchar;
        here.val = 0i32 as libc::c_ushort;
        *next.offset(huff as isize) = here
    }
    *table = (*table).offset(used as isize);
    *bits = root;
    return 0i32;
}
unsafe extern "C" fn fixedtables(mut state: *mut inflate_state) {
    (*state).lencode = lenfix.as_ptr();
    (*state).lenbits = 9i32 as libc::c_uint;
    (*state).distcode = distfix.as_ptr();
    (*state).distbits = 5i32 as libc::c_uint;
}
static mut distfix: [code; 32] =
    [code{op: 16i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 1i32 as libc::c_ushort,},
     code{op: 23i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 257i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 17i32 as libc::c_ushort,},
     code{op: 27i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 4097i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 5i32 as libc::c_ushort,},
     code{op: 25i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 1025i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 65i32 as libc::c_ushort,},
     code{op: 29i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 16385i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 3i32 as libc::c_ushort,},
     code{op: 24i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 513i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 33i32 as libc::c_ushort,},
     code{op: 28i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 8193i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 9i32 as libc::c_ushort,},
     code{op: 26i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 2049i32 as libc::c_ushort,},
     code{op: 22i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 129i32 as libc::c_ushort,},
     code{op: 64i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 2i32 as libc::c_ushort,},
     code{op: 23i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 385i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 25i32 as libc::c_ushort,},
     code{op: 27i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 6145i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 7i32 as libc::c_ushort,},
     code{op: 25i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 1537i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 97i32 as libc::c_ushort,},
     code{op: 29i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 24577i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 4i32 as libc::c_ushort,},
     code{op: 24i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 769i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 49i32 as libc::c_ushort,},
     code{op: 28i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 12289i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 13i32 as libc::c_ushort,},
     code{op: 26i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 3073i32 as libc::c_ushort,},
     code{op: 22i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 193i32 as libc::c_ushort,},
     code{op: 64i32 as libc::c_uchar,
          bits: 5i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,}];
static mut lenfix: [code; 512] =
    [code{op: 96i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 80i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 16i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 115i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 31i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 112i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 48i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 192i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 10i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 96i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 32i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 160i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 128i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 64i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 224i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 6i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 88i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 24i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 144i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 59i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 120i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 56i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 208i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 17i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 104i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 40i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 176i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 8i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 136i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 72i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 240i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 4i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 84i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 20i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 227i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 43i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 116i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 52i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 200i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 13i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 100i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 36i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 168i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 4i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 132i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 68i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 232i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 8i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 92i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 28i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 152i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 83i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 124i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 60i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 216i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 23i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 108i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 44i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 184i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 12i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 140i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 76i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 248i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 3i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 82i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 18i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 163i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 35i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 114i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 50i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 196i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 11i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 98i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 34i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 164i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 2i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 130i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 66i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 228i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 7i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 90i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 26i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 148i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 67i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 122i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 58i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 212i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 19i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 106i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 42i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 180i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 10i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 138i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 74i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 244i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 5i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 86i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 22i32 as libc::c_ushort,},
     code{op: 64i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 51i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 118i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 54i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 204i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 15i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 102i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 38i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 172i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 6i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 134i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 70i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 236i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 9i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 94i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 30i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 156i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 99i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 126i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 62i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 220i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 27i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 110i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 46i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 188i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 14i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 142i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 78i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 252i32 as libc::c_ushort,},
     code{op: 96i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 81i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 17i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 131i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 31i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 113i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 49i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 194i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 10i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 97i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 33i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 162i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 1i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 129i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 65i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 226i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 6i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 89i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 25i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 146i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 59i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 121i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 57i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 210i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 17i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 105i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 41i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 178i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 9i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 137i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 73i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 242i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 4i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 85i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 21i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 258i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 43i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 117i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 53i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 202i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 13i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 101i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 37i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 170i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 5i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 133i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 69i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 234i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 8i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 93i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 29i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 154i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 83i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 125i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 61i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 218i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 23i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 109i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 45i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 186i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 13i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 141i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 77i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 250i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 3i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 83i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 19i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 195i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 35i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 115i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 51i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 198i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 11i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 99i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 35i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 166i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 3i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 131i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 67i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 230i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 7i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 91i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 27i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 150i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 67i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 123i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 59i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 214i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 19i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 107i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 43i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 182i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 11i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 139i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 75i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 246i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 5i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 87i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 23i32 as libc::c_ushort,},
     code{op: 64i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 51i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 119i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 55i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 206i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 15i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 103i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 39i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 174i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 7i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 135i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 71i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 238i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 9i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 95i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 31i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 158i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 99i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 127i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 63i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 222i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 27i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 111i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 47i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 190i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 15i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 143i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 79i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 254i32 as libc::c_ushort,},
     code{op: 96i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 80i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 16i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 115i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 31i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 112i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 48i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 193i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 10i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 96i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 32i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 161i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 128i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 64i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 225i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 6i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 88i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 24i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 145i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 59i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 120i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 56i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 209i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 17i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 104i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 40i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 177i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 8i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 136i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 72i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 241i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 4i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 84i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 20i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 227i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 43i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 116i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 52i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 201i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 13i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 100i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 36i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 169i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 4i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 132i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 68i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 233i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 8i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 92i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 28i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 153i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 83i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 124i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 60i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 217i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 23i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 108i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 44i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 185i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 12i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 140i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 76i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 249i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 3i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 82i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 18i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 163i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 35i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 114i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 50i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 197i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 11i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 98i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 34i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 165i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 2i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 130i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 66i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 229i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 7i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 90i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 26i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 149i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 67i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 122i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 58i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 213i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 19i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 106i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 42i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 181i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 10i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 138i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 74i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 245i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 5i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 86i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 22i32 as libc::c_ushort,},
     code{op: 64i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 51i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 118i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 54i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 205i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 15i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 102i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 38i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 173i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 6i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 134i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 70i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 237i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 9i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 94i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 30i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 157i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 99i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 126i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 62i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 221i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 27i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 110i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 46i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 189i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 14i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 142i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 78i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 253i32 as libc::c_ushort,},
     code{op: 96i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 81i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 17i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 131i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 31i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 113i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 49i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 195i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 10i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 97i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 33i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 163i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 1i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 129i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 65i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 227i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 6i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 89i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 25i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 147i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 59i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 121i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 57i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 211i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 17i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 105i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 41i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 179i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 9i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 137i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 73i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 243i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 4i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 85i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 21i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 258i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 43i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 117i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 53i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 203i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 13i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 101i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 37i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 171i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 5i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 133i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 69i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 235i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 8i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 93i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 29i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 155i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 83i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 125i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 61i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 219i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 23i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 109i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 45i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 187i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 13i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 141i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 77i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 251i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 3i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 83i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 19i32 as libc::c_ushort,},
     code{op: 21i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 195i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 35i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 115i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 51i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 199i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 11i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 99i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 35i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 167i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 3i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 131i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 67i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 231i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 7i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 91i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 27i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 151i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 67i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 123i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 59i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 215i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 19i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 107i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 43i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 183i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 11i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 139i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 75i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 247i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 5i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 87i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 23i32 as libc::c_ushort,},
     code{op: 64i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 0i32 as libc::c_ushort,},
     code{op: 19i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 51i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 119i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 55i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 207i32 as libc::c_ushort,},
     code{op: 17i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 15i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 103i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 39i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 175i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 7i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 135i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 71i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 239i32 as libc::c_ushort,},
     code{op: 16i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 9i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 95i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 31i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 159i32 as libc::c_ushort,},
     code{op: 20i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 99i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 127i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 63i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 223i32 as libc::c_ushort,},
     code{op: 18i32 as libc::c_uchar,
          bits: 7i32 as libc::c_uchar,
          val: 27i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 111i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 47i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 191i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 15i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 143i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 8i32 as libc::c_uchar,
          val: 79i32 as libc::c_ushort,},
     code{op: 0i32 as libc::c_uchar,
          bits: 9i32 as libc::c_uchar,
          val: 255i32 as libc::c_ushort,}];
/*
ZLIBH_compressBound():
    Gives the maximum (worst case) size that can be reached by function ZLIBH_compress.
    Used to know how much memory to allocate for destination buffer.
*/
#[no_mangle]
pub unsafe extern "C" fn ZLIBH_getDistributionTotal() -> libc::c_int {
    return (1u32 << 14i32 - 2i32) as libc::c_int;
}