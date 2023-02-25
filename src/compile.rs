use crate::{ast::ASTNode, error::CompileError, ir::IR, tree::Tree};

pub fn compile(tree: &Tree<ASTNode>) -> Result<IR, CompileError> {}

// pub fn compile(tree: &Tree<ASTNode>) -> Result<Program, CompileError> {
//     verify_ast(tree)?;

//     let mut program = Program::new("test");

//     if let Some(startup) = get_mcfunction(tree, "startup")? {
//         compile_mcfunction(tree, startup, &mut program)?;
//     }

//     if let Some(tick) = get_mcfunction(tree, "tick")? {
//         compile_mcfunction(tree, tick, &mut program)?;
//     }

//     Ok(program)
// }

// /// Compile a single mcfunction
// pub fn compile_mcfunction(
//     tree: &Tree<ASTNode>,
//     function: NodeId,
//     program: &mut Program,
// ) -> Result<(), CompileError> {
//     let func_node = tree.get_node(function)?;

//     match &func_node.node_type {
//         ASTNodeType::MCFunction { name } => {
//             let func_id = program.new_function(name, false);

//             let block = tree.get_only_child(function)?;

//             program.new_command(
//                 &func_id,
//                 ScoreboardCommand::ObjectivesAdd {
//                     id: program.ints_objective.to_owned(),
//                     criteria: ObjectiveCriteria::Dummy,
//                     name: None,
//                 }
//                 .into(),
//             )?;

//             let scope_id = program.new_scope(program.scopes.get_root()?)?;
//             compile_block(tree, block, program, &func_id, None, scope_id)?;

//             Ok(())
//         }
//         _ => Err(CompileError::CompilingNonMCFunction {}),
//     }
// }

// /// Get the identifier for a compiled function and compile the function if it isn't already.
// fn get_compiled_function(
//     tree: &Tree<ASTNode>,
//     mcfl_name: &str,
//     program: &mut Program,
//     caller: NodeId,
// ) -> Result<FunctionVars, CompileError> {
//     if let Some(vars) = program.compiled_functions.get(mcfl_name) {
//         Ok(vars.clone())
//     } else {
//         let func_nodeid = get_function(tree, mcfl_name)?.ok_or(CompileError::UnknownFunction {
//             name: mcfl_name.to_owned(),
//             context: tree.get_node(caller)?.context.to_owned(),
//         })?;
//         let func_node = tree.get_node(func_nodeid)?;
//         Ok(match &func_node.node_type {
//             ASTNodeType::Function {
//                 name,
//                 params,
//                 return_type,
//             } => {
//                 let func_id = program.new_function(name, true);
//                 let scope_id = program.new_scope(program.scopes.get_root()?)?;

//                 let inputs = program
//                     .scopes
//                     .get_node_mut(scope_id)?
//                     .new_vars(params, &func_node.context)?;
//                 let output = if let Some(ret) = return_type {
//                     // Some(
//                     //     program
//                     //         .scopes
//                     //         .get_node_mut(scope_id)?
//                     //         .new_var(*ret, &format!("{}-output", name), &func_node.context)?
//                     //         .clone(),
//                     // )
//                     Some(
//                         program
//                             .new_var(
//                                 *ret,
//                                 &format!("{}-output", name),
//                                 &func_node.context,
//                                 scope_id,
//                                 ScopeModifier::Global,
//                             )?
//                             .clone(),
//                     )
//                 } else {
//                     None
//                 };
//                 let vars = FunctionVars {
//                     func_id: func_id.to_owned(),
//                     inputs,
//                     output: output.clone(),
//                 };
//                 program
//                     .compiled_functions
//                     .insert(name.to_owned(), vars.clone());

//                 let block = tree.get_only_child(func_nodeid)?;
//                 compile_block(tree, block, program, &func_id, output, scope_id)?;

//                 vars
//             }
//             _ => unreachable!(),
//         })
//     }
// }

