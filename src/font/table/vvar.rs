use super::Table;
use crate::font::Offset32;
use crate::impl_table;

pub struct VVAR {
    /// 1
    major_version: u16,
    /// 0
    minor_version: u16,
    item_variation_store_offset: Offset32,
    advance_height_mapping_offset: Offset32,
    tsb_mapping_offset: Offset32,
    bsb_mapping_offset: Offset32,
    v_org_mapping_offset: Offset32,
}

impl_table!(VVAR, "VVAR");
