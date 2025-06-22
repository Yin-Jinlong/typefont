use crate::font::Offset16;
use crate::impl_tag;

pub struct MERG {
    header: MERGHeader,
}

pub struct MERGHeader {
    /// 0
    version: u16,
    merge_class_count: u16,
    merge_data_offset: Offset16,
    class_def_count: u16,
    offset_to_class_def_offsets: Offset16,
}

impl_tag!(MERG, "MERG");

pub struct MergeEntry {
    merge_entry_rows: Vec<MergeEntryRow>,
}

pub struct MergeEntryRow {
    merge_entries: Vec<u8>,
}
