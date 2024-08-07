use corelib::file::FileReader;

#[test]
fn sample_000() {
    let mut file = FileReader::new(&"tests/samples/rar/000.rar".to_string());

    let metadata = corelib::formats::rar::parser::metadata(&mut file);
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "testfile.txt");
    assert_eq!(metadata.files[0].size, 12);
    assert_eq!(metadata.files[0].uncompressed_size.unwrap(), 12);

    let testfile_txt = corelib::formats::rar::parser::get_file(&mut file, &metadata.files[0]);
    assert_eq!(String::from_utf8(testfile_txt).unwrap(), "Testing 123\n");

    assert!(
        corelib::formats::rar::parser::check_integrity(&mut file, &metadata.files[0], &1024)
            .unwrap()
    );
}
