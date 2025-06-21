use crate::font::table::Table;
use crate::font::{Offset16, Tag};
use crate::impl_table;

pub struct JSTF {
    header: JSTFHeader,
}

pub struct JSTFHeader {
    major_version: u16,
    minor_version: u16,
    jstf_script_count: u16,
    jstf_script_records: Vec<JstfScriptRecord>,
}

impl_table!(JSTF, "JSTF");

pub struct JstfScriptRecord {
    jstf_script_tag: Tag,
    jstf_script_offset: Offset16,
}

pub struct JstfScript {
    extender_glyph_offset: Offset16,
    def_jstf_lang_sys_offset: Offset16,
    jstf_lang_sys_count: u16,
    jstf_lang_sys_records: Vec<JstfLangSysRecord>,
}

pub struct JstfLangSysRecord {
    jstf_lang_sys_tag: Tag,
    jstf_lang_sys_offset: Offset16,
}

pub struct ExtenderGlyph {
    glyph_count: u16,
    extender_glyphs: Vec<u16>,
}

pub struct JstfLangSys {
    jstf_priority_count: u16,
    jstf_priority_offsets: Vec<Offset16>,
}

pub struct JstfPriority {
    gsub_shrinkage_enable_offset: Offset16,
    gsub_shrinkage_disable_offset: Offset16,
    gpos_shrinkage_enable_offset: Offset16,
    gpos_shrinkage_disable_offset: Offset16,
    shrinkage_jstf_max_offset: Offset16,
    gsub_extension_enable_offset: Offset16,
    gsub_extension_disable_offset: Offset16,
    gpos_extension_enable_offset: Offset16,
    gpos_extension_disable_offset: Offset16,
    extension_jstf_max_offset: Offset16,
}

pub struct JstfModList {
    lookup_count: u16,
    lookup_indices: Vec<u16>,
}

pub struct JstfMax{
    lookup_count: u16,
    lookup_offsets: Vec<Offset16>,
}
