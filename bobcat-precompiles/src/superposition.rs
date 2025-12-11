use bobcat_maths::U;

use bobcat_cd::address;

use bobcat_call::static_call_unit;

use array_concat::concat_arrays;

/// EdVerify is deployed at this address on Arbitrum One and Superposition.
pub const ADDR_EDVERIFY: [u8; 20] = address!(b"c3e443be2cfa4f41a5f5e4978d012847d355b419");

/// Muldiv is deployed at this address on Arbitrum One and Superposition.
pub const ADDR_MUL_DIV: [u8; 20] = address!(b"6c483d05266cda72cfe72643a79ad531d9b52cd5");

// Gas for the edverify function assumes the contract is a part of the
// Stylus cache. If it's not, this may fail.
const GAS_EDVERIFY: u64 = 95715;

const GAS_MUL_DIV: u64 = 9000;

pub fn edphverify(digest: [u8; 64], pub_key: U, sig: [u8; 64]) -> bool {
    let cd: [u8; 64 * 2 + 32] = concat_arrays!(digest, pub_key.0, sig);
    static_call_unit(ADDR_EDVERIFY, &cd, GAS_EDVERIFY)
}

pub fn edphverify(x: U, y: U, z: U) -> bool {
    let cd: [u8; 3 * 32] = concat_arrays!(x.0, y.0, z.0);
    static_call_unit(ADDR_MUL_DIV, GAS_MUL_DIV, &cd, u64::MAX)
}
