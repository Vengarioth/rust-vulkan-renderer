use std::path::PathBuf;
use structopt::*;

#[derive(Debug, StructOpt)]
#[structopt(about = "RVR CLI")]
pub enum Command {
    Build {
        #[structopt(parse(from_os_str), help = "the path to the workspace you want to build")]
        workspace: PathBuf,

        #[structopt(parse(from_os_str), help = "the path to the output directory")]
        output: PathBuf,
        
        #[structopt(short = "w", long = "watch", help = "watches the workspace for changes and updates assets accordingly")]
        watch: bool,
    },
}

impl Command {
    pub fn parse() -> Self {
        Self::from_args()
    }
}
