use super::eblc::BitmapSize;
use crate::font::table::Table;
use crate::impl_table;

pub struct CBLC {
    header: CblcHeader,
}

impl_table!(CBLC, "CBLC");

pub struct CblcHeader {
    major_version: u16,
    minor_version: u16,
    num_sizes: u32,
    bitmap_sizes: Vec<BitmapSize>,
}
