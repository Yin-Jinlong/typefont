use crate::font::table::Table;
use crate::font::Offset32;
use crate::impl_table;

pub enum CPAL {
    V0(CPAL0),
    V1(CPAL1),
}

impl_table!(CPAL, "CPAL");

pub struct CPAL0 {
    version: u16,
    num_palette_entries: u16,
    num_palettes: u16,
    num_color_records: u16,
    color_records_array_offset: Offset32,
    color_record_indices: Vec<u16>,
}

pub struct CPAL1 {
    base: CPAL0,
    palette_types_array_offset: Offset32,
    palette_labels_array_offset: Offset32,
    palette_entry_labels_array_offset: Offset32,
}

pub struct ColorRecord {
    blue: u8,
    green: u8,
    red: u8,
    alpha: u8,
}

type PaletteTypeArray = Vec<u32>;
type PaletteLabel = Vec<u16>;
type PaletteEntryLabel = Vec<u16>;
