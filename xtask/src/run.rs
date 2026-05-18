use std::path::PathBuf;

use anyhow::{Ok, Result};
use roxy_loader_utils::build_image::build_image;

use crate::{run_command, run_vm::run_vm, utils::cargo_target_dir};

pub fn run() -> Result<()> {
    run_command!("cargo build --target x86_64-unknown-none");
    build_image(
        cargo_target_dir()?
            .join("x86_64-unknown-none")
            .join("debug")
            .join("roxy-loader-template"),
    )?;
    run_vm(cargo_target_dir()?.join("image.img"))?;
    Ok(())
}
