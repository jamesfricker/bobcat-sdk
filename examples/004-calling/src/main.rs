#![no_main]
#![no_std]

use bobcat_sdk::{
    call::static_call_slice,
    entry::*,
    interfaces::eip20::make_fn_allowance,
    maths::U,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { 32 * 3 + 4 });
    let (contract, owner, spender) = read_word_slices!(&args[4..], 3);
    write_result_exit_call!(static_call_slice::<32>(
        contract.into(),
        &make_fn_allowance(owner.into(), spender.into()),
        u64::MAX,
        0,
    ))
}
