use super::var::formats::TupleVariationHeader;
use crate::font::table::Table;
use crate::font::Offset16;
use crate::impl_table;

pub struct Cvar {
    header: CvarHeader,
    data: Vec<u8>,
}

pub struct CvarHeader {
    /// 1
    major_version: u16,
    /// 0
    minor_version: u16,
    tuple_variation_count: u16,
    data_offset: Offset16,
    tuple_variation_headers: Vec<TupleVariationHeader>,
}

impl_table!(Cvar, "cvar");
