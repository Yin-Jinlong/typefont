use super::Table;
use crate::font::{Offset16, Tag};
use crate::impl_table;

pub struct MVAR {
    major_version: u16,
    minor_version: u16,
    reserved: u16,
    value_record_size: u16,
    value_record_count: u16,
    item_variation_store_offset: Offset16,
    value_records: Vec<ValueRecord>,
}

impl_table!(MVAR, "MVAR");

pub struct ValueRecord {
    value_tag: Tag,
    delta_set_outer_index: u16,
    delta_set_inner_index: u16,
}
