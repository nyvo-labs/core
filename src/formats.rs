use crate::archive::ArchiveMetadata;

pub mod rar;
pub mod zip;

pub enum Formats {
    Zip,
    Rar,
}

pub fn from_string(format: &str) -> Formats {
    match format {
        "zip" => Formats::Zip,
        _ => panic!("Unsupported format"),
    }
}

pub fn to_string(format: &Formats) -> String {
    match format {
        Formats::Zip => "zip".to_string(),
        Formats::Rar => "rar".to_string(),
    }
}

pub enum FormatMetadata<'a> {
    Zip(zip::ZipArchiveMetadata<'a>),
}

pub fn to_format_metadata<'a>(
    format: Formats,
    metadata: &'a dyn ArchiveMetadata<'a>,
) -> FormatMetadata<'a> {
    match format {
        Formats::Zip => FormatMetadata::Zip(zip::to_zip_archive_metadata(metadata)),
        Formats::Rar => todo!(),
    }
}
