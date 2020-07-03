mod error;
mod command;
mod tasks;
mod workspace;
mod dependency_graph;
pub mod assets;

use dependency_graph::*;
use error::*;
use command::*;
use tasks::*;
use workspace::*;
use tinypath::Path;

fn main() {
    let command = Command::parse();
    match command {
        Command::Build { workspace, output, watch } => {
            let base_path = Path::from_current_dir().unwrap();
            let workspace_path = Path::from_std_path(&workspace).unwrap().relative_to(&base_path);
            let output_path = Path::from_std_path(&output).unwrap().relative_to(&base_path);

            let mut workspace = Workspace::new(workspace_path, output_path);

            if watch {
                workspace.watch();
            } else {
                workspace.walk();
            }
        },
    }
}
