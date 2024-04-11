use jubjub::affine::Affine;
use jubjub::scalar::Scalar;

pub struct Signature {
    pub(crate) r: [u8; 32],
    pub(crate) s: [u8; 32],
}

impl Signature {
    pub fn new(r: Affine, s: Scalar) -> Self {
        Self {
            r: r.to_bytes(),
            s: s.to_bytes(),
        }
    }

    pub fn get_r(&self) -> Affine {
        Affine::from_bytes(self.r).unwrap()
    }

    pub fn get_s(&self) -> Scalar {
        Scalar::from_bytes(self.s).unwrap()
    }
}
