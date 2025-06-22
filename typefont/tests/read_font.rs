use typefont::font::open_type::read_font;

#[test]
fn test() {
    let r = read_font("test.otf".into());
    assert!(r.is_ok());
}
