use core::panic;
use std::{fs::File, io::Read, path::Path};

use compile::compile;
use datapack::DataPack;
use error::CompileError;
use parse::parse;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate lazy_static;
extern crate rand;

mod ast;
mod compile;
mod datapack;
mod error;
mod id_tracker;
mod mcfunction;
mod mir;
mod parse;
mod tree;

/// TODO:
/// - Remove 'scoreboard objectives add mcfl_ints dummy' from non-startup functions
/// - Make FunctionVars not have to be cloned all the time. That seems so very awful
/// - Make it possible to save state

fn main() {
    let compiled = compile_file("examples/test.mcfl").unwrap();
}

fn compile_file(file_path: &str) -> Result<DataPack, CompileError> {
    let path = Path::new(file_path);
    let path_display = path.display();

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't read file {}: {}", path_display, err),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {}
        Err(err) => panic!("Couldn't read file to string {}: {}", path_display, err),
    };

    compile_string(&s)
}

fn compile_string(toparse: &str) -> Result<DataPack, CompileError> {
    let parsed = parse(toparse)?;
    let compiled = compile(&parsed)?;
    Ok(compiled)
}
