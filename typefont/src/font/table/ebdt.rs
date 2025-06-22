use crate::font::table::Table;
use crate::impl_table;

pub struct EBDT {
    /// 2
    major_version: u16,
    /// 0
    minor_version: u16,
}

impl_table!(EBDT, "EBDT");

pub struct EbdtComponent {
    glyph_id: u16,
    x_offset: u8,
    y_offset: u8,
}
