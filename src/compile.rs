use rand::distributions::{Alphanumeric, DistString};

use crate::{
    ast::{ASTNode, ASTNodeType, VarType},
    error::CompileError,
    ir::IR,
    mir::{FunctionID, MIRNode, MIRNodeType, Variable},
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

    get_mir_node(ast, ast.get_root()?, &mut mir, None, &mut vec![])?;

    Ok(mir)
}

/// Find the a variable matching a particular name from a stack of scopes, or None if no such variable exists
fn find_in_scope(scopes: &Vec<Vec<Variable>>, name: &str) -> Option<Variable> {
    for scope in scopes {
        for var in scope {
            if var.name == name {
                return Some(var.clone());
            }
        }
    }
    None
}

/// Recursively generate the MIR node and descendents for a given AST node and add them to the given MIR tree.
fn get_mir_node(
    ast: &Tree<ASTNode>,
    node: NodeId,
    mir: &mut Tree<MIRNode>,
    mirnode: Option<NodeId>,
    scopes: &mut Vec<Vec<Variable>>,
) -> Result<(), CompileError> {
    let astnode = ast.get_node(node)?;

    // The current scope is the topmost in the stack.
    // If there are no scopes, create a new empty one (we must be at the top of the tree).
    if scopes.is_empty() {
        scopes.push(Vec::new());
    }
    let scope_index = scopes.len() - 1;

    // Tracks whether we've added a new scope that we need to pop from the stack at the end.
    let mut scope_added: bool = false;

    let node_type: Option<MIRNodeType> = match &astnode.node_type {
        ASTNodeType::Program => Some(MIRNodeType::Program),
        ASTNodeType::Function {
            name,
            params,
            return_type,
        } => {
            // Functions create scope
            let mut new_scope: Vec<Variable> = Vec::new();
            scope_added = true;

            let var_params: Vec<Variable> = params
                .iter()
                .map(|v| Variable {
                    name: v.name.clone(),
                    name_internal: v.name.clone(),
                    var_type: v.var_type,
                })
                .collect();
            new_scope.append(&mut var_params.clone());

            let return_var = return_type.map(|var_type| Variable {
                name: "".to_string(),
                name_internal: random_name(),
                var_type,
            });
            if let Some(ret) = return_var.clone() {
                new_scope.push(ret);
            }

            // Add the new scope to the stack
            scopes.push(new_scope.to_vec());

            Some(MIRNodeType::Function {
                name: name.into(),
                params: var_params,
                return_var,
                is_recursive: false, // Will be verified later
            })
        }
        ASTNodeType::MCFunction { name } => {
            // Functions create scope
            let new_scope: Vec<Variable> = Vec::new();
            scopes.push(new_scope.to_vec());
            scope_added = true;

            Some(MIRNodeType::Function {
                name: name.into(),
                params: Vec::new(),
                return_var: None,
                is_recursive: false, // Will be verified later
            })
        }
        ASTNodeType::Block => None,
        ASTNodeType::VariableDeclaration { declaration } => {
            if find_in_scope(scopes, &declaration.name).is_some() {
                return Err(CompileError::VariableAlreadyDeclared {
                    var: declaration.name.to_string(),
                    context: astnode.context.clone(),
                });
            }
            let var = Variable {
                name: declaration.name.clone(),
                name_internal: declaration.name.clone(),
                var_type: declaration.var_type,
            };
            match declaration.scope_modifier {
                crate::ast::ScopeModifier::Default => scopes[scope_index].push(var.clone()),
                crate::ast::ScopeModifier::Global => scopes[0].push(var.clone()),
            };
            Some(MIRNodeType::VarIdentifier { var })
        }
        ASTNodeType::Assignment => Some(MIRNodeType::AssignmentStatement),
        ASTNodeType::Identifier { id } => {
            let var = find_in_scope(scopes, id);
            if let Some(variable) = var {
                Some(MIRNodeType::VarIdentifier { var: variable })
            } else {
                return Err(CompileError::VariableNotDeclared {
                    var_name: id.to_string(),
                    context: astnode.context.clone(),
                });
            }
        }
        ASTNodeType::NumberLiteral { value } => Some(MIRNodeType::NumberLiteral {
            value: value.to_owned(),
        }),
        ASTNodeType::Add => Some(MIRNodeType::Addition),
        ASTNodeType::Subtract => Some(MIRNodeType::Subtraction),
        ASTNodeType::Multiply => Some(MIRNodeType::Multiplication),
        ASTNodeType::Divide => Some(MIRNodeType::Division),
        ASTNodeType::Modulo => Some(MIRNodeType::Modulo),
        ASTNodeType::ReturnStatement => Some(MIRNodeType::ReturnStatement), // Type will be checked for validity later
        ASTNodeType::FunctionCall { id } => Some(MIRNodeType::FunctionCall {
            id: FunctionID {
                name: id.to_string(),
                name_internal: id.to_string(),
            },
        }),
    };

    // Assignment children must be looked through in opposite order so that 'int a = a' doesn't compile
    if let Some(MIRNodeType::AssignmentStatement) = node_type {
        let new_node = MIRNode::new(MIRNodeType::AssignmentStatement, astnode.context.clone());
        let new_id = mir.new_node(new_node);

        if let Some(nid) = mirnode {
            mir.append_to(nid, new_id)?;
        }

        for c in ast.get_children(node)?.iter().rev() {
            get_mir_node(ast, *c, mir, Some(new_id), scopes)?;
        }
    } else if let Some(ntype) = node_type {
        let new_node = MIRNode::new(ntype, astnode.context.clone());
        let new_id = mir.new_node(new_node);

        if let Some(nid) = mirnode {
            mir.append_to(nid, new_id)?;
        }

        for c in ast.get_children(node)? {
            get_mir_node(ast, *c, mir, Some(new_id), scopes)?;
        }
    } else {
        for c in ast.get_children(node)? {
            get_mir_node(ast, *c, mir, mirnode, scopes)?;
        }
    }

    // Pop the new scope if we added one
    if scope_added {
        scopes.pop();
    }

    // Unpop the new scope from the stack if one was created
    Ok(())
}

