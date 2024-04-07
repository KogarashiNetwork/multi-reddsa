use jubjub::scalar::Scalar;

pub struct Signature {
    pub(crate) s: [u8; 32],
    pub(crate) e: [u8; 32],
}

impl Signature {
    pub fn new(s: Scalar, e: Scalar) -> Self {
        Self {
            s: s.to_bytes(),
            e: e.to_bytes(),
        }
    }

    pub fn get_s(&self) -> Scalar {
        Scalar::from_bytes(self.s).unwrap()
    }

    pub fn get_e(&self) -> Scalar {
        Scalar::from_bytes(self.e).unwrap()
    }
}
