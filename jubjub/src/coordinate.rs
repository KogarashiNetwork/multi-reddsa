//! Twisted Edwards coordinate operation
use crate::affine::{Affine, D};
use crate::base::Base;
use crate::extend::Extended;
use crate::limbs::Naf;
use crate::scalar::Scalar;

/// 9M + 4A + 2B
#[inline(always)]
pub(crate) fn add_affine_point(lhs: Affine, rhs: Affine) -> Extended {
    let (x0, y0) = (lhs.x, lhs.y);
    let (x1, y1) = (rhs.x, rhs.y);

    let a = x0 * x1;
    let b = y0 * y1;
    let c = D * a * b;
    let h = a + b;
    let e = (x0 + y0) * (x1 + y1) - h;
    let f = Base::one() - c;
    let g = Base::one() + c;

    let x = e * f;
    let y = g * h;
    let t = e * h;
    let z = f * g;

    Extended::new(x, y, t, z)
}

/// 5M + 3S + 2D + 2B + 1A
#[inline(always)]
pub fn double_projective_point(extended: Extended) -> Extended {
    let (x, y, z) = (extended.x, extended.y, extended.z);

    let a = -x.square();
    let b = y.square();
    let c = z.square().double();
    let d = a - b;
    let e = (x * y).double();
    let g = a + b;
    let f = g - c;

    let x = e * f;
    let y = g * d;
    let t = e * d;
    let z = f * g;

    Extended::new(x, y, t, z)
}

/// 10M + 4A + 2B
#[inline(always)]
pub fn add_mixed_point(lhs: Affine, rhs: Extended) -> Extended {
    let (x0, y0) = (lhs.x, lhs.y);
    let (x1, y1, z1, t1) = (rhs.x, rhs.y, rhs.z, rhs.t);

    let a = x0 * x1;
    let b = y0 * y1;
    let c = D * x0 * y0 * t1;
    let h = a + b;
    let e = (x0 + y0) * (x1 + y1) - h;
    let f = z1 - c;
    let g = z1 + c;

    let x = e * f;
    let y = g * h;
    let t = e * h;
    let z = f * g;

    Extended::new(x, y, t, z)
}

/// 10M + 4A + 2B
#[inline(always)]
pub fn add_projective_point(lhs: Extended, rhs: Extended) -> Extended {
    let (x0, y0, z0, t0) = (lhs.x, lhs.y, lhs.z, lhs.t);
    let (x1, y1, z1, t1) = (rhs.x, rhs.y, rhs.z, rhs.t);

    let a = x0 * x1;
    let b = y0 * y1;
    let c = D * t0 * t1;
    let d = z0 * z1;
    let h = a + b;
    let e = (x0 + y0) * (x1 + y1) - h;
    let f = d - c;
    let g = d + c;

    let x = e * f;
    let y = g * h;
    let t = e * h;
    let z = f * g;

    Extended::new(x, y, t, z)
}

/// coordinate scalar
#[inline(always)]
pub fn scalar_point(point: Extended, scalar: Scalar) -> Extended {
    let mut res = Extended::identity();
    for &naf in scalar.to_nafs().iter() {
        res = double_projective_point(res);
        if naf == Naf::Plus {
            res += point;
        } else if naf == Naf::Minus {
            res -= point;
        }
    }
    res
}
