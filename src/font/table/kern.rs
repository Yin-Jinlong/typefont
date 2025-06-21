use super::Table;
use crate::font::{FWord, Offset16};
use crate::impl_table;

pub struct Kern {
    header: KernHeader,
    sub_header: KernSubtableHeader,
    subtable: KernSubtable,
}

pub struct KernHeader {
    /// 0
    version: u16,
    n_tables: u16,
}

pub struct KernSubtableHeader {
    /// 0
    version: u16,
    length: u16,
    coverage: u16,
}

impl_table!(Kern, "kern");

pub enum KernSubtable {
    Format0(KernSubtableFormat0),
}

pub struct KernSubtableFormat0 {
    n_pairs: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    kern_pairs: u16,
}

pub struct KernPair {
    left: u16,
    right: u16,
    value: FWord,
}

pub struct KernSubtableFormat2 {
    row_width: u16,
    left_class_offset: Offset16,
    right_class_offset: Offset16,
    kerning_array_offset: Offset16,
}

pub struct ClassHeader{
    first_glyph: u16,
    n_glyphs: u16,
}
