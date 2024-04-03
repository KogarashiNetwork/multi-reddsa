//! Jubjub extend point
use crate::base::Base;

/// Jubjub extended coordinate
#[derive(Clone, Debug)]
pub(crate) struct Extended {
    pub(crate) x: Base,
    pub(crate) y: Base,
    pub(crate) t: Base,
    pub(crate) z: Base,
}

impl Extended {
    pub(crate) fn new(x: Base, y: Base, t: Base, z: Base) -> Self {
        Self { x, y, t, z }
    }
}
