#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub use bobcat_maths::U;

pub type Address = [u8; 20];

pub use bobcat_cd::read_word_slices;

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
        pub fn chain_id() -> u64;
        pub(crate) fn account_code_size(address: *const u8) -> usize;
        pub(crate) fn block_timestamp() -> u64;
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "std"))]
mod impls {
    use core::{ptr::copy_nonoverlapping, slice::from_raw_parts};

    #[allow(unused)]
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

    pub(crate) unsafe fn chain_id() -> u64 {
        0
    }

    pub(crate) unsafe fn account_code_size(_: *const u8) -> usize {
        0
    }

    pub(crate) unsafe fn block_timestamp() -> u64 {
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

    pub(crate) unsafe fn chain_id() -> u64 {
        0
    }

    pub(crate) unsafe fn account_code_size(_: *const u8) -> usize {
        0
    }

    pub(crate) unsafe fn block_timestamp() -> u64 {
        0
    }
}

#[unsafe(no_mangle)]
#[cfg(not(feature = "dont-define-symbols"))]
pub unsafe fn mark_used() {
    unsafe { impls::pay_for_memory_grow(0) }
    panic!();
}

pub fn write_result_slice(s: &[u8]) {
    unsafe { impls::write_result(s.as_ptr(), s.len()) }
}

pub fn write_result_word(s: &U) {
    write_result_slice(&s.0)
}

pub fn write_result_bool(v: bool) {
    write_result_slice(&U::from(v).0)
}

pub use bobcat_cd::leftpad_addr;

/// Like write_result_exit_call, except it only reverts with the
/// returndata if the underlying call reverted. If it doesn't, then it
/// just returns the slice.
#[macro_export]
macro_rules! revert_if_bad_call_vec {
    ($e:expr) => {{
        let (rc, rd) = $e;
        if !rc {
            $crate::write_result_slice(&rd);
            return 1;
        }
        rd
    }};
}

/// Reverts if the underlying call failed, using the vector that was
/// returned as the third argument as slice.
#[macro_export]
macro_rules! revert_if_bad_call_unit_vec {
    ($e:expr) => {{
        let (rc, revertdata) = $e;
        match (rc, revertdata) {
            (true, _) => (),
            (false, Some(v)) => {
                $crate::write_result_slice(&v);
                return 1;
            }
            (false, _) => return 1,
        }
    }};
}

/// Reverts with a message if the revertdata is Some, and if the rc is false.
#[macro_export]
macro_rules! revert_if_bad_call_slice_vec {
    ($e:expr) => {{
        let (rc, returndata, revertdata) = $e;
        match (rc, revertdata) {
            (true, _) => returndata,
            (false, Some(v)) => {
                $crate::write_result_slice(&v);
                return 1;
            }
            (false, _) => return 1,
        }
    }};
}

#[macro_export]
macro_rules! write_result_exit_res {
    ($ident:expr) => {{
        match $ident {
            Ok(v) => {
                $crate::write_result_slice(&v);
                0
            }
            Err(v) => {
                $crate::write_result_slice(&v);
                1
            }
        }
    }};
}

#[macro_export]
macro_rules! write_result_exit_create {
    ($ident:expr) => {{
        let (addr, b, i) = $ident;
        if addr != [0u8; 20] {
            $crate::write_result_slice(&leftpad_addr(addr));
            0
        } else {
            $crate::write_result_slice(&b[..i]);
            1
        }
    }};
}

#[macro_export]
macro_rules! write_result_exit_call {
    ($ident:expr) => {{
        let (rc, l, v) = $ident;
        $crate::write_result_slice(&v[..l]);
        if rc {
            0
        } else {
            1
        }
    }};
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
pub fn read_args_vec(len: usize) -> Vec<u8> {
    let mut b = Vec::with_capacity(len);
    unsafe {
        impls::read_args(b.as_mut_ptr());
        b.set_len(len);
    };
    b
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

pub fn code_size(addr: Address) -> usize {
    unsafe { impls::account_code_size(addr.as_ptr()) }
}

pub fn chain_id() -> u64 {
    unsafe { impls::chain_id() }
}

pub fn block_timestamp() -> u64 {
    unsafe { impls::block_timestamp() }
}
