use super::Table;
use crate::font::{Offset32, Tag};
use crate::impl_table;

pub struct Sbix {
    header: SbixHeader,
}

pub struct SbixHeader {
    /// 1
    version: u16,
    flags: u16,
    num_strikes: u32,
    strike_offsets: Vec<Offset32>,
}

impl_table!(Sbix, "sbix");

pub struct StrikeHeader {
    ppem: u16,
    ppi: u16,
    glyph_data_offsets: Offset32,
}

pub struct GlyphData {
    origin_offset_x: i16,
    origin_offset_y: i16,
    graphic_type: Tag,
    data: Vec<u8>,
}
