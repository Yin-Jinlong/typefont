use super::Table;
use crate::impl_table;

pub struct Fpgm {
    data: Vec<u8>,
}

impl_table!(Fpgm, "fpgm");
