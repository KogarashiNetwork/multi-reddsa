#![no_std]
pub(crate) mod hash;
pub(crate) mod private;
pub(crate) mod public;
pub(crate) mod signature;

#[cfg(test)]
mod tests {
    use super::*;
    use jubjub::scalar::Scalar;
    use private::PrivateKey;
    use proptest::{collection::vec, prelude::*};
    use rand_core::OsRng;

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
        fn test_schnorr_signature(value in arb_field()) {
            let message = b"test";
            let private_key = PrivateKey::new(value);
            let public_key = private_key.to_public_key();
            let signature = private_key.sign(message, OsRng);

            assert!(public_key.verify(message, signature))
        }
    }
}
