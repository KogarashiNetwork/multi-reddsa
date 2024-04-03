#[inline(always)]
pub(crate) const fn sbb(a: u64, b: u64, brw: u64) -> (u64, u64) {
    let t = (a as u128).wrapping_sub((b as u128) + (brw >> 63) as u128);
    (t as u64, (t >> 64) as u64)
}
