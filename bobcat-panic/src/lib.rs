#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(feature = "panic-revert", target_arch = "wasm32"))]
use core::fmt::{Result as FmtResult, Write};

#[cfg(target_arch = "wasm32")]
mod wasm {
    #[link(wasm_import_module = "console")]
    #[cfg(feature = "console")]
    unsafe extern "C" {
        pub(crate) fn log_txt(ptr: *const u8, len: usize);
    }

    #[link(wasm_import_module = "vm_hooks")]
    #[allow(unused)]
    unsafe extern "C" {
        pub(crate) fn exit_early(code: i32) -> !;
        pub(crate) fn write_result(d: *const u8, l: usize);
    }
}

fn write_result_slice(s: &[u8]) {
    unsafe { wasm::write_result(s.as_ptr(), s.len()) }
}

const ERROR_PREAMBLE: [u8; 32 + 4] = match const_hex::const_decode_to_array::<{ 32 + 4 }>(
    b"08c379a00000000000000000000000000000000000000000000000000000000000000020",
) {
    Ok(v) => v,
    Err(_) => panic!(),
};

struct SliceWriter<'a>(&'a mut [u8], usize);

impl<'a> Write for SliceWriter<'a> {
    fn write_str(&mut self, s: &str) -> FmtResult {
        self.0[self.1..self.1 + s.len()].copy_from_slice(s.as_bytes());
        self.1 += s.len();
        Ok(())
    }
}

/// Revert buffer size that's used to write the panic. We can afford to
/// use a large page here since a panic will consume all the gas anyway,
/// and a user will see this during simulation hopefully.
const REVERT_BUF_SIZE: usize = 1024 * 10;

#[cfg(all(feature = "panic", target_arch = "wasm32"))]
#[panic_handler]
pub fn panic_handler(_msg: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "console")]
    {
        let msg = alloc::format!("{_msg}");
        unsafe { wasm::log_txt(msg.as_ptr(), msg.len()) }
    }
    #[cfg(feature = "panic-revert")]
    {
        let mut buf = [0u8; REVERT_BUF_SIZE];
        buf[..ERROR_PREAMBLE.len()].copy_from_slice(&ERROR_PREAMBLE);
        let mut w = SliceWriter(&mut buf[ERROR_PREAMBLE.len() + 32..], 0);
        write!(&mut w, "{_msg}").unwrap();
        let len_msg = w.1;
        let len_offset = ERROR_PREAMBLE.len();
        buf[len_offset + 28..len_offset + 32].copy_from_slice(&(len_msg as u32).to_be_bytes());
        let len_full = ERROR_PREAMBLE.len() + 32 + len_msg;
        let len_padded = len_full + (32 - (len_full % 32)) % 32;
        write_result_slice(&buf[..len_padded]);
        unsafe { wasm::exit_early(1) }
    }
    // Prefer the normal behaviour if the user hasn't opted into this
    // feature. Maybe it's better to wipe out the revertdata if this happens,
    // the other behaviour is different.
    #[allow(unreachable_code)]
    core::arch::wasm32::unreachable()
}
