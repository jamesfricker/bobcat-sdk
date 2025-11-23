#![no_std]

use bobcat_maths::U;

use bobcat_call::static_call_slice;

use array_concat::concat_arrays;

type Address = [u8; 20];

pub const ADDR_ECRECOVER: Address = U::ONE.const_addr();

// Upper bound of S to prevent malleability.
const ECRECOVER_MAX_S: U = U::const_from_hex(b"7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0").unwrap();

pub fn ecrecover(hash: U, v: u8, r: U, s: U, gas: u64) -> Option<Address> {
    if v != 27 && v != 28 {
        return None
    }
    if s > ECRECOVER_MAX_S {
        return None
    }
    let cd: [u8; 32 * 4] = concat_arrays!(hash.0, U::from(v).0, r.0, s.0);
    let (rc, _, rd) = static_call_slice::<20>(ADDR_ECRECOVER, &cd, gas, 32 - 20);
    if !rc {
        return None
    }
    Some(rd)
}
