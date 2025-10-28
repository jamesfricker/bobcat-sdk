#![no_main]

use ruint::aliases::U256;

use bobcat_maths::U;

use libfuzzer_sys::{arbitrary, fuzz_target};

use arbitrary::Arbitrary;

// Though the rooti implementation could feature any combination of U, we
// do it this way to reduce the search space of the inputs that
// will likely take place in practice. We don't check so much whether the code
// here is blowing out when it shouldn't.
#[derive(Arbitrary, Debug)]
struct Root {
    x: U,
    y: u32,
}

fuzz_target!(|t: Root| {
    let Root { x, y } = t;
    let Some(r) = x.checked_rooti(y) else {
        return;
    };
    if y == 0 {
        assert_eq!(U::ZERO, r);
    }
    assert_eq!(
        U::from(
            U256::from_be_slice(&x.0)
                .root(y as usize)
                .to_be_bytes::<32>()
        ),
        r
    );
});
