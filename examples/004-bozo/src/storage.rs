macro_rules! storage {
    ($($name:ident($($param:ident),*)),* $(,)?) => {
        storage! {
            @internal
            counter: 0,
            items: [$($name($($param),*)),*]
        }
    };
    (@internal
        counter: $counter:expr,
        items: []
    ) => {};
    (@internal
        counter: $counter:expr,
        items: [$name:ident($($param:ident),*) $(, $($rest:tt)*)?]
    ) => {
        pub mod $name {
            pub(crate) use bobcat_sdk::storage::*;
            use bobcat_sdk::maths::{u, U};

            const SLOT: U = u!($counter);

            storage!(@impl [$($param),*]);
        }
        $(
            storage! {
                @internal
                counter: $counter + 1,
                items: [$($rest)*]
            }
        )?
    };
    (@impl []) => {
        pub fn get() -> U {
            storage_load(&SLOT)
        }
        pub fn set(x: &U) {
            storage_store(&SLOT, x)
        }
        pub fn add(x: &U) -> Option<()> {
            storage_checked_add(&SLOT, x)
        }
        pub fn sub(x: &U) -> Option<()> {
            storage_checked_sub(&SLOT, x)
        }
        pub fn clear() {
            set(&U::ZERO)
        }
    };
    (@impl [$param1:ident]) => {
        pub fn get($param1: &U) -> U {
            storage_load(&slot_map(&SLOT, $param1))
        }
        pub fn set($param1: &U, x: &U) {
            storage_store(&slot_map(&SLOT, $param1), x)
        }
        pub fn add($param1: &U, x: &U) -> Option<()> {
            storage_checked_add(&slot_map(&SLOT, $param1), x)
        }
        pub fn sub($param1: &U, x: &U) -> Option<()> {
            storage_checked_sub(&slot_map(&SLOT, $param1), x)
        }
    };
    (@impl [$param1:ident, $param2:ident]) => {
        pub fn get($param1: &U, $param2: &U) -> U {
            storage_load(&slot_map(&slot_map(&SLOT, $param1), $param2))
        }
        pub fn set($param1: &U, $param2: &U, x: &U) {
            storage_store(&slot_map(&slot_map(&SLOT, $param1), $param2), x)
        }
        pub fn add($param1: &U, $param2: &U, x: &U) -> Option<()> {
            storage_checked_add(&slot_map(&slot_map(&SLOT, $param1), $param2), x)
        }
        pub fn sub($param1: &U, $param2: &U, x: &U) -> Option<()> {
            storage_checked_sub(&slot_map(&slot_map(&SLOT, $param1), $param2), x)
        }
    };
}

storage! {
    initialised(),
    epoch(),
    fees_collected(),
    ts_deadline(epoch),
    last_bettor_addr(epoch),
    last_bettor_amt(epoch),
    pool_size(epoch),
    early_participants(epoch),
    global_tickets(epoch),
    user_lottery_addresses(epoch, pos),
    user_lottery_tickets(epoch, address),
    user_lottery_ticket_len(epoch),
    was_distributed(epoch),
    points_collected(address),
    asset()
}