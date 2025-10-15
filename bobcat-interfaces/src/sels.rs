use bobcat_cd::const_keccak_sel;

macro_rules! selectors {
    ($($name:ident = $str:literal),* $(,)?) => {
        $(
            pub const $name: [u8; 4] = const_keccak_sel($str);
        )*
    };
}

selectors! {
    SEL_IMPLEMENTATION = b"implementation(bytes4)",
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

#[test]
fn test_allowance() {
    assert_eq!([0xdd, 0x62, 0xed, 0x3e], SEL_ALLOWANCE);
}
