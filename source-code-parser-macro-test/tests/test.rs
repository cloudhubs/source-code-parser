#[test]
pub fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/fields_and_lang.rs");
}
