use corelib::{
    archive::{self, EntrySource, OriginalArchiveMetadata},
    file::{FileReader, FileWriter, FsFile},
    formats::Formats,
};

#[test]
fn sample_000_metadata() {
    let metadata = match *archive::metadata(
        Formats::Zip,
        "tests/samples/zip/000.zip".to_string(),
        true,
        1024,
    )
    .unwrap()
    {
        OriginalArchiveMetadata::Zip(metadata) => metadata,
    };
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "test.txt");
    assert_eq!(metadata.files[0].size, 14);
    assert_eq!(metadata.files[0].compression, "stored");
    assert_eq!(metadata.files[0].uncompressed_size, 14);
    assert_eq!(
        metadata.files[0].modified.to_rfc3339(),
        "2024-07-11T18:14:42+00:00"
    );
}

#[test]
fn sample_000_extract() {
    std::fs::create_dir_all("tests/samples/zip/000-external").unwrap();
    archive::extract(
        Formats::Zip,
        "tests/samples/zip/000.zip".to_string(),
        "tests/samples/zip/000-external".to_string(),
        None,
        None,
        true,
        true,
        1024,
    )
    .unwrap();
    let mut reader = FileReader::new(&"tests/samples/zip/000-external/test.txt".to_string());
    assert_eq!(reader.read_utf8(&reader.get_size()), "Hello, world!\n");
    reader.close();
    std::fs::remove_dir_all("tests/samples/zip/000-external").unwrap();
}

#[test]
fn sample_001_metadata() {
    let metadata = match *archive::metadata(
        Formats::Zip,
        "tests/samples/zip/001.zip".to_string(),
        true,
        1024,
    )
    .unwrap()
    {
        OriginalArchiveMetadata::Zip(metadata) => metadata,
    };
    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].path, "test.txt");
    assert_eq!(metadata.files[0].size, 14);
    assert_eq!(metadata.files[0].compression, "stored");
    assert_eq!(metadata.files[0].uncompressed_size, 14);
    assert_eq!(
        metadata.files[0].modified.to_rfc3339(),
        "2024-07-12T18:11:08+00:00"
    );
    assert_eq!(metadata.files[1].path, "test2.txt");
    assert_eq!(metadata.files[1].size, 16);
    assert_eq!(metadata.files[1].compression, "stored");
    assert_eq!(metadata.files[1].uncompressed_size, 16);
    assert_eq!(
        metadata.files[1].modified.to_rfc3339(),
        "2024-07-12T18:11:26+00:00"
    );
}

#[test]
fn sample_001_extract() {
    std::fs::create_dir_all("tests/samples/zip/001-external").unwrap();
    archive::extract(
        Formats::Zip,
        "tests/samples/zip/001.zip".to_string(),
        "tests/samples/zip/001-external".to_string(),
        None,
        None,
        true,
        true,
        1024,
    )
    .unwrap();
    let mut reader = FileReader::new(&"tests/samples/zip/001-external/test.txt".to_string());
    assert_eq!(reader.read_utf8(&reader.get_size()), "Hello, world!\n");
    reader.close();
    let mut reader = FileReader::new(&"tests/samples/zip/001-external/test2.txt".to_string());
    assert_eq!(reader.read_utf8(&reader.get_size()), "Hello, world! 2\n");
    reader.close();
    std::fs::remove_dir_all("tests/samples/zip/001-external").unwrap();
}

#[test]
fn sample_002_metadata() {
    let metadata = match *archive::metadata(
        Formats::Zip,
        "tests/samples/zip/002.zip".to_string(),
        true,
        1024,
    )
    .unwrap()
    {
        OriginalArchiveMetadata::Zip(metadata) => metadata,
    };
    assert_eq!(metadata.files.len(), 3);
    assert_eq!(metadata.files[0].path, "test/");
    assert_eq!(metadata.files[0].size, 0);
    assert_eq!(metadata.files[0].compression, "stored");
    assert_eq!(metadata.files[0].uncompressed_size, 0);
    assert_eq!(
        metadata.files[0].modified.to_rfc3339(),
        "2024-07-13T14:27:00+00:00"
    );
    assert_eq!(metadata.files[1].path, "test/test.txt");
    assert_eq!(metadata.files[1].size, 14);
    assert_eq!(metadata.files[1].compression, "stored");
    assert_eq!(metadata.files[1].uncompressed_size, 14);
    assert_eq!(
        metadata.files[1].modified.to_rfc3339(),
        "2024-07-13T14:26:48+00:00"
    );
    assert_eq!(metadata.files[2].path, "test.txt");
    assert_eq!(metadata.files[2].size, 14);
    assert_eq!(metadata.files[2].compression, "stored");
    assert_eq!(metadata.files[2].uncompressed_size, 14);
    assert_eq!(
        metadata.files[2].modified.to_rfc3339(),
        "2024-07-13T14:26:48+00:00"
    );
}

