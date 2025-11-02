// 004-bozo: A game of chicken played with deposit amounts. Every
// hardcoded address is a contract deployed on Arbitrum One, the network
// this contract lives on.

#![cfg_attr(not(feature = "std"), no_std)]
#![no_main]

use core::cmp::min;

use bobcat_sdk::{
    call::{call_bool, safe_call_bool},
    cd::{address, const_keccak_sel, read_words},
    entry::{
        block_timestamp, contract_address, msg_sender, read_args_safe, write_result_slice,
        write_result_word,
    },
    events::emit,
    interfaces::{
        eip1967::{TOPIC_ADMIN_CHANGED, TOPIC_UPGRADED},
        eip20::{make_fn_transfer, make_fn_transfer_from},
    },
    maths::U,
    storage::{
        const_keccak256, const_slot_off_curve, flush_guard, keccak256, reentrancy_guard_sel,
        storage_load, storage_store,
    },
};

use array_concat::concat_arrays;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

pub mod storage;

type Address = [u8; 20];

/// Slot that contains the admin functionality, for an admin to come in
/// and replace the implementation. May be set to zero to prevent this
/// contract from being upgraded.
const SLOT_ADMIN: U = const_slot_off_curve(b"eip1967.proxy.admin");

/// Slot that contains the implementation address for the proxy to use.
const SLOT_IMPL: U = const_slot_off_curve(b"eip1967.proxy.implementation");

/// Operator that's able to trigger the reset cron.
const ADDR_OPERATOR: [u8; 20] = address!(b"6221a9c005f6e47eb398fd867784cacfdcfff4e7");

/// Asset that assets are converted to, to be used in the game. This is USDC.
const ADDR_ASSET: [u8; 20] = address!(b"af88d065e77c8cC2239327C5EDb3A432268e5831");

/// Event emitted when a deposit is made.
const TOPIC_DEPOSIT_MADE: U = const_keccak256(b"DepositMade(address,uint256,uint256)");

/// A winner was chosen for a game!
const TOPIC_WINNER_CHOSEN: U = const_keccak256(b"WinnerChosen(address,uint256,bool)");

/// Event emitted when the epoch is bumped.
const TOPIC_NEW_EPOCH: U = const_keccak256(b"NewEpoch(uint256)");

/// Fee taken from the users. 3% fee at a dividend
const FEE: U = U::from_u32(3);

/// An hour extra time.
const EXTRA_TIME: U = U::from_u32(3600);

// ~~~~~ View functions: ~~~~
//
const SEL_POOL_SIZE: [u8; 4] = const_keccak_sel(b"poolSize()");
const SEL_POOL_ASSET: [u8; 4] = const_keccak_sel(b"poolAsset()");
const SEL_LAST_BETTOR_AMOUNT: [u8; 4] = const_keccak_sel(b"lastBettorAmount()");
const SEL_LAST_BETTOR_ADDRESS: [u8; 4] = const_keccak_sel(b"lastBettorAddress()");
const SEL_DEADLINE: [u8; 4] = const_keccak_sel(b"deadline()");
const SEL_PLAYER_COUNT: [u8; 4] = const_keccak_sel(b"playerCount()");
const SEL_TICKET_COUNT: [u8; 4] = const_keccak_sel(b"ticketCount()");
const SEL_CURRENT_EPOCH: [u8; 4] = const_keccak_sel(b"currentEpoch()");
const SEL_WAS_EPOCH_COLLECTED: [u8; 4] = const_keccak_sel(b"wasEpochCollected(uint256)");

// ~~~~~ Stateful functions: ~~~~
//
const SEL_INIT: [u8; 4] = const_keccak_sel(b"initialise(address)");
const SEL_PLAY: [u8; 4] = const_keccak_sel(b"play(uint256,address,)");
const SEL_DISTRIBUTE_REWARDS: [u8; 4] =
    const_keccak_sel(b"distributeRewards(uint256,address,uint256)");
