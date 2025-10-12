#![no_main]
#![no_std]

use bobcat_sdk::{cd::read_word_slices, entry::*, maths::U, storage::*};

pub fn get_number() -> U {
    storage_load(&U::ZERO)
}

pub fn set_number(x: &U) {
    storage_store(&U::ZERO, x)
}

pub fn mul_number(x: &U) {
    storage_checking_mul(&U::ZERO, x).unwrap()
}

pub fn add_number(x: &U) {
    storage_checking_add(&U::ZERO, x).unwrap()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    assert!(!msg_reentrant());
    let args = read_args_safe!(args_len, { 32 * 2 + 4 });
    match args[..4] {
        [0x83, 0x81, 0xf5, 0x8a] => write_result(get_number().as_slice()),
        [0x3f, 0xb5, 0xc1, 0xcb] => set_number(&U::from(*read_word_slices!(&args[4..], 1))),
        [0x4d, 0x4f, 0x58, 0xd1] => mul_number(&U::from(*read_word_slices!(&args[4..], 1))),
        [0xfc, 0xe6, 0x80, 0x23] => add_number(&U::from(*read_word_slices!(&args[4..], 1))),
        [0xd0, 0x9d, 0xe0, 0x8a] => add_number(&U::ONE),
        [0xcd, 0x87, 0xba, 0xff] => add_number(&msg_value()),
        _ => unimplemented!(),
    }
    flush_cache(false);
    0
}
