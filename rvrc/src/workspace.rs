use ignore::Walk;
use tinypath::Path;
use crate::{
    Error,
    create_task,
    Task,
    DependencyGraph,
    assets::Asset,
};

fn wrap(result: Result<(), Error>) {
    match result {
        Ok(()) => {},
        Err(error) => {
            println!("{}", error);
        },
    }
}

#[derive(Debug)]
pub struct Workspace {
    path: Path,
    output_path: Path,
    dependencies: DependencyGraph,
}

impl Workspace {
    pub fn new(path: Path, output_path: Path) -> Self {
        Self {
            path,
            output_path,
            dependencies: DependencyGraph::new(),
        }
    }

    fn process_task(&mut self, task: Task) -> Result<(), Error> {
        self.dependencies.insert_asset(task.get_content_path());

        if let Some(asset) = task.get_asset() {
            match asset {
                Asset::Shader(shader_asset) => {
                    let dependencies = shader_asset.get_dependencies(task.get_absolute_path())?;

                    for dependency in dependencies.iter() {
                        let path: std::path::PathBuf = dependency.clone().into();
                        if !path.exists() {
                            return Err(anyhow::anyhow!("Missing dependency file {}", dependency.to_string()));
                        }
                    }

                    for dependency in dependencies.iter() {
                        let content_path = dependency.relative_from(&self.path);
                        self.dependencies.add_dependency(task.get_content_path(), &content_path);
                    }
                },
            }
        }

        Ok(())
    }

    pub fn watch(&mut self) {
        use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
        use std::sync::mpsc::channel;
        use std::time::Duration;
        use std::fs;

        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(250)).unwrap();

        let watch_path: std::path::PathBuf = self.path.clone().into();
        watcher.watch(&watch_path, RecursiveMode::Recursive).unwrap();

        self.walk();
        self.dependencies.print_dot();

        loop {
            match rx.recv() {
                Ok(event) => {
                    match event {
                        DebouncedEvent::Write(path) => {
                            if path.is_dir() {
                                continue;
                            }

                            let metadata = fs::metadata(&path).unwrap();
                            let task = create_task(Path::from_std_path(&path).unwrap(), &self.path, metadata.len()).unwrap();
                            wrap(self.process_task(task));
                            self.dependencies.print_dot();
                        },
                        DebouncedEvent::Create(path) => {
                            if path.is_dir() {
                                continue;
                            }

                            let metadata = fs::metadata(&path).unwrap();
                            let task = create_task(Path::from_std_path(&path).unwrap(), &self.path, metadata.len()).unwrap();
                            wrap(self.process_task(task));
                            self.dependencies.print_dot();
                        },
                        DebouncedEvent::Rename(from, to) => {
                            if from.is_dir() || to.is_dir() {
                                continue;
                            }

                            let content_path = Path::from_std_path(&from).unwrap().relative_from(&self.path);
                            self.dependencies.remove_asset(content_path);

                            let metadata = fs::metadata(&to).unwrap();
                            let task = create_task(Path::from_std_path(&to).unwrap(), &self.path, metadata.len()).unwrap();
                            wrap(self.process_task(task));
                            self.dependencies.print_dot();
                        },
                        DebouncedEvent::Remove(path) => {
                            if path.is_dir() {
                                continue;
                            }

                            let content_path = Path::from_std_path(&path).unwrap().relative_from(&self.path);

                            self.dependencies.remove_asset(content_path);
                            self.dependencies.print_dot();
                        }
                        _ => {},
                    }
                },
                Err(error) => {
                    dbg!(error);
                },
            }
        }
    }

    pub fn walk(&mut self) {
        let walk_path: std::path::PathBuf = self.path.clone().into();
        for result in Walk::new(walk_path) {
            match result {
                Ok(entry) => {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            continue;
                        }

                        let task = create_task(Path::from_std_path(entry.path().into()).unwrap(), &self.path, metadata.len()).unwrap();
                        wrap(self.process_task(task));
                    }

                },
                Err(error) => {
                    dbg!(error);
                },
            }
        }
    }
}
