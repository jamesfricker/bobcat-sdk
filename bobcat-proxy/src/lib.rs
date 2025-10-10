#![no_std]

use array_concat::concat_arrays;

pub type Address= [u8; 20];

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
pub const fn make_eip1967_proxy(addr: Address) -> [u8; 1 + 20 + 204] {
    // Created from eip1967.huff .
    concat_arrays!(
      [0x73],
      addr,
      match const_hex::const_decode_to_array::<204>(b"7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc55603b8060403d393df3365f5f375f5f365f7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc545af45f3d5f5f3e3d911561003957f35bfd") {
         Ok(v) => v,
         Err(_) => panic!("bad eip1967")
      }
    )
}
