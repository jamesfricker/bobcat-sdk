#![no_main]
#![no_std]

use bobcat_sdk::prelude::*;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { (32 * 2) + 4 });
    let target = read_words!(&args[4..], 1);
    safe_call_bool(target, eip20::make_fn_transfer(msg_sender(), &U::from(100u32)));
    0
}
