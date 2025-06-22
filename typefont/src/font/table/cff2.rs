use crate::impl_tag;

// TODO

pub struct CFF2 {}

impl_tag!(CFF2, "CFF2");

pub struct CFF2Header {
    /// 2
    major_version: u8,
    /// 0
    minor_version: u8,
    header_size: u8,
    top_dict_size: u16,
}
