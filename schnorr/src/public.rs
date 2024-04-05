use crate::hash::SchnorrHash;
use crate::signature::Signature;

use jubjub::affine::Affine;
use jubjub::scalar::Scalar;

pub(crate) struct PublicKey(pub(crate) Affine);

impl PublicKey {
    pub(crate) fn verify(self, m: &[u8], sig: Signature) -> bool {
        let s = Scalar::from_bytes(sig.s).unwrap();
        let e = Scalar::from_bytes(sig.e).unwrap();
        let r_v = Affine::basepoint() * s + self.0 * e;
        let e_v = SchnorrHash::execute(&r_v.to_affine().to_bytes(), m);

        e_v == e
    }
}
