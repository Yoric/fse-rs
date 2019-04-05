#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/mem.h"]

pub use super::types::*;

/*-**************************************************************
*  Basic Types
*****************************************************************/
/* C99 */
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed {
    pub u: U32,
    pub c: [BYTE; 4],
}
/* __pack instructions are safer, but compiler specific, hence potentially problematic for some compilers */
/* currently only defined for gcc and icc */
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct unalign16 {
    pub v: U16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct unalign32 {
    pub v: U32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct unalign64 {
    pub v: U64,
}
