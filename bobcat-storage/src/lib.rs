#![cfg_attr(not(feature = "std"), no_std)]

use bobcat_maths::U;

use keccak_const::Keccak256;

use array_concat::concat_arrays;

#[link(wasm_import_module = "vm_hooks")]
#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn storage_load_bytes32(key: *const u8, out: *mut u8);
    fn storage_store_bytes32(key: *const u8, from: *const u8);
    fn transient_load_bytes32(key: *const u8, dest: *mut u8);
    fn transient_store_bytes32(key: *const u8, value: *const u8);
    fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8);
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
            copy_nonoverlapping(val.0.as_ptr(), key, 32);
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

    pub(crate) unsafe fn storage_store_bytes32(key: *const u8, value: *const u8) {
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
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "std")))]
compile_error!("std needs to be enabled for non-wasm");

#[cfg(all(not(target_arch = "wasm32"), feature = "std"))]
use host::*;

pub fn storage_load(x: U) -> U {
    let mut b = [0u8; 32];
    unsafe { storage_load_bytes32(x.0.as_ptr(), b.as_mut_ptr()) }
    U(b)
}

pub fn storage_store(x: &U, y: &U) {
    unsafe { storage_store_bytes32(x.0.as_ptr(), y.0.as_ptr()) }
}

pub fn transient_load(x: &U) -> U {
    let mut b = [0u8; 32];
    unsafe { transient_load_bytes32(x.0.as_ptr(), b.as_mut_ptr()) }
    U(b)
}

pub fn transient_store(x: &U, y: &U) {
    unsafe { transient_store_bytes32(x.0.as_ptr(), y.0.as_ptr()) }
}

pub fn transient_exchange(x: &U, exp: bool) -> Result<(), bool> {
    let set: [u8; 32] = transient_load(x).into();
    let set = set[31] == 1;
    if exp != set {
        return Err(set);
    }
    let b = [!exp as u8; 32];
    transient_store(x, &U(b));
    Ok(())
}

pub fn reentrancy_guard_entry(x: &[u8]) -> Result<(), bool> {
    assert!(x.len() <= 32, "too large");
    transient_exchange(&U::try_from(x).unwrap(), false)
}

pub fn reentrancy_guard_exit(x: &[u8]) {
    assert!(x.len() <= 32, "too large");
    transient_store(&U::try_from(x).unwrap(), &U::ZERO);
}

pub fn reentrancy_guard<R>(k: &[u8], f: impl FnOnce() -> R) -> Result<R, bool> {
    reentrancy_guard_entry(k)?;
    let v = f();
    reentrancy_guard_exit(k);
    Ok(v)
}

/// Compute the slot for a slice, and take it off the curve. Useful for
/// storage slot accesses (and more).
pub const fn get_slot_off_curve(b: &[u8]) -> U {
    bobcat_maths::sub(&get_keccak256(b), &U::ONE)
}

pub const fn get_keccak256(b: &[u8]) -> U {
    U(Keccak256::new().update(b).finalize())
}

pub fn reentrancy_guard_keccak<R>(k: &[u8], f: impl FnOnce() -> R) -> Result<R, bool> {
    reentrancy_guard(&get_keccak256(k).0, f)
}

/// Find the storage map slot using keccak_const. Don't do this during
/// your runtime code, unless you want to pay the codesize price.
pub const fn const_storage_map_slot(k: &U, p: &U) -> U {
    let a: [u8; 32 * 2] = concat_arrays!(k.0, p.0);
    get_keccak256(&a)
}

#[cfg(target_arch = "wasm32")]
pub fn storage_map_slot(k: &U, p: &U) -> U {
    let b: [u8; 32 * 2] = concat_arrays!(k.0, p.0);
    let mut out = [0u8; 32];
    unsafe { native_keccak256(b.as_ptr(), 32 * 2, out.as_mut_ptr()); }
    U(out)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn storage_map_slot(k: &U, p: &U) -> U {
    const_storage_map_slot(k, p)
}

#[cfg(all(feature = "std", test))]
mod test {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_reentrancy_guard(x in any::<[u8; 8]>()) {
            reentrancy_guard(&x, || {
                assert!(transient_load(&U::from(x)).is_true());
            })
            .unwrap();
            assert!(transient_load(&U::from(x)).is_zero());
        }
    }
}
