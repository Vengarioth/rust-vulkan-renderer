use std::fs::File;
use crate::{Error, assets::{Asset}};
use memmap;
use blake3;
use tinypath::Path;

pub fn create_task(absolute_path: Path, base_path: &Path, content_size: u64) -> Result<Task, Error> {
    let content_path = absolute_path.relative_from(base_path);

    let is_asset = if let Some(extension) = content_path.extension() {
        extension == "asset"
    } else {
        false
    };

    let mut hasher = blake3::Hasher::new();
    hasher.update(&content_path.to_string().as_bytes());
    let asset = if content_size > 0 {
        let file_path: std::path::PathBuf = absolute_path.clone().into();
        let file = File::open(&file_path)?;
        let map = unsafe { memmap::MmapOptions::new().map(&file)? };
        hasher.update(&map);

        if is_asset {
            Some(Asset::from_contents(&map)?)
        } else {
            None
        }
    } else {
        None
    };

    let content_hash = hasher.finalize();

    Ok(Task::new(absolute_path, asset, content_path, content_size, content_hash))
}

#[derive(Debug)]
pub struct Task {
    absolute_path: Path,
    asset: Option<Asset>,
    content_path: Path,
    content_size: u64,
    content_hash: blake3::Hash,
}

impl Task {
    pub fn new(absolute_path: Path, asset: Option<Asset>, content_path: Path, content_size: u64, content_hash: blake3::Hash) -> Self {
        Self {
            absolute_path,
            asset,
            content_path,
            content_size,
            content_hash,
        }
    }

    pub fn get_absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    pub fn get_content_path(&self) -> &Path {
        &self.content_path
    }

    pub fn is_asset(&self) -> bool {
        self.asset.is_some()
    }

    pub fn get_asset(&self) -> Option<&Asset> {
        if let Some(ref asset) = self.asset {
            Some(asset)
        } else {
            None
        }
    }
}
