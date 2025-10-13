#![cfg_attr(not(feature = "std"), no_std)]

use keccak_const::Keccak256;

use array_concat::concat_arrays;

pub use bobcat_maths::U;

pub type Address = [u8; 32];

#[link(wasm_import_module = "vm_hooks")]
#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn storage_load_bytes32(key: *const u8, out: *mut u8);
    fn storage_cache_bytes32(key: *const u8, from: *const u8);
    fn transient_load_bytes32(key: *const u8, dest: *mut u8);
    fn transient_store_bytes32(key: *const u8, value: *const u8);
    fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8);
    fn storage_flush_cache(clear: bool);
}

#[cfg(all(not(target_arch = "wasm32"), feature = "std"))]
mod host {
    use super::*;

    use std::{cell::RefCell, collections::HashMap, ptr::copy_nonoverlapping};

    type WordHashMap = HashMap<U, U>;

    thread_local! {
        static STORAGE: RefCell<WordHashMap> = RefCell::default();
        static TRANSIENT: RefCell<WordHashMap> = RefCell::default();
    }

    unsafe fn read_word(key: *const u8) -> U {
        let mut r = [0u8; 32];
        unsafe {
            copy_nonoverlapping(key, r.as_mut_ptr(), 32);
        }
        U(r)
    }

    unsafe fn write_word(key: *mut u8, val: U) {
        unsafe {
            copy_nonoverlapping(val.as_ptr(), key, 32);
        }
    }

    pub(crate) unsafe fn storage_load_bytes32(key: *const u8, out: *mut u8) {
        let k = unsafe { read_word(key) };
        let value = STORAGE.with(|s| match s.borrow().get(&k) {
            Some(v) => *v,
            None => U::ZERO,
        });
        unsafe { write_word(out, value) };
    }

    pub(crate) unsafe fn storage_cache_bytes32(key: *const u8, value: *const u8) {
        let k = unsafe { read_word(key) };
        let v = unsafe { read_word(value) };
        STORAGE.with(|s| s.borrow_mut().insert(k, v));
    }

    pub(crate) unsafe fn transient_load_bytes32(key: *const u8, out: *mut u8) {
        let k = unsafe { read_word(key) };
        let value = TRANSIENT.with(|s| match s.borrow().get(&k) {
            Some(v) => *v,
            None => U::ZERO,
        });
        unsafe { write_word(out, value) };
    }

    pub(crate) unsafe fn transient_store_bytes32(key: *const u8, value: *const u8) {
        let k = unsafe { read_word(key) };
        let v = unsafe { read_word(value) };
        TRANSIENT.with(|s| s.borrow_mut().insert(k, v));
    }

    pub(crate) unsafe fn storage_flush_cache(_: bool) {}
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "std")))]
mod host {
    pub(crate) unsafe fn storage_load_bytes32(_: *const u8, _: *mut u8) {}

    pub(crate) unsafe fn storage_cache_bytes32(_: *const u8, _: *const u8) {}

    pub(crate) unsafe fn transient_load_bytes32(_: *const u8, _: *mut u8) {}

    pub(crate) unsafe fn transient_store_bytes32(_: *const u8, _: *const u8) {}

    pub(crate) unsafe fn storage_flush_cache(_: bool) {}
}

#[cfg(not(target_arch = "wasm32"))]
use host::*;

macro_rules! storage_ops {
    ($($prefix:ident),* $(,)?) => {
        $(
            paste::paste! {
                pub fn [<$prefix _load>](x: &U) -> U {
                    let mut b = [0u8; 32];
                    unsafe { [<$prefix _load_bytes32>](x.as_ptr(), b.as_mut_ptr()) }
                    U(b)
                }

                pub fn [<$prefix _exchange>](k: &U, exp: &U, new: &U) -> Result<(), U> {
                    let t = [<$prefix _load>](k);
                    if &t != exp {
                        return Err(t);
                    }
                    [<$prefix _store>](k, new);
                    Ok(())
                }

                pub fn [<$prefix _exchange_bool>](k: &U, exp: bool) -> Result<(), bool> {
                    [<$prefix _exchange>](k, &U::from(exp), &U::from(!exp))
                        .map_err(|x| x.is_true())
                }
            }
        )*
    };
}

pub fn storage_store(x: &U, y: &U) {
    unsafe { storage_cache_bytes32(x.as_ptr(), y.as_ptr()) }
}

pub fn transient_store(x: &U, y: &U) {
    unsafe { transient_store_bytes32(x.as_ptr(), y.as_ptr()) }
}

pub fn flush_cache(clear: bool) {
    unsafe { storage_flush_cache(clear) }
}

