use crate::{
    Error,
    threading::Worker,
    graphics::TransferDevice,
};
use std::{io::SeekFrom, fs::File, io::prelude::*};
use rvr_assets::shader::ShaderAsset;
use bincode;
use serde::de::DeserializeOwned;

fn read_from_file<T: DeserializeOwned>(path: &str, offset: usize, length: usize) -> Result<T, Error> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(offset as u64))?;
    let mut buffer = vec![0u8; length];
    file.read_exact(&mut buffer)?;

    let result = bincode::deserialize(&buffer)?;

    Ok(result)
}

#[derive(Debug)]
pub enum LoadingTask {
    LoadShader {
        path: String,
        offset: usize,
        length: usize,
    },
}

#[derive(Debug)]
pub enum LoadingResult {
    ShaderLoaded(()),
}

pub struct LoadingWorker {
    transfer_device: TransferDevice,
}

impl LoadingWorker {
    pub fn new(transfer_device: TransferDevice) -> Self {
        Self {
            transfer_device,
        }
    }
}

impl Worker for LoadingWorker {
    type Task = LoadingTask;
    type TaskResult = LoadingResult;

    fn execute(&mut self, task: Self::Task) -> Result<Self::TaskResult, Error> {
        match task {
            LoadingTask::LoadShader { path, offset, length, } => {
                let shader_asset = read_from_file(&path, offset, length)?;
                let pipeline = self.transfer_device.create_pipeline(&shader_asset)?;
                Ok(LoadingResult::ShaderLoaded(pipeline))
            },
        }
    }
}