const SEL_UPGRADE: [u8; 4] = const_keccak_sel(b"upgrade(address)");
const SEL_CHANGE_ADMIN: [u8; 4] = const_keccak_sel(b"changeAdmin(address)");
const SEL_COLLECT_FEES: [u8; 4] = const_keccak_sel(b"collectFees()");

fn view_deadline() -> usize {
    write_result_word(&storage::ts_deadline::get(&pick_epoch().0));
    0
}

fn view_pool_size() -> usize {
    write_result_word(&storage::pool_size::get(&pick_epoch().0));
    0
}

fn view_pool_asset() -> usize {
    write_result_word(&U::from(ADDR_ASSET));
    0
}

fn view_last_bettor_amount() -> usize {
    write_result_word(&storage::last_bettor_amt::get(&pick_epoch().0));
    0
}

fn view_last_bettor_address() -> usize {
    write_result_word(&storage::last_bettor_addr::get(&pick_epoch().0));
    0
}

fn view_player_count() -> usize {
    write_result_word(&storage::user_lottery_ticket_len::get(&pick_epoch().0));
    0
}

fn view_ticket_count() -> usize {
    write_result_word(&storage::global_tickets::get(&pick_epoch().0));
    0
}

fn view_current_epoch() -> usize {
    write_result_word(&pick_epoch().0);
    0
}

fn view_was_epoch_collected() -> usize {
    write_result_word(&storage::was_distributed::get(&pick_epoch().0));
    0
}

const ONE_HUNDRED: U = U::from_u32(100);

fn state_init(admin: Address) -> usize {
    assert!(!storage::initialised::get(), "already created");
    storage_store(&SLOT_ADMIN, &admin.into());
    storage::initialised::set(true);
    0
}

// Get the current epoch by comparing the timestamp with the deadline. If
// we've exceeded the deadline, we return the next epoch, and a flag to
// indicate it should be set to the result returned from this function.
fn pick_epoch() -> (U, bool) {
    let e = storage::epoch::get();
    if block_timestamp() > storage::ts_deadline::get(&e).into() {
        (e + U::ONE, true)
    } else {
        (e, false)
    }
}

fn state_play(amt: U, recipient: Address) -> usize {
    assert!(amt.is_some(), "amount is zero");
    assert!(recipient != [0u8; 20], "recipient is zero");
    let timestamp = U::from(block_timestamp());
    let (epoch, needs_epoch_setting) = pick_epoch();
    if needs_epoch_setting {
        // If we've exceeded the timestamp, we need to set a new epoch.
        storage::epoch::set(&epoch);
        emit!(TOPIC_NEW_EPOCH, epoch);
    }
    // Transfer the asset to us:
    assert!(
        safe_call_bool(
            ADDR_ASSET,
            &make_fn_transfer_from(msg_sender(), contract_address(), &amt),
            &U::ZERO,
            u64::MAX,
        ),
        "transferFrom revert"
    );
    let fee_paid = amt.mul_div_round_up(&FEE, ONE_HUNDRED).unwrap();
    let amt = amt - fee_paid;
    // Get the last deposit made by a user to know how much to beat. Take 105%:
    let extra_amt = storage::last_bettor_amt::get(&epoch)
        .mul_div_round_up(&U::from(105u32), U::from(100u32))
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
    storage::last_bettor_addr::set(&epoch, &U::from(msg_sender()));
    storage::last_bettor_amt::set(&epoch, &amt);
    storage::fees_collected::add(&fee_paid);
    storage::pool_size::add(&epoch, &amt);
    storage::early_participants::add(&epoch, &U::ONE);
    storage::global_tickets::add(&epoch, &lottery_tickets);
    let recipient = U::from(recipient);
    let existing_tickets = storage::user_lottery_tickets::get(&epoch, &recipient);
    if existing_tickets.is_zero() {
        // If this is the first time that the recipient is playing, we need to track them:
        let ticket_len = storage::user_lottery_ticket_len::get(&epoch);
        storage::user_lottery_addresses::set(&epoch, &ticket_len, &recipient);
        storage::user_lottery_ticket_len::set(&epoch, &(ticket_len + U::ONE));
    }
    storage::user_lottery_tickets::set(
        &epoch,
        &recipient,
        &existing_tickets.checked_add(&lottery_tickets).unwrap(),
    );
    emit!(
        TOPIC_DEPOSIT_MADE,
        recipient,
        amt,
        storage::pool_size::get(&epoch)
    );
    storage::ts_deadline::add(&epoch, &(timestamp + EXTRA_TIME));
    let r: [u8; 32 * 2] = concat_arrays!(epoch.0, amt.0);
    write_result_slice(&r);
    0
}

