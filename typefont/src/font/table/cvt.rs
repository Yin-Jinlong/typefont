use crate::font::FWord;
use crate::impl_tag;

pub struct Cvt {
    data: Vec<FWord>,
}

impl_tag!(Cvt, "cvt");
