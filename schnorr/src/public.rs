use crate::hash::SchnorrHash;
use crate::signature::Signature;

use core::ops::{Add, Mul};
use jubjub::affine::Affine;
use jubjub::extend::Extended;
use jubjub::scalar::Scalar;

#[derive(Clone, Copy, Debug)]
pub struct PublicKey(pub(crate) Affine);

impl PublicKey {
    pub fn new(value: Affine) -> Self {
        Self(value)
    }

    pub(crate) fn verify(self, m: &[u8], sig: Signature) -> bool {
        let s = Scalar::from_bytes(sig.s).unwrap();
        let e = Scalar::from_bytes(sig.e).unwrap();
        let r_v = Affine::basepoint() * s + self.0 * e;
        let e_v = SchnorrHash::execute(&r_v.to_affine().to_bytes(), m);

        e_v == e
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
}

impl Add for PublicKey {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.0 + rhs.0;
        Self(sum.to_affine())
    }
}

impl Mul<Scalar> for PublicKey {
    type Output = Extended;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.0 * rhs
    }
}
