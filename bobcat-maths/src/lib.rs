#![cfg_attr(not(feature = "std"), no_std)]

use core::{
    cmp::{Eq, Ordering},
    ops::{Add, Div, Index, Mul, Rem, Sub},
};

use num_traits::{One, Zero};

#[link(wasm_import_module = "vm_hooks")]
#[cfg(not(feature = "alloy-enabled"))]
unsafe extern "C" {
    fn math_div(x: *const u8, y: *const u8, out: *mut u8);
    fn math_mod(x: *const u8, y: *const u8, out: *mut u8);
    fn math_add_mod(a: *mut u8, b: *const u8, c: *const u8);
    fn math_mul_mod(a: *mut u8, b: *const u8, c: *const u8);
}

#[cfg(feature = "alloy-enabled")]
mod alloy {
    use core::ptr::copy_nonoverlapping;

    pub(crate) use alloy_primitives::U256;

    #[cfg(test)]
    pub(crate) use alloy_primitives::I256;

    pub(crate) unsafe fn math_div(x: *const u8, y: *const u8, out: *mut u8) {
        unsafe {
            let x = U256::from_be_slice(&*(x as *const [u8; 32]));
            let y = U256::from_be_slice(&*(y as *const [u8; 32]));
            let z = x / y;
            copy_nonoverlapping(z.to_be_bytes::<32>().as_ptr(), out, 32);
        }
    }

    pub(crate) unsafe fn math_mod(x: *const u8, y: *const u8, out: *mut u8) {
        unsafe {
            let x = U256::from_be_slice(&*(x as *const [u8; 32]));
            let y = U256::from_be_slice(&*(y as *const [u8; 32]));
            let z = x % y;
            copy_nonoverlapping(z.to_be_bytes::<32>().as_ptr(), out, 32);
        }
    }

    pub(crate) unsafe fn math_add_mod(a: *mut u8, b: *const u8, c: *const u8) {
        unsafe {
            let x = U256::from_be_slice(&*(a as *const [u8; 32]));
            let y = U256::from_be_slice(&*(b as *const [u8; 32]));
            let z = U256::from_be_slice(&*(c as *const [u8; 32]));
            let x = x.add_mod(y, z);
            copy_nonoverlapping(x.to_be_bytes::<32>().as_ptr(), a, 32);
        }
    }

    pub(crate) unsafe fn math_mul_mod(a: *mut u8, b: *const u8, c: *const u8) {
        unsafe {
            let x = U256::from_be_slice(&*(a as *const [u8; 32]));
            let y = U256::from_be_slice(&*(b as *const [u8; 32]));
            let z = U256::from_be_slice(&*(c as *const [u8; 32]));
            let x = x.add_mod(y, z);
            copy_nonoverlapping(x.to_be_bytes::<32>().as_ptr(), a, 32);
        }
    }
}

#[cfg(feature = "alloy-enabled")]
use alloy::*;

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct U(pub [u8; 32]);

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct I(pub [u8; 32]);

fn div(x: &U, y: &U) -> U {
    let mut b = [0u8; 32];
    unsafe { math_div(x.0.as_ptr(), y.0.as_ptr(), b.as_mut_ptr()) }
    U(b)
}

fn modd(x: &U, y: &U) -> U {
    let mut b = [0u8; 32];
    unsafe { math_mod(x.0.as_ptr(), y.0.as_ptr(), b.as_mut_ptr()) }
    U(b)
}

pub const fn add(x: &U, y: &U) -> U {
    // Implemented like this since unfortunately ruint (which the node
    // uses) does not have special behaviour for mod being 0, except
    // returning zero. So we need to implement this ourselves.
    // TODO: if wasm ever sees SIMD or something, try that too.
    let mut r = [0u8; 32];
    let mut c = 0;
    let mut i = 31;
    loop {
        let sum = x.0[i] as u16 + y.0[i] as u16 + c;
        r[i] = sum as u8;
        c = sum >> 8;
        if i == 0 {
            break;
        }
        i -= 1;
    }
    U(r)
}

pub const fn sub(x: &U, y: &U) -> U {
    let mut neg_y = y.0;
    let mut i = 0;
    while i < 32 {
        neg_y[i] = !neg_y[i];
        i += 1;
    }
    let mut c = 1u16;
    let mut i = 31;
    loop {
        let sum = neg_y[i] as u16 + c;
        neg_y[i] = sum as u8;
        c = sum >> 8;
        if i == 0 {
            break;
        }
        i -= 1;
    }
    add(x, &U(neg_y))
}

pub const fn mul(x: &U, y: &U) -> U {
    let mut r = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        let mut c = 0u16;
        let mut j = 0;
        while j < 32 {
            let i_r = i + j;
            if i_r >= 32 {
                break;
            }
            let r_idx = 31 - i_r;
            let xi = x.0[31 - i] as u16;
            let yj = y.0[31 - j] as u16;
            let prod = xi * yj + r[r_idx] as u16 + c;
            r[r_idx] = prod as u8;
            c = prod >> 8;
            j += 1;
        }
        if i + j < 32 {
            let idx = 31 - (i + j);
            r[idx] = r[idx] + c as u8;
        }
        i += 1;
    }
    U(r)
}

