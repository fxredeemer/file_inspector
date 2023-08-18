use std::{ffi::OsString, fs, path::Path};

use anyhow::Ok;

use crate::histogram::HistogramCreator;

#[derive(Default)]
pub struct FileInfo {
    pub extension: Option<OsString>,
    pub size: usize,
    pub file_properties: Vec<FileProperties>,
}

pub enum FileProperties {
    TextFile,
    Base64Encoded,
    Compressed(Compression),
}

pub enum Compression {
    GZip,
    SevenZip,
    Zip,
}

pub struct FileInfoReader<'a> {
    histogram_creator: &'a HistogramCreator,
}

impl<'a> FileInfoReader<'a> {
    pub fn new(histogram_creator: &'a HistogramCreator) -> Self {
        Self { histogram_creator }
    }

    pub fn read_file_info(&self, path: &Path) -> anyhow::Result<FileInfo> {
        let extension = path.extension().map(|e| e.to_owned());
        let bytes = fs::read(path)?;
        let size = bytes.len();
        let mut file_properties = vec![];

        if let Some(compression) = self.determine_possible_compression(&bytes) {
            file_properties.push(FileProperties::Compressed(compression));
        }

        Ok(FileInfo {
            extension,
            size,
            file_properties,
        })
    }

    fn determine_possible_compression(&self, bytes: &Vec<u8>) -> Option<Compression> {
        let histogram = &self.histogram_creator.get_file_histogram(bytes);

        if histogram.len() < 255 {
            return None;
        }

        let max_counts: f64 = histogram.iter().map(|d| *d.1).max().unwrap_or(0).into();

        if histogram.iter().any(|v| (*v.1 as f64) < max_counts * 0.8) {
            return None;
        }

        if self.starts_with(bytes, &[0x50_u8, 0x4b_u8, 0x03_u8, 0x04_u8]) {
            return Some(Compression::Zip);
        }
        if self.starts_with(bytes, &[0x1f, 0x8b, 0x08]) {
            return Some(Compression::GZip);
        }

        None
    }

    fn starts_with(&self, bytes: &[u8], magic_numbers: &[u8]) -> bool {
        bytes
            .iter()
            .take(magic_numbers.len())
            .copied()
            .collect::<Vec<_>>()
            == magic_numbers
    }
}
