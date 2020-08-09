mod error;
mod command;
mod tasks;
mod workspace;
mod dependency_graph;
mod assets;
mod builder;
mod bundle_builder;

use dependency_graph::*;
use error::*;
use command::*;
use tasks::*;
use workspace::*;
use bundle_builder::*;

use tinypath::Path;
use colored::Colorize;

fn main() {
    let command = Command::parse();
    match command {
        Command::Build { workspace, output, watch } => {
            let base_path = Path::from_current_dir().unwrap();
            let workspace_path = Path::from_std_path(&workspace).unwrap().relative_to(&base_path);
            let output_path = Path::from_std_path(&output).unwrap().relative_to(&base_path);

            println!("{} {}", "workspace".green(), workspace_path.to_string().yellow());
            println!("{} {}", "output".green(), output_path.to_string().yellow());

            let mut workspace = Workspace::new(workspace_path, output_path);

            if watch {
            println!("{}", "initializing watcher".green());
                workspace.watch();
            } else {
            println!("{}", "building assets".green());
                workspace.walk();
            }
        },
    }
}
