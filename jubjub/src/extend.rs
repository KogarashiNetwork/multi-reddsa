//! Jubjub extend point
use core::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::affine::Affine;
use crate::base::Base;
use crate::coordinate::{add_mixed_point, add_projective_point, double_projective_point};
use crate::limbs::Naf;
use crate::scalar::Scalar;

/// Jubjub extended coordinate
#[derive(Clone, Copy, Debug)]
pub struct Extended {
    pub(crate) x: Base,
    pub(crate) y: Base,
    pub(crate) t: Base,
    pub(crate) z: Base,
}

impl Extended {
    pub(crate) fn new(x: Base, y: Base, t: Base, z: Base) -> Self {
        Self { x, y, t, z }
    }

    pub(crate) fn identity() -> Self {
        Self {
            x: Base::zero(),
            y: Base::one(),
            t: Base::zero(),
            z: Base::one(),
        }
    }

    pub fn to_affine(self) -> Affine {
        let z_inv = self.z.invert().unwrap();
        Affine {
            x: self.x * z_inv,
            y: self.y * z_inv,
        }
    }

    pub(crate) fn double(self) -> Self {
        double_projective_point(self)
    }
}

impl Add<Extended> for Extended {
    type Output = Extended;

    fn add(self, rhs: Extended) -> Extended {
        add_projective_point(self, rhs)
    }
}

impl AddAssign<Extended> for Extended {
    fn add_assign(&mut self, rhs: Extended) {
        *self = *self + rhs
    }
}

impl Neg for Extended {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
            t: -self.t,
            z: self.z,
        }
    }
}

impl Sub<Extended> for Extended {
    type Output = Extended;

    fn sub(self, rhs: Extended) -> Extended {
        add_projective_point(self, rhs.neg())
    }
}

impl SubAssign<Extended> for Extended {
    fn sub_assign(&mut self, rhs: Extended) {
        *self = *self - rhs;
    }
}

impl Add<Affine> for Extended {
    type Output = Extended;

    fn add(self, rhs: Affine) -> Extended {
        add_mixed_point(rhs, self)
    }
}

impl AddAssign<Affine> for Extended {
    fn add_assign(&mut self, rhs: Affine) {
        *self = *self + rhs
    }
}

impl Sub<Affine> for Extended {
    type Output = Extended;

    fn sub(self, rhs: Affine) -> Extended {
        add_mixed_point(rhs.neg(), self)
    }
}

impl SubAssign<Affine> for Extended {
    fn sub_assign(&mut self, rhs: Affine) {
        *self = *self - rhs;
    }
}

impl Mul<Scalar> for Extended {
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

    prop_compose! {
        fn arb_point()(
            bytes in vec(any::<u8>(), 64)
        ) -> Extended {
            let r = Scalar::from_bytes_wide(&<[u8; 64]>::try_from(bytes).unwrap());
            Affine::generator() * r
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn test_add_and_double(a in arb_point(), b in arb_point()) {
            let additive = a + a + b + b;
            let doubling = a.double() + b.double();

            assert_eq!(additive.to_affine(), doubling.to_affine())
        }
    }
}
