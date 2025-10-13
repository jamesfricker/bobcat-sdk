#![cfg_attr(not(feature = "std"), no_std)]

use core::{
    cmp::{Eq, Ordering},
    ops::{Add, Deref, DerefMut, Div, Index, Mul, Rem, Sub},
};

use num_traits::{One, Zero};

#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSerialize};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
use alloc::vec;

#[cfg(feature = "std")]
use core::fmt::{Display, Formatter};

pub type Address = [u8; 20];

#[link(wasm_import_module = "vm_hooks")]
#[cfg(not(feature = "alloy-enabled"))]
unsafe extern "C" {
    fn math_div(x: *mut u8, y: *const u8);
    fn math_mod(x: *mut u8, y: *const u8);
    fn math_add_mod(a: *mut u8, b: *const u8, c: *const u8);
    fn math_mul_mod(a: *mut u8, b: *const u8, c: *const u8);
}

#[cfg(feature = "alloy-enabled")]
mod alloy {
    use core::ptr::copy_nonoverlapping;

    pub(crate) use alloy_primitives::U256;

    #[cfg(test)]
    pub(crate) use alloy_primitives::I256;

    pub(crate) unsafe fn math_div(out: *mut u8, y: *const u8) {
        unsafe {
            let x = U256::from_be_slice(&*(out as *const [u8; 32]));
            let y = U256::from_be_slice(&*(y as *const [u8; 32]));
            let z = if y.is_zero() {
                // TODO: I think the node returns 0 when this is the case.
                U256::ZERO
            } else {
                x / y
            };
            copy_nonoverlapping(z.to_be_bytes::<32>().as_ptr(), out, 32);
        }
    }

    pub(crate) unsafe fn math_mod(out: *mut u8, y: *const u8) {
        unsafe {
            let x = U256::from_be_slice(&*(out as *const [u8; 32]));
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
            let x = x.mul_mod(y, z);
            copy_nonoverlapping(x.to_be_bytes::<32>().as_ptr(), a, 32);
        }
    }
}

#[cfg(feature = "alloy-enabled")]
use alloy::*;

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize))]
#[repr(transparent)]
pub struct U(pub [u8; 32]);

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize))]
#[repr(transparent)]
pub struct I(pub [u8; 32]);

pub fn wrapping_div(x: &U, y: &U) -> U {
    let mut b = *x;
    unsafe { math_div(b.as_mut_ptr(), y.as_ptr()) }
    b
}

#[cfg_attr(test, mutants::skip)]
pub fn checking_div(x: &U, y: &U) -> Option<U> {
    if y.is_zero() {
        None
    } else {
        Some(wrapping_div(x, y))
    }
}

pub fn modd(x: &U, y: &U) -> U {
    let mut b = *x;
    unsafe { math_mod(b.as_mut_ptr(), y.as_ptr()) }
    b
}

pub const fn wrapping_add(x: &U, y: &U) -> U {
    let mut r = [0u8; 32];
    let mut c = 0;
    let mut i = 31;
    loop {
        let s = x.0[i] as u16 + y.0[i] as u16 + c;
        r[i] = s as u8;
        c = s >> 8;
        if i == 0 {
            break;
        }
        i -= 1;
    }
    U(r)
}

#[cfg_attr(test, mutants::skip)]
pub fn checking_add(x: &U, y: &U) -> Option<U> {
    if x > &(U::MAX - *y) {
        None
    } else {
        let z = x.add_mod(y, &U::MAX);
        if z.is_zero() && (x.is_some() || y.is_some()) {
            Some(U::MAX)
        } else {
            Some(z)
        }
    }
}

pub const fn wrapping_sub(x: &U, y: &U) -> U {
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
    wrapping_add(x, &U(neg_y))
}

#[cfg_attr(test, mutants::skip)]
pub fn checking_sub(x: &U, y: &U) -> Option<U> {
    if x < y {
        None
    } else {
        Some(wrapping_sub(x, y))
    }
}

pub const fn wrapping_mul(x: &U, y: &U) -> U {
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

#[cfg_attr(test, mutants::skip)]
pub fn checking_mul(x: &U, y: &U) -> Option<U> {
    if x.is_zero() || y.is_zero() {
        return Some(U::zero());
    }
    if x > &(U::MAX / *y) {
        None
    } else {
        let z = x.mul_mod(y, &U::MAX);
        if z.is_zero() {
            Some(U::MAX)
        } else {
            Some(z)
        }
    }
}

impl Add for U {
    type Output = U;

    fn add(self, rhs: U) -> U {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                checking_add(&self, &rhs).expect("overflow when add")
            } else {
                wrapping_add(&self, &rhs)
            }
        }
    }
}

impl Add for &U {
    type Output = U;

    fn add(self, rhs: &U) -> U {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                checking_add(self, rhs).expect("overflow when add")
            } else {
                wrapping_add(self, rhs)
            }
        }
    }
}

impl Sub for U {
    type Output = U;

    fn sub(self, rhs: U) -> U {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                checking_sub(&self, &rhs).expect("overflow when sub")
            } else {
                wrapping_sub(&self, &rhs)
            }
        }
    }
}

