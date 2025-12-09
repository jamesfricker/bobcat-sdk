use bobcat_maths::U;

use bobcat_cd::address;

use bobcat_call::static_call_unit;

use array_concat::concat_arrays;

/// EdVerify is deployed at this address to Arbitrum One and Superposition.
pub const ADDR_EDVERIFY: [u8; 20] = address!(b"c3e443be2cfa4f41a5f5e4978d012847d355b419");

// Gas for the edverify function assumes the contract is a part of the
// Stylus cache. If it's not, this may fail.
const GAS_EDVERIFY: u64 = 93715;

pub fn edverify(digest: [u8; 64], pub_key: U, sig: [u8; 64]) -> bool {
    let cd: [u8; 64 * 2 + 32] = concat_arrays!(digest, pub_key.0, sig);
    static_call_unit(ADDR_EDVERIFY, &cd, GAS_EDVERIFY)
}
