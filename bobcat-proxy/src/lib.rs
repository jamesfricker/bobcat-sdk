#![no_std]

use array_concat::concat_arrays;

pub type Address = [u8; 20];

pub const fn make_minimal_proxy(addr: Address) -> [u8; 18 + 16 + 20] {
    // I can't remember where this comes from (TODO), but we use this in
    // 9lives for our share proxies.
    concat_arrays!(
        [
            0x60, 0x2d, 0x5f, 0x81, 0x60, 0x09, 0x5f, 0x39, 0xf3, 0x5f, 0x5f, 0x36, 0x5f, 0x5f,
            0x37, 0x36, 0x5f, 0x73,
        ],
        addr,
        [
            0x5a, 0xf4, 0x3d, 0x5f, 0x5f, 0x3e, 0x60, 0x29, 0x57, 0x3d, 0x5f, 0xfd, 0x5b, 0x3d,
            0x5f, 0xf3,
        ]
    )
}

/// Make a EIP1967 proxy that reads from the standard storage slot.
pub const fn make_eip1967_proxy(logic: Address) -> [u8; 1 + 20 + 102] {
    // Created from eip1967.huff .
    concat_arrays!(
      [0x73],
      logic,
      match const_hex::const_decode_to_array::<101>(b"7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc55603a8060403d393df3365f5f375f5f365f7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc545af45f3d5f5f3e3d9161003857fd5bf3") {
         Ok(v) => v,
         Err(_) => panic!("bad eip1967")
      }
    )
}

/// Make a non-upgradeable beacon proxy that calls
/// "implementation()(address)" to get the logic address to delegate to.
pub const fn make_beacon_proxy(beacon: Address) -> [u8; 1 + 20 + 123] {
    // Created from beacon-proxy.huff .
    concat_arrays!(
        [0x73],
        beacon,
        match const_hex::const_decode_to_array::<123>(b"7fa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d505560508060403d393df3365f5f375f5f365f635c60da1b602052602060206004603c7fa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50545afa506020515af45f3d5f5f3e3d9161004e57fd5bf3") {
            Ok(v) => v,
            Err(_) => panic!()
        }
    )
}
