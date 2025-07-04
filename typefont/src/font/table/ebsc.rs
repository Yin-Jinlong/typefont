use super::bm::BitmapScale;
use crate::impl_named;

pub struct EBSC {
    major_version: u16,
    minor_version: u16,
    num_sizes: u32,
    strikes: Vec<BitmapScale>,
}

impl_named!(EBSC, "EBSC");
