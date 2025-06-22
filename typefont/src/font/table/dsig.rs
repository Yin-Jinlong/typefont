use crate::font::Offset32;
use crate::impl_tag;

pub struct DSIG {
    version: u32,
    num_signatures: u16,
    flags: u16,
    signature_records: Vec<SignatureRecord>,
}

impl_tag!(DSIG, "DSIG");

pub struct SignatureRecord {
    format: u32,
    length: u32,
    signature_block_offset: Offset32,
}

pub enum SignatureBlock {
    Format1(SignatureBlockFormat1),
}

pub struct SignatureBlockFormat1 {
    reserved1: u16,
    reserved2: u16,
    signature_length: u32,
    signature: Vec<u8>,
}