// fn compile_block(
//     tree: &Tree<ASTNode>,
//     block: NodeId,
//     program: &mut Program,
//     func_id: &FunctionID,
//     return_to: Option<Variable>,
//     scope_id: NodeId,
// ) -> Result<(), CompileError> {
//     for line in tree.get_children(block)? {
//         let n = tree.get_node(*line)?;
//         match &n.node_type {
//             ASTNodeType::VariableDeclaration { declaration } => {
//                 let var = program
//                     .new_var(
//                         declaration.var_type,
//                         &declaration.name,
//                         &n.context,
//                         scope_id,
//                         declaration.scope_modifier,
//                     )?
//                     .clone();
//                 set_int(program, func_id, &var, 0)?;
//             }
//             ASTNodeType::Assignment => {
//                 let first_child = tree.get_first_child(*line)?;
//                 let dest_var = match &tree.get_node(first_child)?.node_type {
//                     ASTNodeType::VariableDeclaration { declaration } => program
//                         .new_var(
//                             declaration.var_type,
//                             &declaration.name,
//                             &n.context,
//                             scope_id,
//                             declaration.scope_modifier,
//                         )?
//                         .clone(),
//                     ASTNodeType::Identifier { id } => {
//                         program.get_var(id, &n.context, scope_id)?.clone()
//                     }
//                     _ => unreachable!(),
//                 };
//                 eval_expression(
//                     program,
//                     func_id,
//                     tree,
//                     tree.get_last_child(*line)?,
//                     &dest_var,
//                     scope_id,
//                 )?;
//             }
//             ASTNodeType::ReturnStatement => {
//                 if let Some(ref ret_var) = return_to {
//                     eval_expression(
//                         program,
//                         func_id,
//                         tree,
//                         tree.get_only_child(*line)?,
//                         ret_var,
//                         scope_id,
//                     )?;
//                 } else {
//                     return Err(CompileError::AttemptedIllegalReturn {
//                         context: n.context.clone(),
//                     });
//                 }
//             }
//             _ => unreachable!("{:?}", n.node_type),
//         }
//     }

//     Ok(())
// }

// /// Evaluate an expression and put the result in `dest_var`
// /// TODO: Support declaring new variable after evaluation
// fn eval_expression(
//     program: &mut Program,
//     func_id: &FunctionID,
//     tree: &Tree<ASTNode>,
//     expression: NodeId,
//     dest_var: &Variable,
//     scope_id: NodeId,
// ) -> Result<(), CompileError> {
//     let node = tree.get_node(expression)?;
//     match &node.node_type {
//         ASTNodeType::Identifier { id } => program.new_command(
//             func_id,
//             ScoreboardCommand::PlayersOperation {
//                 target: CommandTarget::Name {
//                     name: dest_var.get_mc_name().to_owned(),
//                 },
//                 objective: program.ints_objective.to_owned(),
//                 operation: ScoreboardOperation::Assign,
//                 source: CommandTarget::Name {
//                     name: program
//                         .get_var(id, &node.context, scope_id)?
//                         .get_mc_name()
//                         .to_owned(),
//                 },
//                 source_objective: program.ints_objective.to_owned(),
//             }
//             .into(),
//         )?,
//         ASTNodeType::NumberLiteral { value } => set_int(program, func_id, dest_var, *value)?,
//         ASTNodeType::Add
//         | ASTNodeType::Subtract
//         | ASTNodeType::Multiply
//         | ASTNodeType::Divide
//         | ASTNodeType::Modulo => {
//             eval_binary_op(program, func_id, tree, expression, dest_var, scope_id)?
//         }
//         ASTNodeType::FunctionCall { id } => {
//             let func_to_call = get_compiled_function(tree, id, program, expression)?;
//             let namespace = if func_to_call.func_id.private {
//                 program.private_namespace_name.to_owned()
//             } else {
//                 program.program_name.to_owned()
//             };

//             let arg_expressions = tree.get_children(expression)?;
//             if arg_expressions.len() != func_to_call.inputs.len() {
//                 return Err(CompileError::MismatchedParamCount {
//                     func_name: id.to_owned(),
//                     expected: func_to_call.inputs.len(),
//                     received: arg_expressions.len(),
//                     context: node.context.clone(),
//                 });
//             }
//             for i in 0..arg_expressions.len() {
//                 let arg_exp = arg_expressions.get(i).unwrap();
//                 let arg_var = func_to_call.inputs.get(i).unwrap();
//                 eval_expression(program, func_id, tree, *arg_exp, arg_var, scope_id)?;
//             }

//             program.new_command(
//                 func_id,
//                 Command::Function {
//                     function: MCFunctionID {
//                         namespace,
//                         path: vec![func_to_call.func_id.mc_name],
//                     },
//                 },
//             )?;

//             if let Some(out_var) = func_to_call.output {
//                 program.new_command(
//                     func_id,
//                     ScoreboardCommand::PlayersOperation {
//                         target: CommandTarget::Name {
//                             name: dest_var.get_mc_name().to_owned(),
//                         },
//                         objective: program.ints_objective.to_owned(),
//                         operation: ScoreboardOperation::Assign,
//                         source: CommandTarget::Name {
//                             name: out_var.get_mc_name().to_owned(),
//                         },
//                         source_objective: program.ints_objective.to_owned(),
//                     }
//                     .into(),
//                 )?;
//             }
//         }
//         _ => unreachable!(),
//     };
//     Ok(())
// }

