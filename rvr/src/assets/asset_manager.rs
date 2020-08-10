use crate::{
    Error,
    threading::WorkerThread,
    graphics::{
        Renderer,
        TransferDevice,
    },
    assets::{
        LoadingWorker,
        LoadingTask,
        LoadingResult,
        AssetIndex,
        AssetError,
    },
    impl_generational_list,
    impl_secondary_list,
};
use rvr_assets::AssetType;

impl_generational_list!(ShaderList<Shader, ShaderData>);

pub struct ShaderData;

pub struct AssetManager {
    index: AssetIndex,
    worker: WorkerThread<LoadingTask, LoadingResult>,
    shader: ShaderList,
}

impl AssetManager {
    pub fn new(index: AssetIndex, transfer_device: TransferDevice) -> Self {
        let worker_impl = LoadingWorker::new(transfer_device);
        let worker = WorkerThread::new(Box::new(worker_impl));

        Self {
            index,
            worker,
            shader: ShaderList::new(),
        }
    }

    pub fn load_shader(&mut self, name: &str) -> Result<Shader, Error> {
        if let Some(location) = self.index.find_location(name) {
            if location.asset_type != AssetType::Shader {
                return Err(AssetError::AssetHasWrongType(AssetType::Shader, location.asset_type).into());
            }

            self.worker.enqueue(LoadingTask::LoadShader {
                path: location.path.to_string(),
                offset: location.offset,
                length: location.length,
            })?;
            let shader = Shader::new(0, 0);

            Ok(shader)
        } else {
            Err(AssetError::ShaderNotFound(name.to_string()).into())
        }
    }

    pub fn is_shader_resident(&self, shader: Shader) -> Result<bool, Error> {
        Ok(true)
    }

    pub fn update(&mut self, renderer: &mut Renderer) -> Result<(), Error> {
        if let Some(result) = self.worker.poll() {
            let result = result?;

            match result {
                LoadingResult::ShaderLoaded(shader_asset) => {
                    dbg!(shader_asset);
                }
            }
        }

        Ok(())
    }
}