fn state_distribute_rewards(epoch: &U, rng: &U) -> usize {
    assert_eq!(ADDR_OPERATOR, msg_sender(), "operator only");
    assert!(
        U::from(block_timestamp()) > storage::ts_deadline::get(&epoch),
        "not concluded"
    );
    // Take 80% of the pool, and send to the winning depositor:
    let last_bettor_addr: Address = storage::last_bettor_addr::get(&epoch).into();
    let ticket_count = storage::global_tickets::get(&epoch)
        - storage::user_lottery_tickets::get(&epoch, &last_bettor_addr.into());
    if ticket_count.is_zero() {
        // We only had one player! Let's transfer them the full amount, and stop.
        if last_bettor_addr != [0u8; 20] {
            let winner_amt = storage::pool_size::get(&epoch);
            assert!(
                call_bool(
                    ADDR_ASSET,
                    &make_fn_transfer(last_bettor_addr, &winner_amt),
                    &U::ZERO,
                    u64::MAX
                ),
                "transfer revert"
            );
            emit!(TOPIC_WINNER_CHOSEN, last_bettor_addr, winner_amt, false);
        }
        let r: [u8; 32 * 3] = concat_arrays!([0u8; 32], U::from(64u32).0, [0u8; 32]);
        write_result_slice(&r);
        return 0;
    }
    // If we had more than one player, we give the top 80% to the last user:
    let winner_reward = storage::pool_size::get(&epoch)
        .mul_div(&U::from(8u32), U::from(10u32))
        .unwrap()
        .0;
    assert!(
        call_bool(
            ADDR_ASSET,
            &make_fn_transfer(last_bettor_addr, &winner_reward),
            &U::ZERO,
            u64::MAX
        ),
        "transfer revert"
    );
    emit!(TOPIC_WINNER_CHOSEN, last_bettor_addr, winner_reward, false);
    // Using the random word, we start to pick some random words using
    // keccak. We're only ever going to see 10 winners at max, since we
    // divide the winnings up to at most 10 people. We take the 20%:
    let full_lottery_reward = storage::pool_size::get(&epoch)
        .mul_div(&U::from(2u32), U::from(10u32))
        .unwrap()
        .0;
    let ticket_len: usize = storage::user_lottery_ticket_len::get(&epoch).into();
    let max_winners = min(ticket_len, 10usize);
    let user_lottery_reward = full_lottery_reward / U::from(max_winners);
    let mut winners = [[0u8; 20]; 10];
    let mut i = 0usize;
    while max_winners > i {
        let rng_preimage: [u8; 32 + size_of::<usize>()] = concat_arrays!(rng.0, i.to_be_bytes());
        let rng = keccak256(&rng_preimage);
        // The amount outstanding that we can distribute here. We'll reduce this
        // until it's zero, then do the reward:
        let mut leftover_tickets = rng % ticket_count;
        // We prefer to use the storage, though we could actually just load this
        // in ourselves. We'll repeatedly use the storage cache instead to get
        // the elements we want.
        let player_count: usize = storage::user_lottery_ticket_len::get(&epoch).into();
        // The size of the players won't exceed usize, which should be u32 in our wasm.
        let mut p: usize = ((rng >> 64) % U::from(player_count)).into();
        loop {
            let p_u = U::from(p);
            let user_amt = storage::user_lottery_tickets::get(&epoch, &p_u);
            if user_amt >= leftover_tickets {
                let w: Address = storage::user_lottery_addresses::get(&epoch, &p_u).into();
                // Instead of swap/popping the storage, we do a search over the slice:
                let has_user_won_already = winners.contains(&w);
                if !has_user_won_already {
                    winners[i] = w;
                    // Transfer the winner their amount:
                    assert!(
                        call_bool(
                            ADDR_ASSET,
                            &make_fn_transfer(w, &user_lottery_reward),
                            &U::ZERO,
                            u64::MAX
                        ),
                        "transfer revert"
                    );
                    emit!(TOPIC_WINNER_CHOSEN, w, user_lottery_reward, true);
                    break;
                }
            }
            // We do a saturating sub so that if we saw the winner before, we set to
            // 0 safely:
            leftover_tickets = leftover_tickets.saturating_sub(&user_amt);
            p += 1;
            if p >= player_count {
                p = 0;
            }
        }
        i += 1;
    }
    storage::was_distributed::set(&epoch, &U::from(true));
    let r: [u8; 32 * 3] = concat_arrays!([0u8; 32], U::from(64u32).0, [0u8; 32]);
    write_result_slice(&r);
    0
}

