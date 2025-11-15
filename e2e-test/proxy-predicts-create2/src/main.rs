#![no_main]
#![no_std]

use bobcat_sdk::prelude::*;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

const SEL_DEPLOY: [u8; 4] = const_keccak_sel(b"deploy()");
const SEL_PREDICT: [u8; 4] = const_keccak_sel(b"predict()");

const CODE: [u8; SIZE_BEACON_SEL_PROXY] =
    make_beacon_sel_proxy(address!(b"6221a9c005f6e47eb398fd867784cacfdcfff4e7"));

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, 4);
    write_result_word(
        &match args[..4].try_into().unwrap() {
            SEL_DEPLOY => create2_vec(&CODE, U::ZERO, msg_sender().into()).unwrap(),
            SEL_PREDICT => estimate_addr(contract_address(), keccak256(&CODE), msg_sender().into()),
            _ => panic!("call not supported")
        }
        .into(),
    );
    0
}
