use super::Table;
use crate::impl_table;

pub struct Prep {
    data: Vec<u8>,
}

impl_table!(Prep, "prep");
