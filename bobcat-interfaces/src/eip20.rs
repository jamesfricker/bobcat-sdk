use bobcat_maths::U;

use array_concat::concat_arrays;

use crate::sels::*;

type Address = [u8; 20];

pub const fn make_fn_total_supply() -> [u8; 4] {
    SEL_TOTAL_SUPPLY
}

pub const fn make_fn_balance_of(user: Address) -> [u8; 4 + 32] {
    concat_arrays!(SEL_BALANCE_OF, leftpad_addr(user))
}

pub const fn make_fn_allowance(owner: Address, spender: Address) -> [u8; 4 + 32 * 2] {
    concat_arrays!(SEL_ALLOWANCE, leftpad_addr(owner), leftpad_addr(spender))
}

pub const fn make_fn_transfer(recipient: Address, amt: &U) -> [u8; 4 + 32 * 2] {
    concat_arrays!(SEL_TRANSFER, leftpad_addr(recipient), amt.0)
}

pub const fn make_fn_transfer_from(from: Address, to: Address, amt: &U) -> [u8; 4 + 32 * 3] {
    concat_arrays!(SEL_TRANSFER_FROM, leftpad_addr(from), leftpad_addr(to), amt.0)
}

pub const fn make_fn_approve(spender: Address, amt: &U) -> [u8; 4 + 32 * 2] {
    concat_arrays!(SEL_APPROVE, leftpad_addr(spender), amt.0)
}