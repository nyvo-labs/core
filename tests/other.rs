use corelib;

#[test]
fn returns_real_version() {
    assert_eq!(corelib::get_version(), env!("CARGO_PKG_VERSION"));
}