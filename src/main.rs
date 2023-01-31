use std::path::Path;

use crate::{ast::ASTNode, compile::compile, tree::Tree};
use datapack::DataPack;
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
mod ir;
mod mcfunction;
mod parse;
mod program;
mod tree;

/// TODO:
/// - Remove 'scoreboard objectives add mcfl_ints dummy' from non-startup functions
/// - Make FunctionVars not have to be cloned all the time. That seems so very awful
/// - Make it possible to save state

fn main() {
    let parsed = parse(
        "
        mcfunction startup {
            int f = f; // TODO: This shouldn't compile

            global int counter = 0;
            int seconds = 0;

            int added = add(99, 101);
        }

        mcfunction tick {
            counter = counter + 1; // TODO: This compiles to something stupid...
            int seconds = counter / 20;
        }
        
        function add(int a_arg, int b_arg) -> int {
            return a_arg + b_arg;
        }",
    )
    .unwrap();
    println!("{:?}", parsed);
    let compiled = compile(&parsed).unwrap();
    // println!("{}", compiled.unwrap());
    DataPack::from(compiled).save(Path::new("C:\\Program Files\\MultiMC\\instances\\1.13.2\\.minecraft\\saves\\MCFL Playground\\datapacks")).unwrap();
}
