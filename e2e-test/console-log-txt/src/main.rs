// console-log-txt: Print the results of adding two numbers together
// before returning.

#![no_std]
#![no_main]

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

extern crate alloc;

use bobcat_sdk::{
    cd::*,
    entry::*,
    console::console
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { 32 * 2 + 4 });
    let (x, y) = read_word_slices!(&args[4..], 2);
    let z = x + y;
    console!(x, y, z);
    write_result_word(&z);
    0
}
