use typefont::font::open_type::read_font;

#[test]
fn test() {
    let r = read_font("test.otf".into());
    match r {
        Ok(r) => {
            println!("{}", r.tables.len());
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}
