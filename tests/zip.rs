use corelib;

#[test]
fn file_count_000() {
    assert_eq!(corelib::formats::zip::parser::metadata("tests/samples/zip/000.zip").file_count, 1);
}