use std::collections::HashMap;

use rand::distributions::{Alphanumeric, DistString};

use crate::{
    ast::{ASTNode, ASTNodeType},
    error::CompileError,
    ir::IR,
    mir::{MIRNode, MIRNodeType, Variable},
    tree::{NodeId, Tree},
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

/// Create a MIR (MCFL intermediate representation) from an AST (abstract syntax tree)
fn generate_mir(ast: &Tree<ASTNode>) -> Result<Tree<MIRNode>, CompileError> {
    let mut mir: Tree<MIRNode> = Tree::new();

    get_mir_node(ast, ast.get_root()?, &mut mir, None)?;

    Ok(mir)
}

/// Recursively generate the MIR node and descendents for a given AST node and add them to the given MIR tree
fn get_mir_node(
    ast: &Tree<ASTNode>,
    node: NodeId,
    mir: &mut Tree<MIRNode>,
    mirnode: Option<NodeId>,
) -> Result<(), CompileError> {
    let astnode = ast.get_node(node)?;

    let node_type: Option<MIRNodeType> = match &astnode.node_type {
        ASTNodeType::Program => Some(MIRNodeType::Program),
        ASTNodeType::Function {
            name,
            params,
            return_type,
        } => {
            let var_params = params
                .iter()
                .map(|v| Variable {
                    name: v.name.clone(),
                    var_type: v.var_type,
                })
                .collect();

            let return_var = return_type.map(|var_type| Variable {
                name: random_name(),
                var_type,
            });

            Some(MIRNodeType::Function {
                name: name.into(),
                params: var_params,
                return_var,
            })
        }
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
    };

    if let Some(ntype) = node_type {
        let new_node = MIRNode::new(ntype, astnode.context.clone());
        let new_id = mir.new_node(new_node);

        if let Some(nid) = mirnode {
            mir.append_to(nid, new_id)?;
        }

        for c in ast.get_children(node)? {
            get_mir_node(ast, *c, mir, Some(new_id))?;
        }
    }

    Ok(())
}

/// Generate an IR (intermediate representation) from an AST (abstract syntax tree)
pub fn compile(ast: &Tree<ASTNode>) -> Result<IR, CompileError> {
    let mut ir = IR::new();

    let funcs = get_mcfuncs(ast)?;

    for id in funcs.values() {
        compile_mcfunc(&mut ir, ast, *id)?;
    }

    Ok(ir)
}

/// Compile a single mcfunction and add it to the IR.
fn compile_mcfunc(ir: &mut IR, ast: &Tree<ASTNode>, mcfunc: NodeId) -> Result<(), CompileError> {
    todo!()
}

/// Compile a single block and add it to the ir as one or multiple IRFuncs.
fn compile_block(ir: &mut IR, ast: &Tree<ASTNode>, block: NodeId) -> Result<(), CompileError> {
    todo!()
}

/// Find all the mcfunctions that must be compiled in an AST. Resulting HashMap maps mfunction names (as they appear in MCFL source) to node ID's.
///
/// Note that other mcfunction files will also be generated by the compilation process, but each of these is guaranteed its own file.
fn get_mcfuncs(ast: &Tree<ASTNode>) -> Result<HashMap<String, NodeId>, CompileError> {
    let mut map = HashMap::new();

    for child in ast.get_children(ast.get_root()?)? {
        if let ASTNodeType::MCFunction { name } = &ast.get_node(*child)?.node_type {
            map.insert(name.to_string(), *child);
        }
    }

    Ok(map)
}
