#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use bobcat_storage::const_keccak256;

pub use bobcat_maths::U;

pub type Address = [u8; 20];

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
    if i > 0 {
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
    create1_partial(code, endowment).map_err(|i| {
        let mut b = Vec::with_capacity(i);
        let l = unsafe { impls::read_return_data(b.as_mut_ptr(), 0, i) };
        unsafe { b.set_len(l) }
        b
    })
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
    salt: &[u8],
) -> Result<Address, ([u8; REVERT_CAP], usize)> {
    create2_slice::<REVERT_CAP>(code, endowment, const_keccak256(salt))
}

#[cfg(feature = "alloc")]
pub fn create2_vec_salt_keccak256(
    code: &[u8],
    endowment: U,
    salt: &[u8],
) -> Result<Address, Vec<u8>> {
    create2_vec(code, endowment, const_keccak256(salt))
}
