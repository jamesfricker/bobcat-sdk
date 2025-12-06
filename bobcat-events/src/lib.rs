#![no_std]

pub use bobcat_maths::U;

use array_concat::concat_arrays;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod impls {
    #[link(wasm_import_module = "vm_hooks")]
    unsafe extern "C" {
        pub(crate) fn emit_log(data: *const u8, len: usize, topics: usize);
    }
}

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
mod impls {
    pub(crate) unsafe fn emit_log(_: *const u8, _: usize, _: usize) {}
}

pub fn emit_log_0_slice<const D: usize, const ALL: usize>(t0: &U, d: [u8; D]) {
    assert_eq!(ALL, D + 32, "not properly sized: {}", D + 32);
    let x: [u8; ALL] = concat_arrays!(t0.0, d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 1);
    }
}

pub fn emit_log_1_slice<const D: usize, const ALL: usize>(t0: &U, t1: &U, d: [u8; D]) {
    assert_eq!(ALL, D + 32 * 2, "not properly sized: {}", D + 64);
    let x: [u8; ALL] = concat_arrays!(t0.0, t1.0, d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 2);
    }
}

pub fn emit_log_2_slice<const D: usize, const ALL: usize>(t0: &U, t1: &U, t2: &U, d: [u8; D]) {
    assert_eq!(ALL, D + 32 * 3, "not properly sized: {}", D + 96);
    let x: [u8; ALL] = concat_arrays!(t0.0, t1.0, t2.0, d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 3);
    }
}

pub fn emit_log_3_slice<const D: usize, const ALL: usize>(
    t0: &U,
    t1: &U,
    t2: &U,
    t3: &U,
    d: [u8; D],
) {
    assert_eq!(ALL, D + 32 * 4, "not properly sized: {}", D + 128);
    let x: [u8; ALL] = concat_arrays!(t0.0, t1.0, t2.0, t3.0, d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 4);
    }
}

pub fn emit_log_4_slice<const D: usize, const ALL: usize>(
    t0: &U,
    t1: &U,
    t2: &U,
    t3: &U,
    t4: &U,
    d: [u8; D],
) {
    assert_eq!(ALL, D + 32 * 5, "not properly sized: {}", D + 160);
    let x: [u8; ALL] = concat_arrays!(t0.0, t1.0, t2.0, t3.0, t4.0, d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 5);
    }
}

#[cfg(feature = "alloc")]
pub fn emit_log_0_vec(t0: &U, d: &[u8]) {
    let mut x = t0.to_vec();
    x.extend_from_slice(d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 1);
    }
}

#[cfg(feature = "alloc")]
pub fn emit_log_1_vec(t0: &U, t1: &U, d: &[u8]) {
    let mut x = t0.to_vec();
    x.extend_from_slice(&t1.0);
    x.extend_from_slice(d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 2);
    }
}

#[cfg(feature = "alloc")]
pub fn emit_log_2_vec(t0: &U, t1: &U, t2: &U, d: &[u8]) {
    let mut x = t0.to_vec();
    x.extend_from_slice(&t1.0);
    x.extend_from_slice(&t2.0);
    x.extend_from_slice(d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 3);
    }
}

#[cfg(feature = "alloc")]
pub fn emit_log_3_vec(t0: &U, t1: &U, t2: &U, t3: &U, d: &[u8]) {
    let mut x = t0.to_vec();
    x.extend_from_slice(&t1.0);
    x.extend_from_slice(&t2.0);
    x.extend_from_slice(&t3.0);
    x.extend_from_slice(d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 4);
    }
}

#[cfg(feature = "alloc")]
pub fn emit_log_4_vec(t0: &U, t1: &U, t2: &U, t3: &U, t4: &U, d: &[u8]) {
    let mut x = t0.to_vec();
    x.extend_from_slice(&t1.0);
    x.extend_from_slice(&t2.0);
    x.extend_from_slice(&t3.0);
    x.extend_from_slice(&t4.0);
    x.extend_from_slice(d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 5);
    }
}