impl Sub for &U {
    type Output = U;

    fn sub(self, rhs: &U) -> U {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                checking_sub(self, rhs).expect("overflow when sub")
            } else {
                wrapping_sub(self, rhs)
            }
        }
    }
}

impl Mul for U {
    type Output = U;

    fn mul(self, rhs: U) -> U {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                checking_mul(&self, &rhs).expect("overflow when mul")
            } else {
                wrapping_mul(&self, &rhs)
            }
        }
    }
}

impl Mul for &U {
    type Output = U;

    fn mul(self, rhs: &U) -> U {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                checking_mul(self, rhs).expect("overflow when mul")
            } else {
                wrapping_mul(self, rhs)
            }
        }
    }
}

impl Div for U {
    type Output = U;

    fn div(self, rhs: U) -> U {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                checking_div(&self, &rhs).expect("overflow when div")
            } else {
                wrapping_div(&self, &rhs)
            }
        }
    }
}

impl Div for &U {
    type Output = U;

    fn div(self, rhs: &U) -> U {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                checking_div(self, rhs).expect("overflow when div")
            } else {
                wrapping_div(self, rhs)
            }
        }
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

    pub const MAX: Self = U([u8::MAX; 32]);

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

    pub fn wrapping_add(&self, y: &Self) -> U {
        wrapping_add(self, y)
    }

    pub fn checking_add(&self, y: &Self) -> Option<Self> {
        checking_add(self, y)
    }

    pub fn wrapping_sub(&self, y: &Self) -> U {
        wrapping_sub(self, y)
    }

    pub fn checking_sub(&self, y: &Self) -> Option<Self> {
        checking_sub(self, y)
    }

    pub fn wrapping_mul(&self, y: &Self) -> U {
        wrapping_mul(self, y)
    }

    pub fn checking_mul(&self, y: &Self) -> Option<Self> {
        checking_mul(self, y)
    }

    pub fn wrapping_div(&self, y: &Self) -> U {
        wrapping_div(self, y)
    }

    pub fn checking_div(&self, y: &Self) -> Option<Self> {
        checking_div(self, y)
    }

    pub fn mul_mod(&self, y: &Self, z: &Self) -> Self {
        let mut b = self.0;
        unsafe { math_mul_mod(b.as_mut_ptr(), y.as_ptr(), z.as_ptr()) }
        Self(b)
    }

    pub fn add_mod(&self, y: &Self, z: &Self) -> Self {
        let mut b = self.0;
        unsafe { math_add_mod(b.as_mut_ptr(), y.as_ptr(), z.as_ptr()) }
        Self(b)
    }

    pub fn widening_mul(&self, y: &U) -> (U, U) {
        let shift_128 = &U([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        let x_hi = self / shift_128;
        let x_lo = self % shift_128;
        let y_hi = y / shift_128;
        let y_lo = y % shift_128;
        let t0 = x_lo.mul_mod(&y_lo, &U::MAX);
        let t1 = x_hi.mul_mod(&y_lo, &U::MAX);
        let t2 = x_lo.mul_mod(&y_hi, &U::MAX);
        let t3 = x_hi.mul_mod(&y_hi, &U::MAX);
        let t0_hi = &t0 / shift_128;
        let t0_lo = &t0 % shift_128;
        let t1_hi = &t1 / shift_128;
        let t1_lo = &t1 % shift_128;
        let t2_hi = &t2 / shift_128;
        let t2_lo = &t2 % shift_128;
        let mid = (t0_hi + t1_lo) + t2_lo;
        let mid_hi = &mid / shift_128;
        let mid_lo = &mid % shift_128;
        let mid_lo_shifted = mid_lo.mul_mod(shift_128, &U::MAX);
        let out_low = t0_lo + mid_lo_shifted;
        let out_high = t3 + t1_hi + t2_hi + mid_hi;
        (out_high, out_low)
    }

    pub fn mul_div(&self, _y: &U, _denom: &U) -> Option<(U, bool)> {
        todo!()
    }

    pub fn mul_div_round_up(&self, y: &U, denom_and_rem: &U) -> Option<U> {
        let (x, y) = self.mul_div(y, denom_and_rem)?;
        Some(if y { x + U::ONE } else { x })
    }
}

#[cfg(feature = "std")]
impl Display for U {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = vec![0u8];
        for &byte in &self.0 {
            let mut carry = byte as u32;
            for digit in result.iter_mut() {
                let temp = (*digit as u32) * 256 + carry;
                *digit = (temp % 10) as u8;
                carry = temp / 10;
            }
            while carry > 0 {
                result.push((carry % 10) as u8);
                carry /= 10;
            }
        }
        if result.iter().all(|&d| d == 0) {
            return write!(f, "0");
        }
        write!(
            f,
            "{}",
            result
                .iter()
                .rev()
                .skip_while(|&&d| d == 0)
                .map(|&d| (d + b'0') as char)
                .collect::<String>()
        )
    }
}

impl From<U> for [u8; 32] {
    fn from(x: U) -> Self {
        x.0
    }
}

