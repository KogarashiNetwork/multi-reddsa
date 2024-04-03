//! Jubjub extend point
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::affine::Affine;
use crate::base::Base;
use crate::coordinate::{add_mixed_point, add_projective_point};

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
