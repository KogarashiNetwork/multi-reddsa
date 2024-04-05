use crate::{affine::Affine, base::Base, extend::Extended};

const BASE_X: Base = Base::to_mont([
    0x47bf46920a95a753,
    0xd5b9a7d3ef8e2827,
    0xd418a7ff26753b6a,
    0x0926d4f32059c712,
]);

const BASE_Y: Base = Base::to_mont([
    0x305632adaaf2b530,
    0x6d65674dcedbddbc,
    0x53bb37d0c21cfd05,
    0x57a1019e6de9b675,
]);

const BASE_T: Base = Base::to_mont([
    0xd2280d93b46f4ab9,
    0x1d9ce6cfa60e6dc7,
    0x7225482d17a11235,
    0x05539d52ecc71057,
]);

impl Affine {
    pub fn basepoint() -> Self {
        Self {
            x: BASE_X,
            y: BASE_Y,
        }
    }
}

impl Extended {
    pub fn basepoint() -> Self {
        Self {
            x: BASE_X,
            y: BASE_Y,
            t: BASE_T,
            z: Base::one(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basepoint() {
        assert_eq!(BASE_X * BASE_Y, BASE_T)
    }
}
