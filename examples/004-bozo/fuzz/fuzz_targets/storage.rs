#![no_main]

use libfuzzer_sys::{
    arbitrary::{self, Arbitrary},
    fuzz_target,
};

use libbozo::{bobcat_sdk::maths::U, storage::*};

#[derive(Debug, Arbitrary, Clone, Copy, PartialEq)]
pub enum Slot {
    Initialised,
    Epoch,
    OwnerFeesCollected,
    DaoFeesCollected,
    TsDeadline(U),
    LastBettorAddr(U),
    LastBettorAmt(U),
    PoolSize(U),
    EarlyParticipants(U),
    GlobalTickets(U),
    UserLotteryPos(U, U),
    UserLotteryAddresses(U, U),
    UserLotteryTickets(U, U),
    UserLotteryTicketLen(U),
    WasDistributed(U),
    PointsCollected(U),
    Asset,
}

fn slot(w: Slot) -> U {
    match w {
        Slot::Initialised => initialised::slot(),
        Slot::Epoch => epoch::slot(),
        Slot::OwnerFeesCollected => owner_fees_collected::slot(),
        Slot::DaoFeesCollected => dao_fees_collected::slot(),
        Slot::TsDeadline(epoch) => ts_deadline::slot(&epoch),
        Slot::LastBettorAddr(epoch) => last_bettor_addr::slot(&epoch),
        Slot::LastBettorAmt(epoch) => last_bettor_amt::slot(&epoch),
        Slot::PoolSize(epoch) => pool_size::slot(&epoch),
        Slot::EarlyParticipants(epoch) => early_participants::slot(&epoch),
        Slot::GlobalTickets(epoch) => global_tickets::slot(&epoch),
        Slot::UserLotteryPos(epoch, addr) => user_lottery_pos::slot(&epoch, &addr),
        Slot::UserLotteryAddresses(epoch, pos) => user_lottery_addresses::slot(&epoch, &pos),
        Slot::UserLotteryTickets(epoch, pos) => user_lottery_tickets::slot(&epoch, &pos),
        Slot::UserLotteryTicketLen(epoch) => user_lottery_ticket_len::slot(&epoch),
        Slot::WasDistributed(epoch) => was_distributed::slot(&epoch),
        Slot::PointsCollected(address) => points_collected::slot(&address),
        Slot::Asset => asset::slot(),
    }
}

#[derive(Debug, Arbitrary, Clone, Copy, PartialEq)]
struct Args {
    x: Slot,
    y: Slot,
}

fuzz_target!(|d: Args| {
    let Args { x, y } = d;
    if x == y {
        return;
    }
    assert!(slot(x) != slot(y), "{} == {}", slot(x), slot(y));
});
