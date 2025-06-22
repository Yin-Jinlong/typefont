use super::bm::BitmapSize;
use crate::impl_tag;

pub struct CBLC {
    header: CblcHeader,
}

impl_tag!(CBLC, "CBLC");

pub struct CblcHeader {
    major_version: u16,
    minor_version: u16,
    num_sizes: u32,
    bitmap_sizes: Vec<BitmapSize>,
}
