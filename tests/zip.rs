use corelib::{self, File};

#[test]
fn metadata_000() {
    let mut file = File::new("tests/samples/zip/000.zip");

    let metadata = corelib::formats::zip::parser::metadata(&mut file);
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].file.path, "test.txt");
    assert_eq!(metadata.files[0].file.size, 14);
    assert_eq!(metadata.files[0].compression, "stored");
    assert_eq!(metadata.files[0].uncompressed_size, 14);
    assert_eq!(
        metadata.files[0].file.modified.to_rfc3339(),
        "2024-07-11T18:14:42+00:00"
    );
    //println!("{:#?}", metadata);

    let test_txt = corelib::formats::zip::parser::get_file(&mut file, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!\n");
}

#[test]
fn metadata_001() {
    let mut file = File::new("tests/samples/zip/001.zip");

    let metadata = corelib::formats::zip::parser::metadata(&mut file);
    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].file.path, "test.txt");
    assert_eq!(metadata.files[0].file.size, 14);
    assert_eq!(metadata.files[0].compression, "stored");
    assert_eq!(metadata.files[0].uncompressed_size, 14);
    assert_eq!(
        metadata.files[0].file.modified.to_rfc3339(),
        "2024-07-12T18:11:08+00:00"
    );
    assert_eq!(metadata.files[1].file.path, "test2.txt");
    assert_eq!(metadata.files[1].file.size, 16);
    assert_eq!(metadata.files[1].compression, "stored");
    assert_eq!(metadata.files[1].uncompressed_size, 16);
    assert_eq!(
        metadata.files[1].file.modified.to_rfc3339(),
        "2024-07-12T18:11:26+00:00"
    );

    let test_txt = corelib::formats::zip::parser::get_file(&mut file, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!\n");
    let test2_txt = corelib::formats::zip::parser::get_file(&mut file, &metadata.files[1]);
    assert_eq!(String::from_utf8(test2_txt).unwrap(), "Hello, world! 2\n");

    std::fs::create_dir_all("tests/samples/zip/001").unwrap();

    corelib::formats::zip::parser::extract(
        &mut file,
        metadata.files,
        1024,
        &|path| format!("tests/samples/zip/001/{}", path),
    );

    let extracted_test_txt = std::fs::read("tests/samples/zip/001/test.txt").unwrap();
    assert_eq!(String::from_utf8(extracted_test_txt).unwrap(), "Hello, world!\n");
    let extracted_test2_txt = std::fs::read("tests/samples/zip/001/test2.txt").unwrap();
    assert_eq!(String::from_utf8(extracted_test2_txt).unwrap(), "Hello, world! 2\n");

    std::fs::remove_dir_all("tests/samples/zip/001").unwrap();
}
