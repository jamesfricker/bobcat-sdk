#![no_main]
#![no_std]

use bobcat_sdk::{
    proxy::make_eip1967_proxy,
    entry::{write_result_word, read_args_safe},
    maths::U,
    cd::read_words,
    create::create1_vec
};

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { 32 + 4 });
    let impl_ = read_words!(&args[4..], 1);
    let addr = create1_vec(&make_eip1967_proxy(impl_.into()), U::ZERO).unwrap();
    write_result_word(&addr.into());
    0
}
