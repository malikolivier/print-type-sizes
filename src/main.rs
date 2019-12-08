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
    let options = ops::CompileOptions::new(&config, CompileMode::Build)?;
    let path = env::current_dir()?.join("Cargo.toml");
    let ws = core::Workspace::new(&path, &config)?;
    ops::compile(&ws, &options)?;
    Ok(())
}

struct Type {
    name: String,
    size: usize,
}

fn parse_output<R: Read>(stdout: R) -> Vec<Type> {
    const PRINT_TYPE_SIZE: &str = "print-type-size ";
    const PRINT_TYPE_SIZE_TYPE: &str = "print-type-size type: ";
    use std::cmp::Ordering;

    let mut types = Vec::new();

    let f = io::BufReader::new(stdout);
    for line in f.lines() {
        if let Ok(line) = line {
            if line.starts_with(PRINT_TYPE_SIZE) {
                if line.starts_with(PRINT_TYPE_SIZE_TYPE) {
                    let mut split = line.split("`");
                    if let Some(type_name) = split.nth(1) {
                        if let Some(metadata) = split.next() {
                            let mut space_split = metadata.split(" ");
                            space_split.next();
                            if let Some(size_str) = space_split.next() {
                                if let Ok(size) = size_str.parse::<usize>() {
                                    types.push(Type {
                                        name: type_name.to_owned(),
                                        size,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    types.sort_by(|t1, t2| match t2.size.cmp(&t1.size) {
        Ordering::Equal => t1.name.cmp(&t2.name),
        ord => ord,
    });

    types
}

fn main() -> Result<(), Box<dyn error::Error>> {
    set_rust_flags();

    let types = {
        let mut stdout = gag::BufferRedirect::stdout()?;
        compile()?;
        parse_output(&mut stdout)
    };

    for t in types {
        println!("{}\t{}", t.name, t.size);
    }

    Ok(())
}
