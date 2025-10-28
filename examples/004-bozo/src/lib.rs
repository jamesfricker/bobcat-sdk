#![cfg_attr(not(feature = "std"), no_std)]

use bobcat_sdk::{
    cd::{address, const_keccak_sel},
    entry::{read_args_safe, write_result_word},
    maths::{mul_div_round_up, U},
};

pub mod storage;

/// Asset that assets are converted to, to be used in the game.
const ASSET: [u8; 20] = address!(b"af88d065e77c8cC2239327C5EDb3A432268e5831");

/// Fee taken from the users. 3% fee at a dividend
const FEE: U = U::from(3u32);

// ~~~~~ View functions: ~~~~
//
const SEL_POOL_SIZE: [u8; 4] = const_keccak_sel(b"poolSize()");
const SEL_POOL_ASSET: [u8; 4] = const_keccak_sel(b"poolAsset()");

// ~~~~~ Stateful functions: ~~~~
//
const SEL_PLAY: [u8; 4] = const_keccak_sel(b"play(uint256,address)");
const SEL_DISTRIBUTE_REWARDS: [u8; 4] = const_keccak_sel(b"distributeRewards(address)");

fn view_pool_size() -> usize {
    write_result_word(&storage::pool_size::get(&storage::epoch::get()));
    0
}

fn view_pool_asset() -> usize {
    write_result_word(&U::from(ASSET));
    0
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Err {
    AmtIsZero,
}

impl From<Err> for u8 {
    fn from(x: Err) -> Self {
        x as u8
    }
}

macro_rules! require {
    ($chk:expr, $msg:expr) => {
        if !$chk {
            let mut b = [0u8; 32];
            b[31] = $msg.into();
            write_result_slice(b);
            return 1
        }
    }
}

const ONE_HUNDRED: U = U::from(100u32);

fn state_play(amt: mut U, recipient: Address) -> usize {
    require!(amt.is_some(), Err::AmtIsZero);
    let fee_paid = amt.mul_div_round_up(&FEE, &ONE_HUNDRED).unwrap();
    storage::
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { 32 + 4 });
    let sel: [u8; 4] = args[..4].try_into().unwrap();
    match sel {
        // View functions:
        SEL_POOL_SIZE => view_pool_size(),
        SEL_POOL_ASSET => view_pool_asset(),
        _ => 1,
    }
}
