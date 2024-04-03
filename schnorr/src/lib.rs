#![no_std]
pub(crate) mod hash;
pub(crate) mod private;
pub(crate) mod public;
pub(crate) mod signature;

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;
    use jubjub::scalar::Scalar;
    use private::PrivateKey;

    #[test]
    fn schnorr_signature_test() {
        let message = b"test";
        let value = Scalar::random(OsRng);
        let private_key = PrivateKey::new(value);
        let public_key = private_key.to_public_key();
        let signature = private_key.sign(message, OsRng);

        assert!(public_key.verify(message, signature))
    }
}
