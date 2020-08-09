use crate::{
    Error,
    threading::Worker,
};

#[derive(Debug)]
pub enum LoadingTask {
    LoadShader,
}

#[derive(Debug)]
pub enum LoadingResult {
    ShaderLoaded,
}

pub struct LoadingWorker;

impl LoadingWorker {
    pub fn new() -> Self {
        Self
    }
}

impl Worker for LoadingWorker {
    type Task = LoadingTask;
    type TaskResult = LoadingResult;

    fn execute(&mut self, task: Self::Task) -> Result<Self::TaskResult, Error> {
        match task {
            LoadingTask::LoadShader => {
                Ok(LoadingResult::ShaderLoaded)
            },
        }
    }
}