impl Add for U {
    type Output = U;

    fn add(self, rhs: U) -> U {
        add(&self, &rhs)
    }
}

impl Add for &U {
    type Output = U;

    fn add(self, rhs: &U) -> U {
        add(self, rhs)
    }
}

impl Sub for U {
    type Output = U;

    fn sub(self, rhs: U) -> U {
        sub(&self, &rhs)
    }
}

impl Sub for &U {
    type Output = U;

    fn sub(self, rhs: &U) -> U {
        sub(self, rhs)
    }
}

impl Mul for U {
    type Output = U;

    fn mul(self, rhs: U) -> U {
        mul(&self, &rhs)
    }
}

impl Mul for &U {
    type Output = U;

    fn mul(self, rhs: &U) -> U {
        mul(self, rhs)
    }
}

impl Div for U {
    type Output = U;

    fn div(self, rhs: U) -> U {
        div(&self, &rhs)
    }
}

impl Div for &U {
    type Output = U;

    fn div(self, rhs: &U) -> U {
        div(self, rhs)
    }
}

impl Rem for U {
    type Output = U;

    fn rem(self, rhs: U) -> U {
        modd(&self, &rhs)
    }
}

impl Rem for &U {
    type Output = U;

    fn rem(self, rhs: &U) -> U {
        modd(self, rhs)
    }
}

impl Eq for U {}

impl PartialOrd for U {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl U {
    pub const ZERO: Self = U([0u8; 32]);

    pub const ONE: Self = U([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]);

    pub fn is_true(&self) -> bool {
        self.0[31] == 1
    }

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    pub fn is_some(&self) -> bool {
        !self.is_zero()
    }

    pub fn as_slice(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn mul_mod(&self, y: &Self, z: &Self) -> Self {
        let mut b = self.0;
        unsafe { math_mul_mod(b.as_mut_ptr(), y.0.as_ptr(), z.0.as_ptr()) }
        Self(b)
    }

    pub fn add_mod(x: &Self, y: &Self, z: &Self) -> Self {
        let mut b = x.0;
        unsafe { math_add_mod(b.as_mut_ptr(), y.0.as_ptr(), z.0.as_ptr()) }
        Self(b)
    }
}

impl From<U> for [u8; 32] {
    fn from(x: U) -> Self {
        x.0
    }
}

impl From<[u8; 32]> for U {
    fn from(x: [u8; 32]) -> Self {
        U(x)
    }
}

impl TryFrom<&[u8]> for U {
    type Error = &'static str;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() > 32 {
            return Err("too large");
        }
        let mut b = [0u8; 32];
        b[32-v.len()..].copy_from_slice(v);
        Ok(U(b))
    }
}

impl Zero for U {
    fn zero() -> Self {
        U::ZERO
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }
}

impl Default for U {
    fn default() -> Self {
        U::ZERO
    }
}

impl One for U {
    fn one() -> Self {
        U::ONE
    }
}

impl Index<usize> for U {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl I {
    fn is_neg(&self) -> bool {
        self.0[0] & 0x80 != 0
    }

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    pub fn is_some(&self) -> bool {
        !self.is_zero()
    }

    pub fn as_slice(&self) -> &[u8; 32] {
        &self.0
    }

    fn neg(&self) -> Self {
        I((U(self.0.map(|b| !b)) + U::ONE).0)
    }

    fn abs(self) -> U {
        if self.is_neg() {
            U(self.neg().0)
        } else {
            U(self.0)
        }
    }
}

macro_rules! from_slices {
    ($($n:expr),+ $(,)?) => {
        $(
            impl From<&[u8; $n]> for U {
                fn from(x: &[u8; $n]) -> Self {
                    let mut b = [0u8; 32];
                    b[32 - $n..].copy_from_slice(x);
                    U(b)
                }
            }

            impl From<[u8; $n]> for U {
                fn from(x: [u8; $n]) -> Self {
                    U::from(&x)
                }
            }
        )+
    };
}

from_slices!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31
);

impl From<I> for [u8; 32] {
    fn from(x: I) -> Self {
        x.0
    }
}

impl From<[u8; 32]> for I {
    fn from(x: [u8; 32]) -> Self {
        I(x)
    }
}

fn i_add(x: &I, y: &I) -> I {
    I(add(&U(x.0), &U(y.0)).0)
}

fn i_sub(x: &I, y: &I) -> I {
    I(sub(&U(x.0), &U(y.0)).0)
}

fn i_mul(x: &I, y: &I) -> I {
    let result = mul(&U(x.0), &U(y.0));
    I(result.0)
}

fn i_div(x: &I, y: &I) -> I {
    let r = div(&x.abs(), &y.abs());
    if x.is_neg() ^ y.is_neg() {
        I(r.0).neg()
    } else {
        I(r.0)
    }
}

