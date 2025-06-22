use super::Table;
use crate::font::{Offset16, Offset32};
use crate::impl_table;

pub struct GDEF {
    header: GDEFHeader,
}

pub enum GDEFHeader {
    V1_0(GDEFHeader1_0),
    V1_2(GDEFHeader1_2),
    V1_3(GDEFHeader1_3),
}

impl_table!(GDEF, "GDEF");

pub struct GDEFHeader1_0 {
    /// 1
    major_version: u16,
    /// 0
    minor_version: u16,
    glyph_class_def_offset: Offset16,
    attach_list_offset: Offset16,
    lig_caret_list_offset: Offset16,
    mark_attach_class_def_offset: Offset16,
}

pub struct GDEFHeader1_2 {
    base: GDEFHeader1_0,
    mark_glyph_sets_def_offset: Offset16,
}

pub struct GDEFHeader1_3 {
    base: GDEFHeader1_2,
    item_var_store_offset: Offset32,
}

pub enum GlyphClassDef {
    BaseGlyph = 1,
    LigatureGlyph = 2,
    MarkGlyph = 3,
    ComponentGlyph = 4,
}

pub struct AttachList {
    coverage_offset: Offset16,
    glyph_count: u16,
    attach_point_offsets: Vec<Offset16>,
}

pub struct AttachPoint {
    point_count: u16,
    point_indices: Vec<u16>,
}

pub struct LigCaretList {
    coverage_offset: Offset16,
    lig_glyph_count: u16,
    lig_glyph_offsets: Vec<Offset16>,
}

pub struct LigGlyph {
    caret_count: u16,
    caret_value_offsets: Vec<Offset16>,
}

pub enum CaretValue {
    Format1(CaretValueFormat1),
    Format2(CaretValueFormat2),
    Format3(CaretValueFormat3),
}

pub struct CaretValueFormat1 {
    /// 1
    format: u16,
    coordinate: i16,
}

pub struct CaretValueFormat2 {
    /// 2
    format: u16,
    caret_value_point_index: u16,
}

pub struct CaretValueFormat3 {
    /// 3
    format: u16,
    coordinate: i16,
    device_offset: Offset16,
}

pub struct MarkGlyphSets {
    /// 1
    format: u16,
    mark_glyph_set_count: u16,
    coverage_offsets: Vec<Offset32>,
}
