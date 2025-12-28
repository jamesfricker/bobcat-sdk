#![no_std]
#![no_main]

use bobcat_sdk::prelude::*;

const TOPIC: U = const_keccak256(b"hello()");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(_: usize) -> usize {
    emit!(TOPIC);
    0
}
