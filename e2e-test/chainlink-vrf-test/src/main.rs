// chainlink-vrf-test: Mock Chainlink VRF with the help of arbos-foundry.
// Test to see if things would work under mocked circumstances for an
// interaction this way.

#![no_std]
#![no_main]

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use bobcat_sdk::{
    call::call_word_err_vec, cd::*, entry::*,
    interfaces::chainlink_vrf::make_fn_request_words_in_native_no_bytes, storage::*,
};

type Address = [u8; 20];

pub const ADDR_CHAINLINK_VRF_COORDINATOR_SEPOLIA: Address =
    address!(b"50d47e4142598E3411aA864e08a44284e471AC6f");

pub const SEL_INITIATE: [u8; 4] = const_keccak_sel(b"initiate()");

pub const SEL_RAW_FULFILL_RANDOM_WORDS: [u8; 4] =
    const_keccak_sel(b"rawFulfillRandomWords(uint256,uint256[])");

pub const SEL_WAS_CALLED: [u8; 4] =
    const_keccak_sel(b"wasCalled()");

// Number of words we're going to request from Chainlink.
pub const WORD_COUNT: usize = 5;

// We're going to request 5 words from Chainlink. We need space for the
// length, the offset:
pub const WORD_BUFFER: usize = (WORD_COUNT + 2) * 32;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { WORD_BUFFER + 4 });
    let sel: [u8; 4] = args[..4].try_into().unwrap();
    match sel {
        SEL_INITIATE => {
            // This allocates a word for a simple U256 return, or reverts with a vec
            // if that's what's needed. For the allocation of the request for the
            // random words, we don't need any values, so we use the simple version.
            write_result_word(&revert_if_bad_call_slice_vec!(call_word_err_vec(
                ADDR_CHAINLINK_VRF_COORDINATOR_SEPOLIA,
                &make_fn_request_words_in_native_no_bytes(100, 2, WORD_COUNT as u32),
                &U::ZERO,
                u64::MAX
            )));
            0
        }
        SEL_RAW_FULFILL_RANDOM_WORDS => {
            storage_store(&U::ZERO, &U::from(true));
            0
        }
        SEL_WAS_CALLED => {
            write_result_word(&storage_load(&U::ZERO));
            0
        }
        _ => 1
    }
}
