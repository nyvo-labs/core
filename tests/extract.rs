use acridotheres::{archive::extract, formats::Format};
use dh::recommended::*;
use std::path::Path;

#[test]
fn zip_001() {
    let output = Path::new("tests/output/001-zip");

    extract::extract_all(
        Path::new("tests/samples/001.zip"),
        output,
        Format::Zip,
        1024,
    )
    .unwrap();

    assert!(output.join("test.txt").exists());
    assert!(output.join("test2.txt").exists());

    let mut test_txt = dh::file::open_r(output.join("test.txt")).unwrap();
    let mut test2_txt = dh::file::open_r(output.join("test2.txt")).unwrap();

    assert_eq!(test_txt.read_utf8_at(0, 14).unwrap(), "Hello, world!\n");
    assert_eq!(test2_txt.read_utf8_at(0, 16).unwrap(), "Hello, world! 2\n");

    test_txt.close().unwrap();
    test2_txt.close().unwrap();

    std::fs::remove_dir_all(output).unwrap();
}

#[test]
fn umsbt_000() {
    let output = Path::new("tests/output/000-umsbt");

    extract::extract_all(
        Path::new("tests/samples/000.umsbt"),
        output,
        Format::Umsbt,
        1024,
    )
    .unwrap();

    assert!(output.join("00000000.msbt").exists());
    assert!(output.join("00000001.msbt").exists());
    assert!(output.join("00000002.msbt").exists());
    assert!(output.join("00000003.msbt").exists());
    assert!(output.join("00000004.msbt").exists());

    std::fs::remove_dir_all(output).unwrap();
}

#[test]
fn msbt_000() {
    let output = Path::new("tests/output/000-msbt");

    extract::extract_all(
        Path::new("tests/samples/000.msbt"),
        output,
        Format::Msbt,
        1024,
    )
    .unwrap();

    assert!(output.join("001.txt").exists());
    assert!(output.join("002.txt").exists());
    assert!(output.join("003.txt").exists());
    assert!(output.join("004.txt").exists());
    assert!(output.join("005.txt").exists());
    assert!(output.join("006.txt").exists());
    assert!(output.join("007.txt").exists());
    assert!(output.join("008.txt").exists());
    assert!(output.join("009.txt").exists());
    assert!(output.join("010.txt").exists());
    assert!(output.join("011.txt").exists());
    assert!(output.join("012.txt").exists());
    assert!(output.join("013.txt").exists());
    assert!(output.join("014.txt").exists());
    assert!(output.join("015.txt").exists());
    assert!(output.join("016.txt").exists());

    std::fs::remove_dir_all(output).unwrap();
}