fn i_rem(x: &I, y: &I) -> I {
    let r = modd(&x.abs(), &y.abs());
    if x.is_neg() {
        I(r.0).neg()
    } else {
        I(r.0)
    }
}

impl Add for I {
    type Output = I;
    fn add(self, rhs: I) -> I {
        i_add(&self, &rhs)
    }
}

impl Add for &I {
    type Output = I;
    fn add(self, rhs: &I) -> I {
        i_add(self, rhs)
    }
}

impl Sub for I {
    type Output = I;
    fn sub(self, rhs: I) -> I {
        i_sub(&self, &rhs)
    }
}

impl Sub for &I {
    type Output = I;
    fn sub(self, rhs: &I) -> I {
        i_sub(self, rhs)
    }
}

impl Mul for I {
    type Output = I;
    fn mul(self, rhs: I) -> I {
        i_mul(&self, &rhs)
    }
}

impl Mul for &I {
    type Output = I;
    fn mul(self, rhs: &I) -> I {
        i_mul(self, rhs)
    }
}

impl Div for I {
    type Output = I;
    fn div(self, rhs: I) -> I {
        i_div(&self, &rhs)
    }
}

impl Div for &I {
    type Output = I;
    fn div(self, rhs: &I) -> I {
        i_div(self, rhs)
    }
}

impl Rem for I {
    type Output = I;
    fn rem(self, rhs: I) -> I {
        i_rem(&self, &rhs)
    }
}

impl Eq for I {}

impl PartialOrd for I {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for I {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_sign = self.0[0] & 0x80;
        let other_sign = other.0[0] & 0x80;
        match (self_sign, other_sign) {
            (0, 0x80) => Ordering::Greater,
            (0x80, 0) => Ordering::Less,
            _ => self.0.cmp(&other.0),
        }
    }
}

impl Rem for &I {
    type Output = I;
    fn rem(self, rhs: &I) -> I {
        i_rem(self, rhs)
    }
}

impl I {
    pub const ZERO: Self = I([0u8; 32]);

    pub const ONE: Self = I([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]);
}

impl Zero for I {
    fn zero() -> Self {
        I::ZERO
    }
    fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }
}

impl Default for I {
    fn default() -> Self {
        I::ZERO
    }
}

impl One for I {
    fn one() -> Self {
        I::ONE
    }
}

#[test]
fn test_is_zeroes() {
    assert!(U::ZERO.is_zero());
    assert!(U::ONE.is_some());
    assert!(I::ZERO.is_zero());
    assert!(I::ONE.is_some());
}

#[cfg(all(test, feature = "alloy-enabled", feature = "std"))]
mod test {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn test_u_is_zero(x in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            assert_eq!(ex.is_zero(), x.is_zero());
        }

        #[test]
        fn test_u_div(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_div(ey)).to_be_bytes(), (x / y).0);
        }

        #[test]
        fn test_u_mul(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_mul(ey)).to_be_bytes(), (x * y).0);
        }

        #[test]
        fn test_u_mod(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!((ex % ey).to_be_bytes(), (x % y).0);
        }

        #[test]
        fn test_u_add(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_add(ey)).to_be_bytes(), (x + y).0);
        }

        #[test]
        fn test_u_sub(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_sub(ey)).to_be_bytes(), (x - y).0);
        }

        #[test]
        fn test_u_cmp(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!(ex.cmp(&ey), x.cmp(&y));
        }

        #[test]
        fn test_i_is_zero(x in any::<U>()) {
            let ex = I256::from_be_bytes(x.0);
            assert_eq!(ex.is_zero(), x.is_zero());
        }

        #[test]
        fn test_i_div(x in any::<I>(), y in any::<I>()) {
            let ex = I256::from_be_bytes(x.0);
            let ey = I256::from_be_bytes(y.0);
            assert_eq!((ex / ey).to_be_bytes(), (x / y).0);
        }

        #[test]
        fn test_i_mul(x in any::<I>(), y in any::<I>()) {
            let ex = I256::from_be_bytes(x.0);
            let ey = I256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_mul(ey)).to_be_bytes(), (x * y).0);
        }

        #[test]
        fn test_i_mod(x in any::<I>(), y in any::<I>()) {
            let ex = I256::from_be_bytes(x.0);
            let ey = I256::from_be_bytes(y.0);
            assert_eq!((ex % ey).to_be_bytes(), (x % y).0);
        }

        #[test]
        fn test_i_add(x in any::<I>(), y in any::<I>()) {
            let ex = I256::from_be_bytes(x.0);
            let ey = I256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_add(ey)).to_be_bytes(), (x + y).0);
        }

        #[test]
        fn test_i_sub(x in any::<I>(), y in any::<I>()) {
            let ex = I256::from_be_bytes(x.0);
            let ey = I256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_sub(ey)).to_be_bytes(), (x - y).0);
        }

        #[test]
        fn test_i_cmp(x in any::<I>(), y in any::<I>()) {
            let ex = I256::from_be_bytes(x.0);
            let ey = I256::from_be_bytes(y.0);
            assert_eq!(ex.cmp(&ey), x.cmp(&y));
        }
    }
}
