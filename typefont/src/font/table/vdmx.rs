use super::Table;
use crate::font::Offset16;
use crate::impl_table;

pub struct VDMX {
    /// 0 or 1
    version: u16,
    num_recs: u16,
    num_ratios: u16,
    rat_range: Vec<RatioRange>,
    vdmx_group_offsets: Vec<Offset16>,
}

impl_table!(VDMX, "VDMX");

pub struct RatioRange {
    b_char_set: u8,
    x_ratio: u8,
    y_start_ratio: u8,
    y_end_ratio: u8,
}

pub struct VDMXGroup {
    recs: u16,
    starts_z: u8,
    ends_z: u8,
    entry: Vtable,
}

pub struct Vtable {
    y_pel_height: u16,
    y_max: u16,
    y_min: u16,
}