// Emit an event, doing some into() use and copying of reference Us to
// make it easier to do printing.
#[macro_export]
macro_rules! emit {
    ($t0:expr) => {{
        const DATA_LEN: usize = 0;
        const ALL_LEN: usize = 32;
        let t0: $crate::U = $t0.into();
        $crate::emit_log_0_slice::<DATA_LEN, ALL_LEN>(&t0, [])
    }};

    ($t0:expr, data: $data:expr) => {{
        let t0: $crate::U = $t0.into();
        $crate::emit_log_0_vec(&t0, $data)
    }};

    ($t0:expr, data: $data:expr, $data_len:expr) => {{
        const DATA_LEN: usize = $data_len;
        const ALL_LEN: usize = DATA_LEN + 32;
        let t0: $crate::U = $t0.into();
        $crate::emit_log_0_slice::<DATA_LEN, ALL_LEN>(&t0, $data)
    }};

    ($t0:expr, $t1:expr) => {{
        const DATA_LEN: usize = 0;
        const ALL_LEN: usize = 32 * 2;
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        $crate::emit_log_1_slice::<DATA_LEN, ALL_LEN>(&t0, &t1, [])
    }};

    ($t0:expr, $t1:expr, data: $data:expr) => {{
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        $crate::emit_log_1_vec(&t0, &t1, $data)
    }};

    ($t0:expr, $t1:expr, data: $data:expr, $data_len:expr) => {{
        const DATA_LEN: usize = $data_len;
        const ALL_LEN: usize = DATA_LEN + 32 * 2;
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        $crate::emit_log_1_slice::<DATA_LEN, ALL_LEN>(&t0, &t1, $data)
    }};

    ($t0:expr, $t1:expr, $t2:expr) => {{
        const DATA_LEN: usize = 0;
        const ALL_LEN: usize = 32 * 3;
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        let t2: $crate::U = $t2.into();
        $crate::emit_log_2_slice::<DATA_LEN, ALL_LEN>(&t0, &t1, &t2, [])
    }};

    ($t0:expr, $t1:expr, $t2:expr, data: $data:expr) => {{
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        let t2: $crate::U = $t2.into();
        $crate::emit_log_2_vec(&t0, &t1, &t2, $data)
    }};

    ($t0:expr, $t1:expr, $t2:expr, data: $data:expr, $data_len:expr) => {{
        const DATA_LEN: usize = $data_len;
        const ALL_LEN: usize = DATA_LEN + 32 * 3;
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        let t2: $crate::U = $t2.into();
        $crate::emit_log_2_slice::<DATA_LEN, ALL_LEN>(&t0, &t1, &t2, $data)
    }};

    ($t0:expr, $t1:expr, $t2:expr, $t3:expr) => {{
        const DATA_LEN: usize = 0;
        const ALL_LEN: usize = 32 * 4;
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        let t2: $crate::U = $t2.into();
        let t3: $crate::U = $t3.into();
        $crate::emit_log_3_slice::<DATA_LEN, ALL_LEN>(&t0, &t1, &t2, &t3, [])
    }};

    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, data: $data:expr) => {{
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        let t2: $crate::U = $t2.into();
        let t3: $crate::U = $t3.into();
        $crate::emit_log_3_vec(&t0, &t1, &t2, &t3, $data)
    }};

    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, data: $data:expr, $data_len:expr) => {{
        const DATA_LEN: usize = $data_len;
        const ALL_LEN: usize = DATA_LEN + 32 * 4;
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        let t2: $crate::U = $t2.into();
        let t3: $crate::U = $t3.into();
        $crate::emit_log_3_slice::<DATA_LEN, ALL_LEN>(&t0, &t1, &t2, &t3, $data)
    }};

    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr) => {{
        const DATA_LEN: usize = 0;
        const ALL_LEN: usize = 32 * 5;
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        let t2: $crate::U = $t2.into();
        let t3: $crate::U = $t3.into();
        let t4: $crate::U = $t4.into();
        $crate::emit_log_4_slice::<DATA_LEN, ALL_LEN>(&t0, &t1, &t2, &t3, &t4, [])
    }};

    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, data: $data:expr) => {{
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        let t2: $crate::U = $t2.into();
        let t3: $crate::U = $t3.into();
        let t4: $crate::U = $t4.into();
        $crate::emit_log_4_vec(&t0, &t1, &t2, &t3, &t4, $data)
    }};

    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, data: $data:expr, $data_len:expr) => {{
        const DATA_LEN: usize = $data_len;
        const ALL_LEN: usize = DATA_LEN + 32 * 5;
        let t0: $crate::U = $t0.into();
        let t1: $crate::U = $t1.into();
        let t2: $crate::U = $t2.into();
        let t3: $crate::U = $t3.into();
        let t4: $crate::U = $t4.into();
        $crate::emit_log_4_slice::<DATA_LEN, ALL_LEN>(&t0, &t1, &t2, &t3, &t4, $data)
    }};
}
