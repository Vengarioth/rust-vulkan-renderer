use crate::Error;
use rvr_assets::{shader::ShaderAsset, AssetType};
use tinypath::Path;
use std::io::prelude::*;
use std::fs::OpenOptions;

const MAX_FILE_SIZE: usize = 4294967295;

#[derive(Debug)]
struct BundleEntry {
    address: String,
    asset_type: AssetType,
    data: Vec<u8>,
}

struct Bucket {
    offset: usize,
    file: std::fs::File,
}

impl Bucket {
    pub fn new(file: std::fs::File) -> Self {
        Self { offset: 0, file }
    }

    pub fn get_current_offset(&self) -> usize {
        self.offset
    }

    pub fn can_fit(&self, size: usize) -> bool {
        self.offset + size <= MAX_FILE_SIZE
    }

    pub fn insert(&mut self, data: &[u8]) -> Result<(), Error> {
        if !self.can_fit(data.len()) {
            panic!("TODO create proper error");
        }

        self.file.write_all(&data)?;
        self.offset += data.len();

        Ok(())
    }
}

#[derive(Debug)]
pub struct BundleBuilder {
    entries: Vec<BundleEntry>,
}

impl BundleBuilder {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_shader(&mut self, address: String, shader_asset: ShaderAsset) -> Result<(), Error> {
        let asset_type = AssetType::Shader;

        let data = bincode::serialize(&shader_asset)?;

        self.entries.push(BundleEntry {
            address,
            asset_type,
            data,
        });

        Ok(())
    }

    pub fn build(self, output_dir: Path) -> Result<(), Error> {
        let mut files = Vec::new();
        let mut locations = Vec::new();

        let mut buckets: Vec<Bucket> = Vec::new();
        for entry in self.entries {
            if let Some((index, bucket)) = buckets
                .iter_mut()
                .enumerate()
                .find(|(_, bucket)| bucket.can_fit(entry.data.len()))
            {
                let offset = bucket.get_current_offset();
                bucket.insert(&entry.data)?;

                let location = rvr_assets::AssetLocation::new(
                    entry.address,
                    entry.asset_type,
                    index,
                    offset,
                    entry.data.len(),
                );
                locations.push(location);


            } else {
                let id = buckets.len();
                let file_name = format!("{}.data", id);
                let mut path = output_dir.clone();
                files.push(file_name.clone());
                path.push(file_name);


                let path: std::path::PathBuf = path.into();

                let file = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(path)?;

                let mut bucket = Bucket::new(file);

                let index = buckets.len();
                let offset = bucket.get_current_offset();
                bucket.insert(&entry.data)?;

                let location = rvr_assets::AssetLocation::new(
                    entry.address,
                    entry.asset_type,
                    index,
                    offset,
                    entry.data.len(),
                );
                locations.push(location);

                buckets.push(bucket);
            }
        }

        
        let index = rvr_assets::AssetIndex::new(files, locations);
        let index_data = bincode::serialize(&index)?;
        dbg!(index);

        let mut path = output_dir.clone();
        path.push("index.db");
        let path: std::path::PathBuf = path.into();
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        file.write_all(&index_data)?;

        Ok(())
    }
}
