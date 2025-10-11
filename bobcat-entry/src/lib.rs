#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(target_arch = "wasm32")]
mod impls {
    #[link(wasm_import_module = "vm_hooks")]
    unsafe extern "C" {
        pub(crate) fn pay_for_memory_grow(pages: u16);
        pub(crate) fn write_result(d: *const u8, l: usize);
        pub(crate) fn read_args(out: *mut u8);
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "std"))]
mod impls {
    use std::slice::from_raw_parts;

    pub(crate) unsafe fn pay_for_memory_grow(_: u16) {}

    pub(crate) unsafe fn write_result(d: *const u8, l: usize) {
        println!("{}", const_hex::encode(unsafe { from_raw_parts(d, l) }));
    }

    pub(crate) unsafe fn read_args(_out: *mut u8) {
        unimplemented!("read from stdin separately. todo");
    }
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "std")))]
mod impls {
    pub(crate) unsafe fn pay_for_memory_grow(_: u16) {}

    pub(crate) unsafe fn write_result(_: *const u8, _: usize) {}

    pub(crate) unsafe fn read_args(_out: *mut u8) {}
}

#[unsafe(no_mangle)]
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
    }}
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
