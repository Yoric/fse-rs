use super::types::*;

#[header_src = "/Volumes/Code/dteller/blurbs/FiniteStateEntropy/lib/huf.h"]
pub type HUF_repeat = libc::c_uint;
/* *< Can use the previous table and it is assumed to be valid */
pub const HUF_repeat_valid: HUF_repeat = 2;
/* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
pub const HUF_repeat_check: HUF_repeat = 1;
/* *< Cannot use the previous table */
pub const HUF_repeat_none: HUF_repeat = 0;
/* incomplete type */
pub type HUF_CElt = HUF_CElt_s;
use super::huf_compress_c::HUF_CElt_s;
use super::mem_common::{BYTE, U32};
extern "C" {
    /* ! HUF_readStats() :
     *  Read compact Huffman tree, saved by HUF_writeCTable().
     * `huffWeight` is destination buffer.
     * @return : size read from `src` , or an error Code .
     *  Note : Needed by HUF_readCTable() and HUF_readDTableXn() . */
    #[no_mangle]
    pub fn HUF_readStats(
        huffWeight: *mut BYTE,
        hwSize: size_t,
        rankStats: *mut U32,
        nbSymbolsPtr: *mut U32,
        tableLogPtr: *mut U32,
        src: *const libc::c_void,
        srcSize: size_t,
    ) -> size_t;
}
