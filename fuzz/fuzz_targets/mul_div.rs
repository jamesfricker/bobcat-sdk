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

fn r_mul_div(a: U256, b: U256, mut denom_and_rem: U256) -> Option<(U256, U256)> {
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
    Some((U256::from_limbs_slice(&limbs[0..4]), denom_and_rem))
}

/* let (l, r) = x.widening_mul(&y);
let mut v = [0u8; 64];
v[..32].copy_from_slice(&l.0);
v[32..].copy_from_slice(&r.0);
assert_eq!(
    U256::from_be_bytes::<32>(x.0)
        .widening_mul::<256, 4, 512, 8>(U256::from_be_bytes::<32>(y.0))
        .to_be_bytes::<64>(),
    v
); */

fuzz_target!(|mul: MulDiv| {
    let MulDiv { x, y, z } = mul;
    let e = r_mul_div(
        U256::from_be_bytes(x.0),
        U256::from_be_bytes(y.0),
        U256::from_be_bytes(z.0),
    );
    let v = x.mul_div(&y, &z);
    match (e, v) {
        (Some((e, _)), Some((v, _))) => assert_eq!(e.to_be_bytes::<32>(), v.0, "{e} ( != {v}"),
        (None, None) => (),
        (l, r) => {
            panic!("{l:?} != {r:?}");
        }
    }
});
