#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "panic-revert")]
use bobcat_entry::{write_result_slice, U};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "panic-revert")]
use alloc::vec::Vec;

#[cfg(feature = "panic-revert")]
use core::fmt::{Result as FmtResult, Write};

#[cfg(target_arch = "wasm32")]
mod wasm {
    #[link(wasm_import_module = "vm_hooks")]
    #[allow(unused)]
    unsafe extern "C" {
        pub(crate) fn log_txt(ptr: *const u8, len: usize);
        pub(crate) fn exit_early(code: i32) -> !;
    }
}

#[cfg(feature = "panic-revert")]
const ERROR_PREAMBLE: [u8; 32 + 4] = match const_hex::const_decode_to_array::<{ 32 + 4 }>(
    b"08c379a00000000000000000000000000000000000000000000000000000000000000020",
) {
    Ok(v) => v,
    Err(_) => panic!(),
};

#[cfg(feature = "panic-revert")]
struct VecWriter<'a>(&'a mut Vec<u8>);

#[cfg(feature = "panic-revert")]
impl<'a> Write for VecWriter<'a> {
    fn write_str(&mut self, s: &str) -> FmtResult {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

#[cfg(all(not(feature = "std"), target_arch = "wasm32"))]
#[panic_handler]
pub fn panic_handler(_msg: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "console")]
    {
        let msg = alloc::format!("{_msg}");
        unsafe { wasm::log_txt(msg.as_ptr(), msg.len()) }
    }
    let mut d = ERROR_PREAMBLE.to_vec();
    let mut b = Vec::new();
    #[cfg(feature = "panic-revert")]
    {
        write!(VecWriter(&mut b), "{_msg}").unwrap();
        let l = b.len();
        let p = (32 - (l % 32)) % 32;
        d.extend_from_slice(&U::from(l).0);
        d.append(&mut b);
        d.resize(d.len() + p, 0);
        write_result_slice(&d);
        unsafe { wasm::exit_early(1) }
    }
    // Prefer the normal behaviour if the user hasn't opted into this
    // feature. Why? Maybe the user has special handling here. SPN does!
    #[allow(unreachable_code)]
    core::arch::wasm32::unreachable()
}