#[test]
fn sample_002_extract() {
    std::fs::create_dir_all("tests/samples/zip/002-external").unwrap();
    archive::extract(
        Formats::Zip,
        "tests/samples/zip/002.zip".to_string(),
        "tests/samples/zip/002-external".to_string(),
        None,
        None,
        true,
        true,
        1024,
    )
    .unwrap();
    let mut reader = FileReader::new(&"tests/samples/zip/002-external/test.txt".to_string());
    assert_eq!(reader.read_utf8(&reader.get_size()), "Hello, world!\n");
    reader.close();
    let mut reader = FileReader::new(&"tests/samples/zip/002-external/test/test.txt".to_string());
    assert_eq!(reader.read_utf8(&reader.get_size()), "Hello, world!\n");
    reader.close();
    std::fs::remove_dir_all("tests/samples/zip/002-external").unwrap();
}

#[test]
fn create_000_metadata() {
    std::fs::create_dir_all("tests/samples/zip/c000-external").unwrap();
    let mut test_txt = FileWriter::new(
        &"tests/samples/zip/c000-external/test.txt".to_string(),
        &false,
    );
    test_txt.write(b"Hello, world!\n");
    test_txt.close();

    archive::create(
        Formats::Zip,
        "tests/samples/zip/c000-external.zip".to_string(),
        &mut vec![EntrySource {
            path: "test.txt".to_string(),
            source: &mut FsFile::new(&"tests/samples/zip/c000-external/test.txt".to_string()),
        }],
        1024,
    )
    .unwrap();

    std::fs::remove_dir_all("tests/samples/zip/c000-external").unwrap();

    let metadata = match *archive::metadata(
        Formats::Zip,
        "tests/samples/zip/c000-external.zip".to_string(),
        true,
        1024,
    )
    .unwrap()
    {
        OriginalArchiveMetadata::Zip(metadata) => metadata,
    };

    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "test.txt");
    assert_eq!(metadata.files[0].size, 14);
    assert_eq!(metadata.files[0].compression, "stored");
    assert_eq!(metadata.files[0].uncompressed_size, 14);

    std::fs::remove_file("tests/samples/zip/c000-external.zip").unwrap();
}

#[test]
fn create_000_extract() {
    std::fs::create_dir_all("tests/samples/zip/c000-external2").unwrap(); // this is has another name to avoid conflicts
    let mut test_txt = FileWriter::new(
        &"tests/samples/zip/c000-external2/test.txt".to_string(),
        &false,
    );
    test_txt.write(b"Hello, world!\n");
    test_txt.close();

    std::fs::create_dir_all("tests/samples/zip/c000-external2").unwrap();
    let mut test_txt = FileWriter::new(
        &"tests/samples/zip/c000-external2/test.txt".to_string(),
        &false,
    );
    test_txt.write(b"Hello, world!\n");
    test_txt.close();

    archive::create(
        Formats::Zip,
        "tests/samples/zip/c000-external2.zip".to_string(),
        &mut vec![EntrySource {
            path: "test.txt".to_string(),
            source: &mut FsFile::new(&"tests/samples/zip/c000-external2/test.txt".to_string()),
        }],
        1024,
    )
    .unwrap();

    std::fs::remove_dir_all("tests/samples/zip/c000-external2").unwrap();

    archive::extract(
        Formats::Zip,
        "tests/samples/zip/c000-external2.zip".to_string(),
        "tests/samples/zip/c000-external2".to_string(),
        None,
        None,
        true,
        true,
        1024,
    )
    .unwrap();

    let mut reader = FileReader::new(&"tests/samples/zip/c000-external2/test.txt".to_string());
    assert_eq!(reader.read_utf8(&reader.get_size()), "Hello, world!\n");
    reader.close();

    std::fs::remove_dir_all("tests/samples/zip/c000-external2").unwrap();
    std::fs::remove_file("tests/samples/zip/c000-external2.zip").unwrap();
}

/*
#[test]
fn create_000() {
    let mut output = FileWriter::new(&"tests/samples/zip/c000.zip".to_string(), &false);

    std::fs::create_dir_all("tests/samples/zip/c000").unwrap();
    let mut test_txt = FileWriter::new(&"tests/samples/zip/c000/test.txt".to_string(), &false);
    test_txt.write(b"Hello, world!\n");
    test_txt.close();

    let mut input = FileReader::new(&"tests/samples/zip/c000/test.txt".to_string());
    let size = input.get_size();
    corelib::formats::zip::writer::write(
        &mut output,
        &mut ZipArchiveData {
            files: vec![ZipFile {
                checksum: crc32::hash(&mut input, &0, &size, &1024),
                path: "test.txt".to_string(),
                offset: 0,
                size,
                modified: input.get_times().modified,
                is_directory: false,
                source: &mut input,
            }],
        },
        &1024,
    );
    input.close();

    output.close();

    let mut file = FileReader::new(&"tests/samples/zip/c000.zip".to_string());

    let metadata = corelib::formats::zip::parser::metadata(&mut file);
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "test.txt");
    assert_eq!(metadata.files[0].size, 14);
    assert_eq!(metadata.files[0].compression, "stored");
    assert_eq!(metadata.files[0].uncompressed_size, 14);

    let test_txt = corelib::formats::zip::parser::get_file(&mut file, &metadata.files[0]);
    assert_eq!(String::from_utf8(test_txt).unwrap(), "Hello, world!\n");

    std::fs::remove_dir_all("tests/samples/zip/c000").unwrap();

    assert!(corelib::formats::zip::parser::check_integrity(
        &mut file,
        &metadata.files[0],
        &1024
    ));

    std::fs::remove_file("tests/samples/zip/c000.zip").unwrap();
}
*/
