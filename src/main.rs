use crate::compile::compile;
use parse::parse;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate lazy_static;

mod ast;
mod compile;
mod datapack;
mod error;
mod parse;
mod tree;

fn main() {
    let parsed = parse(
        "
        mcfunction tick() {
            int a;
            a = 1;
            int b = 1 + (a + 2) * 3 + 4;
}
",
    )
    .unwrap();
    println!("{:?}", parsed);
    let compiled = compile(&parsed);
    println!(
        "{}",
        compiled
            .unwrap()
            .mc_namespace
            .functions
            .get("tick")
            .unwrap()
    );
}
