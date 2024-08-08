use corelib::file::FileReader;

#[test]
fn sample_000() {
    let mut file = FileReader::new(&"tests/samples/rar/000.rar".to_string());

    let metadata = corelib::formats::rar::parser::metadata(&mut file);
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "testfile.txt");
    assert_eq!(metadata.files[0].size, 12);
    assert_eq!(metadata.files[0].uncompressed_size.unwrap(), 12);

    let testfile_txt =
        corelib::formats::rar::parser::get_file(&mut file, &metadata.files[0]).unwrap();
    assert_eq!(String::from_utf8(testfile_txt).unwrap(), "Testing 123\n");

    assert!(
        corelib::formats::rar::parser::check_integrity(&mut file, &metadata.files[0], &1024)
            .unwrap()
            .unwrap()
    );
}

#[test]
fn sample_001() {
    let mut file = FileReader::new(&"tests/samples/rar/001.rar".to_string());

    let metadata = corelib::formats::rar::parser::metadata(&mut file);

    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "testfile.txt");
    assert_eq!(metadata.files[0].size, 12);
    assert_eq!(metadata.files[0].uncompressed_size.unwrap(), 12);

    let testfile_txt =
        corelib::formats::rar::parser::get_file(&mut file, &metadata.files[0]).unwrap();
    assert_eq!(String::from_utf8(testfile_txt).unwrap(), "Testing 123\n");

    assert!(
        corelib::formats::rar::parser::check_integrity(&mut file, &metadata.files[0], &1024)
            .unwrap()
            .unwrap()
    );

    assert!(metadata.locked);
}

#[test]

fn sample_002() {
    let mut file = FileReader::new(&"tests/samples/rar/002.rar".to_string());

    let metadata = corelib::formats::rar::parser::metadata(&mut file);

    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "testfile.txt");
    assert_eq!(metadata.files[0].size, 12);
    assert_eq!(metadata.files[0].uncompressed_size.unwrap(), 12);

    let testfile_txt =
        corelib::formats::rar::parser::get_file(&mut file, &metadata.files[0]).unwrap();
    assert_eq!(String::from_utf8(testfile_txt).unwrap(), "Testing 123\n");

    assert!(
        corelib::formats::rar::parser::check_integrity(&mut file, &metadata.files[0], &1024)
            .unwrap()
            .unwrap()
    );

    assert!(metadata.rr_offset.is_some());
}

#[test]

fn sample_003() {
    let mut file = FileReader::new(&"tests/samples/rar/003.rar".to_string());

    let metadata = corelib::formats::rar::parser::metadata(&mut file);
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "testfile.txt");
    assert_eq!(metadata.files[0].uncompressed_size.unwrap(), 12);

    // We can't extract compressed files yet as there is no native RAR decompressor in Rust, this has to be written later from scratch
    let testfile_txt = corelib::formats::rar::parser::get_file(&mut file, &metadata.files[0]); // This IS compressed
    assert!(match testfile_txt {
        Ok(_) => false,
        Err(msg) => msg == "Compressed RAR files are not supported yet.",
    });
    //assert_eq!(String::from_utf8(testfile_txt.unwrap()).unwrap(), "Testing 123\n");

    /*assert!(
        corelib::formats::rar::parser::check_integrity(&mut file, &metadata.files[0], &1024)
            .unwrap().unwrap()
    );*/

    assert!(metadata.solid);
}
