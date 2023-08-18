use std::{fs, path::Path};

use anyhow::Ok;

pub struct FileInfo {
    pub extension: Option<String>,
    pub size: usize,
}

impl FileInfo {
    pub fn read_from_path(path: &Path) -> anyhow::Result<Self> {
        let extension = path.extension().map(|e| e.to_str().unwrap().to_owned());
        let size = fs::read(path)?.len();

        Ok(Self { extension, size })
    }
}
