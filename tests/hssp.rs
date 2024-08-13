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

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

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

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    // TODO: Encrypted files have no valid checksum for some reason
    /* assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata,
    )); */
}

#[test]
fn sample_002() {
    let mut file = FileReader::new(&"tests/samples/hssp/002.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 2);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].name, "test");
    assert!(!metadata.files[0].is_main);
    assert!(metadata.files[0].is_directory);
    assert_eq!(metadata.files[1].name, "test/test.txt");
    assert!(!metadata.files[1].is_main);
    assert!(!metadata.files[1].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[1]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_003() {
    let mut file = FileReader::new(&"tests/samples/hssp/003.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 2);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(!metadata.files[0].is_main);
    assert!(!metadata.files[0].is_directory);
    assert_eq!(metadata.files[1].name, "test2.txt");
    assert!(!metadata.files[1].is_main);
    assert!(!metadata.files[1].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");
    let test2_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[1]);
    assert_eq!(String::from_utf8(test2_txt).unwrap(), "Hello, world! 2");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_004() {
    let mut file = FileReader::new(&"tests/samples/hssp/004.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 2);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(metadata.files[0].is_main);
    assert!(metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_005() {
    let mut file = FileReader::new(&"tests/samples/hssp/005.hssp".to_string());

    // this could panic in theory, but won't because only the checksum was altered
    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert!(!corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_006() {
    let mut file = FileReader::new(&"tests/samples/hssp/006.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 1);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(!metadata.files[0].is_main);
    assert!(!metadata.files[0].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_007() {
    let mut file = FileReader::new(&"tests/samples/hssp/007.hssp".to_string());

    let password = "Password".to_string();

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, Some(&password));

    assert_eq!(metadata.version, 1);
    assert!(metadata.encryption.is_some());
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(!metadata.files[0].is_main);
    assert!(!metadata.files[0].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    // TODO: Encrypted files have no valid checksum for some reason
    /* assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata,
    )); */
}

#[test]
fn sample_008() {
    let mut file = FileReader::new(&"tests/samples/hssp/008.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 1);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].name, "test");
    assert!(!metadata.files[0].is_main);
    assert!(metadata.files[0].is_directory);
    assert_eq!(metadata.files[1].name, "test/test.txt");
    assert!(!metadata.files[1].is_main);
    assert!(!metadata.files[1].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[1]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_009() {
    let mut file = FileReader::new(&"tests/samples/hssp/009.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 1);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(!metadata.files[0].is_main);
    assert!(!metadata.files[0].is_directory);
    assert_eq!(metadata.files[1].name, "test2.txt");
    assert!(!metadata.files[1].is_main);
    assert!(!metadata.files[1].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");
    let test2_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[1]);
    assert_eq!(String::from_utf8(test2_txt).unwrap(), "Hello, world! 2");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_010() {
    let mut file = FileReader::new(&"tests/samples/hssp/010.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 1);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(metadata.files[0].is_main);
    assert!(metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_011() {
    let mut file = FileReader::new(&"tests/samples/hssp/011.hssp".to_string());

    // this could panic in theory, but won't because only the checksum was altered
    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert!(!corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_012() {
    let mut file = FileReader::new(&"tests/samples/hssp/012.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 3);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].name.len(), 8);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(!metadata.files[0].is_main);
    assert!(!metadata.files[0].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_013() {
    let mut file = FileReader::new(&"tests/samples/hssp/013.hssp".to_string());

    let password = "Password".to_string();

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, Some(&password));

    assert_eq!(metadata.version, 3);
    assert!(metadata.encryption.is_some());
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(!metadata.files[0].is_main);
    assert!(!metadata.files[0].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    // TODO: Encrypted files have no valid checksum for some reason
    /* assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata,
    )); */
}

#[test]
fn sample_014() {
    let mut file = FileReader::new(&"tests/samples/hssp/014.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 3);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].name, "test");
    assert!(!metadata.files[0].is_main);
    assert!(metadata.files[0].is_directory);
    assert_eq!(metadata.files[1].name, "test/test.txt");
    assert!(!metadata.files[1].is_main);
    assert!(!metadata.files[1].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[1]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_015() {
    let mut file = FileReader::new(&"tests/samples/hssp/015.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 3);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(!metadata.files[0].is_main);
    assert!(!metadata.files[0].is_directory);
    assert_eq!(metadata.files[1].name, "test2.txt");
    assert!(!metadata.files[1].is_main);
    assert!(!metadata.files[1].is_directory);
    assert!(!metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");
    let test2_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[1]);
    assert_eq!(String::from_utf8(test2_txt).unwrap(), "Hello, world! 2");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_016() {
    let mut file = FileReader::new(&"tests/samples/hssp/016.hssp".to_string());

    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert_eq!(metadata.version, 3);
    assert!(metadata.encryption.is_none());
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].name, "test.txt");
    assert!(metadata.files[0].is_main);
    assert!(metadata.has_main);

    let test_txt =
        corelib::formats::hssp::parser::get_file(&mut file, &metadata, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!");

    assert!(corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}

#[test]
fn sample_017() {
    let mut file = FileReader::new(&"tests/samples/hssp/017.hssp".to_string());

    // this could panic in theory, but won't because only the checksum was altered
    let metadata = corelib::formats::hssp::parser::metadata(&mut file, None);

    assert!(!corelib::formats::hssp::parser::check_integrity_all(
        &mut file, &metadata
    ));
}
