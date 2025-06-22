use crate::impl_tag;

pub struct Fpgm {
    data: Vec<u8>,
}

impl_tag!(Fpgm, "fpgm");