/// Verify that all return types in a MIR match the function signature.
fn check_return_types(mir: &Tree<MIRNode>) -> Result<(), CompileError> {
    for func_node in mir.get_children(mir.get_root()?)? {
        let MIRNodeType::Function {
            name,
            params,
            return_var,
            is_recursive,
        } = &mir.get_node(*func_node)?.node_type
        else {
            unreachable!()
        };
        let ret_nodes = mir.find_children_recursive(*func_node, &|_, node| -> bool {
            matches!(node.node_type, MIRNodeType::ReturnStatement)
        })?;
        match return_var {
            Some(ret_var) => {
                verify_return_reached(mir, *func_node)?; // Make sure a return value is always reached

                // Check that return types match for all return statements
                for ret_node in ret_nodes {
                    let ret_expr_type = get_expression_type(mir, mir.get_first_child(ret_node)?)?;
                    if ret_expr_type != ret_var.var_type {
                        return Err(CompileError::MismatchedReturnType {
                            func_name: name.to_string(),
                            expected: ret_var.var_type,
                            received: ret_expr_type,
                            context: mir
                                .get_node(mir.get_first_child(ret_node)?)?
                                .context
                                .clone(),
                        });
                    }
                }
            }
            None => {
                for ret_node in ret_nodes {
                    if mir.has_children(ret_node)? {
                        // A child indicates something is being returned
                        return Err(CompileError::ReturnFromVoid {
                            func_name: name.to_string(),
                            context: mir
                                .get_node(mir.get_first_child(ret_node)?)?
                                .context
                                .clone(),
                        });
                    }
                }
            }
        }
    }
    Ok(())
}

/// Verify that a return expression is always reached in a code block
fn verify_return_reached(mir: &Tree<MIRNode>, func_node: NodeId) -> Result<(), CompileError> {
    match is_return_reached(mir, func_node)? {
        true => Ok(()),
        false => {
            let MIRNodeType::Function {
                name,
                params: _,
                return_var: _,
                is_recursive: _,
            } = &mir.get_node(func_node)?.node_type
            else {
                unreachable!()
            };
            Err(CompileError::NoReturnStatement {
                func_name: name.to_string(),
                context: mir.get_node(func_node)?.context.clone(),
            })
        }
    }
}

/// Return whether a return expression is always reached in a code block
fn is_return_reached(mir: &Tree<MIRNode>, block_node: NodeId) -> Result<bool, CompileError> {
    // Algorithm: Start at end and move backwards until you hit a return statement.
    //            If we hit a code block, check if one is guaranteed in the block.

    if !mir.has_children(block_node)? {
        return Ok(false);
    }

    let mut cur_node = mir.get_last_child(block_node).unwrap();
    loop {
        match &mir.get_node(cur_node)?.node_type {
            MIRNodeType::Program => unreachable!(),
            MIRNodeType::Function {
                name: _,
                params: _,
                return_var: _,
                is_recursive: _,
            } => unreachable!(),
            MIRNodeType::ReturnStatement => return Ok(true),
            _ => {}
        }

        match mir.get_prev_sibling(cur_node)? {
            Some(node_id) => cur_node = node_id,
            None => break,
        }
    }

    Ok(false)
}

fn get_expression_type(mir: &Tree<MIRNode>, expr_node: NodeId) -> Result<VarType, CompileError> {
    match &mir.get_node(expr_node)?.node_type {
        MIRNodeType::VarIdentifier { var } => Ok(var.var_type),
        MIRNodeType::Addition
        | MIRNodeType::Subtraction
        | MIRNodeType::Multiplication
        | MIRNodeType::Division
        | MIRNodeType::Modulo => get_expression_type(mir, mir.get_first_child(expr_node)?),
        MIRNodeType::NumberLiteral { value } => Ok(VarType::Int),
        MIRNodeType::FunctionCall { id } => todo!(), // Need to find the function's return type
        _ => unreachable!(),
    }
}

/// Mark all recursive functions in a MIR tree as recursive.
fn mark_recursive_funcs(mir: &mut Tree<MIRNode>) -> Result<(), CompileError> {
    todo!();
}

/// Get a table mapping String function ID's (as appears in MCFL) to function nodes in the MIR
fn get_func_table(mir: &Tree<MIRNode>) -> Result<HashMap<String, NodeId>, CompileError> {
    let mut map = HashMap::new();
    for child in mir.get_children(mir.get_root()?)? {
        match &mir.get_node(*child)?.node_type {
            MIRNodeType::Function { name, .. } => {
                map.insert(name.to_string(), *child);
            }
            _ => unreachable!(),
        }
    }
    Ok(map)
}

/// Generate an IR (intermediate representation) from an AST (abstract syntax tree)
pub fn compile(ast: &Tree<ASTNode>) -> Result<IR, CompileError> {
    println!("{:?}", ast);

    let mut mir = generate_mir(ast)?;
    let func_table = get_func_table(&mir)?;

    println!("{:?}", mir);

    check_return_types(&mir)?;
    mark_recursive_funcs(&mut mir)?;

    Ok(IR::new())
}
