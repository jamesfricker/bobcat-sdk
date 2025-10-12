#![no_main]

use libfuzzer_sys::{
    arbitrary::{self, Arbitrary},
    fuzz_target,
};

use bobcat_maths::U;

use ruint::aliases::U256;

#[derive(Arbitrary, Debug)]
struct Mul {
    x: U,
    y: U,
}

fuzz_target!(|data: Mul| {
    match (
        U256::from_be_bytes(data.x.0).checked_mul(U256::from_be_bytes(data.y.0)),
        bobcat_maths::checked_mul(&data.x, &data.y),
    ) {
        (None, None) => (),
        (Some(x), Some(y)) => {
            assert_eq!(x.to_be_bytes(), y.0, "{x} != {y} ({}, {})", data.x, data.y)
        },
        (x, y) => panic!("bad checked, {x:?} != {y:?}")
    }
});
