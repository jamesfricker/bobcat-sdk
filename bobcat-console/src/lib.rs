#![no_std]

extern crate alloc;

pub use alloc::format;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod impls {
    #[link(wasm_import_module = "console")]
    unsafe extern "C" {
        pub fn log_txt(ptr: *const u8, len: usize);
    }
}

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
mod impls {
    pub unsafe fn log_txt(_: *const u8, _: usize) {}
}

pub use impls::log_txt;

#[macro_export]
macro_rules! console {
    ($val:expr) => {
    {
        let tmp = $val;
        let msg = $crate::format!("[{}:{}] {} = {:#?}\n", file!(), line!(), stringify!($val), &tmp);
        unsafe { $crate::log_txt(msg.as_ptr(), msg.len()) };
        tmp
    }};
    ($($vals:expr),+ $(,)?) => {
    {
        let tup = ($($vals),+);
        let msg = $crate::format!("[{}:{}] {} = {:#?}\n", file!(), line!(), stringify!(($($vals),+)), &tup);
        unsafe { $crate::log_txt(msg.as_ptr(), msg.len()) };
        tup
    }};
}
