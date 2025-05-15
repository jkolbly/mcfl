use std::collections::{HashMap, HashSet};

use rand::distributions::{Alphanumeric, DistString};

use crate::{
    ast::{ASTNode, ASTNodeType, VarType, AST}, datapack::DataPack, error::CompileError, tree::{NodeId, Tree}
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

/// Fill the symbol tables for an AST
fn name_analysis(ast: &mut AST) -> Result<(), CompileError> {
    ast.variables = Vec::new();

    fn analyze(ast: &mut AST, node: NodeId) -> Result<(), CompileError> {
        match &ast.tree.get_node(node)?.node_type {
            ASTNodeType::Program => todo!(),
            ASTNodeType::Function { name, params, return_type } => todo!(),
            ASTNodeType::MCFunction { name } => todo!(),
            ASTNodeType::Block => todo!(),
            ASTNodeType::VariableDeclaration { declaration } => todo!(),
            ASTNodeType::Assignment => todo!(),
            ASTNodeType::Identifier { id } => todo!(),
            ASTNodeType::NumberLiteral { value } => todo!(),
            ASTNodeType::Add => todo!(),
            ASTNodeType::Subtract => todo!(),
            ASTNodeType::Multiply => todo!(),
            ASTNodeType::Divide => todo!(),
            ASTNodeType::Modulo => todo!(),
            ASTNodeType::ReturnStatement => todo!(),
            ASTNodeType::FunctionCall { id } => todo!(),
        }

        Ok(())
    }

    analyze(ast, ast.tree.get_root()?)
}

/// Generate a datapack from an AST (abstract syntax tree)
pub fn compile(mut ast: AST) -> Result<DataPack, CompileError> {
    name_analysis(&mut ast)?;

    println!("{:?}", ast);

    todo!()
}
