use jubjub::affine::Affine;
use jubjub::scalar::Scalar;
use schnorr::hash::SchnorrHash;
use schnorr::private::PrivateKey;
use schnorr::public::PublicKey;
use schnorr::signature::Signature;

pub(crate) struct PublicParams {
    randomness: Affine,
    public_key: PublicKey,
    challenge: Scalar,
}

impl PublicParams {
    fn new(m: &[u8], a: PublicKey, b: PublicKey, a_r: Affine, b_r: Affine) -> Self {
        let randomness = (a_r + b_r).to_affine();
        let public_key = a + b;
        let challenge = SchnorrHash::aggregate(&randomness.to_bytes(), &public_key.to_bytes(), m);

        Self {
            randomness,
            public_key,
            challenge,
        }
    }

    fn cosign(&self, randomness: Scalar, private_key: PrivateKey) -> Scalar {
        randomness + private_key * self.challenge
    }

    fn generate_signature(&self, e_1: Scalar, e_2: Scalar) -> Signature {
        let e = e_1 + e_2;

        Signature::new(self.challenge, e)
    }

    fn verify(&self, m: &[u8], sig: Signature) -> bool {
        let s = sig.get_s();
        let e = sig.get_e();
        let r_v = Affine::basepoint() * s + self.public_key * e;
        let e_v = SchnorrHash::execute(&r_v.to_affine().to_bytes(), m);

        e_v == e
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
        fn test_naive_signature_aggregation(alice in arb_field(), bob in arb_field(), r1 in arb_field(), r2 in arb_field()) {
            let message = b"test";
            let alice_private_key = PrivateKey::new(alice);
            let alice_public_key = alice_private_key.to_public_key();
            let alice_public_r = Affine::basepoint() * r1;
            let bob_private_key = PrivateKey::new(bob);
            let bob_public_key = bob_private_key.to_public_key();
            let bob_public_r = Affine::basepoint() * r2;
            let public_params = PublicParams::new(message, alice_public_key, bob_public_key, alice_public_r.to_affine(), bob_public_r.to_affine());

            let s_1 = public_params.cosign(r1, alice_private_key);
            let s_2 = public_params.cosign(r2, bob_private_key);
            let signature = public_params.generate_signature(s_1, s_2);

            assert!(public_params.verify(message, signature))
        }
    }
}
