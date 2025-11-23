#![no_main]
#![no_std]

use bobcat_sdk::{
    cd::{const_keccak_sel, read_words},
    entry::*,
    precompiles::ecrecover
};

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

const SEL_ECRECOVER: [u8; 4] = const_keccak_sel(b"ecrecover_(bytes32,uint8,bytes32,bytes32)");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { (32 * 4) + 4 });
    let (hash, v, r, s) = read_words!(&args[4..], 4);
    write_result_word(&match args[..4].try_into().unwrap() {
        SEL_ECRECOVER => ecrecover(*hash, (*v).into(), *r, *s, u64::MAX).unwrap().into(),
        _ => unimplemented!()
    });
    0
}
