use crate::font::{Fixed, Offset16, Offset32, Tag};
use crate::impl_named;

pub struct STAT {
    header: STATHeader,
    design_axes: Vec<AxisRecord>,
    axis_value_offsets: Vec<Offset16>,
}

pub struct STATHeader {
    /// 1
    major_version: u16,
    /// 2
    minor_version: u16,
    design_axis_size: u16,
    design_axis_count: u16,
    design_axes_offset: Offset32,
    axis_value_count: u16,
    offset_to_axis_value_offsets: u32,
    elided_fallback_name_id: u16,
}

impl_named!(STAT, "STAT");

pub struct AxisRecord {
    axis_tag: Tag,
    axis_name_id: u16,
    axis_ordering: u16,
}

pub enum AxisValueTable {
    Format1(AxisValueTableFormat1),
    Format2(AxisValueTableFormat2),
    Format3(AxisValueTableFormat3),
    Format4(AxisValueTableFormat4),
}

pub struct AxisValueTableFormat1 {
    /// 1
    format: u16,
    axis_index: u16,
    flags: u16,
    value_name_id: u16,
    value: Fixed,
}

pub struct AxisValueTableFormat2 {
    /// 2
    format: u16,
    axis_index: u16,
    flags: u16,
    value_name_id: u16,
    nominal_value: Fixed,
    range_min_value: Fixed,
    range_max_value: Fixed,
}

pub struct AxisValueTableFormat3 {
    /// 3
    format: u16,
    axis_index: u16,
    flags: u16,
    value_name_id: u16,
    value: Fixed,
    linked_value: Fixed,
}

pub struct AxisValueTableFormat4 {
    /// 4
    format: u16,
    axis_index: u16,
    flags: u16,
    value_name_id: u16,
    axis_values: Vec<AxisValue>,
}

pub struct AxisValue {
    axis_index: u16,
    value: Fixed,
}
