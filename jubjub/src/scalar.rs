//! Jubjub scalar field
use core::ops::{Add, Mul, Neg, Sub};
use rand_core::RngCore;

use crate::limbs::{add, from_u512, mont, mul, neg, sub, to_nafs, Nafs};

const MODULUS: [u64; 4] = [
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
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

const INV: u64 = 0xfffffffeffffffff;

// Jubjub scalar field
#[derive(Clone, Copy, Debug)]
pub struct Scalar(pub [u64; 4]);

impl Scalar {
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
