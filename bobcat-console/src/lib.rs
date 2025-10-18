#![no_std]

extern crate alloc;

#[link(wasm_import_module = "console")]
unsafe extern "C" {
    pub fn log_txt(ptr: *const u8, len: usize);
}

#[macro_export]
macro_rules! console {
    ($val:expr) => {
    {
        let tmp = $val;
        let msg = alloc::format!("[{}:{}] {} = {:#?}\n", file!(), line!(), stringify!($val), &tmp);
        unsafe { $crate::log_txt(msg.as_ptr(), msg.len()) };
        tmp
    }};
    ($($vals:expr),+ $(,)?) => {
    {
        let tup = ($($vals),+);
        let msg = alloc::format!("[{}:{}] {} = {:#?}\n", file!(), line!(), stringify!(($($vals),+)), &tup);
        unsafe { $crate::log_txt(msg.as_ptr(), msg.len()) };
        tup
    }};
}
