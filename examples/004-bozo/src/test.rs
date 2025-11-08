// For bozo, we use arbos-foundry to test the end to end application. We
// use Rust tests to test the math in an isolated way.

use proptest::prelude::*;

use bobcat_sdk::maths::U;

use crate::get_min_deposit;

proptest! {
    #[test]
    fn test_get_min_deposit_similar_to_ref(pool_size in any::<u64>(), last_deposit in any::<u64>()) {
        let floor_pct = 0.005;
        let escalation_multiplier = 1.05;
        let cap_pct = 0.02;
        let floor = floor_pct * pool_size as f64;
        let escalation = escalation_multiplier * last_deposit as f64;
        let cap = cap_pct * pool_size as f64;
        let e = {
            let m = if escalation > cap { cap } else { escalation };
            if floor > m { floor } else { m }
        };
        let v = get_min_deposit(&U::from(pool_size), &U::from(last_deposit)).unwrap();
        let e = U::from(e as u64);
        let d = if e > v { e - v } else { v - e };
        assert!(U::from_u32(100) > d, "{e} != {v}");
    }
}
