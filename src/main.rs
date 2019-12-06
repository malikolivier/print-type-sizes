extern crate cargo;

use std::env;
use std::error;

use cargo::core;
use cargo::core::compiler::CompileMode;
use cargo::ops;
use cargo::util::config::Config;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::default()?;
    let options = ops::CompileOptions::new(&config, CompileMode::Check { test: false })?;
    let path = env::current_dir()?.join("Cargo.toml");
    let ws = core::Workspace::new(&path, &config)?;
    ops::compile(&ws, &options)?;
    Ok(())
}