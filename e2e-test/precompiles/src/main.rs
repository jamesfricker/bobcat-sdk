#![no_main]
#![no_std]

use bobcat_sdk::{
    cd::{const_keccak_sel, read_words},
    entry::*,
    precompiles::{ethereum::ecrecover, superposition::edverify},
};

use ed25519_dalek::{Digest, Sha512, SigningKey};

use sha2::digest::Update;

use array_concat::concat_arrays;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

const SEL_ECRECOVER: [u8; 4] = const_keccak_sel(b"ecrecover_(bytes32,uint8,bytes32,bytes32)");

const SEL_CREATE_ED25519: [u8; 4] = const_keccak_sel(b"createEd25519(bytes32,bytes)");
const SEL_EDVERIFY: [u8; 4] =
    const_keccak_sel(b"testEd25519(bytes32,bytes32,bytes32,bytes32,bytes32)");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = read_args_vec(args_len);
    match args[..4].try_into().unwrap() {
        SEL_ECRECOVER => {
            let (hash, v, r, s) = read_words!(&args[4..], 4);
            write_result_word(
                &ecrecover(*hash, (*v).into(), *r, *s, u64::MAX)
                    .unwrap()
                    .into(),
            );
            0
        }
        SEL_CREATE_ED25519 => {
            let key = SigningKey::from_bytes(&args[4..4 + 32].try_into().unwrap());
            let mut d = Sha512::new();
            Update::update(&mut d, &args[4 + 32 * 3..]);
            let hash: [u8; 64] = d.clone().finalize().into();
            let sig = key.sign_prehashed(d, None).unwrap();
            let x: [u8; 32 * 5] =
                concat_arrays!(hash, *key.verifying_key().as_bytes(), sig.to_bytes());
            write_result_slice(&x);
            0
        }
        SEL_EDVERIFY => {
            let digest: [u8; 64] = args[4..4 + 64].try_into().unwrap();
            let key: U = args[4 + 64..4 + 64 + 32].try_into().unwrap();
            let sig: [u8; 64] = args[4 + 64 + 32..].try_into().unwrap();
            edverify(digest, key, sig);
            0
        }
        _ => unimplemented!(),
    }
}
