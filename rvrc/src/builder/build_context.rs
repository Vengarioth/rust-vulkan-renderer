use memmap::{MmapOptions, Mmap};
use tinypath::Path;
use std::fs::File;
use crate::Error;

#[derive(Debug)]
pub struct BuildContext {
    asset_path: Path,
}

impl BuildContext {
    pub fn new(asset_path: Path) -> Self {
        Self {
            asset_path,
        }
    }

    pub fn load(&self, path: &Path) -> Result<Mmap, Error> {
        let path = path.relative_to(&self.asset_path);

        let file_path: std::path::PathBuf = path.into();

        let file = File::open(&file_path)?;
        let map = unsafe { MmapOptions::new().map(&file)? };

        Ok(map)
    }

    pub fn get_full_path(&self, path: &Path) -> Path {
        path.relative_to(&self.asset_path)
    }
}
