use crate::signature::Signature;

use jubjub::affine::Affine;
use jubjub::scalar::Scalar;
use schnorr::hash::SchnorrHash;
use schnorr::private::PrivateKey;
use schnorr::public::PublicKey;

pub(crate) struct PublicParams {
    // R
    randomness: Affine,
    // X
    public_key: PublicKey,
    // c
    challenge: Scalar,
}

impl PublicParams {
    fn new(m: &[u8], a: PublicKey, b: PublicKey, a_r: Affine, b_r: Affine) -> Self {
        let randomness = (a_r + b_r).to_affine();
        let a_1 = SchnorrHash::aggregate(&a.to_bytes(), &b.to_bytes(), &a.to_bytes());
        let a_2 = SchnorrHash::aggregate(&a.to_bytes(), &b.to_bytes(), &b.to_bytes());
        let aggregated_point = a * a_1 + b * a_2;
        let public_key = PublicKey::new(aggregated_point.to_affine());
        let challenge = SchnorrHash::aggregate(&randomness.to_bytes(), &public_key.to_bytes(), m);

        Self {
            randomness,
            public_key,
            challenge,
        }
    }

    fn cosign(&self, a: Scalar, r: Scalar, private_key: PrivateKey) -> Scalar {
        r + private_key * self.challenge * a
    }

    fn generate_signature(&self, s_1: Scalar, s_2: Scalar) -> Signature {
        let s = s_1 + s_2;

        Signature::new(self.randomness, s)
    }

    fn verify(&self, m: &[u8], sig: Signature) -> bool {
        let r = sig.get_r();
        let s = sig.get_s();
        let rc = r + self.public_key * self.challenge;
        let gs = Affine::basepoint() * s;

        rc.to_affine() == gs.to_affine()
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
        fn test_signature_aggregation(alice in arb_field(), bob in arb_field(), r1 in arb_field(), r2 in arb_field()) {
            let message = b"test";
            let alice_private_key = PrivateKey::new(alice);
            let alice_public_key = alice_private_key.to_public_key();
            let alice_public_r = Affine::basepoint() * r1;
            let bob_private_key = PrivateKey::new(bob);
            let bob_public_key = bob_private_key.to_public_key();
            let bob_public_r = Affine::basepoint() * r2;

            let a1 = SchnorrHash::aggregate(&alice_public_key.to_bytes(), &bob_public_key.to_bytes(), &alice_public_key.to_bytes());
            let a2 = SchnorrHash::aggregate(&alice_public_key.to_bytes(), &bob_public_key.to_bytes(), &bob_public_key.to_bytes());

            let public_params = PublicParams::new(message, alice_public_key, bob_public_key, alice_public_r.to_affine(), bob_public_r.to_affine());

            let s_1 = public_params.cosign(a1, r1, alice_private_key);
            let s_2 = public_params.cosign(a2, r2, bob_private_key);
            let signature = public_params.generate_signature(s_1, s_2);

            assert!(public_params.verify(message, signature))
        }
    }
}
