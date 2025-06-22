use super::eblc::SbitLineMetrics;
use crate::font::Offset32;

pub struct BitmapSize {
    index_subtable_list_offset: Offset32,
    index_subtable_list_size: u32,
    number_of_index_subtables: u32,
    color_ref: u32,
    hori: SbitLineMetrics,
    vert: SbitLineMetrics,
    start_glyph_index: u16,
    end_glyph_index: u16,
    ppem_x: u8,
    ppem_y: u8,
    bit_depth: u8,
    flags: i8,
}

pub struct BitmapScale {
    hori: SbitLineMetrics,
    vert: SbitLineMetrics,
    ppem_x: u8,
    ppem_y: u8,
    substitute_ppem_x: u8,
    substitute_ppem_y: u8,
}
