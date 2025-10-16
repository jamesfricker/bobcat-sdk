#[macro_export]
macro_rules! selectors {
    ($($name:ident = $str:literal),* $(,)?) => {
        $(
            const $name: [u8; 4] = $crate::bobcat_cd::const_keccak_sel($str);
        )*
    };
}
