use crate::{
    ast::{ASTNode, ASTNodeType, FunctionType},
    datapack::{DataPack, MCFunction},
    error::CompileError,
    tree::{NodeId, Tree},
};

pub fn compile(tree: &Tree<ASTNode>) -> Result<DataPack, CompileError> {
    verify_ast(tree)?;

    let mut dp = DataPack::new();

    if let Some(tick) = get_function(tree, "tick")? {
        compile_function(tree, tick, &mut dp)?;
    }

    Ok(dp)
}

/// Compile a single function
pub fn compile_function(
    tree: &Tree<ASTNode>,
    function: NodeId,
    datapack: &mut DataPack,
) -> Result<(), CompileError> {
    let func_node = tree.get_node(function)?;

    match &func_node.node_type {
        ASTNodeType::Function {
            func_type,
            name,
            params,
            return_type,
        } => {
            if !matches!(func_type, FunctionType::MCFunction) {
                return Err(CompileError::CompilingNonMCFunction {});
            }

            let compiled_func = MCFunction::new(name);

            for line in tree.get_children(function)? {
                let n = tree.get_node(*line)?;
                match &n.node_type {
                    ASTNodeType::VariableDeclaration { declaration } => todo!(),
                    ASTNodeType::Assignment => todo!(),
                    _ => unreachable!(),
                }
            }

            Ok(())
        }
        _ => Err(CompileError::CompilingNonFunction {}),
    }
}

fn get_function(tree: &Tree<ASTNode>, func_name: &str) -> Result<Option<NodeId>, CompileError> {
    Ok(
        tree.find_child(tree.get_root()?, &|_id, node| match &node.node_type {
            ASTNodeType::Function {
                func_type: _,
                name,
                params: _,
                return_type: _,
            } => name == func_name,
            _ => unreachable!(),
        })?,
    )
}

fn verify_ast(tree: &Tree<ASTNode>) -> Result<(), CompileError> {
    let tick = get_function(tree, "tick")?;
    let startup = get_function(tree, "startup")?;

    if tick.is_none() && startup.is_none() {
        return Err(CompileError::NoEntryPoint {});
    }

    if let Some(tick) = tick {
        if let ASTNodeType::Function {
            func_type,
            name: _,
            params,
            return_type: _,
        } = &tree.get_node(tick)?.node_type
        {
            if !params.is_empty() {
                return Err(CompileError::TickParams {});
            }

            if !matches!(func_type, FunctionType::MCFunction) {
                return Err(CompileError::TickNotMCFunction {});
            }
        }
    }

    if tree.child_exists(tree.get_root()?, &|_id, node| match &node.node_type {
        ASTNodeType::Function {
            func_type,
            name: _,
            params: _,
            return_type,
        } => matches!(func_type, FunctionType::MCFunction) && return_type.is_some(),
        _ => unreachable!(),
    })? {
        return Err(CompileError::MCFunctionReturnType {});
    }

    Ok(())
}
