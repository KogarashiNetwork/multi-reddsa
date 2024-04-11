//! Jubjub base field
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Add, Mul, Neg, Sub};

use crate::limbs::{add, double, from_u512, invert, little_fermat, mont, mul, neg, square, sub};
use crate::math::sbb;

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

/// R^3 = 2^768 mod r
const R3: [u64; 4] = [
    0xc62c1807439b73af,
    0x1b3e0d188cf06990,
    0x73d13c71c7b5f418,
    0x6e2a5bb9c8db33e9,
];

const INV: u64 = 0xfffffffeffffffff;

const S: usize = 32;

const ROOT_OF_UNITY: Base = Base([
    0xb9b58d8c5f0e466a,
    0x5b1b4c801819d7ec,
    0x0af53ae352a31e64,
    0x5bf3adda19e9b27b,
]);

// Bls scalar and Jubjub base field
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Base(pub [u64; 4]);

impl Base {
    // map raw limbs to montgomery form
    pub(crate) const fn to_mont(raw: [u64; 4]) -> Self {
        Self(mul(raw, R2, MODULUS, INV))
    }

    // map montomery form limbs to raw
    pub(crate) const fn to_raw(self) -> [u64; 4] {
        mont(
            [self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0],
            MODULUS,
            INV,
        )
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

    pub(crate) fn invert(self) -> Option<Self> {
        match invert(self.0, little_fermat(MODULUS), R, MODULUS, INV) {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }

    pub fn to_bytes(self) -> [u8; 32] {
        let tmp = self.to_raw();
        let mut res = [0; 32];

        res[0..8].copy_from_slice(&tmp[0].to_le_bytes());
        res[8..16].copy_from_slice(&tmp[1].to_le_bytes());
        res[16..24].copy_from_slice(&tmp[2].to_le_bytes());
        res[24..32].copy_from_slice(&tmp[3].to_le_bytes());

        res
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Option<Self> {
        let l0 = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let l1 = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        let l2 = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let l3 = u64::from_le_bytes(bytes[24..32].try_into().unwrap());

        let (_, borrow) = sbb(l0, MODULUS[0], 0);
        let (_, borrow) = sbb(l1, MODULUS[1], borrow);
        let (_, borrow) = sbb(l2, MODULUS[2], borrow);
        let (_, borrow) = sbb(l3, MODULUS[3], borrow);

        if borrow & 1 == 1 {
            Some(Self([l0, l1, l2, l3]) * Self(R2))
        } else {
            None
        }
    }

    pub fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        Self(from_u512(
            [
                u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[0..8]).unwrap()),
                u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap()),
                u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[16..24]).unwrap()),
                u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[24..32]).unwrap()),
                u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[32..40]).unwrap()),
                u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[40..48]).unwrap()),
                u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[48..56]).unwrap()),
                u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[56..64]).unwrap()),
            ],
            R2,
            R3,
            MODULUS,
            INV,
        ))
    }

    pub fn pow_vartime(&self, by: &[u64; 4]) -> Self {
        let mut res = Self::one();
        for e in by.iter().rev() {
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res = res * *self;
                }
            }
        }
        res
    }

    pub fn is_odd(self) -> bool {
        let raw = self.to_raw();
        (raw[0] % 2) != 0
    }

    pub fn sqrt(&self) -> Option<Self> {
        let w = self.pow_vartime(&[
            0x7fff2dff7fffffff,
            0x04d0ec02a9ded201,
            0x94cebea4199cec04,
            0x39f6d3a9,
        ]);

        let mut v = S;
        let mut x = w * *self;
        let mut b = x * w;
        let mut z = ROOT_OF_UNITY;

        for max_v in (1..=S).rev() {
            let mut k = 1;
            let mut b2k = b.square();
            let mut j_less_than_v = true;

            for j in 2..max_v {
                j_less_than_v &= j != v;
                if b2k == Self::one() {
                    if j_less_than_v {
                        z = z.square()
                    };
                } else {
                    b2k = b2k.square();
                    k = j;
                };
            }

            if b != Self::one() {
                x = x * z
            };
            z = z.square();
            b = b * z;
            v = k;
        }

        if &x.square() == self {
            Some(x)
        } else {
            None
        }
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

impl Debug for Base {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "0x")?;
        for limb in self.to_raw().iter().rev() {
            for byte in limb.to_be_bytes() {
                write!(f, "{:02x}", byte)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{collection::vec, prelude::*};

    prop_compose! {
        fn arb_field()(
            bytes in vec(any::<u8>(), 64)
        ) -> Base {
            Base::from_bytes_wide(&<[u8; 64]>::try_from(bytes).unwrap())
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn test_raw_and_mont(a in arb_field()) {
            let raw = a.to_raw();
            let mont = Base::to_mont(raw);

            assert_eq!(a, mont)
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn test_add_and_double(a in arb_field(), b in arb_field()) {
            let additive = a + a + b + b;
            let doubling = a.double() + b.double();

            assert_eq!(additive, doubling)
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn test_mul_and_square(a in arb_field(), b in arb_field()) {
            let additive = a * a + b * b;
            let doubling = a.square() + b.square();

            assert_eq!(additive, doubling)
        }
    }
}
