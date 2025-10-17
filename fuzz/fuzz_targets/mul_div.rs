#![no_main]

use libfuzzer_sys::{
    arbitrary::{self, Arbitrary},
    fuzz_target,
};

use bobcat_maths::{U};

use ruint::aliases::{U256};

#[derive(Arbitrary, Debug)]
struct MulDiv {
    x: U,
    y: U,
    z: U,
}

fn r_mul_div(a: U256, b: U256, mut denom_and_rem: U256) -> Option<(U256, bool)> {
    if denom_and_rem == U256::ZERO {
        return None;
    }
    let mut mul_and_quo = a.widening_mul::<256, 4, 512, 8>(b);
    unsafe {
        ruint::algorithms::div(mul_and_quo.as_limbs_mut(), denom_and_rem.as_limbs_mut());
    }
    let limbs = mul_and_quo.into_limbs();
    if limbs[4..] != [0_u64; 4] {
        return None;
    }
    let has_carry = denom_and_rem != U256::ZERO;
    Some((U256::from_limbs_slice(&limbs[0..4]), has_carry))
}

fuzz_target!(|mul: MulDiv| {
    let MulDiv { x, y, z } = mul;
    let e = r_mul_div(
        U256::from_be_bytes(x.0),
        U256::from_be_bytes(y.0),
        U256::from_be_bytes(z.0),
    );
    let v = x.mul_div(&y, &z);
    match (e, v) {
        (Some((e, e0)), Some((v, v0))) => {
            assert_eq!((e.to_be_bytes::<32>(), e0), (v.0, v0), "{e} != {v}")
        },
        (None, None) => (),
        (l, r) => {
            panic!("{l:?} != {r:?}");
        }
    }
});
