use crate::impl_tag;

pub struct CBDT {
    header: CbdtHeader,
    /// CBDT 表其余部分为位图数据。数据可采用三种可能的格式呈现，
    /// 这些格式由 CBLC 表中的信息进行指示。
    /// 部分格式包含指标信息及图像数据，
    /// 其他格式仅包含图像数据。
    /// 这些子表无需长字对齐，仅需字节对齐即可。
    bmp_data: Vec<u8>,
}

impl_tag!(CBDT, "CBDT");

pub struct CbdtHeader {
    /// 3
    major_version: u16,
    /// 0
    minor_version: u16,
}
