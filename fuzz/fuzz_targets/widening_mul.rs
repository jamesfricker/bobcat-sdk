#![no_main]

use libfuzzer_sys::{
    arbitrary::{self, Arbitrary},
    fuzz_target,
};

use bobcat_maths::U;

use ruint::aliases::{U256, U512};

use array_concat::concat_arrays;

#[derive(Arbitrary, Debug)]
struct Mul {
    x: U,
    y: U,
}

fuzz_target!(|data: Mul| {
    let x = data.x;
    let y = data.y;
    let e: U512 = U256::from_be_bytes(x.0).widening_mul(U256::from_be_bytes(y.0));
    let (zx, zy) = x.widening_mul(&y);
    let z = concat_arrays!(zx.0, zy.0);
    assert_eq!(e.to_be_bytes(), z);
});
