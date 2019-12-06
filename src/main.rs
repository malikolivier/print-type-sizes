extern crate cargo;
extern crate gag;

use std::env;
use std::error;
use std::io::{self, BufRead, Read};

use cargo::core;
use cargo::core::compiler::CompileMode;
use cargo::ops;
use cargo::util::config::Config;

fn set_rust_flags() {
    let rust_flags = if let Ok(flags) = env::var("RUSTFLAGS") {
        flags
    } else {
        String::new()
    };
    env::set_var("RUSTFLAGS", format!("{} -Z print-type-sizes", rust_flags));
}

fn compile() -> Result<(), Box<dyn error::Error>> {
    let config = Config::default()?;
    let options = ops::CompileOptions::new(&config, CompileMode::Check { test: false })?;
    let path = env::current_dir()?.join("Cargo.toml");
    let ws = core::Workspace::new(&path, &config)?;
    ops::compile(&ws, &options)?;
    Ok(())
}

fn parse_output<R: Read>(stdout: R) {
    let f = io::BufReader::new(stdout);
    for line in f.lines() {
        eprintln!("{:?}", line);
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    set_rust_flags();

    {
        let mut stdout = gag::BufferRedirect::stdout()?;
        compile()?;
        parse_output(&mut stdout);
    }

    Ok(())
}
