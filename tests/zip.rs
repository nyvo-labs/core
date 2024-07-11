use corelib;

#[test]
fn metadata_000() {
    let metadata = corelib::formats::zip::parser::metadata("tests/samples/zip/000.zip");
    assert_eq!(metadata.lfh_matches, true);
    //assert_eq!(metadata.file_count, 1);
}
