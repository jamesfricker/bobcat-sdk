#![no_main]
#![no_std]

use bobcat_sdk::prelude::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(_: usize) -> usize {
    trace_guard! {
        ()
    }
    panic!("shit")
}
