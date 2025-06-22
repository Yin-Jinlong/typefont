use super::Table;
use crate::font::{Offset16, Offset32};
use crate::impl_table;

pub enum Loca {
    Short { offsets: Vec<Offset16> },
    Long { offsets: Vec<Offset32> },
}

impl_table!(Loca, "loca");
