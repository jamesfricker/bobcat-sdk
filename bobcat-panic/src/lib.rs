#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(target_arch = "wasm32", feature = "console"))]
mod wasm {
    #[link(wasm_import_module = "vm_hooks")]
    unsafe extern "C" {
        pub(crate) fn log_txt(ptr: *const u8, len: usize);
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
    core::arch::wasm32::unreachable()
}
