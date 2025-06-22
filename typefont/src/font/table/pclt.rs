use crate::impl_tag;

pub struct PCLT {
    major_version: u16,
    minor_version: u16,
    font_number: u32,
    pitch: u16,
    x_height: u16,
    style: u16,
    type_family: u16,
    cap_height: u16,
    symbol_set: u16,
    typeface: [i8; 16],
    character_complement: [i8; 8],
    file_name: [i8; 6],
    stroke_weight: i8,
    width_type: i8,
    serif_style: u8,
    /// 0
    reserved: u8,
}

impl_tag!(PCLT, "PCLT");
