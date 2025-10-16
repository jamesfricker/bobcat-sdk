#![no_main]
#![no_std]

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use bobcat_sdk::{
    call::call_vec,
    cd::{const_keccak_sel, read_word_slices},
    entry::*,
    interfaces::{
        camelotv3_swap_router::make_fn_exact_input_single,
        eip20::{make_fn_approve, make_fn_transfer_from},
    },
    maths::U,
};

#[link(wasm_import_module = "vm_hooks")]
unsafe extern "C" {
    fn msg_reentrant() -> bool;
}

const SWAP_ROUTER: [u8; 20] =
    match const_hex::const_decode_to_array::<20>(b"6221a9c005f6e47eb398fd867784cacfdcfff4e7") {
        Ok(v) => v,
        Err(_) => panic!(),
    };

const SEL: [u8; 4] = const_keccak_sel(b"makeSwap(address,address,uint256,uint256)");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    assert!(!unsafe { msg_reentrant() });
    let args = read_args_safe!(args_len, { 32 * 4 + 4 });
    let (token_in, token_out, amount_in, amount_out_min) = read_word_slices!(&args[4..], 4);
    if args[..4] != SEL {
        return 1;
    }
    let sender = msg_sender();
    revert_if_bad_call_vec!(call_vec(
        SWAP_ROUTER,
        &make_fn_transfer_from(msg_sender(), contract_address(), amount_in),
        &U::ZERO,
        u64::MAX,
        0
    ));
    revert_if_bad_call_vec!(call_vec(
        SWAP_ROUTER,
        &make_fn_approve(SWAP_ROUTER, amount_in),
        &U::ZERO,
        u64::MAX,
        0
    ));
    let deadline = block_timestamp() + 1;
    let w: [u8; 32] = revert_if_bad_call_vec!(call_vec(
        SWAP_ROUTER,
        &make_fn_exact_input_single(
            token_in.into(),
            token_out.into(),
            sender,
            U::from(deadline),
            *amount_in,
            *amount_out_min,
            [255u8; 20] // This is U160::MAX
        ),
        &U::ZERO,
        u64::MAX,
        0
    ))[..32].try_into().unwrap();
    // I tried to have this resemble the reference, even though this isn't necessary.
    write_result_slice(&w);
    0
}
