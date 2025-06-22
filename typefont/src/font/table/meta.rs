use crate::font::{Offset32, Tag};
use crate::impl_named;

pub struct Meta {}

pub struct MetaHeader {
    major_version: u32,
    flags: u32,
    reserved: u32,
    data_maps_count: u32,
    data_maps: Vec<DataMap>,
}

impl_named!(Meta, "meta");

pub struct DataMap {
    tag: Tag,
    data_offset: Offset32,
    data_length: u32,
}
