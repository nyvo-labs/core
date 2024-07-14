use corelib::{self, helpers::hash::crc32, File, FileReader, FileWriter, ZipArchiveData, ZipFile};

#[test]
fn sample_000() {
    let mut file = FileReader::new("tests/samples/zip/000.zip");

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

    assert!(corelib::formats::zip::parser::check_integrity(
        &mut file,
        &metadata.files[0],
        1024
    ));
}

#[test]
fn sample_001() {
    let mut file = FileReader::new("tests/samples/zip/001.zip");

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

    corelib::formats::zip::parser::extract(&mut file, &metadata.files, 1024, &|path| {
        format!("tests/samples/zip/001/{}", path)
    });

    let extracted_test_txt = std::fs::read("tests/samples/zip/001/test.txt").unwrap();
    assert_eq!(
        String::from_utf8(extracted_test_txt).unwrap(),
        "Hello, world!\n"
    );
    let extracted_test2_txt = std::fs::read("tests/samples/zip/001/test2.txt").unwrap();
    assert_eq!(
        String::from_utf8(extracted_test2_txt).unwrap(),
        "Hello, world! 2\n"
    );

    std::fs::remove_dir_all("tests/samples/zip/001").unwrap();

    assert!(corelib::formats::zip::parser::check_integrity(
        &mut file,
        &metadata.files[0],
        1024
    ));
    assert!(corelib::formats::zip::parser::check_integrity(
        &mut file,
        &metadata.files[1],
        1024
    ));
}

#[test]
fn sample_002() {
    let mut file = FileReader::new("tests/samples/zip/002.zip");

    let metadata = corelib::formats::zip::parser::metadata(&mut file);
    assert_eq!(metadata.files.len(), 3);
    assert_eq!(metadata.files[0].file.path, "test/");
    assert_eq!(metadata.files[0].file.size, 0);
    assert_eq!(metadata.files[0].compression, "stored");
    assert_eq!(metadata.files[0].uncompressed_size, 0);
    assert_eq!(
        metadata.files[0].file.modified.to_rfc3339(),
        "2024-07-13T14:27:00+00:00"
    );
    assert_eq!(metadata.files[1].file.path, "test/test.txt");
    assert_eq!(metadata.files[1].file.size, 14);
    assert_eq!(metadata.files[1].compression, "stored");
    assert_eq!(metadata.files[1].uncompressed_size, 14);
    assert_eq!(
        metadata.files[1].file.modified.to_rfc3339(),
        "2024-07-13T14:26:48+00:00"
    );
    assert_eq!(metadata.files[2].file.path, "test.txt");
    assert_eq!(metadata.files[2].file.size, 14);
    assert_eq!(metadata.files[2].compression, "stored");
    assert_eq!(metadata.files[2].uncompressed_size, 14);
    assert_eq!(
        metadata.files[2].file.modified.to_rfc3339(),
        "2024-07-13T14:26:48+00:00"
    );

    let test_txt = corelib::formats::zip::parser::get_file(&mut file, &metadata.files[1]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!\n");
    let test_test_txt = corelib::formats::zip::parser::get_file(&mut file, &metadata.files[2]);
    assert_eq!(String::from_utf8(test_test_txt).unwrap(), "Hello, world!\n");

    std::fs::create_dir_all("tests/samples/zip/002").unwrap();

    corelib::formats::zip::parser::extract(&mut file, &metadata.files, 1024, &|path| {
        format!("tests/samples/zip/002/{}", path)
    });

    let extracted_test_txt = std::fs::read("tests/samples/zip/002/test.txt").unwrap();
    assert_eq!(
        String::from_utf8(extracted_test_txt).unwrap(),
        "Hello, world!\n"
    );
    let extracted_test2_txt = std::fs::read("tests/samples/zip/002/test/test.txt").unwrap();
    assert_eq!(
        String::from_utf8(extracted_test2_txt).unwrap(),
        "Hello, world!\n"
    );

    std::fs::remove_dir_all("tests/samples/zip/002").unwrap();

    assert!(corelib::formats::zip::parser::check_integrity(
        &mut file,
        &metadata.files[0],
        1024
    ));
    assert!(corelib::formats::zip::parser::check_integrity(
        &mut file,
        &metadata.files[1],
        1024
    ));
    assert!(corelib::formats::zip::parser::check_integrity(
        &mut file,
        &metadata.files[2],
        1024
    ));
}

#[test]
fn create_000() {
    let mut output = FileWriter::new("tests/samples/zip/c000.zip", false);

    std::fs::create_dir_all("tests/samples/zip/c000").unwrap();
    {
        let mut test_txt = FileWriter::new("tests/samples/zip/c000/test.txt", false);
        test_txt.write(b"Hello, world!\n");
        test_txt.close();

        let mut input = FileReader::new("tests/samples/zip/c000/test.txt");
        let size = input.get_size();
        corelib::formats::zip::writer::write(
            &mut output,
            &mut ZipArchiveData {
                files: vec![ZipFile {
                    checksum: crc32::hash(&mut input, 0, size, 1024),
                    file: File {
                        path: "test.txt".to_string(),
                        offset: 0,
                        size,
                        modified: input.get_times().modified,
                        is_directory: false,
                        source: &mut input,
                    },
                }],
            },
            1024,
        );
    } // that test_txt dies here, that shit caused bugs as fuck
    output.close();

    let mut file = FileReader::new("tests/samples/zip/c000.zip");

    let metadata = corelib::formats::zip::parser::metadata(&mut file);
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].file.path, "test.txt");
    assert_eq!(metadata.files[0].file.size, 14);
    assert_eq!(metadata.files[0].compression, "stored");
    assert_eq!(metadata.files[0].uncompressed_size, 14);

    let test_txt = corelib::formats::zip::parser::get_file(&mut file, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!\n");

    std::fs::remove_dir_all("tests/samples/zip/c000").unwrap();

    assert!(corelib::formats::zip::parser::check_integrity(
        &mut file,
        &metadata.files[0],
        1024
    ));

    std::fs::remove_file("tests/samples/zip/c000.zip").unwrap();
}
