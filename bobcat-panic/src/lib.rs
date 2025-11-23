#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[allow(unused)]
use core::fmt::{Result as FmtResult, Write};

use paste::paste;

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

#[cfg(target_arch = "wasm32")]
fn write_result_slice(s: &[u8]) {
    unsafe { wasm::write_result(s.as_ptr(), s.len()) }
}

//Panic(uint256)
pub const PANIC_PREAMBLE_WORD: [u8; 32 + 4] = match const_hex::const_decode_to_array::<{ 32 + 4 }>(
    b"4e487b710000000000000000000000000000000000000000000000000000000000000000",
) {
    Ok(v) => v,
    Err(_) => panic!(),
};

//Error(string)
pub const ERROR_PREAMBLE_OFFSET: [u8; 4 + 32] = match const_hex::const_decode_to_array::<{ 4 + 32 }>(
    b"08c379a00000000000000000000000000000000000000000000000000000000000000020",
) {
    Ok(v) => v,
    Err(_) => panic!(),
};

#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum PanicCodes {
    OverflowOrUnderflow = 0x11,
    DivByZero = 0x12,
    DecodingError = 0x22,
}

#[cfg(target_arch = "wasm32")]
pub fn panic_with_code(x: PanicCodes) -> ! {
    let mut b = PANIC_PREAMBLE_WORD;
    b[4 + 32 - 1] = x as u8;
    write_result_slice(&b);
    unsafe { wasm::exit_early(1) }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn panic_with_code(x: PanicCodes) -> ! {
    panic!("panicked with code: {x:?}");
}

#[macro_export]
macro_rules! define_panic_macros {
    (
        $(($error_msg:expr, $panic_code:ident)),* $(,)?
    ) => {
        $(
            paste! {
                #[macro_export]
                macro_rules! [<panic_on_err_ $error_msg>] {
                    ($e:expr, $msg:expr) => {{
                        match $e {
                            Some(v) => v,
                            None => {
                                #[cfg(feature = "msg-on-sdk-err")]
                                panic!("{}: {}", $error_msg, $msg);
                                #[cfg(not(feature = "msg-on-sdk-err"))]
                                $crate::panic_with_code($crate::PanicCodes::$panic_code);
                            }
                        }
                    }};
                }
            }
        )*
    };
}

define_panic_macros!(
    ("overflow", OverflowOrUnderflow),
    ("div_by_zero", DivByZero),
);

#[macro_export]
macro_rules! panic_on_err_bad_decoding_bool {
    ($msg:expr) => {{
        #[cfg(feature = "msg-on-sdk-err")]
        panic!("error decoding: {}", $msg);
        #[cfg(not(feature = "msg-on-sdk-err"))]
        $crate::panic_with_code($crate::PanicCodes::DecodingError);
    }};
    ($e:expr, $msg:expr) => {{
        if !$e {
            panic_on_err_bad_decoding_bool!($msg);
        }
    }};
}

#[allow(unused)]
struct SliceWriter<'a>(&'a mut [u8], usize);

impl<'a> Write for SliceWriter<'a> {
    fn write_str(&mut self, s: &str) -> FmtResult {
        let v = s.len().min(REVERT_BUF_SIZE.saturating_sub(self.1));
        self.0[self.1..self.1 + v].copy_from_slice(&s.as_bytes()[..v]);
        self.1 += v;
        Ok(())
    }
}

/// Revert buffer size that's used to write the panic. We can afford to
/// use a large page here since a panic will consume all the gas anyway,
/// and a user will see this during simulation hopefully.
#[allow(unused)]
const REVERT_BUF_SIZE: usize = 1024;

#[cfg(all(feature = "panic-revert", feature = "panic-loc"))]
compile_error!("panic-revert and panic-loc simultaneously enabled");

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
#[cfg_attr(all(feature = "panic", not(feature = "std")), panic_handler)]
pub fn panic_handler(_msg: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "console")]
    {
        let msg = alloc::format!("{_msg}");
        unsafe { wasm::log_txt(msg.as_ptr(), msg.len()) }
    }
    #[cfg(any(feature = "panic-revert", feature = "panic-loc"))]
    {
        let mut buf = [0u8; REVERT_BUF_SIZE];
        buf[..ERROR_PREAMBLE_OFFSET.len()].copy_from_slice(&ERROR_PREAMBLE_OFFSET);
        let mut w = SliceWriter(&mut buf[ERROR_PREAMBLE_OFFSET.len() + 32..], 0);
        #[cfg(feature = "panic-revert")]
        {
            write!(&mut w, "{_msg}").unwrap();
        }
        #[cfg(feature = "panic-loc")]
        if let Some(loc) = _msg.location() {
            write!(&mut w, "panic: {}:{}", loc.file(), loc.line()).unwrap();
        } else {
            write!(&mut w, "panic: unknown").unwrap();
        }
        let len_msg = w.1;
        let len_offset = ERROR_PREAMBLE_OFFSET.len();
        buf[len_offset + 28..len_offset + 32].copy_from_slice(&(len_msg as u32).to_be_bytes());
        let len_full = ERROR_PREAMBLE_OFFSET.len() + 32 + len_msg;
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
