macro_rules! storage {
    ($($name:ident($($param:ident),*)),* $(,)?) => {
        storage! {
            @internal
            counter: 2u8,
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

        pub fn add($param1: &U, x: &U) -> Option<()> {
            storage_checked_add(&slot_map(&U::from($counter), $param1), x)
        }

        pub fn sub($param1: &U, x: &U) -> Option<()> {
            storage_checked_sub(&slot_map(&U::from($counter), $param1), x)
        }
    };

    (@impl $counter:expr, [$param1:ident, $param2:ident]) => {
        pub fn get($param1: &U, $param2: &U) -> U {
            storage_load(&slot_map(&slot_map(&U::from($counter), $param1), $param2))
        }

        pub fn set($param1: &U, $param2: &U, x: &U) {
            storage_store(&slot_map(&slot_map(&U::from($counter), $param1), $param2), x)
        }

        pub fn add($param1: &U, $param2: &U, x: &U) -> Option<()> {
            storage_checked_add(&slot_map(&slot_map(&U::from($counter), $param1), $param2), x)
        }

        pub fn sub($param1: &U, $param2: &U, x: &U) -> Option<()> {
            storage_checked_sub(&slot_map(&slot_map(&U::from($counter), $param1), $param2), x)
        }
    };
}

pub mod initialised {
    use bobcat_sdk::{maths::U, storage::*};

    pub fn get() -> bool {
        storage_load_bool(&U::ZERO)
    }

    pub fn set(x: bool) {
        storage_store_bool(&U::ZERO, x)
    }
}

pub mod epoch {
    use bobcat_sdk::{maths::U, storage::*};

    pub fn get() -> U {
        storage_load(&U::ONE)
    }

    pub fn set(x: &U) {
        storage_store(&U::ONE, x)
    }

    pub fn incr() {
        storage_checked_add(&U::ONE, &U::ONE).unwrap()
    }
}

storage! {
    // The point where this contract is considered as having hit its
    // deadline, and won't run:
    ts_deadline(epoch),
    // Last amount invested's address.
    last_bettor_addr(epoch),
    // Last amount invested by a user.
    last_bettor_amt(epoch),
    fee_paid(epoch),
    // The entire amount invested in this epoch.
    pool_size(epoch),
    // The count of early purchases in this game.
    early_participants(epoch),
    // The global amount of tickets in circulation.
    global_tickets(epoch),
    // Very simple append-only storage of addresses that have played the game
    // this epoch. epoch => uint => address
    user_lottery_addresses(epoch, pos),
    // Amount of lottery tickets the user has. epoch => address => uint
    user_lottery_tickets(epoch, address),
    // Count of the users in the lottery tickets array.
    user_lottery_ticket_len(epoch),
}
