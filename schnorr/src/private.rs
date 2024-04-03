use crate::hash::SchnorrHash;
use crate::signature::Signature;

use jubjub::affine::Affine;
use jubjub::scalar::Scalar;
use rand_core::RngCore;

pub(crate) struct PrivateKey(Scalar);

impl PrivateKey {
    pub(crate) fn new(value: Scalar) -> Self {
        Self(value)
    }

    pub(crate) fn sign(&self, m: &[u8], mut rand: impl RngCore) -> Signature {
        // chose random
        let k = Scalar::random(rand);
        let r = Affine::generator() * k;
        let e = SchnorrHash::execute(&r.to_affine().to_bytes(), m);
        let s = k - self.0 * e;

        Signature::new(s, e)
    }
}
