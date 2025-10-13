#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub use bobcat_maths::U;

pub type Address = [u8; 20];

#[cfg(target_arch = "wasm32")]
mod impls {
    #[link(wasm_import_module = "vm_hooks")]
    unsafe extern "C" {
        #[allow(unused)]
        pub(crate) fn pay_for_memory_grow(pages: u16);
        pub(crate) fn write_result(d: *const u8, l: usize);
        pub(crate) fn read_args(out: *mut u8);
        pub(crate) fn msg_sender(addr: *mut u8);
        pub(crate) fn contract_address(addr: *mut u8);
        pub(crate) fn msg_value(value: *mut u8);
        pub(crate) fn msg_reentrant() -> bool;
        pub fn chain_id() -> u64;
        pub(crate) fn account_code_size(address: *const u8) -> usize;
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "std"))]
mod impls {
    use core::{ptr::copy_nonoverlapping, slice::from_raw_parts};

    pub(crate) unsafe fn pay_for_memory_grow(_: u16) {}

    pub(crate) unsafe fn write_result(d: *const u8, l: usize) {
        println!("{}", const_hex::encode(unsafe { from_raw_parts(d, l) }));
    }

    pub(crate) unsafe fn read_args(_out: *mut u8) {
        unimplemented!("read from stdin separately. todo");
    }

    pub(crate) unsafe fn msg_sender(out: *mut u8) {
        unsafe { copy_nonoverlapping([0u8; 32].as_ptr(), out, 32) }
    }

    pub(crate) unsafe fn contract_address(out: *mut u8) {
        unsafe { copy_nonoverlapping([0u8; 32].as_ptr(), out, 32) }
    }

    pub(crate) unsafe fn msg_value(out: *mut u8) {
        unsafe { copy_nonoverlapping([0u8; 32].as_ptr(), out, 32) }
    }

    pub(crate) unsafe fn msg_reentrant() -> bool {
        false
    }

    pub fn chain_id() -> u64 {
        0
    }

    pub(crate) unsafe fn account_code_size(_: *const u8) -> usize {
        0
    }
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "std")))]
mod impls {
    pub(crate) unsafe fn pay_for_memory_grow(_: u16) {}

    pub(crate) unsafe fn write_result(_: *const u8, _: usize) {}

    pub(crate) unsafe fn read_args(_out: *mut u8) {}

    pub(crate) unsafe fn msg_sender(_: *mut u8) {}

    pub(crate) unsafe fn contract_address(_: *mut u8) {}

    pub(crate) unsafe fn msg_value(_: *mut u8) {}

    pub(crate) unsafe fn msg_reentrant() -> bool {
        false
    }

    pub fn chain_id() -> u64 {
        0
    }

    pub(crate) unsafe fn account_code_size(_: *const u8) -> usize {
        0
    }
}

#[unsafe(no_mangle)]
#[cfg(not(feature = "dont-define-symbols"))]
pub unsafe fn mark_used() {
    unsafe { impls::pay_for_memory_grow(0) }
    panic!();
}

pub fn write_result(s: &[u8]) {
    unsafe { impls::write_result(s.as_ptr(), s.len()) }
}

pub fn read_args<const CAP: usize>(len: usize) -> ([u8; CAP], usize) {
    assert!(CAP >= len, "cap not enough");
    let mut b = [0u8; CAP];
    unsafe { impls::read_args(b.as_mut_ptr()) };
    (b, len)
}

#[macro_export]
macro_rules! read_args_safe {
    ($len:expr, $max_len:expr) => {{
        assert!($max_len >= $len);
        $crate::read_args::<$max_len>($len).0
    }};
}

#[cfg(feature = "alloc")]
pub fn read_args_vec(len: usize) -> (Vec<u8>, usize) {
    let mut b = Vec::with_capacity(len);
    unsafe {
        impls::read_args(b.as_mut_ptr());
        b.set_len(len);
    };
    (b, len)
}

pub fn msg_sender() -> Address {
    let mut b = [0u8; 20];
    unsafe { impls::msg_sender(b.as_mut_ptr()) }
    b
}

pub fn contract_address() -> Address {
    let mut b = [0u8; 20];
    unsafe { impls::contract_address(b.as_mut_ptr()) }
    b
}

pub fn msg_value() -> U {
    let mut b = [0u8; 32];
    unsafe { impls::msg_value(b.as_mut_ptr()) }
    U(b)
}

pub fn msg_reentrant() -> bool {
    unsafe { impls::msg_reentrant() }
}

pub fn code_size(addr: Address) -> usize {
    unsafe { impls::account_code_size(addr.as_ptr()) }
}

pub fn chain_id() -> u64 {
    unsafe { impls::chain_id() }
}