#![no_std]

pub use bobcat_maths::{U};

#[macro_export]
macro_rules! read_word_slices {
    ($slice:expr, 1) => {{
        let s = $slice;
        assert!(s.len() >= 32);
        unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) }
    }};
    ($slice:expr, 2) => {{
        let s = $slice;
        assert!(s.len() >= 64);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 3) => {{
        let s = $slice;
        assert!(s.len() >= 96);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 4) => {{
        let s = $slice;
        assert!(s.len() >= 128);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 5) => {{
        let s = $slice;
        assert!(s.len() >= 160);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 6) => {{
        let s = $slice;
        assert!(s.len() >= 192);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 7) => {{
        let s = $slice;
        assert!(s.len() >= 224);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 8) => {{
        let s = $slice;
        assert!(s.len() >= 256);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 9) => {{
        let s = $slice;
        assert!(s.len() >= 288);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 10) => {{
        let s = $slice;
        assert!(s.len() >= 320);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 11) => {{
        let s = $slice;
        assert!(s.len() >= 352);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 12) => {{
        let s = $slice;
        assert!(s.len() >= 384);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[352..384].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 13) => {{
        let s = $slice;
        assert!(s.len() >= 416);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[352..384].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[384..416].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 14) => {{
        let s = $slice;
        assert!(s.len() >= 448);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[352..384].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[384..416].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[416..448].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 15) => {{
        let s = $slice;
        assert!(s.len() >= 480);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[352..384].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[384..416].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[416..448].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[448..480].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 16) => {{
        let s = $slice;
        assert!(s.len() >= 512);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[352..384].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[384..416].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[416..448].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[448..480].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[480..512].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 17) => {{
        let s = $slice;
        assert!(s.len() >= 544);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[352..384].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[384..416].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[416..448].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[448..480].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[480..512].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[512..544].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 18) => {{
        let s = $slice;
        assert!(s.len() >= 576);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[352..384].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[384..416].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[416..448].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[448..480].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[480..512].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[512..544].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[544..576].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 19) => {{
        let s = $slice;
        assert!(s.len() >= 608);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[352..384].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[384..416].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[416..448].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[448..480].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[480..512].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[512..544].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[544..576].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[576..608].as_ptr() as *const [u8; 32]) },
        )
    }};
    ($slice:expr, 20) => {{
        let s = $slice;
        assert!(s.len() >= 640);
        (
            unsafe { &*(s[0..32].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[32..64].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[64..96].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[96..128].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[128..160].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[160..192].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[192..224].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[224..256].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[256..288].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[288..320].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[320..352].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[352..384].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[384..416].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[416..448].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[448..480].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[480..512].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[512..544].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[544..576].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[576..608].as_ptr() as *const [u8; 32]) },
            unsafe { &*(s[608..640].as_ptr() as *const [u8; 32]) },
        )
    }};
}

#[test]
fn test_access() {
    use bobcat_maths::Address;
    let cd = const_hex::const_decode_to_array::<{ 32 * 2 + 4 }>(b"a9059cbb0000000000000000000000006221a9c005f6e47eb398fd867784cacfdcfff4e7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let a = const_hex::const_decode_to_array::<20>(b"6221a9c005f6e47eb398fd867784cacfdcfff4e7")
        .unwrap();
    let (addr, amt) = read_word_slices!(&cd[4..], 2);
    assert_eq!((a, U::MAX), (Address::from(U::from(*addr)), U::from(*amt)));
}
