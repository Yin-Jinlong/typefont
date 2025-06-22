use super::super::{FWord, UFWord};
use crate::impl_tag;

pub struct Hmtx {
    /// h_metrics\[number_of_h_metrics]
    h_metrics: Vec<LongHorMetric>,
    /// left_side_bearings\[num_glyphs - number_of_h_metrics]
    left_side_bearings: FWord,
}

impl_tag!(Hmtx, "hmtx");

pub struct LongHorMetric {
    advance_width: UFWord,
    lsb: FWord,
}
