use bobcat_storage::keccak256;

use bobcat_call::{static_call_slice, static_call_word};

use bobcat_maths::U;

use array_concat::concat_arrays;

type Address = [u8; 20];

pub const ADDR_ECRECOVER: Address = U::ONE.const_addr();

pub const ADDR_SECP256R1: Address = U::from_u32(256).const_addr();

pub const GAS_ECRECOVER: u64 = u64::MAX;

pub const GAS_SECP256: u64 = u64::MAX;

// Upper bound of S to prevent malleability.
#[allow(unused)]
const ECRECOVER_MAX_S: U =
    U::const_from_hex(b"7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0").unwrap();

pub fn ecrecover_pre(preimage: &[u8], v: u8, r: U, s: U) -> Option<Address> {
    ecrecover_post(keccak256(preimage), v, r, s)
}

pub fn ecrecover_post(hash: U, v: u8, r: U, s: U) -> Option<Address> {
    if v != 27 && v != 28 {
        return None;
    }
    if s > ECRECOVER_MAX_S {
        return None;
    }
    let cd: [u8; 32 * 4] = concat_arrays!(hash.0, U::from(v).0, r.0, s.0);
    let (rc, _, rd) = static_call_slice::<20>(ADDR_ECRECOVER, &cd, GAS_ECRECOVER, 32 - 20);
    if !rc {
        return None;
    }
    Some(rd)
}

pub fn secp256r1_post(hash: U, r: U, s: U, qx: U, qy: U) -> Option<U> {
    let cd: [u8; 32 * 5] = concat_arrays!(hash.0, r.0, s.0, qx.0, qy.0);
    let (rc, rd) = static_call_word(ADDR_SECP256R1, &cd, GAS_SECP256, 0);
    if !rc {
        return None;
    }
    Some(rd)
}

pub fn secp256r1_pre(preimage: &[u8], r: U, s: U, qx: U, qy: U) -> Option<U> {
    secp256r1_post(keccak256(preimage), r, s, qx, qy)
}
