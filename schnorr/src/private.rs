use crate::hash::SchnorrHash;
use crate::public::PublicKey;
use crate::signature::Signature;

use jubjub::affine::Affine;
use jubjub::scalar::Scalar;
use rand_core::RngCore;

#[derive(Clone, Copy, Debug)]
pub(crate) struct PrivateKey(pub(crate) Scalar);

impl PrivateKey {
    pub(crate) fn new(value: Scalar) -> Self {
        Self(value)
    }

    pub(crate) fn to_public_key(self) -> PublicKey {
        let value = Affine::basepoint() * self.0;
        PublicKey(value.to_affine())
    }

    pub(crate) fn sign(&self, m: &[u8], rand: impl RngCore) -> Signature {
        // chose random
        let k = Scalar::random(rand);
        let r = Affine::basepoint() * k;
        let e = SchnorrHash::execute(&r.to_affine().to_bytes(), m);
        let s = k - self.0 * e;

        Signature::new(s, e)
    }
}
