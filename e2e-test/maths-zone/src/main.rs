#![no_main]
#![no_std]

use bobcat_maths_zone::maths_zone;
use bobcat_sdk::{
    cd::{const_keccak_sel, read_words},
    entry::*,
    maths,
};

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

const SEL_CONST_BASIC: [u8; 4] = const_keccak_sel(b"constBasic()");
const SEL_CONST_SHIFT: [u8; 4] = const_keccak_sel(b"constShift()");
const SEL_CONST_POW: [u8; 4] = const_keccak_sel(b"constPow()");
const SEL_ADD_CONST: [u8; 4] = const_keccak_sel(b"addConst(uint256)");
const SEL_MASK: [u8; 4] = const_keccak_sel(b"mask(uint256)");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { (32 * 1) + 4 });
    let sel: [u8; 4] = args[..4].try_into().unwrap();
    let result = match sel {
        SEL_CONST_BASIC => maths_zone!("1 + 2 * 3"),
        SEL_CONST_SHIFT => maths_zone!("0xff << 8"),
        SEL_CONST_POW => maths_zone!("2 ** 16"),
        SEL_ADD_CONST => {
            let x = read_words!(&args[4..], 1);
            let k = maths_zone!("1_000_000");
            maths::checked_add(x, &k)
        }
        SEL_MASK => {
            let x = read_words!(&args[4..], 1);
            let mask = maths_zone!("0xff");
            *x & mask
        }
        _ => unimplemented!("{:?}", &args[..4]),
    };
    write_result_word(&result);
    0
}
