#![no_main]
#![no_std]

use bobcat_sdk::{entry::*, maths::U};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, 32)[4..];
    let w = U::from(*read_word_slices!(&args, 1));
    let d = U::from(1e6 as u32);
    flush_guard(|| match args[..4] {
        // muldiv(uint256)
        [0x8d, 0xd3, 0x76, 0x31] => {
            let x = get_number().mul_div_round_up(&d, &w).unwrap();
            set_number(&x);
            write_result(&x.0);
        }
        _ => unimplemented!(),
    });
    0
}
