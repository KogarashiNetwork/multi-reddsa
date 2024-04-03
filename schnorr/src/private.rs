use jubjub::affine::Affine;
use jubjub::scalar::Scalar;
use rand_core::RngCore;

pub struct PrivateKey(Scalar);

impl PrivateKey {
    pub fn new(value: Scalar) -> Self {
        Self(value)
    }

    pub fn sign(&self, m: &[u8], mut rand: impl RngCore) {
        // chose random
        let k = Scalar::random(rand);
        let r = Affine::generator() * k;
    }
}
