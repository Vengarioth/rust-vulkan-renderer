use crate::{
    Error,
    threading::WorkerThread,
    assets::{
        LoadingWorker,
        LoadingTask,
        LoadingResult,
    },
};

pub struct AssetManager {
    worker: WorkerThread<LoadingTask, LoadingResult>,
}

impl AssetManager {
    pub fn new() -> Self {

        let worker_impl = LoadingWorker::new();
        let worker = WorkerThread::new(Box::new(worker_impl));

        Self {
            worker,
        }
    }

    pub fn load_shader(&mut self) -> Result<(), Error> {
        self.worker.enqueue(LoadingTask::LoadShader)?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Error> {
        if let Some(result) = self.worker.poll() {
            let result = result?;

            dbg!(result);
        }

        Ok(())
    }
}
