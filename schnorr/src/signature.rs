use jubjub::scalar::Scalar;

pub(crate) struct Signature {
    s: [u8; 32],
    e: [u8; 32],
}

impl Signature {
    pub(crate) fn new(s: Scalar, e: Scalar) -> Self {
        Self {
            s: s.to_bytes(),
            e: e.to_bytes(),
        }
    }
}
