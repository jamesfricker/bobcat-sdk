#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::cmp::min;

use bobcat_maths::U;

pub type Address = [u8; 20];

#[cfg(target_arch = "wasm32")]
mod impls {
    #[link(wasm_import_module = "vm_hooks")]
    unsafe extern "C" {
        pub(crate) fn call_contract(
            contract: *const u8,
            calldata: *const u8,
            calldata_len: usize,
            value: *const u8,
            gas: u64,
            return_data_len: *mut usize,
        ) -> u8;

        pub(crate) fn static_call_contract(
            contract: *const u8,
            calldata: *const u8,
            calldata_len: usize,
            gas: u64,
            return_data_len: *mut usize,
        ) -> u8;

        pub(crate) fn delegate_call_contract(
            contract: *const u8,
            calldata: *const u8,
            calldata_len: usize,
            gas: u64,
            return_data_len: *mut usize,
        ) -> u8;

        pub(crate) fn read_return_data(dest: *mut u8, offset: usize, size: usize) -> usize;

        pub(crate) fn account_code_size(address: *const u8) -> usize;
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod impls {
    pub(crate) unsafe fn call_contract(
        _contract: *const u8,
        _calldata: *const u8,
        _calldata_len: usize,
        _value: *const u8,
        _gas: u64,
        _return_data_len: *mut usize,
    ) -> u8 {
        0
    }

    pub(crate) unsafe fn static_call_contract(
        _contract: *const u8,
        _calldata: *const u8,
        _calldata_len: usize,
        _gas: u64,
        _return_data_len: *mut usize,
    ) -> u8 {
        0
    }

    pub(crate) unsafe fn delegate_call_contract(
        _contract: *const u8,
        _calldata: *const u8,
        _calldata_len: usize,
        _gas: u64,
        _return_data_len: *mut usize,
    ) -> u8 {
        0
    }

    pub(crate) unsafe fn read_return_data(_: *mut u8, _: usize, _: usize) -> usize {
        0
    }

    pub(crate) unsafe fn account_code_size(_: *const u8) -> usize {
        0
    }
}

use impls::{
    call_contract as call, delegate_call_contract as delegate_call,
    static_call_contract as static_call,
};

fn code_size(addr: Address) -> usize {
    unsafe { impls::account_code_size(addr.as_ptr()) }
}

macro_rules! generate_call_variants {
    ($base_fn:ident, has_value) => {
        generate_call_variants!(@impl $base_fn, value: &U);
    };
    ($base_fn:ident) => {
        generate_call_variants!(@impl $base_fn,);
    };
    (@impl $base_fn:ident, $($value_param:ident: $value_ty:ty)?) => {
        paste::paste! {
            /// Call a contract with the given parameters. Returns a tuple of
            /// (success, return_data_length).
            pub fn [<$base_fn _partial>](
                contract: Address,
                calldata: &[u8],
                $($value_param: $value_ty,)?
                gas: u64,
            ) -> (bool, usize) {
                let mut return_data_len = 0usize;
                let status = unsafe {
                    $base_fn(
                        contract.as_ptr(),
                        calldata.as_ptr(),
                        calldata.len(),
                        $($value_param.as_ptr(),)?
                        gas,
                        &mut return_data_len as *mut usize,
                    )
                };
                (status != 0, return_data_len)
            }

            /// Call a contract, writing its returndata to the slice given. Returns
            /// true for if the contract ran without issue, or false if a revert
            /// happened. An offset can be used to start reading the return data from,
            /// writing to the buffer given. The function will panic if the returndata
            /// exceeds the capacity. Enforces write control if static, and delegates if
            /// delegatecall.
            pub fn [<$base_fn _slice>]<const DATA_CAP: usize>(
                contract: Address,
                calldata: &[u8],
                $($value_param: $value_ty,)?
                gas: u64,
                offset: usize,
                size: usize,
            ) -> (bool, usize, [u8; DATA_CAP]) {
                assert!(DATA_CAP >= size, "not enough cap for size");
                let (rc, rd_len) = [<$base_fn _partial>](contract, calldata, $($value_param,)? gas);
                let mut b = [0u8; DATA_CAP];
                let size = min(size, rd_len - offset);
                unsafe { impls::read_return_data(b.as_mut_ptr(), offset, size) };
                (rc, size, b)
            }

            /// Same as the slice variant, except check the length of the code for the
            /// address first. If code doesn't exist, then we return None. This might
            /// be useful for implementing ERC20 when you relax the check on the
            /// return value.
            pub fn [<safe_ $base_fn _slice>]<const DATA_CAP: usize>(
                contract: Address,
                calldata: &[u8],
                $($value_param: $value_ty,)?
                gas: u64,
                offset: usize,
                size: usize,
            ) -> Option<(bool, usize, [u8; DATA_CAP])> {
                if code_size(contract) > 0 {
                    Some([<$base_fn _slice>]::<DATA_CAP>(
                        contract, calldata, $($value_param,)? gas, offset, size,
                    ))
                } else {
                    None
                }
            }

            /// Call a contract, writing its returndata to the vector given. Behaves
            /// the same way as the slice function.
            #[cfg(feature = "alloc")]
            pub fn [<$base_fn _vec>](
                contract: Address,
                calldata: &[u8],
                $($value_param: $value_ty,)?
                gas: u64,
                offset: usize,
                size: usize,
            ) -> (bool, Vec<u8>) {
                let (rc, rd_len) = [<$base_fn _partial>](contract, calldata, $($value_param,)? gas);
                let size = min(size, rd_len - offset);
                let mut b = Vec::with_capacity(size);
                unsafe { b.set_len(size) }
                unsafe { impls::read_return_data(b.as_mut_ptr(), offset, size) };
                (rc, b)
            }

            /// Safely call a contract, with the same checks as the safe slice variant.
            #[cfg(feature = "alloc")]
            pub fn [<safe_ $base_fn _vec>](
                contract: Address,
                calldata: &[u8],
                $($value_param: $value_ty,)?
                gas: u64,
                offset: usize,
                size: usize,
            ) -> Option<(bool, Vec<u8>)> {
                if code_size(contract) > 0 {
                    Some([<$base_fn _vec>](contract, calldata, $($value_param,)? gas, offset, size))
                } else {
                    None
                }
            }
        }
    };
}

generate_call_variants!(call, has_value);
generate_call_variants!(static_call);
generate_call_variants!(delegate_call);
