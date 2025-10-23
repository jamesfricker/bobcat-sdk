#![no_std]
#![no_main]

#[allow(unused)]
use bobcat_sdk;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(_: usize) -> usize {
    panic!("Look at me!")
}
