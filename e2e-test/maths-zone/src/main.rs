#![no_main]
#![no_std]

use bobcat_maths_zone::{bobcat_math, maths_zone};
use bobcat_sdk::{
    cd::{const_keccak_sel, read_words},
    entry::*,
    maths::U,
};

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

const SEL_CONST_VALUE: [u8; 4] = const_keccak_sel(b"constValue()");
const SEL_COMBINED: [u8; 4] = const_keccak_sel(b"combined(uint256,uint256)");
const SEL_XOR_MIX: [u8; 4] = const_keccak_sel(b"xorMix(uint256,uint256)");

const CONST_VALUE: U = maths_zone!("2 ** 8 + 5");
const MASK: U = maths_zone!("255");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = read_args_safe!(args_len, { (32 * 2) + 4 });
    match args[..4].try_into().unwrap() {
        SEL_CONST_VALUE => {
            write_result_word(&CONST_VALUE);
        }
        SEL_COMBINED => {
            let (x, y) = read_words!(&args[4..], 2);
            let x = *x;
            let y = *y;
            let result = bobcat_math!(r#"
                [label = "maths-zone", unwrap]
                a = x + y
                b = a * 3
                c = b // 2
                return c
            "#);
            write_result_word(&result);
        }
        SEL_XOR_MIX => {
            let (x, y) = read_words!(&args[4..], 2);
            let x = *x;
            let y = *y;
            let mask = MASK;
            let result = bobcat_math!(r#"
                [unwrap]
                a = x ^ y
                b = a | mask
                return b
            "#);
            write_result_word(&result);
        }
        _ => unimplemented!(),
    }
    0
}
