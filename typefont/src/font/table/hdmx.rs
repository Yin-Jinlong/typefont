use crate::impl_named;

pub struct Hdmx {}

pub struct HdmxHeader {
    /// 0
    version: u16,
    num_records: u16,
    size_device_record: u32,
    records: Vec<DeviceRecord>,
}

impl_named!(Hdmx, "hdmx");

pub struct DeviceRecord {
    pixel_size: u8,
    max_width: u8,
    widths: Vec<u8>,
}
