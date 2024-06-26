use std::collections::{HashMap, HashSet};

use rand::distributions::{Alphanumeric, DistString};

use crate::{
    ast::{ASTNode, ASTNodeType, VarType},
    error::CompileError,
    ir::IR,
    mir::{MIRNode, MIRNodeType, Mir, Variable},
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
fn generate_mir(ast: &Tree<ASTNode>) -> Result<Mir, CompileError> {
    let mut mir_tree: Tree<MIRNode> = Tree::new();

    get_mir_node(ast, ast.get_root()?, &mut mir_tree, None, &mut vec![])?;

    Mir::new(mir_tree)
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
        ASTNodeType::FunctionCall { id } => Some(MIRNodeType::FunctionCall { id: id.to_string() }),
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
fn check_return_types(mir: &Mir) -> Result<(), CompileError> {
    for func_node in mir.func_table.values() {
        let MIRNodeType::Function {
            name,
            params: _,
            return_var,
            is_recursive: _,
        } = &mir.tree.get_node(*func_node)?.node_type
        else {
            unreachable!()
        };
        let ret_nodes = mir
            .tree
            .find_children_recursive(*func_node, &|_, node| -> bool {
                matches!(node.node_type, MIRNodeType::ReturnStatement)
            })?;
        match return_var {
            Some(ret_var) => {
                verify_return_reached(mir, *func_node)?; // Make sure a return value is always reached

                // Check that return types match for all return statements
                for ret_node in ret_nodes {
                    let ret_expr_type = match mir.tree.get_first_child(ret_node) {
                        Ok(ret_child) => get_expression_type(mir, ret_child)?,
                        Err(_) => {
                            return Err(CompileError::EmptyReturnStatement {
                                func_name: name.to_string(),
                                context: mir.tree.get_node(ret_node)?.context.clone(),
                            })
                        }
                    };
                    if ret_expr_type != ret_var.var_type {
                        return Err(CompileError::MismatchedReturnType {
                            func_name: name.to_string(),
                            expected: ret_var.var_type,
                            received: ret_expr_type,
                            context: mir
                                .tree
                                .get_node(mir.tree.get_first_child(ret_node)?)?
                                .context
                                .clone(),
                        });
                    }
                }
            }
            None => {
                for ret_node in ret_nodes {
                    if mir.tree.has_children(ret_node)? {
                        // A child indicates something is being returned
                        return Err(CompileError::ReturnFromVoid {
                            func_name: name.to_string(),
                            context: mir
                                .tree
                                .get_node(mir.tree.get_first_child(ret_node)?)?
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
fn verify_return_reached(mir: &Mir, func_node: NodeId) -> Result<(), CompileError> {
    match is_return_reached(mir, func_node)? {
        true => Ok(()),
        false => {
            let MIRNodeType::Function {
                name,
                params: _,
                return_var: _,
                is_recursive: _,
            } = &mir.tree.get_node(func_node)?.node_type
            else {
                unreachable!()
            };
            Err(CompileError::NoReturnStatement {
                func_name: name.to_string(),
                context: mir.tree.get_node(func_node)?.context.clone(),
            })
        }
    }
}

/// Return whether a return expression is always reached in a code block
fn is_return_reached(mir: &Mir, block_node: NodeId) -> Result<bool, CompileError> {
    // Algorithm: Start at end and move backwards until you hit a return statement.
    //            If we hit a code block, check if one is guaranteed in the block.

    if !mir.tree.has_children(block_node)? {
        return Ok(false);
    }

    let mut cur_node = mir.tree.get_last_child(block_node).unwrap();
    loop {
        match &mir.tree.get_node(cur_node)?.node_type {
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

        match mir.tree.get_prev_sibling(cur_node)? {
            Some(node_id) => cur_node = node_id,
            None => break,
        }
    }

    Ok(false)
}

fn get_expression_type(mir: &Mir, expr_node: NodeId) -> Result<VarType, CompileError> {
    match &mir.tree.get_node(expr_node)?.node_type {
        MIRNodeType::VarIdentifier { var } => Ok(var.var_type),
        MIRNodeType::Addition
        | MIRNodeType::Subtraction
        | MIRNodeType::Multiplication
        | MIRNodeType::Division
        | MIRNodeType::Modulo => get_expression_type(mir, mir.tree.get_first_child(expr_node)?),
        MIRNodeType::NumberLiteral { value: _ } => Ok(VarType::Int),
        MIRNodeType::FunctionCall { id } => {
            let MIRNodeType::Function {
                name: _,
                params: _,
                return_var,
                is_recursive: _,
            } = &mir
                .tree
                .get_node(*mir.func_table.get(id).unwrap())?
                .node_type
            else {
                unreachable!()
            };
            // Ok(return_var.as_ref().map(|var| var.var_type))
            match return_var {
                Some(var) => Ok(var.var_type),
                None => Err(CompileError::UsingVoidReturn {
                    func_name: id.to_string(),
                    context: mir.tree.get_node(expr_node)?.context.clone(),
                }),
            }
        }
        _ => unreachable!(),
    }
}

/// Check that all assignments match the type of what they are assigning to
fn check_assignment_types(mir: &Mir) -> Result<(), CompileError> {
    // Maps expected expression nodes to the variable being assigned
    let mut assignments: HashMap<NodeId, Variable> = HashMap::new();

    // Put all assignment statements into the assignments map
    let assignment_statements = mir
        .tree
        .find_children_recursive(mir.tree.get_root()?, &|_, node| {
            matches!(node.node_type, MIRNodeType::AssignmentStatement)
        })?;
    for assign_stmnt in assignment_statements {
        let assigned_identifier = mir.tree.get_node(mir.tree.get_last_child(assign_stmnt)?)?;
        if let MIRNodeType::VarIdentifier { var } = &assigned_identifier.node_type {
            assignments.insert(mir.tree.get_first_child(assign_stmnt)?, var.clone());
        } else {
            unreachable!(
                "An assignment expression must have a VarIdentifier as its last (second) child"
            )
        }
    }

    // Check that the expressions in the assignments map match their expected type
    for (expr, var) in assignments {
        let expr_type = get_expression_type(mir, expr)?;
        if expr_type != var.var_type {
            return Err(CompileError::MismatchedAssignmentType {
                var_id: var.name,
                expected: var.var_type,
                received: expr_type,
                context: mir.tree.get_node(expr)?.context.clone(),
            });
        }
    }

    Ok(())
}

fn check_param_duplicates(mir: &Mir) -> Result<(), CompileError> {
    for func_node in mir.func_table.values() {
        if let MIRNodeType::Function {
            name,
            params,
            return_var: _,
            is_recursive: _,
        } = &mir.tree.get_node(*func_node)?.node_type
        {
            let mut param_names = Vec::new();
            for param in params {
                if param_names.contains(&&param.name) {
                    return Err(CompileError::DuplicateParamName {
                        func_name: name.to_string(),
                        param_name: param.name.to_string(),
                        context: mir.tree.get_node(*func_node)?.context.clone(),
                    });
                }
                param_names.push(&param.name);
            }
        }
    }
    Ok(())
}

/// Check that all function calls match an existing function name and have the correct arguments.
fn check_function_calls(mir: &Mir) -> Result<(), CompileError> {
    // Get all function calls anywhere in the program
    let func_calls = mir
        .tree
        .find_children_recursive(mir.tree.get_root()?, &|_, node| {
            matches!(&node.node_type, MIRNodeType::FunctionCall { id: _ })
        })?;

    for call in func_calls {
        let MIRNodeType::FunctionCall { id: func_id } = &mir.tree.get_node(call)?.node_type else {
            unreachable!("func_calls should only contain MIRNodeType::FunctionCall (in check_function_parameters)");
        };

        let func_node =
            *mir.func_table
                .get(func_id)
                .ok_or_else(|| match mir.tree.get_node(call) {
                    Ok(node) => CompileError::UnknownFunction {
                        name: func_id.to_string(),
                        context: node.context.clone(),
                    },
                    Err(err) => err.into(),
                })?;

        let MIRNodeType::Function {
            name,
            params,
            return_var: _,
            is_recursive: _,
        } = &mir.tree.get_node(func_node)?.node_type
        else {
            unreachable!();
        };

        let supplied_args = mir.tree.get_children(call)?;
        if params.len() != supplied_args.len() {
            return Err(CompileError::MismatchedParamCount {
                func_name: name.to_string(),
                expected: params.len(),
                received: supplied_args.len(),
                context: mir.tree.get_node(call)?.context.clone(),
            });
        }

        for i_arg in 0..params.len() {
            let supplied_arg = supplied_args[i_arg];
            let supplied_type = get_expression_type(mir, supplied_arg)?;

            let expected = &params[i_arg];

            if supplied_type != expected.var_type {
                return Err(CompileError::MismatchedParamType {
                    func_name: name.to_string(),
                    expected: expected.var_type,
                    received: supplied_type,
                    arg_index: i_arg,
                    arg_name: expected.name.to_string(),
                    context: mir.tree.get_node(supplied_arg)?.context.clone(),
                });
            }
        }
    }

    Ok(())
}

/// Mark all recursive functions in a MIR tree as recursive.
fn mark_recursive_funcs(mir: &mut Mir) -> Result<(), CompileError> {
    // Maps function nodes to a list of all functions called by that function
    let mut func_calls: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();

    for func_node in mir.func_table.values() {
        let mut calls: HashSet<NodeId> = HashSet::new();
        for child in mir.tree.iter_subtree(*func_node)? {
            if let MIRNodeType::FunctionCall { id } = &mir.tree.get_node(child)?.node_type {
                match mir.func_table.get(id) {
                    Some(called_func_id) => {
                        calls.insert(*called_func_id);
                    }
                    None => {
                        return Err(CompileError::UnknownFunction {
                            name: id.to_string(),
                            context: mir.tree.get_node(child)?.context.clone(),
                        })
                    }
                }
            }
        }
        func_calls.insert(*func_node, calls);
    }

    // The functions that have already been marked as (non)recursive
    let mut known_recursive: HashSet<NodeId> = HashSet::new();
    let mut known_nonrecursive: HashSet<NodeId> = HashSet::new();

    for func_node in mir.func_table.values() {
        mark_recursive_funcs_helper(
            &func_calls,
            *func_node,
            &mut Vec::new(),
            &mut known_recursive,
            &mut known_nonrecursive,
        )?;
    }

    println!(
        "Nonrecursive: {:?}",
        known_nonrecursive
            .iter()
            .map(|node| {
                let MIRNodeType::Function { name, .. } =
                    &mir.tree.get_node(*node).unwrap().node_type
                else {
                    unreachable!()
                };
                name
            })
            .collect::<Vec<&String>>()
    );
    println!(
        "Recursive: {:?}",
        known_recursive
            .iter()
            .map(|node| {
                let MIRNodeType::Function { name, .. } =
                    &mir.tree.get_node(*node).unwrap().node_type
                else {
                    unreachable!()
                };
                name
            })
            .collect::<Vec<&String>>()
    );

    for recursive in known_recursive {
        let MIRNodeType::Function {
            name: _,
            params: _,
            return_var: _,
            is_recursive,
        } = &mut mir.tree.get_node_mut(recursive)?.node_type
        else {
            unreachable!()
        };
        *is_recursive = true;
    }

    Ok(())
}

/// Update a set of known recursive functions by adding all functions in loops called inside `func_node`
fn mark_recursive_funcs_helper(
    func_calls: &HashMap<NodeId, HashSet<NodeId>>,
    func_node: NodeId,
    callstack: &mut Vec<NodeId>,
    known_recursive: &mut HashSet<NodeId>,
    known_nonrecursive: &mut HashSet<NodeId>,
) -> Result<(), CompileError> {
    // If we already know this function isn't recursive, skip it
    if known_nonrecursive.contains(&func_node) {
        callstack.pop();
        return Ok(());
    }

    // If we've found a loop, everything in the loop is recursive
    if callstack.contains(&func_node) {
        while let Some(recursive) = callstack.pop() {
            known_recursive.insert(recursive);

            // If we've looked at an entire loop, we're done
            if recursive == func_node {
                break;
            }
        }
        return Ok(());
    }

    // Otherwise, keep searching
    callstack.push(func_node);

    for call in func_calls.get(&func_node).unwrap() {
        if known_recursive.contains(call) {
            known_recursive.insert(func_node);
        } else {
            mark_recursive_funcs_helper(
                func_calls,
                *call,
                callstack,
                known_recursive,
                known_nonrecursive,
            )?;
        }
    }

    if !known_recursive.contains(&func_node) {
        known_nonrecursive.insert(func_node);
    }

    Ok(())
}

/// Generate an IR from a MIR.
fn generate_ir(mir: &Mir) -> Result<IR, CompileError> {
    let mut to_compile: Vec<NodeId> = mir.func_table.values().cloned().collect();
    let mut compiled: Vec<NodeId> = Vec::new();

    let mut ir = IR::new();

    while let Some(block) = to_compile.pop() {
        compiled.push(block);

        let new_to_compile = compile_ir_func(&mut ir, mir, block)?;

        for new_block in new_to_compile {
            if !compiled.contains(&new_block) && !to_compile.contains(&new_block) {
                to_compile.push(new_block);
            }
        }
    }

    Ok(ir)
}

/// Compile a single IR function from a block in the MIR.
/// Returns a list of blocks that also need to be compiled (though they may already be compiled).
fn compile_ir_func(ir: &mut IR, mir: &Mir, block: NodeId) -> Result<Vec<NodeId>, CompileError> {
    todo!()
}

/// Generate an IR (intermediate representation) from an AST (abstract syntax tree)
pub fn compile(ast: &Tree<ASTNode>) -> Result<IR, CompileError> {
    println!("{:?}", ast);

    let mut mir = generate_mir(ast)?;

    println!("{:?}", mir.tree);

    check_param_duplicates(&mir)?;
    check_function_calls(&mir)?;
    check_return_types(&mir)?;
    check_assignment_types(&mir)?;
    mark_recursive_funcs(&mut mir)?;

    let ir = generate_ir(&mir)?;

    Ok(ir)
}
