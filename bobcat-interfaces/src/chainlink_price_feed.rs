use crate::selectors;

selectors! {
    SEL_DECIMALS = b"decimals()",
    SEL_LATEST_ROUND_DATA = b"latestRoundData()"
}

pub const fn make_fn_decimals() -> [u8; 4] {
    SEL_DECIMALS
}

pub const fn make_latest_round_data() -> [u8; 4] {
    SEL_LATEST_ROUND_DATA
}
