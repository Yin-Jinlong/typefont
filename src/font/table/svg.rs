use super::Table;
use crate::font::Offset32;
use crate::impl_table;

pub struct SVG {
    header: SVGHeader,
    document_list: SVGDocumentList,
}

pub struct SVGHeader {
    /// 0
    version: u16,
    svg_document_list_offset: Offset32,
    /// 0
    reserved: u32,
}

impl_table!(SVG, "SVG");

pub struct SVGDocumentList {
    num_entries: u16,
    document_records: Vec<SVGDocumentRecord>,
}

pub struct SVGDocumentRecord {
    start_glyph_id: u16,
    end_glyph_id: u16,
    svg_doc_offset: u16,
    svg_doc_length: u16,
}
