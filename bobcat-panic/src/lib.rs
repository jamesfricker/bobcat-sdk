#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(target_arch = "wasm32", feature = "console"))]
mod wasm {
    #[link(wasm_import_module = "console")]
    unsafe extern "C" {
        pub(crate) fn log_txt(ptr: *const u8, len: usize);
    }
}

#[macro_export]
#[cfg(all(target_arch = "wasm32", feature = "console"))]
macro_rules! console_dbg {
    ($val:expr) => {
    {
        let tmp = $val;
        let msg = alloc::format!("[{}:{}] {} = {:#?}\n", file!(), line!(), stringify!($val), &tmp);
        unsafe { $crate::host::log_txt(msg.as_ptr(), msg.len()) };
        tmp
    }};
    ($($vals:expr),+ $(,)?) => {
    {
        let tup = ($($vals),+);
        let msg = alloc::format!("[{}:{}] {} = {:#?}\n", file!(), line!(), stringify!(($($vals),+)), &tup);
        unsafe { $crate::host::log_txt(msg.as_ptr(), msg.len()) };
        tup
    }};
}

#[macro_export]
#[cfg(all(not(target_arch = "wasm32"), feature = "std"))]
macro_rules! harness_dbg {
    ($val:expr) => { dbg!($val) };
    ($($vals:expr),+ $(,)?) => { dbg!($($vals),+) };
}

#[macro_export]
#[cfg(any(
    all(not(target_arch = "wasm32"), not(feature = "std")),
    all(target_arch = "wasm32", not(feature = "console"))
))]
macro_rules! harness_dbg {
    ($val:expr) => { dbg!($val) };
    ($($vals:expr),+ $(,)?) => { dbg!($($vals),+) };
}

#[cfg(all(not(feature = "std"), target_arch = "wasm32"))]
#[panic_handler]
pub fn panic_handler(_msg: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "console")]
    {
        let msg = alloc::format!("{_msg}");
        unsafe { wasm::log_txt(msg.as_ptr(), msg.len()) }
    }
    core::arch::wasm32::unreachable()
}
