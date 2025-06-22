use crate::font::table::Table;
use crate::font::FWord;
use crate::impl_table;

pub struct Cvt {
    data: Vec<FWord>,
}

impl_table!(Cvt, "cvt");