// fn eval_binary_op(
//     program: &mut Program,
//     func_id: &FunctionID,
//     tree: &Tree<ASTNode>,
//     expression: NodeId,
//     dest_var: &Variable,
//     scope_id: NodeId,
// ) -> Result<(), CompileError> {
//     let op_node = tree.get_node(expression)?;
//     let operator = match op_node.node_type {
//         ASTNodeType::Add => ScoreboardOperation::Addition,
//         ASTNodeType::Subtract => ScoreboardOperation::Subtraction,
//         ASTNodeType::Multiply => ScoreboardOperation::Multiplication,
//         ASTNodeType::Divide => ScoreboardOperation::Division,
//         ASTNodeType::Modulo => ScoreboardOperation::Modulo,
//         _ => unreachable!(),
//     };

//     let lhs = tree.get_first_child(expression)?;
//     let lhs_node = tree.get_node(lhs)?;
//     match &lhs_node.node_type {
//         ASTNodeType::Identifier { id } => {
//             let old_var = program.get_var(id, &lhs_node.context, scope_id)?;
//             program.new_command(
//                 func_id,
//                 ScoreboardCommand::PlayersOperation {
//                     target: CommandTarget::Name {
//                         name: dest_var.get_mc_name().to_owned(),
//                     },
//                     objective: program.ints_objective.to_owned(),
//                     operation: ScoreboardOperation::Assign,
//                     source: CommandTarget::Name {
//                         name: old_var.get_mc_name().to_owned(),
//                     },
//                     source_objective: program.ints_objective.to_owned(),
//                 }
//                 .into(),
//             )?;
//         }
//         ASTNodeType::NumberLiteral { value: _ } => {
//             eval_expression(program, func_id, tree, lhs, dest_var, scope_id)?;
//         }
//         ASTNodeType::Add
//         | ASTNodeType::Subtract
//         | ASTNodeType::Multiply
//         | ASTNodeType::Divide
//         | ASTNodeType::Modulo => {
//             eval_expression(program, func_id, tree, lhs, dest_var, scope_id)?;
//         }
//         _ => unreachable!(),
//     };

//     let rhs = tree.get_last_child(expression)?;
//     let rhs_var = program
//         .scopes
//         .get_node_mut(scope_id)?
//         .new_var_rand(VarType::Int, &tree.get_node(rhs)?.context)?
//         .clone();
//     eval_expression(program, func_id, tree, rhs, &rhs_var, scope_id)?;

//     program.new_command(
//         func_id,
//         ScoreboardCommand::PlayersOperation {
//             target: CommandTarget::Name {
//                 name: dest_var.get_mc_name().to_owned(),
//             },
//             objective: program.ints_objective.to_owned(),
//             operation: operator,
//             source: CommandTarget::Name {
//                 name: rhs_var.get_mc_name().to_owned(),
//             },
//             source_objective: program.ints_objective.to_owned(),
//         }
//         .into(),
//     )?;
//     Ok(())
// }

// fn set_int(
//     program: &mut Program,
//     func_id: &FunctionID,
//     variable: &Variable,
//     val: i32,
// ) -> Result<(), CompileError> {
//     program.new_command(
//         func_id,
//         ScoreboardCommand::PlayersSet {
//             target: CommandTarget::Name {
//                 name: variable.get_mc_name().to_owned(),
//             },
//             objective: program.ints_objective.to_owned(),
//             score: val,
//         }
//         .into(),
//     )?;
//     Ok(())
// }

// /// Get a function. Set `mcfunc` to true to search for mcfunctions and false to search for functions
// fn get_func(
//     tree: &Tree<ASTNode>,
//     func_name: &str,
//     mcfunc: bool,
// ) -> Result<Option<NodeId>, CompileError> {
//     Ok(
//         tree.find_child(tree.get_root()?, &|_id, node| match &node.node_type {
//             ASTNodeType::MCFunction { name } => mcfunc && name == func_name,
//             ASTNodeType::Function {
//                 name,
//                 params: _,
//                 return_type: _,
//             } => !mcfunc && name == func_name,
//             _ => unreachable!(),
//         })?,
//     )
// }

// fn get_mcfunction(tree: &Tree<ASTNode>, func_name: &str) -> Result<Option<NodeId>, CompileError> {
//     get_func(tree, func_name, true)
// }

// fn get_function(tree: &Tree<ASTNode>, func_name: &str) -> Result<Option<NodeId>, CompileError> {
//     get_func(tree, func_name, false)
// }

// fn verify_ast(tree: &Tree<ASTNode>) -> Result<(), CompileError> {
//     let tick = get_mcfunction(tree, "tick")?;
//     let startup = get_mcfunction(tree, "startup")?;

//     if tick.is_none() && startup.is_none() {
//         return Err(CompileError::NoEntryPoint {});
//     }

//     if let Some(tick) = tick {
//         if let ASTNodeType::MCFunction { name: _ } = &tree.get_node(tick)?.node_type {}
//     }

//     Ok(())
// }
