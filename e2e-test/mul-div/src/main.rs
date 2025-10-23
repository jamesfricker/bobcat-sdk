#![no_main]
#![no_std]

use bobcat_sdk::{
    cd::{const_keccak_sel, read_words},
    entry::*,
    maths::U,
};

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_primitives::U256;

const SEL_ONLINE: [u8; 4] = const_keccak_sel(b"online(uint256,uint256,uint256)");
const SEL_OFFLINE: [u8; 4] = const_keccak_sel(b"offline(uint256,uint256,uint256)");

fn r_mul_div(a: U256, b: U256, mut denom_and_rem: U256) -> Option<(U256, U256)> {
    if denom_and_rem == U256::ZERO {
        return None;
    }
    let mut mul_and_quo = a.widening_mul::<256, 4, 512, 8>(b);
    unsafe {
        ruint::algorithms::div(mul_and_quo.as_limbs_mut(), denom_and_rem.as_limbs_mut());
    }
    let limbs = mul_and_quo.into_limbs();
    if limbs[4..] != [0_u64; 4] {
        return None;
    }
    Some((U256::from_limbs_slice(&limbs[0..4]), denom_and_rem))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { (32 * 3) + 4 });
    let (x, y, z) = read_words!(&args[4..], 3);
    write_result_slice(&match args[..4].try_into().unwrap() {
        SEL_ONLINE => x.mul_div(&y, &z).unwrap().0 .0,
        SEL_OFFLINE => r_mul_div(
            U256::from_be_bytes(x.0),
            U256::from_be_bytes(y.0),
            U256::from_be_bytes(z.0),
        )
        .unwrap()
        .0
        .to_be_bytes(),
        _ => unimplemented!(),
    });
    0
}
