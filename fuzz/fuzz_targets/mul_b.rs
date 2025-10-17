#![no_main]

use libfuzzer_sys::{
    arbitrary::{self, Arbitrary},
    fuzz_target,
};

use bobcat_maths::{wrapping_mul_b};

use ruint::aliases::U512;

#[derive(Arbitrary, Debug)]
struct Mul {
    x: [u8; 64],
    y: [u8; 64],
}

macro_rules! assert_eq_t {
    ($e:expr, $x:expr, $($o:expr),*) => {
        assert_eq!($e.to_be_bytes::<64>(), $x, $($o),*)
    };
}

fuzz_target!(|data: Mul| {
    let ex = U512::from_be_bytes(data.x);
    let ey = U512::from_be_bytes(data.y);
    let Mul { x, y } = data;
    if y == [0u8; 64] {
        return
    }
    assert_eq_t!(ex.wrapping_mul(ey), wrapping_mul_b(&x, &y),);
});
