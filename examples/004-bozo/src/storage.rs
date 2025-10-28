macro_rules! storage {
    ($($name:ident($($param:ident),*)),* $(,)?) => {
        storage! {
            @internal
            counter: 1u8,
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
            use bobcat_sdk::storage::*;
            use bobcat_sdk::maths::U;

            storage!(@impl $counter, [$($param),*]);
        }

        $(
            storage! {
                @internal
                counter: ($counter + 1u8),
                items: [$($rest)*]
            }
        )?
    };

    (@impl $counter:expr, [$param1:ident]) => {
        pub fn get($param1: &U) -> U {
            storage_load(&slot_map(&U::from($counter), $param1))
        }

        pub fn set($param1: &U, x: &U) {
            storage_store(&slot_map(&U::from($counter), $param1), x)
        }

        pub fn add($param1: &U, x: &U) {
            storage_add(&slot_map(&U::from($counter), $param1), x)
        }

        pub fn sub($param1: &U, x: &U) {
            storage_sub(&slot_map(&U::from($counter), $param1), x)
        }
    };

    (@impl $counter:expr, [$param1:ident, $param2:ident]) => {
        pub fn get($param1: &U, $param2: &U) -> U {
            storage_load(&slot_map(&slot_map(&U::from($counter), $param1), $param2))
        }

        pub fn set($param1: &U, $param2: &U, x: &U) {
            storage_store(&slot_map(&slot_map(&U::from($counter), $param1), $param2), x)
        }

        pub fn add($param1: &U, $param2: &U, x: &U) {
            storage_add(&slot_map(&slot_map(&U::from($counter), $param1), $param2), x)
        }

        pub fn sub($param1: &U, $param2: &U, x: &U) {
            storage_sub(&slot_map(&slot_map(&U::from($counter), $param1), $param2), x)
        }
    };
}

pub mod epoch {
    use bobcat_sdk::{maths::U, storage::*};

    pub fn get() -> U {
        storage_load(&U::ZERO)
    }

    pub fn set(x: &U) {
        storage_store(&U::ZERO, x)
    }
}

storage! {
    last_bettor_addr(epoch),
    last_bettor_amt_usd(epoch),
    fee_paid(epoch),
    pool_size(epoch),
    early_participants(epoch),
    global_tickets(epoch),
    user_lottery_tickets(epoch, user),
    global_lottery_tickets(epoch),
    lottery_paid_out(epoch),
    lottery_ticket_search(epoch, user),
    assets_supplied(epoch, number),
    assets_supplied_count(epoch)
}
