use corelib::file::FileReader;

#[test]
fn sample_000() {
    let mut file = FileReader::new(&"tests/samples/rar/000.rar".to_string());
    let metadata = corelib::formats::rar::parser::metadata(&mut file);

    println!("{:#?}", metadata);
}
