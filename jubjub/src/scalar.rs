//! Jubjub scalar field
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Add, Mul, Sub};
use rand_core::RngCore;

use crate::limbs::{add, double, from_u512, mont, mul, square, sub, to_nafs, Nafs};
use crate::math::sbb;

const MODULUS: [u64; 4] = [
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
];

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x25f80bb3b99607d9,
    0xf315d62f66b6e750,
    0x932514eeeb8814f4,
    0x09a6fc6f479155c6,
];

/// R^2 = 2^512 mod r
const R2: [u64; 4] = [
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x04f6547b8d127688,
];

/// R^3 = 2^768 mod r
const R3: [u64; 4] = [
    0xe0d6c6563d830544,
    0x323e3883598d0f85,
    0xf0fea3004c2e2ba8,
    0x05874f84946737ec,
];

const INV: u64 = 0x1ba3a358ef788ef9;

// Jubjub scalar field
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scalar(pub [u64; 4]);

impl Scalar {
    pub(crate) fn one() -> Self {
        Self(R)
    }

    pub(crate) fn double(self) -> Self {
        Self(double(self.0, MODULUS))
    }

    pub(crate) fn square(self) -> Self {
        Self(square(self.0, MODULUS, INV))
    }

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

    pub fn random(mut rand: impl RngCore) -> Self {
        Self(from_u512(
            [
                rand.next_u64(),
                rand.next_u64(),
                rand.next_u64(),
                rand.next_u64(),
                rand.next_u64(),
                rand.next_u64(),
                rand.next_u64(),
                rand.next_u64(),
            ],
            R2,
            R3,
            MODULUS,
            INV,
        ))
    }

    pub(crate) fn to_nafs(self) -> Nafs {
        to_nafs(self.to_raw())
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
}

impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(add(self.0, rhs.0, MODULUS))
    }
}

impl Sub for Scalar {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(sub(self.0, rhs.0, MODULUS))
    }
}

impl Mul<Self> for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(mul(self.0, rhs.0, MODULUS, INV))
    }
}

impl Debug for Scalar {
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
        ) -> Scalar {
            Scalar::from_bytes_wide(&<[u8; 64]>::try_from(bytes).unwrap())
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn test_raw_and_mont(a in arb_field()) {
            let raw = a.to_raw();
            let mont = Scalar::to_mont(raw);

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
