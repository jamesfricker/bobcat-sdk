#![no_main]

use libfuzzer_sys::{
    arbitrary::{self, Arbitrary},
    fuzz_target,
};

use bobcat_maths::U;

use ruint::aliases::U256;

#[derive(Arbitrary, Debug)]
struct Div {
    x: U,
    y: U,
}

macro_rules! assert_eq_t {
    ($e:expr, $x:expr, $($o:expr),*) => {
        assert_eq!($e.to_be_bytes::<32>(), $x.0, $($o),*)
    };
}

fuzz_target!(|data: Div| {
    let ex = U256::from_be_bytes(data.x.0);
    let ey = U256::from_be_bytes(data.y.0);
    let Div { x, y } = data;
    if y.is_some() {
        assert_eq_t!(ex.wrapping_div(ey), x.wrapping_div(&y),);
    }
    match (ex.checked_div(ey), bobcat_maths::checked_div(&x, &y)) {
        (None, None) => (),
        (Some(x), Some(y)) => {
            assert_eq_t!(x, y, "{x} != {y} ({}, {})", data.x, data.y)
        }
        (x, y) => panic!("bad checked, {x:?} != {y:?}"),
    }
});