impl From<&[u8; 32]> for &U {
    fn from(x: &[u8; 32]) -> Self {
        unsafe { &*(x as *const [u8; 32] as *const U) }
    }
}

impl From<[u8; 32]> for U {
    fn from(x: [u8; 32]) -> Self {
        U(x)
    }
}

impl Deref for U {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for U {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<bool> for U {
    fn from(x: bool) -> Self {
        U::from(&[x as u8])
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
        let x = wrapping_add(&U(self.0.map(|b| !b)), &U::ONE);
        I(x.0)
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

            impl From<U> for [u8; $n] {
                fn from(x: U) -> Self {
                    unsafe { *(x.as_ptr().add(32 - $n) as *const [u8; $n]) }
                }
            }
        )+
    };
}

from_slices!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31
);

macro_rules! from_ints {
    ($($t:ty),+ $(,)?) => {
        $(
            impl From<$t> for U {
                fn from(x: $t) -> Self {
                    let mut b = [0u8; 32];
                    b[32 - core::mem::size_of::<$t>()..].copy_from_slice(&x.to_be_bytes());
                    U(b)
                }
            }

            impl From<U> for $t {
                fn from(x: U) -> Self {
                    Self::from_be_bytes(x.into())
                }
            }
        )+
    };
}

from_ints! { u8, u16, u32, u64, u128 }

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
    I(wrapping_add(&U(x.0), &U(y.0)).0)
}

fn i_sub(x: &I, y: &I) -> I {
    I(wrapping_sub(&U(x.0), &U(y.0)).0)
}

fn i_mul(x: &I, y: &I) -> I {
    let result = wrapping_mul(&U(x.0), &U(y.0));
    I(result.0)
}

fn i_div(x: &I, y: &I) -> I {
    let r = wrapping_div(&x.abs(), &y.abs());
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
        fn test_u_is_zero(x in any::<[u8; 32]>()) {
            let x = U::from(x);
            let ex = U256::from_be_bytes(x.0);
            assert_eq!(ex.is_zero(), x.is_zero());
        }

        #[test]
        fn test_u_div(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_div(ey)).to_be_bytes(), x.wrapping_div(&y).0);
        }

        #[test]
        fn test_u_mul(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_mul(ey)).to_be_bytes(), wrapping_mul(&x,  &y).0);
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
            let e = U::from(ex.wrapping_add(ey).to_be_bytes::<32>());
            assert_eq!(e, x.wrapping_add(&y), "{e} != {}", x + y);
        }

        #[test]
        fn test_u_sub(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!((ex.wrapping_sub(ey)).to_be_bytes(), x.wrapping_sub(&y).0);
        }

        #[test]
        fn test_u_cmp(x in any::<U>(), y in any::<U>()) {
            let ex = U256::from_be_bytes(x.0);
            let ey = U256::from_be_bytes(y.0);
            assert_eq!(ex.cmp(&ey), x.cmp(&y));
        }

        #[test]
        #[cfg(feature = "alloc")]
        fn test_u_str(x in any::<U>()) {
            assert_eq!(U256::from_be_bytes(x.0).to_string(), x.to_string());
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

        #[test]
        fn test_u_u8(x in any::<u8>()) {
            let mut b = [0u8; 32];
            b[32-std::mem::size_of::<u8>()..].copy_from_slice(&x.to_be_bytes());
            assert_eq!(&U256::from_be_bytes(b).to_be_bytes(), U::from(x).as_slice());
        }

        #[test]
        fn test_u_u16(x in any::<u16>()) {
            let mut b = [0u8; 32];
            b[32-std::mem::size_of::<u16>()..].copy_from_slice(&x.to_be_bytes());
            assert_eq!(&U256::from_be_bytes(b).to_be_bytes(), U::from(x).as_slice());
        }

        #[test]
        fn test_u_u32(x in any::<u32>()) {
            let mut b = [0u8; 32];
            b[32-std::mem::size_of::<u32>()..].copy_from_slice(&x.to_be_bytes());
            assert_eq!(&U256::from_be_bytes(b).to_be_bytes(), U::from(x).as_slice());
        }

        #[test]
        fn test_u_u64(x in any::<u64>()) {
            let mut b = [0u8; 32];
            b[32-std::mem::size_of::<u64>()..].copy_from_slice(&x.to_be_bytes());
            assert_eq!(&U256::from_be_bytes(b).to_be_bytes(), U::from(x).as_slice());
        }

        #[test]
        fn test_u_u128(x in any::<u128>()) {
            let mut b = [0u8; 32];
            b[32-std::mem::size_of::<u128>()..].copy_from_slice(&x.to_be_bytes());
            assert_eq!(&U256::from_be_bytes(b).to_be_bytes(), U::from(x).as_slice());
        }

        #[test]
        fn test_to_and_from_addrs(x in any::<Address>()) {
            let y: [u8; 20] = U::from(x).into();
            assert_eq!(x, y)
        }

        #[test]
        fn test_u_conv_to_and_from_u8(x in any::<u8>()) {
            assert_eq!(x.wrapping_add(1), U::from(x).wrapping_add(&U::ONE).into());
        }
    }
}
