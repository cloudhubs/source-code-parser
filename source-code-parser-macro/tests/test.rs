#[test]
pub fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/child_fields.rs");
}
