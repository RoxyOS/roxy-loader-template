use anyhow::{Ok, Result};
use clap::Parser;
use workspace_root::get_workspace_root;
use xshell::Shell;

use crate::{
    command::{Args, Command},
    run::run,
};

pub mod command;
pub mod run;
pub mod run_vm;
pub mod utils;

mod private {
    pub use xshell;
}

fn main() -> Result<()> {
    chdir_to_workspace_root()?;

    let args = Args::parse();

    match args.command {
        Command::Run => run()?,
    }

    Ok(())
}

fn chdir_to_workspace_root() -> Result<()> {
    let workspace_root = get_workspace_root();
    Shell::new()?.change_dir(workspace_root);
    Ok(())
}
