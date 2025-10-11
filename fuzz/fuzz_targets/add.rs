#![no_main]

use libfuzzer_sys::{
    arbitrary::{self, Arbitrary},
    fuzz_target,
};

use bobcat_maths::U;

use ruint::Uint;

type U256 = Uint<256, 4>;

#[derive(Arbitrary, Debug)]
struct Add {
    x: U,
    y: U,
}

fuzz_target!(|data: Add| {
    let v = data.x + data.y;
    let e = U256::from_be_bytes(data.x.0).wrapping_add(U256::from_be_bytes(data.y.0));
    assert_eq!(v.0, e.to_be_bytes(), "{e} != {v}");
});
