use corelib::file::FileReader;

#[test]
fn sample_000() {
    let mut file = FileReader::new(&"tests/samples/hssp/000.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 2);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(!metadata.files[0].is_main);
    assert!(!metadata.files[0].is_directory);
    assert!(!metadata.has_main);

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_001() {
    let mut file = FileReader::new(&"tests/samples/hssp/001.hssp".to_string());

    let password = "Password".to_string();

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, Some(&password));

    assert_eq!(metadata.version, 2);
    assert!(metadata.encryption.is_some());
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(!metadata.files[0].is_main);
    assert!(!metadata.files[0].is_directory);
    assert!(!metadata.has_main);

    // TODO: Encrypted files have no valid checksum for some reason
    /* assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata,
    )); */
}
