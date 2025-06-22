use crate::font::{Offset16, Offset32, Tag};
use crate::impl_tag;

pub struct BASE {
    header: BaseHeader,
    vert_axis_table: AxisTable,
    horiz_axis_table: AxisTable,
    tag_list: BaseTagList,
    script_list: BaseScriptList,
    // TODO ItemVariationStore
}

pub enum BaseHeader {
    V1_0(BaseHeader1_0),
    V1_1(BaseHeader1_1),
}

pub struct BaseHeader1_0 {
    major_version: u16,
    minor_version: u16,
    horiz_axis_offset: Offset16,
    vert_axis_offset: Offset16,
}

pub struct BaseHeader1_1 {
    major_version: u16,
    minor_version: u16,
    horiz_axis_offset: Offset16,
    vert_axis_offset: Offset16,
    item_var_store_offset: Offset32,
}

impl_tag!(BASE, "BASE");

pub struct AxisTable {
    base_tag_list_offset: Offset16,
    base_script_list_offset: Offset16,
}

pub struct BaseTagList {
    base_tag_count: u16,
    baseline_tags: Vec<Tag>,
}

pub struct BaseScriptList {
    base_script_count: u16,
    base_script_records: Vec<BaseScriptRecord>,
}

pub struct BaseScriptRecord {
    base_script_tag: Tag,
    base_script_offset: Offset16,
}

pub struct BaseScript {
    base_values_offset: Offset16,
    default_min_max_offset: Offset16,
    base_lang_sys_count: u16,
    base_lang_sys_records: Vec<BaseLangSys>,
}

pub struct BaseLangSys {
    base_lang_sys_tag: Tag,
    min_max_offset: Offset16,
}

pub struct BaseValues {
    default_baseline_index: u16,
    base_coord_count: u16,
    base_coord_offsets: Offset16,
}

pub struct MinMax {
    min_coord_offset: Offset16,
    max_coord_offset: Offset16,
    feat_min_max_count: u16,
    feat_min_max_records: Vec<FeatMinMax>,
}

pub struct FeatMinMax {
    feature_tag: Tag,
    min_coord_offset: Offset16,
    max_coord_offset: Offset16,
}

pub enum BaseCoord {
    Format1(BaseCoord1),
    Format2(BaseCoord2),
    Format3(BaseCoord3),
}

pub struct BaseCoord1 {
    /// 1
    format: u16,
    coordinate: i16,
}

pub struct BaseCoord2 {
    /// 2
    format: u16,
    coordinate: i16,
    reference_glyph: u16,
    base_coord_point: u16,
}

pub struct BaseCoord3 {
    /// 3
    format: u16,
    coordinate: i16,
    device_offset: Offset16,
}
