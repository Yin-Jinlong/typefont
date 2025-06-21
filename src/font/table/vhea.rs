use super::Table;
use crate::font::{FWord, UFWord, Version16Dot16};
use crate::impl_table;

pub struct Vhea {
    header: VheaHeader,
}

pub enum VheaHeader {
    V1_0(VheaHeader1_0),
    V1_1(VheaHeader1_1),
}

pub struct VheaHeader1_0 {
    version: Version16Dot16,
    ascent: FWord,
    descent: FWord,
    line_gap: FWord,
    advance_height_max: UFWord,
    min_top_side_bearing: FWord,
    min_bottom_side_bearing: FWord,
    y_max_extent: FWord,
    caret_slope_rise: i16,
    caret_slope_run: i16,
    caret_offset: i16,
    /// 0
    reserved1: i16,
    /// 0
    reserved2: i16,
    /// 0
    reserved3: i16,
    /// 0
    reserved4: i16,
    metric_data_format: i16,
    num_of_long_ver_metrics: u16,
}

pub struct VheaHeader1_1 {
    version: Version16Dot16,
    vert_typo_ascender: FWord,
    vert_typo_descender: FWord,
    vert_typo_line_gap: FWord,
    advance_height_max: FWord,
    min_top_side_bearing: FWord,
    min_bottom_side_bearing: FWord,
    y_max_extent: FWord,
    caret_slope_rise: i16,
    caret_slope_run: i16,
    caret_offset: i16,
    /// 0
    reserved1: i16,
    /// 0
    reserved2: i16,
    /// 0
    reserved3: i16,
    /// 0
    reserved4: i16,
    metric_data_format: i16,
    num_of_long_ver_metrics: u16,
}

impl_table!(Vhea, "vhea");
