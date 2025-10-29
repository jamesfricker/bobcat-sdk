#![cfg_attr(not(feature = "std"), no_std)]
#![no_main]

use bobcat_sdk::{
    call::{call_bool, call_word_err_vec},
    cd::{address, const_keccak_sel, read_words},
    entry::{
        contract_address, msg_sender, read_args_safe, revert_if_bad_call_slice_vec,
        write_result_word
    },
    interfaces::{
        camelotv3_swap_router::make_fn_exact_input_single,
        eip20::{make_fn_approve, make_fn_transfer_from},
    },
    maths::U,
    storage::{flush_guard, reentrancy_guard_sel},
};

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

pub mod storage;

type Address = [u8; 20];

/// Asset that assets are converted to, to be used in the game.
const ASSET: [u8; 20] = address!(b"af88d065e77c8cC2239327C5EDb3A432268e5831");

/// Fee taken from the users. 3% fee at a dividend
const FEE: U = U::from_u32(3);

// ~~~~~ View functions: ~~~~
//
const SEL_POOL_SIZE: [u8; 4] = const_keccak_sel(b"poolSize()");
const SEL_POOL_ASSET: [u8; 4] = const_keccak_sel(b"poolAsset()");

// ~~~~~ Stateful functions: ~~~~
//
const SEL_PLAY: [u8; 4] = const_keccak_sel(b"play(address,uint256,uint256,uint256,address)");
const SEL_DISTRIBUTE_REWARDS: [u8; 4] = const_keccak_sel(b"distributeRewards(address)");

fn view_pool_size() -> usize {
    write_result_word(&storage::pool_size::get(&storage::epoch::get()));
    0
}

fn view_pool_asset() -> usize {
    write_result_word(&U::from(ASSET));
    0
}

const ONE_HUNDRED: U = U::from_u32(100);

const ADDR_CAMELOT_SWAP_ROUTER: Address = address!(b"1f721e2e82f6676fce4ea07a5958cf098d339e18");

fn state_play(
    asset: Address,
    camelot_min_asset_out: &U,
    camelot_deadline: &U,
    mut amt: U,
    _recipient: Address,
) -> usize {
    assert!(amt.is_some(), "amount is zero");
    if asset != ASSET {
        // Transfer the asset to us:
        assert!(
            // FIXME: there's a bug in arbos-foundry with codesize checking right
            // now. Use safe_call_bool instead.
            call_bool(
                asset,
                &make_fn_transfer_from(msg_sender(), contract_address(), &amt),
                &U::ZERO,
                u64::MAX,
            ),
            "transferFrom revert"
        );
        // Approve the swap router so that we can spend this using a call:
        assert!(
            call_bool(
                asset,
                &make_fn_approve(ADDR_CAMELOT_SWAP_ROUTER, &amt),
                &U::ZERO,
                u64::MAX
            ),
            "approval revert"
        );
        // Swap so that we may receive some of the asset in use here:
        amt = revert_if_bad_call_slice_vec!(call_word_err_vec(
            ADDR_CAMELOT_SWAP_ROUTER,
            &make_fn_exact_input_single(
                asset,
                ASSET,
                contract_address(),
                *camelot_deadline,
                amt,
                *camelot_min_asset_out,
                [0u8; 20],
            ),
            &U::ZERO,
            u64::MAX
        ));
    }
    let fee_paid = amt.mul_div_round_up(&FEE, ONE_HUNDRED).unwrap();
    let epoch = storage::epoch::get();
    // Track the user's fee earned:
    storage::fee_paid::add(&epoch, &fee_paid);
    // Get the last deposit made by a user to know how much to beat:
    let extra_amt = storage::last_bettor_amt::get(&epoch)
        .mul_div_round_up(&U::from(5u32), U::from(100u32))
        .unwrap();
    assert!(amt > extra_amt, "amount not enough: {extra_amt} needed");
    // Figure out how many "lottery tickets" to give the user -- aka, the
    // chance of them winning 20% of the prize without actually being the
    // one to win.
    let lottery_tickets = if U::from(100u32) > storage::early_participants::get(&epoch) {
        // Since we have less than 100 participants, give the user extra tickets!
        amt.mul_div(&U::from(190u32), U::from(100u32)).unwrap().0
    } else {
        amt
    };
    storage::global_tickets::add(&epoch, &lottery_tickets);
    storage::pool_size::add(&epoch, &amt);
    storage::last_bettor_addr::set(&epoch, &U::from(msg_sender()));
    storage::last_bettor_amt::set(&epoch, &amt);
    write_result_word(&epoch);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    // Allocate the full amount that we will see possibly:
    let args = &read_args_safe!(args_len, { 32 * 5 + 4 });
    let sel: [u8; 4] = args[..4].try_into().unwrap();
    match sel {
        // View functions:
        SEL_POOL_SIZE => view_pool_size(),
        SEL_POOL_ASSET => view_pool_asset(),
        SEL_PLAY => flush_guard(|| {
            reentrancy_guard_sel(&SEL_PLAY, || {
                let (asset, camelot_min_asset_out, camelot_deadline, amt, recipient) =
                    read_words!(&args[4..], 5);
                state_play(
                    asset.into(),
                    camelot_min_asset_out,
                    camelot_deadline,
                    *amt,
                    recipient.into(),
                )
            })
        }),
        SEL_DISTRIBUTE_REWARDS => 0,
        _ => 1,
    }
}
