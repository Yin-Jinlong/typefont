use super::bm::BitmapSize;
use super::glyph::BigGlyphMetrics;
use super::Table;
use crate::font::{Offset16, Offset32};
use crate::impl_table;

pub struct Eblc {
    /// 2
    major_version: u16,
    /// 0
    minor_version: u16,
    num_sizes: u32,
    bitmap_sizes: Vec<BitmapSize>,
}

impl_table!(Eblc, "EBLC");

pub struct SbitLineMetrics {
    ascender: i8,
    descender: i8,
    width_max: u8,
    caret_slope_numerator: i8,
    caret_slope_denominator: i8,
    caret_offset: i8,
    min_origin_sb: i8,
    min_advance_sb: i8,
    max_before_bl: i8,
    min_after_bl: i8,
    pad1: i8,
    pad2: i8,
}

pub struct IndexSubtableList {
    index_subtable_records: Vec<IndexSubtableRecord>,
}

pub struct IndexSubtableRecord {
    first_glyph_index: u16,
    last_glyph_index: u16,
    index_subtable_offset: Offset32,
}

pub struct IndexSubHeader {
    index_format: u16,
    image_format: u16,
    image_data_offset: Offset32,
}

pub enum IndexSubtable {
    Format1(IndexSubtableFormat1),
    Format2(IndexSubtableFormat2),
}

pub struct IndexSubtableFormat1 {
    header: IndexSubHeader,
    sbit_offsets: Vec<Offset32>,
}

pub struct IndexSubtableFormat2 {
    header: IndexSubHeader,
    image_size: u32,
    big_metrics: BigGlyphMetrics,
}

pub struct IndexSubtableFormat3 {
    header: IndexSubHeader,
    sbit_offsets: Vec<Offset16>,
}

pub struct IndexSubtableFormat4 {
    header: IndexSubHeader,
    num_glyphs: u32,
    glyph_array: Vec<GlyphIdOffsetPair>,
}

pub struct GlyphIdOffsetPair {
    glyph_id: u16,
    sbit_offset: u16,
}

pub struct IndexSubtableFormat5 {
    header: IndexSubHeader,
    image_size: u32,
    num_glyphs: u32,
    glyph_id_array: Vec<u16>,
}
