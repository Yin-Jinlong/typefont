use crate::font::table::Table;
use crate::font::{Fixed, Offset16, Tag};
use crate::impl_table;

pub struct Fvar {
    header: FvarHeader,
    axes: Vec<VariationAxisRecord>,
    instances: Vec<InstanceRecord>,
}

pub struct FvarHeader {
    /// 1
    major_version: u16,
    /// 0
    minor_version: u16,
    axes_array_offset: Offset16,
    /// 2
    reserved: u16,
    axis_count: u16,
    axis_size: u16,
    instance_count: u16,
    instance_size: u16,
}

impl_table!(Fvar, "fvar");

pub struct VariationAxisRecord {
    axis_tag: Tag,
    min_value: Fixed,
    default_value: Fixed,
    max_value: Fixed,
    flags: u16,
    axis_name_id: u16,
}

pub struct InstanceRecord {
    coordinates: Vec<Fixed>,
}