fn state_upgrade(new_impl: Address) -> usize {
    assert_eq!(storage_load(&SLOT_ADMIN), msg_sender().into());
    storage_store(&SLOT_IMPL, &U::from(new_impl));
    emit!(TOPIC_UPGRADED, new_impl);
    0
}

fn state_change_admin(new_admin: Address) -> usize {
    let last_admin = storage_load(&SLOT_ADMIN);
    assert_eq!(last_admin, msg_sender().into());
    storage_store(&SLOT_ADMIN, &U::from(new_admin));
    emit!(TOPIC_ADMIN_CHANGED, last_admin, new_admin);
    0
}

fn state_collect_fees() -> usize {
    let f = storage::fees_collected::get();
    assert!(
        safe_call_bool(
            ADDR_ASSET,
            &make_fn_transfer(ADDR_OPERATOR, &f),
            &U::ZERO,
            u64::MAX
        ),
        "transfer revert"
    );
    storage::fees_collected::clear();
    write_result_word(&f);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    // Allocate the full amount that we will see possibly:
    let args = &read_args_safe!(args_len, { 32 * 5 + 4 });
    let sel: [u8; 4] = args[..4].try_into().unwrap();
    flush_guard(|| {
        match sel {
            // View functions:
            SEL_DEADLINE => view_deadline(),
            SEL_POOL_SIZE => view_pool_size(),
            SEL_POOL_ASSET => view_pool_asset(),
            SEL_LAST_BETTOR_AMOUNT => view_last_bettor_amount(),
            SEL_LAST_BETTOR_ADDRESS => view_last_bettor_address(),
            SEL_PLAYER_COUNT => view_player_count(),
            SEL_TICKET_COUNT => view_ticket_count(),
            SEL_CURRENT_EPOCH => view_current_epoch(),
            SEL_WAS_EPOCH_COLLECTED => view_was_epoch_collected(),
            // Side effect generating functions:
            SEL_INIT => {
                let admin = read_words!(&args[4..], 1);
                state_init(admin.into())
            }
            SEL_PLAY => reentrancy_guard_sel(&SEL_PLAY, || {
                let (amt, recipient) = read_words!(&args[4..], 2);
                state_play(*amt, recipient.into())
            }),
            SEL_DISTRIBUTE_REWARDS => {
                let (epoch, _recipient, rng) = read_words!(&args[4..], 3);
                state_distribute_rewards(epoch, rng)
            }
            SEL_UPGRADE => {
                let new_impl = read_words!(&args[4..], 1);
                state_upgrade(new_impl.into())
            }
            SEL_CHANGE_ADMIN => {
                let new_admin = read_words!(&args[4..], 1);
                state_change_admin(new_admin.into())
            }
            SEL_COLLECT_FEES => state_collect_fees(),
            _ => 1,
        }
    })
}
