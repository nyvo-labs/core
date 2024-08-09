use corelib::{
    archive::{self, OriginalArchiveMetadata},
    file::FileReader,
    formats::Formats,
};

#[test]
fn sample_000_metadata() {
    let metadata = match *archive::metadata(
        Formats::Rar,
        "tests/samples/rar/000.rar".to_string(),
        true,
        1024,
    )
    .unwrap()
    {
        OriginalArchiveMetadata::Rar(metadata) => metadata,
        _ => panic!("This could never happen, this is only here for type safety"),
    };
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "testfile.txt");
    assert_eq!(metadata.files[0].size, 12);
    assert_eq!(metadata.files[0].uncompressed_size.unwrap(), 12);
}

#[test]
fn sample_000_extract() {
    std::fs::create_dir_all("tests/samples/rar/000-external").unwrap();
    archive::extract(
        Formats::Rar,
        "tests/samples/rar/000.rar".to_string(),
        "tests/samples/rar/000-external".to_string(),
        None,
        None,
        true,
        true,
        1024,
    )
    .unwrap();
    let mut reader = FileReader::new(&"tests/samples/rar/000-external/testfile.txt".to_string());
    assert_eq!(reader.read_utf8(&reader.get_size()), "Testing 123\n");
    reader.close();
    std::fs::remove_dir_all("tests/samples/rar/000-external").unwrap();
}

#[test]
fn sample_001_metadata() {
    let metadata = match *archive::metadata(
        Formats::Rar,
        "tests/samples/rar/001.rar".to_string(),
        true,
        1024,
    )
    .unwrap()
    {
        OriginalArchiveMetadata::Rar(metadata) => metadata,
        _ => panic!("This could never happen, this is only here for type safety"),
    };
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "testfile.txt");
    assert_eq!(metadata.files[0].size, 12);
    assert_eq!(metadata.files[0].uncompressed_size.unwrap(), 12);
}

#[test]
fn sample_001_extract() {
    std::fs::create_dir_all("tests/samples/rar/001-external").unwrap();
    archive::extract(
        Formats::Rar,
        "tests/samples/rar/001.rar".to_string(),
        "tests/samples/rar/001-external".to_string(),
        None,
        None,
        true,
        true,
        1024,
    )
    .unwrap();
    let mut reader = FileReader::new(&"tests/samples/rar/001-external/testfile.txt".to_string());
    assert_eq!(reader.read_utf8(&reader.get_size()), "Testing 123\n");
    reader.close();
    std::fs::remove_dir_all("tests/samples/rar/001-external").unwrap();
}

#[test]
fn sample_002_metadata() {
    let metadata = match *archive::metadata(
        Formats::Rar,
        "tests/samples/rar/002.rar".to_string(),
        true,
        1024,
    )
    .unwrap()
    {
        OriginalArchiveMetadata::Rar(metadata) => metadata,
        _ => panic!("This could never happen, this is only here for type safety"),
    };
    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "testfile.txt");
    assert_eq!(metadata.files[0].size, 12);
    assert_eq!(metadata.files[0].uncompressed_size.unwrap(), 12);
}

#[test]
fn sample_002_extract() {
    std::fs::create_dir_all("tests/samples/rar/002-external").unwrap();
    archive::extract(
        Formats::Rar,
        "tests/samples/rar/002.rar".to_string(),
        "tests/samples/rar/002-external".to_string(),
        None,
        None,
        true,
        true,
        1024,
    )
    .unwrap();
    let mut reader = FileReader::new(&"tests/samples/rar/002-external/testfile.txt".to_string());
    assert_eq!(reader.read_utf8(&reader.get_size()), "Testing 123\n");
    reader.close();
    std::fs::remove_dir_all("tests/samples/rar/002-external").unwrap();
}

// Sample 003 is compressed and therefore not supported yet
