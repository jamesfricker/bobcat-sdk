#![no_main]
#![no_std]

use bobcat_sdk::{cd::read_word_slices, entry::*, maths::U};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { (32 * 2) + 4 })[4..];
    let (x, y) = read_word_slices!(&args, 2);
    let x = <&U>::from(x);
    let y = <&U>::from(y);
    let w = U::from(10 as u32);
    write_result(&x.widening_mul(&y).1.0);
    0
}
