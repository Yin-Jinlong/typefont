use super::bm::BitmapScale;
use super::Table;
use crate::impl_table;

pub struct EBSC {
    major_version: u16,
    minor_version: u16,
    num_sizes: u32,
    strikes: Vec<BitmapScale>,
}

impl_table!(EBSC, "EBSC");
