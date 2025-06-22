use crate::impl_tag;

pub struct EBDT {
    /// 2
    major_version: u16,
    /// 0
    minor_version: u16,
}

impl_tag!(EBDT, "EBDT");

pub struct EbdtComponent {
    glyph_id: u16,
    x_offset: u8,
    y_offset: u8,
}
