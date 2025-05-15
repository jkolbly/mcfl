use std::collections::{HashMap, HashSet};

use rand::distributions::{Alphanumeric, DistString};

use crate::{
    ast::{ASTNode, ASTNodeType, VarType}, datapack::DataPack, error::CompileError, mir::{MIRNode, MIRNodeType, Mir, Variable}, tree::{NodeId, Tree}
};

lazy_static::lazy_static! {
    static ref USED_NAMES: Vec<String> = Vec::new();
}

/// Generate a random name to be used for MCFL identifiers
///
/// Names are guaranteed to be unique
fn random_name() -> String {
    let mut name = "".to_string();
    while USED_NAMES.contains(&name) {
        name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    }
    name
}

/// Generate a datapack from an AST (abstract syntax tree)
pub fn compile(ast: &Tree<ASTNode>) -> Result<DataPack, CompileError> {
    println!("{:?}", ast);

    todo!()
}
