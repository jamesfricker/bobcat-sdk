#![no_main]
#![no_std]

use bobcat_sdk::{
    cd::{const_keccak_sel, read_word_slices},
    create::create1_slice,
    entry::*,
    maths::U,
    proxy::make_beacon_proxy,
};

const SEL: [u8; 4] = const_keccak_sel(b"deploy(address)");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { 32 + 4 });
    if args[..4] != SEL {
        return 1;
    }
    let beacon = read_word_slices!(&args[4..], 1);
    let r = create1_slice::<1024>(&make_beacon_proxy(beacon.into()), U::ZERO);
    write_result_exit_create!(r)
}
