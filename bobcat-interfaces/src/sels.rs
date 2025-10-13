
use bobcat_storage::const_keccak256;

use array_concat::concat_arrays;

pub(crate) const fn leftpad_addr(x: [u8; 20]) -> [u8; 32] {
    concat_arrays!([0u8; 32 - 20], x)
}

pub(crate) const fn leftpad_u8(x: u8) -> [u8; 32] {
    concat_arrays!([0u8; 32 - 1], [x])
}

const fn keccak_sel(x: &[u8]) -> [u8; 4] {
    let x = const_keccak256(x).0;
    [x[0], x[1], x[2], x[3]]
}

#[macro_export]
macro_rules! selectors {
    ($($name:ident = $str:literal),* $(,)?) => {
        $(
            pub(crate) const $name: [u8; 4] = keccak_sel($str);
        )*
    };
}

selectors! {
    SEL_TOTAL_SUPPLY = b"totalSupply()",
    SEL_BALANCE_OF = b"balanceOf(address)",
    SEL_ALLOWANCE = b"allowance(address,address)",
    SEL_TRANSFER = b"transfer(address,uint256)",
    SEL_TRANSFER_FROM = b"transferFrom(address,address,uint256)",
    SEL_APPROVE = b"approve(address,uint256)",
    SEL_PERMIT = b"permit(address,address,uint256,uint256,uint8,bytes32,bytes32)",
    SEL_NONCES = b"nonces(address)",
    SEL_DOMAIN_SEPARATOR = b"DOMAIN_SEPARATOR()"
}
