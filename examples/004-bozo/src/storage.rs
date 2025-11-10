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
    owner_fees_collected(),
    dao_fees_collected(),
    ts_deadline(epoch),
    last_bettor_addr(epoch),
    last_bettor_amt(epoch),
    pool_size(epoch),
    early_participants(epoch),
    global_tickets(epoch),
    // Address => position for (has played . pos):
    user_lottery_pos(epoch, addr),
    // Position => address for ticket count:
    user_lottery_addresses(epoch, pos),
    // Position => tickets owned by user:
    user_lottery_tickets(epoch, pos),
    // Amount of tickets in circulation:
    user_lottery_ticket_len(epoch),
    // Whether this epoch had its winners given out:
    was_distributed(epoch),
    // The points this address owns:
    points_collected(address),
    // The asset in use:
    asset()
}
