use crate::impl_named;
use crate::types::F2D14;

pub struct Avar {
    major_version: u16,
    minor_version: u16,
    reserved: u16,
    axis_count: u16,
    axis_segment_maps: Vec<SegmentMaps>,
}

pub struct SegmentMaps {
    position_map_count: u16,
    axis_value_maps: Vec<AxisValueMap>,
}

pub struct AxisValueMap {
    from_coordinate: F2D14,
    to_coordinate: F2D14,
}

impl_named!(Avar, "avar");
