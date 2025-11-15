#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use bobcat_storage::{const_keccak256, keccak256};

pub use bobcat_maths::U;

type Address = [u8; 20];

use array_concat::concat_arrays;

#[cfg(target_arch = "wasm32")]
mod impls {
    #[link(wasm_import_module = "vm_hooks")]
    unsafe extern "C" {
        pub(crate) fn create1(
            code: *const u8,
            code_len: usize,
            endowment: *const u8,
            contract: *mut u8,
            revert_data_len: *mut usize,
        );

        pub(crate) fn create2(
            code: *const u8,
            code_len: usize,
            endowment: *const u8,
            salt: *const u8,
            contract: *mut u8,
            revert_data_len: *mut usize,
        );

        pub(crate) fn read_return_data(dest: *mut u8, offset: usize, size: usize) -> usize;
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod impls {
    // Sorry -- on the host, these don't do anything.

    pub(crate) unsafe fn create1(
        _code: *const u8,
        _code_len: usize,
        _endowment: *const u8,
        _contract: *mut u8,
        _revert_data_len: *mut usize,
    ) {
    }

    pub(crate) unsafe fn create2(
        _code: *const u8,
        _code_len: usize,
        _endowment: *const u8,
        _salt: *const u8,
        _contract: *mut u8,
        _revert_data_len: *mut usize,
    ) {
    }

    pub(crate) unsafe fn read_return_data(_dest: *mut u8, _offset: usize, _size: usize) -> usize {
        0
    }
}

fn create1_partial(code: &[u8], endowment: U) -> (Address, usize) {
    let mut addr = [0u8; 20];
    let mut revert_len = 0;
    unsafe {
        impls::create1(
            code.as_ptr(),
            code.len(),
            endowment.as_ptr(),
            addr.as_mut_ptr(),
            &mut revert_len as *mut usize,
        )
    }
    (addr, revert_len)
}

pub fn create1_slice<const REVERT_CAP: usize>(
    code: &[u8],
    endowment: U,
) -> (Address, [u8; REVERT_CAP], usize) {
    let (addr, i) = create1_partial(code, endowment);
    let mut b = [0u8; REVERT_CAP];
    let mut l = 0;
    if addr != [0u8; 20] {
        assert!(REVERT_CAP >= i, "create1 not enough space");
        l = unsafe { impls::read_return_data(b.as_mut_ptr(), 0, i) };
    }
    (addr, b, l)
}

pub fn create1_slice_res<const REVERT_CAP: usize>(
    code: &[u8],
    endowment: U,
) -> Result<Address, ([u8; REVERT_CAP], usize)> {
    let (addr, b, l) = create1_slice(code, endowment);
    if addr != [0u8; 20] {
        Ok(addr)
    } else {
        Err((b, l))
    }
}

#[cfg(feature = "alloc")]
pub fn create1_vec(code: &[u8], endowment: U) -> Result<Address, Vec<u8>> {
    let (addr, i) = create1_partial(code, endowment);
    if addr != [0u8; 20] {
        Ok(addr)
    } else {
        let mut b = Vec::with_capacity(i);
        let l = unsafe { impls::read_return_data(b.as_mut_ptr(), 0, i) };
        unsafe { b.set_len(l) }
        Err(b)
    }
}

fn create2_partial(code: &[u8], endowment: U, salt: U) -> Result<Address, usize> {
    let mut addr = [0u8; 20];
    let mut revert_len = 0;
    unsafe {
        impls::create2(
            code.as_ptr(),
            code.len(),
            endowment.as_ptr(),
            salt.as_ptr(),
            addr.as_mut_ptr(),
            &mut revert_len as *mut usize,
        )
    }
    if addr == [0u8; 20] {
        Err(revert_len)
    } else {
        Ok(addr)
    }
}

pub fn create2_slice<const REVERT_CAP: usize>(
    code: &[u8],
    endowment: U,
    salt: U,
) -> Result<Address, ([u8; REVERT_CAP], usize)> {
    create2_partial(code, endowment, salt).map_err(|i| {
        let mut b = [0u8; REVERT_CAP];
        assert!(REVERT_CAP >= i, "create2 not enough space");
        let l = unsafe { impls::read_return_data(b.as_mut_ptr(), 0, i) };
        (b, l)
    })
}

#[cfg(feature = "alloc")]
pub fn create2_vec(code: &[u8], endowment: U, salt: U) -> Result<Address, Vec<u8>> {
    create2_partial(code, endowment, salt).map_err(|i| {
        let mut b = Vec::with_capacity(i);
        let l = unsafe { impls::read_return_data(b.as_mut_ptr(), 0, i) };
        unsafe {
            b.set_len(l);
        }
        b
    })
}

pub fn create2_slice_salt_keccak256<const REVERT_CAP: usize>(
    code: &[u8],
    endowment: U,
    salt_pre: &[u8],
) -> Result<Address, ([u8; REVERT_CAP], usize)> {
    create2_slice::<REVERT_CAP>(code, endowment, const_keccak256(salt_pre))
}

#[cfg(feature = "alloc")]
pub fn create2_vec_salt_keccak256(
    code: &[u8],
    endowment: U,
    salt_pre: &[u8],
) -> Result<Address, Vec<u8>> {
    create2_vec(code, endowment, keccak256(salt_pre))
}

/// Estimate the address of the create2 deployment.
pub const fn const_estimate_addr_pre(
    factory: Address,
    initcode_pre: &[u8],
    salt_pre: &[u8],
) -> Address {
    let b: [u8; 1 + 20 + 32 * 2] = concat_arrays!(
        [0xff],
        factory,
        const_keccak256(salt_pre).0,
        const_keccak256(initcode_pre).0
    );
    let x = const_keccak256(&b);
    todo!()
}

pub fn estimate_addr(factory: Address, initcode: U, salt: U) -> Address {
    let b: [u8; 1 + 20 + 32 * 2] = concat_arrays!([0xff], factory, salt.0, initcode.0);
    keccak256(&b).into()
}

/// Estimate the address of the create2 deployment.
pub fn estimate_addr_pre(factory: Address, initcode_pre: &[u8], salt_pre: &[u8]) -> Address {
    estimate_addr(factory, keccak256(initcode_pre), keccak256(salt_pre))
}
