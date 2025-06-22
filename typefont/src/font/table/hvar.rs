use crate::font::Offset32;
use crate::impl_tag;

pub struct HVAR {
    major_version: u16,
    minor_version: u16,
    item_variation_store_offset: Offset32,
    advance_width_mapping_offset: Offset32,
    lsb_mapping_offset: Offset32,
    rsb_mapping_offset: Offset32,
}

impl_tag!(HVAR, "HVAR");
