//! Jubjub affine point
use core::ops::{Add, Mul, Neg};

use crate::base::Base;
use crate::coordinate::{
    add_affine_point, add_mixed_point, double_affine_point, double_projective_point,
};
use crate::extend::Extended;
use crate::limbs::Naf;
use crate::scalar::Scalar;

// Jubjub D param
pub(crate) const D: Base = Base::to_mont([
    0x01065fd6d6343eb1,
    0x292d7f6d37579d26,
    0xf5fd9207e6bd7fd4,
    0x2a9318e74bfa2b48,
]);

const X: Base = Base::to_mont([
    0xe4b3d35df1a7adfe,
    0xcaf55d1b29bf81af,
    0x8b0f03ddd60a8187,
    0x62edcbb8bf3787c8,
]);

const Y: Base = Base::to_mont([
    0x000000000000000b,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
]);

const T: Base = Base::to_mont([
    0xd3ba1512623479e1,
    0xc6e03c0fcb495697,
    0x2c9c923fdbc2f8a5,
    0x2cdcdf03c0d96e14,
]);

/// Jubjub affine coordinate
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Affine {
    pub(crate) x: Base,
    pub(crate) y: Base,
}

impl Affine {
    pub const fn generator() -> Self {
        Self { x: X, y: Y }
    }

    pub(crate) fn identity() -> Self {
        Self {
            x: Base::zero(),
            y: Base::one(),
        }
    }

    pub(crate) fn extend(self) -> Extended {
        Extended {
            x: self.x,
            y: self.y,
            t: self.x * self.y,
            z: Base::one(),
        }
    }

    pub fn to_bytes(self) -> [u8; 32] {
        let mut tmp = self.y.to_bytes();
        let x = self.x.to_bytes();
        tmp[31] |= x[0] << 7;

        tmp
    }

    pub fn double(self) -> Extended {
        double_affine_point(self)
    }
}

impl Add for Affine {
    type Output = Extended;

    fn add(self, rhs: Affine) -> Extended {
        add_affine_point(self, rhs)
    }
}

impl Add<Extended> for Affine {
    type Output = Extended;

    fn add(self, rhs: Extended) -> Extended {
        add_mixed_point(self, rhs)
    }
}

impl Mul<Scalar> for Affine {
    type Output = Extended;

    fn mul(self, scalar: Scalar) -> Extended {
        let mut res = Extended::identity();
        for naf in scalar.to_nafs().iter() {
            res = double_projective_point(res);
            if naf == &Naf::Plus {
                res += self;
            } else if naf == &Naf::Minus {
                res -= self;
            }
        }
        res
    }
}

impl Neg for Affine {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
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
        fn test_add_and_double(a in arb_field(), b in arb_field()) {
            let a_point = Affine::generator() * a;
            let b_point = Affine::generator() * b;

            let additive = a_point + a_point + b_point + b_point;
            let doubling = a_point.double() + b_point.double();

            assert_eq!(additive.to_affine(), doubling.to_affine())
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn test_scalar(r in arb_field()) {
            let nine = Scalar::one().double().double().double() + Scalar::one();
            let r_point = Affine::generator() * r;

            let scalar = r_point * nine;
            let naive = r_point.double().double().double() + r_point;

            assert_eq!(scalar.to_affine(), naive.to_affine())
        }
    }
}
