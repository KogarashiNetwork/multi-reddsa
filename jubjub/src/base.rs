//! Jubjub base field
use core::ops::{Add, Mul, Neg, Sub};

use crate::limbs::{add, double, mul, neg, square, sub};

const MODULUS: [u64; 4] = [
    0xffffffff00000001,
    0x53bda402fffe5bfe,
    0x3339d80809a1d805,
    0x73eda753299d7d48,
];

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x00000001fffffffe,
    0x5884b7fa00034802,
    0x998c4fefecbc4ff5,
    0x1824b159acc5056f,
];

/// R^2 = 2^512 mod r
const R2: [u64; 4] = [
    0xc999e990f3f29c6d,
    0x2b6cedcb87925c23,
    0x05d314967254398f,
    0x0748d9d99f59ff11,
];

const INV: u64 = 0xfffffffeffffffff;

// Bls scalar and Jubjub base field
#[derive(Clone, Copy, Debug)]
pub(crate) struct Base(pub [u64; 4]);

impl Base {
    // map raw limbs to montgomery form
    pub(crate) const fn to_mont(raw: [u64; 4]) -> Self {
        Self(mul(raw, R2, MODULUS, INV))
    }

    pub(crate) fn zero() -> Self {
        Self([0; 4])
    }

    pub(crate) fn one() -> Self {
        Self(R)
    }

    pub(crate) fn double(self) -> Self {
        Self(double(self.0, MODULUS))
    }

    pub(crate) fn square(self) -> Self {
        Self(square(self.0, MODULUS, INV))
    }
}

impl Add for Base {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(add(self.0, rhs.0, MODULUS))
    }
}

impl Neg for Base {
    type Output = Self;

    fn neg(self) -> Self {
        Self(neg(self.0, MODULUS))
    }
}

impl Sub for Base {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(sub(self.0, rhs.0, MODULUS))
    }
}

impl Mul<Self> for Base {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(mul(self.0, rhs.0, MODULUS, INV))
    }
}
