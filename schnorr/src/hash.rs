use blake2b_simd::{Params, State};
use jubjub::scalar::Scalar;

const SCHNORR_PERSONAL: &[u8; 16] = b"Schnorr_Sig_Hash";

pub(crate) struct SchnorrHash(State);

impl Default for SchnorrHash {
    fn default() -> Self {
        let state = Params::new()
            .hash_length(64)
            .personal(SCHNORR_PERSONAL)
            .to_state();

        Self(state)
    }
}

impl SchnorrHash {
    fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(bytes);
        self
    }

    fn finalize(&self) -> Scalar {
        let digest = self.0.finalize();
        Scalar::from_bytes_wide(digest.as_array())
    }

    pub(crate) fn execute(a: &[u8], b: &[u8]) -> Scalar {
        SchnorrHash::default().update(a).update(b).finalize()
    }
}
