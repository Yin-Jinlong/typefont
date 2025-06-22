use crate::font::io::ReadFrom;
use crate::font::{Offset16, Offset32, Tag};
use crate::impl_tag;
use crate::io::error::IOError;
use crate::io::reader::ReaderBoxed;

pub struct BASE {
    pub header: BaseHeader,
    pub vert_axis_table: AxisTable,
    pub horiz_axis_table: AxisTable,
    // TODO ItemVariationStore
}

pub enum BaseHeader {
    V1_0(BaseHeader1_0),
    V1_1(BaseHeader1_1),
}

pub struct BaseHeader1_0 {
    pub major_version: u16,
    pub minor_version: u16,
    pub horiz_axis_offset: Offset16,
    pub vert_axis_offset: Offset16,
}

pub struct BaseHeader1_1 {
    pub major_version: u16,
    pub minor_version: u16,
    pub horiz_axis_offset: Offset16,
    pub vert_axis_offset: Offset16,
    pub item_var_store_offset: Offset32,
}

impl_tag!(BASE, "BASE");

impl ReadFrom<ReaderBoxed> for BaseHeader {
    fn read_from(reader: &mut ReaderBoxed) -> Result<Self, IOError> {
        let major_version = reader.read_u16()?;
        let minor_version = reader.read_u16()?;
        let horiz_axis_offset = reader.read_u16()?;
        let vert_axis_offset = reader.read_u16()?;

        if major_version == 1 && minor_version == 0 {
            Ok(Self::V1_0(BaseHeader1_0 {
                major_version,
                minor_version,
                horiz_axis_offset,
                vert_axis_offset,
            }))
        } else if major_version == 1 && minor_version == 1 {
            let item_var_store_offset = reader.read_u32()?;
            Ok(Self::V1_1(BaseHeader1_1 {
                major_version,
                minor_version,
                horiz_axis_offset,
                vert_axis_offset,
                item_var_store_offset,
            }))
        } else {
            Err(IOError::BadFormat(format!(
                "Unsupported BASE version: {}.{}",
                major_version, minor_version
            )))
        }
    }
}

impl ReadFrom<ReaderBoxed> for BASE {
    fn read_from(reader: &mut ReaderBoxed) -> Result<Self, IOError> {
        let header = BaseHeader::read_from(reader)?;
        let vert_axis_table = AxisTable::read_from(reader)?;
        let horiz_axis_table = AxisTable::read_from(reader)?;

        Ok(Self {
            header,
            vert_axis_table,
            horiz_axis_table,
        })
    }
}

pub struct AxisTable {
    base_tag_list_offset: Offset16,
    base_script_list_offset: Offset16,
    //
    tag_list: BaseTagList,
    script_list: BaseScriptList,
}

impl ReadFrom<ReaderBoxed> for AxisTable {
    fn read_from(reader: &mut ReaderBoxed) -> Result<Self, IOError> {
        let pos = reader.position();
        let base_tag_list_offset = reader.read_u16()?;
        reader.mark()?;

        let tag_list = if base_tag_list_offset != 0 {
            reader.seek(pos + base_tag_list_offset as usize)?;
            BaseTagList::read_from(reader)?
        } else {
            BaseTagList::default()
        };

        reader.reset()?;
        let base_script_list_offset = reader.read_u16()?;

        let script_list = if base_script_list_offset != 0 {
            reader.seek(pos + base_script_list_offset as usize)?;
            BaseScriptList::read_from(reader)?
        } else {
            BaseScriptList::default()
        };

        Ok(Self {
            base_tag_list_offset,
            base_script_list_offset,
            tag_list,
            script_list,
        })
    }
}

#[derive(Default, Debug)]
pub struct BaseTagList {
    base_tag_count: u16,
    baseline_tags: Vec<Tag>,
}

impl ReadFrom<ReaderBoxed> for BaseTagList {
    fn read_from(reader: &mut ReaderBoxed) -> Result<Self, IOError> {
        let base_tag_count = reader.read_u16()?;
        let mut baseline_tags: Vec<Tag> = vec![];
        for _ in 0..base_tag_count {
            baseline_tags.push(Tag::read_from(reader)?);
        }
        Ok(Self {
            base_tag_count,
            baseline_tags,
        })
    }
}

#[derive(Default)]
pub struct BaseScriptList {
    base_script_count: u16,
    base_script_records: Vec<BaseScriptRecord>,
}

impl ReadFrom<ReaderBoxed> for BaseScriptList {
    fn read_from(reader: &mut ReaderBoxed) -> Result<Self, IOError> {
        let base_script_count = reader.read_u16()?;
        let mut base_script_records: Vec<BaseScriptRecord> = vec![];

        for _ in 0..base_script_count {
            base_script_records.push(BaseScriptRecord::read_from(reader)?);
        }

        Ok(Self {
            base_script_count,
            base_script_records,
        })
    }
}

pub struct BaseScriptRecord {
    base_script_tag: Tag,
    base_script_offset: Offset16,
}

impl ReadFrom<ReaderBoxed> for BaseScriptRecord {
    fn read_from(reader: &mut ReaderBoxed) -> Result<Self, IOError> {
        Ok(Self {
            base_script_tag: Tag::read_from(reader)?,
            base_script_offset: reader.read_u16()?,
        })
    }
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
