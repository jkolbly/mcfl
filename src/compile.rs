use crate::{ast::ASTNode, error::CompileError, ir::IR, tree::Tree};

/// Generate an IR (intermediate representation) from an AST (abstract syntax tree)
pub fn compile(ast: &Tree<ASTNode>) -> Result<IR, CompileError> {
    let mut ir = IR::new();

    Ok(ir)
}
