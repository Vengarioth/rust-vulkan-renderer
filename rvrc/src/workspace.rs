use ignore::Walk;
use tinypath::Path;
use crate::{
    Error,
    create_task,
    Task,
    DependencyGraph,
    assets::Asset,
};
use colored::Colorize;

fn wrap<T>(result: Result<T, Error>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            println!("{}", error.to_string().red());
            None
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

    fn process_task(&mut self, task: Task, build_dependencies: bool) -> Result<(), Error> {
        println!("{} {}", "Processing".blue(), task.get_absolute_path().to_string().yellow());

        self.dependencies.insert_asset(task.get_content_path());

        if let Some(asset) = task.get_asset() {
            println!("{}", "Building Asset".yellow());
            
            // insert known dependencies
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

            println!("{}", "Asset built".green());
        } else {
            println!("{}", "Querying Dependencies".yellow());

            // find parent asset
            let dependants = self.dependencies.find_dependant_assets(task.get_content_path())?;
            for dependant in dependants {

                if build_dependencies {
                    let absolute_path =  dependant.relative_to(&self.path);
                    let std_path: std::path::PathBuf = absolute_path.clone().into();
                    let metadata = std::fs::metadata(&std_path).unwrap();
    
                    if let Some(task) = wrap(create_task(absolute_path, &self.path, metadata.len())) {
                        self.process_task(task, build_dependencies)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn rebuild(&mut self, path: std::path::PathBuf) -> Result<(), Error> {
        if path.is_dir() {
            return Ok(());
        }

        let metadata = std::fs::metadata(&path)?;
        let task = create_task(Path::from_std_path(&path).unwrap(), &self.path, metadata.len())?;
        self.process_task(task, true)?;

        Ok(())
    }

    pub fn watch(&mut self) {
        use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
        use std::sync::mpsc::channel;
        use std::time::Duration;

        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(250)).unwrap();

        let watch_path: std::path::PathBuf = self.path.clone().into();
        watcher.watch(&watch_path, RecursiveMode::Recursive).unwrap();

        self.walk();

        loop {
            match rx.recv() {
                Ok(event) => {
                    match event {
                        DebouncedEvent::Write(path) => {
                            wrap(self.rebuild(path));
                        },
                        DebouncedEvent::Create(path) => {
                            wrap(self.rebuild(path));
                        },
                        DebouncedEvent::Rename(from, to) => {
                            if from.is_dir() || to.is_dir() {
                                continue;
                            }

                            let content_path = Path::from_std_path(&from).unwrap().relative_from(&self.path);
                            self.dependencies.remove_asset(content_path);

                            wrap(self.rebuild(to));
                        },
                        DebouncedEvent::Remove(path) => {
                            if path.is_dir() {
                                continue;
                            }

                            let content_path = Path::from_std_path(&path).unwrap().relative_from(&self.path);
                            self.dependencies.remove_asset(content_path);
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

                        if let Some(task) = wrap(create_task(Path::from_std_path(entry.path().into()).unwrap(), &self.path, metadata.len())) {
                            // only run asset tasks on build or first run
                            if task.is_asset() {
                                wrap(self.process_task(task, false));
                            }
                        }
                    }

                },
                Err(error) => {
                    dbg!(error);
                },
            }
        }
    }
}
