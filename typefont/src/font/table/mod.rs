pub mod avar;
pub mod base;
pub mod bm;
pub mod cbdt;
pub mod cblc;
pub mod cff;
pub mod cff2;
pub mod cmap;
pub mod colr;
pub mod cpal;
pub mod cvar;
pub mod cvt;
pub mod dsig;
pub mod ebdt;
pub mod eblc;
pub mod ebsc;
pub mod fpgm;
pub mod fvar;
pub mod gasp;
pub mod gdef;
pub mod glyf;
pub mod glyph;
pub mod gpos;
pub mod gsub;
pub mod hdmx;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod hvar;
pub mod jstf;
pub mod kern;
pub mod loca;
pub mod ltsh;
pub mod math;
pub mod maxp;
pub mod merg;
pub mod meta;
pub mod mvar;
pub mod name;
pub mod os2;
pub mod pclt;
pub mod post;
pub mod prep;
pub mod sbix;
pub mod stat;
pub mod svg;
pub mod var;
pub mod vdmx;
pub mod vhea;
pub mod vmtx;
pub mod vorg;
pub mod vvar;

pub trait WithTag {
    const TAG: crate::font::Tag;
    const TAG_U32: u32;
}

pub enum Table {
    Avar(avar::Avar),
    BASE(base::BASE),
    CBDT(cbdt::CBDT),
    CBLC(cblc::CBLC),
    CFF(cff::CFF),
    CFF2(cff2::CFF2),
    Cmap(cmap::Cmap),
    COLR(colr::COLR),
    CPAL(cpal::CPAL),
    Cvar(cvar::Cvar),
    Cvt(cvt::Cvt),
    DSIG(dsig::DSIG),
    EBDT(ebdt::EBDT),
    Eblc(eblc::Eblc),
    EBSC(ebsc::EBSC),
    Fpgm(fpgm::Fpgm),
    Fvar(fvar::Fvar),
    Gasp(gasp::Gasp),
    GDEF(gdef::GDEF),
    Glyf(glyf::Glyf),
    GPOS(gpos::GPOS),
    GSUB(gsub::GSUB),
    Hdmx(hdmx::Hdmx),
    Head(head::Head),
    Hhea(hhea::Hhea),
    Hmtx(hmtx::Hmtx),
    HVAR(hvar::HVAR),
    JSTF(jstf::JSTF),
    Kern(kern::Kern),
    Loca(loca::Loca),
    LTSH(ltsh::LTSH),
    MATH(math::MATH),
    Maxp(maxp::Maxp),
    MERG(merg::MERG),
    Meta(meta::Meta),
    MVAR(mvar::MVAR),
    Name(name::Name),
    OS2(os2::Os2),
    PCLT(pclt::PCLT),
    Post(post::Post),
    Prep(prep::Prep),
    Sbix(sbix::Sbix),
    STAT(stat::STAT),
    SVG(svg::SVG),
    VDMX(vdmx::VDMX),
    Vhea(vhea::Vhea),
    Vmtx(vmtx::Vmtx),
    VORG(vorg::VORG),
    VVAR(vvar::VVAR),
}

#[macro_export]
macro_rules! impl_tag {
    ($table:ty,$name:literal) => {
        use super::WithTag;

        impl WithTag for $table {
            const TAG: crate::font::Tag = crate::font::Tag::from_raw_unchecked($name);
            const TAG_U32: u32 = crate::font::Tag::tag_str_to_u32_unchecked($name);
        }
    };
}
