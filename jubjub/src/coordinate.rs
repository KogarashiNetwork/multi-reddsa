//! Twisted Edwards coordinate operation
use crate::affine::{JubjubAffine, D};
use crate::base::Base;
use crate::extend::JubjubExtended;

/// 9M + 4A + 2B
#[inline(always)]
pub(crate) fn add_affine_point(lhs: JubjubAffine, rhs: JubjubAffine) -> JubjubExtended {
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

    JubjubExtended::new(x, y, t, z)
}
