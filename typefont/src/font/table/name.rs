use super::super::Offset16;
use crate::impl_named;

pub enum Name {
    V0(NameV0),
    V1(NameV1),
}

impl_named!(Name, "name");

pub struct NameV0 {
    version: u16,
    count: u16,
    storage_offset: Offset16,
    name_record: Vec<NameRecord>,
    data: String,
}

pub struct NameV1 {
    version: u16,
    count: u16,
    storage_offset: Offset16,
    name_record: Vec<NameRecord>,
    lang_tag_count: u16,
    lang_tag_record: Vec<LangTagRecord>,
    data: String,
}

pub struct LangTagRecord {
    length: u16,
    lang_tag_offset: Offset16,
}

pub struct NameRecord {
    platform_id: u16,
    encoding_id: u16,
    language_id: u16,
    name_id: u16,
    length: u16,
    string_offset: Offset16,
}
