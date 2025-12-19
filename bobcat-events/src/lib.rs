#![no_std]

pub use bobcat_maths::U;

#[cfg(not(feature = "shadow"))]
use array_concat::concat_arrays;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod impls {
    #[link(wasm_import_module = "vm_hooks")]
    unsafe extern "C" {
        pub(crate) fn emit_log(data: *const u8, len: usize, topics: usize);
        pub(crate) fn write_result(ptr: *const u8, len: usize);
    }
}

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
#[allow(unused)]
mod impls {
    pub(crate) unsafe fn emit_log(_: *const u8, _: usize, _: usize) {}
    pub(crate) unsafe fn write_result(_: *const u8, _: usize) {}
}

// ShadowTable that's intended for the 32 wasm machine to do shadow event
// logging with custom Nitro instances with a whitelisted pool of nodes
// to log from.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
struct ShadowTable<const DATA: usize> {
    magic: u8,
    topics_len: usize,
    data_len: usize,
    topics: [U; 5],
    data: [u8; DATA],
}

const SHADOW_MAGIC_BYTE: u8 = 0xee;

impl<const DATA: usize> Default for ShadowTable<DATA> {
    fn default() -> Self {
        ShadowTable {
            magic: SHADOW_MAGIC_BYTE,
            topics_len: 0,
            data_len: 0,
            topics: [U::ZERO; 5],
            data: [0u8; DATA],
        }
    }
}

#[cfg(feature = "shadow")]
pub fn emit_log_0_slice<const D: usize, const ALL: usize>(t0: &U, d: [u8; D]) {
    let mut t = ShadowTable {
        data_len: D,
        data: d,
        ..ShadowTable::<D>::default()
    };
    t.topics[0] = *t0;
    unsafe {
        impls::write_result(&t as *const _ as *const u8, 0);
    }
}

#[cfg(not(feature = "shadow"))]
pub fn emit_log_0_slice<const D: usize, const ALL: usize>(t0: &U, d: [u8; D]) {
    assert_eq!(ALL, D + 32, "not properly sized: {}", D + 32);
    let x: [u8; ALL] = concat_arrays!(t0.0, d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 1);
    }
}

#[cfg(not(feature = "shadow"))]
pub fn emit_log_1_slice<const D: usize, const ALL: usize>(t0: &U, t1: &U, d: [u8; D]) {
    assert_eq!(ALL, D + 32 * 2, "not properly sized: {}", D + 64);
    let x: [u8; ALL] = concat_arrays!(t0.0, t1.0, d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 2);
    }
}

#[cfg(feature = "shadow")]
pub fn emit_log_1_slice<const D: usize, const ALL: usize>(t0: &U, t1: &U, d: [u8; D]) {
    let mut t = ShadowTable {
        topics_len: 1,
        data_len: D,
        data: d,
        ..ShadowTable::<D>::default()
    };
    t.topics[0] = *t0;
    t.topics[1] = *t1;
    unsafe {
        impls::write_result(&t as *const _ as *const u8, 0);
    }
}

#[cfg(not(feature = "shadow"))]
pub fn emit_log_2_slice<const D: usize, const ALL: usize>(t0: &U, t1: &U, t2: &U, d: [u8; D]) {
    assert_eq!(ALL, D + 32 * 3, "not properly sized: {}", D + 96);
    let x: [u8; ALL] = concat_arrays!(t0.0, t1.0, t2.0, d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 3);
    }
}

#[cfg(feature = "shadow")]
pub fn emit_log_2_slice<const D: usize, const ALL: usize>(t0: &U, t1: &U, t2: &U, d: [u8; D]) {
    let mut t = ShadowTable {
        topics_len: 2,
        data_len: D,
        data: d,
        ..ShadowTable::<D>::default()
    };
    t.topics[0] = *t0;
    t.topics[1] = *t1;
    t.topics[2] = *t2;
    unsafe {
        impls::write_result(&t as *const _ as *const u8, 0);
    }
}

#[cfg(not(feature = "shadow"))]
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

#[cfg(feature = "shadow")]
pub fn emit_log_3_slice<const D: usize, const ALL: usize>(
    t0: &U,
    t1: &U,
    t2: &U,
    t3: &U,
    d: [u8; D],
) {
    let mut t = ShadowTable {
        topics_len: 3,
        data_len: D,
        data: d,
        ..ShadowTable::<D>::default()
    };
    t.topics[0] = *t0;
    t.topics[1] = *t1;
    t.topics[2] = *t2;
    t.topics[3] = *t3;
    unsafe {
        impls::write_result(&t as *const _ as *const u8, 0);
    }
}

#[cfg(not(feature = "shadow"))]
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

#[cfg(feature = "shadow")]
pub fn emit_log_4_slice<const D: usize, const ALL: usize>(
    t0: &U,
    t1: &U,
    t2: &U,
    t3: &U,
    t4: &U,
    d: [u8; D],
) {
    let mut t = ShadowTable {
        topics_len: 4,
        data_len: D,
        data: d,
        ..ShadowTable::<D>::default()
    };
    t.topics[0] = *t0;
    t.topics[1] = *t1;
    t.topics[2] = *t2;
    t.topics[3] = *t3;
    t.topics[4] = *t4;
    unsafe {
        impls::write_result(&t as *const _ as *const u8, 0);
    }
}

#[cfg(all(feature = "alloc", not(feature = "shadow")))]
pub fn emit_log_0_vec(t0: &U, d: &[u8]) {
    let mut x = t0.to_vec();
    x.extend_from_slice(d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 1);
    }
}

#[cfg(all(feature = "alloc", not(feature = "shadow")))]
pub fn emit_log_1_vec(t0: &U, t1: &U, d: &[u8]) {
    let mut x = t0.to_vec();
    x.extend_from_slice(&t1.0);
    x.extend_from_slice(d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 2);
    }
}

#[cfg(all(feature = "alloc", not(feature = "shadow")))]
pub fn emit_log_2_vec(t0: &U, t1: &U, t2: &U, d: &[u8]) {
    let mut x = t0.to_vec();
    x.extend_from_slice(&t1.0);
    x.extend_from_slice(&t2.0);
    x.extend_from_slice(d);
    unsafe {
        impls::emit_log(x.as_ptr(), x.len(), 3);
    }
}

#[cfg(all(feature = "alloc", not(feature = "shadow")))]
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

#[cfg(all(feature = "alloc", not(feature = "shadow")))]
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