pub fn flush_guard<R, F: FnOnce() -> R>(f: F) -> R {
    let r = f();
    flush_cache(false);
    r
}

storage_ops!(storage, transient);

macro_rules! storage_mutate_ops {
    ($prefix:ident, $($op:expr),* $(,)?) => {
        $(
            paste::paste! {
                pub fn [<$prefix _wrapping_ $op>](x: &U, new: &U) {
                    [<$prefix _store>](x, &bobcat_maths::[<wrapping_ $op>](&[<$prefix _load>](x), new))
                }

                pub fn [<$prefix _checking_ $op>](x: &U, new: &U) -> Option<()> {
                    let y = [<$prefix _load>](x);
                    let Some(v) = bobcat_maths::[<checked_ $op>](&y, new) else {
                        return None
                    };
                    [<$prefix _store>](x, &v);
                    Some(())
                }

                pub fn [<$prefix _checking_ $op _res>](x: &U, y: &U) -> Result<(), (U, U)> {
                    [<$prefix _checking_ $op>](x, y).ok_or((*x, *y))
                }
            }
        )*
    };
}

storage_mutate_ops!(storage, add, sub, mul, div);
storage_mutate_ops!(transient, add, sub, mul, div);

#[cfg(not(target_arch = "wasm32"))]
pub fn slot_map_slot(k: &U, p: &U) -> U {
    const_slot_map(k, p)
}

pub fn reentrancy_guard_entry(x: &U) -> Result<(), bool> {
    assert!(x.len() <= 32, "too large");
    transient_exchange_bool(x, false)
}

pub fn reentrancy_guard_exit(x: &U) {
    assert!(x.len() <= 32, "too large");
    transient_store(x, &U::ZERO);
}

pub fn reentrancy_guard<R>(k: &U, f: impl FnOnce() -> R) -> Result<R, bool> {
    reentrancy_guard_entry(k)?;
    let v = f();
    reentrancy_guard_exit(k);
    Ok(v)
}

/// Compute the slot for a slice, and take it off the curve. Useful for
/// storage slot accesses (and more).
pub const fn const_slot_off_curve(b: &[u8]) -> U {
    bobcat_maths::wrapping_sub(&const_keccak256(b), &U::ONE)
}

pub fn slot_off_curve(b: &[u8]) -> U {
    // This won't result in 0 from the keccak, so we can use checked_sub to
    // use the code the host gives us for a slightly lower codesize profile.
    bobcat_maths::checked_sub(&keccak256(b), &U::ONE).unwrap()
}

#[cfg(target_arch = "wasm32")]
pub fn keccak256(b: &[u8]) -> U {
    let mut out = [0u8; 32];
    unsafe {
        native_keccak256(b.as_ptr(), b.len(), out.as_mut_ptr());
    }
    U(out)
}

pub const fn const_keccak256(b: &[u8]) -> U {
    U(Keccak256::new().update(b).finalize())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn keccak256(b: &[u8]) -> U {
    const_keccak256(b)
}

pub fn reentrancy_guard_const_keccak<R>(k: &[u8], f: impl FnOnce() -> R) -> Result<R, bool> {
    reentrancy_guard(&const_keccak256(k), f)
}

pub fn reentrancy_guard_keccak<R>(k: &[u8], f: impl FnOnce() -> R) -> Result<R, bool> {
    reentrancy_guard(&keccak256(k), f)
}

/// Find the storage map slot using keccak_const. Don't do this during
/// your runtime code, unless you want to pay the codesize price.
pub const fn const_slot_map(k: &U, p: &U) -> U {
    let a: [u8; 32 * 2] = concat_arrays!(k.0, p.0);
    const_keccak256(&a)
}

#[cfg(target_arch = "wasm32")]
pub fn slot_map(k: &U, p: &U) -> U {
    let b: [u8; 32 * 2] = concat_arrays!(k.0, p.0);
    keccak256(&b)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn slot_map(k: &U, p: &U) -> U {
    const_slot_map(k, p)
}

#[test]
fn test_slot_edd25519_count() {
    assert_eq!(
        U::from(
            const_hex::const_decode_to_array::<32>(
                b"709318ac04e7c3155ef66c30be7220b3243d7e2378fa4153b5f14ebd3ea771ab"
            )
            .unwrap()
        ),
        const_slot_off_curve(b"superposition.passport.ed25519_count")
    );
}

#[cfg(all(feature = "std", test))]
mod test {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_reentrancy_guard(x in any::<[u8; 8]>()) {
            reentrancy_guard(&U::from(x), || {
                assert!(transient_load(&U::from(x)).is_true());
            })
            .unwrap();
            assert!(transient_load(&U::from(x)).is_zero());
        }
    }
}
