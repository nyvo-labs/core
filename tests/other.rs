#[test]
fn returns_real_version() {
    assert_eq!(acridotheres::get_version(), env!("CARGO_PKG_VERSION"));
}
