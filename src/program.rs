use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use crate::{
    ast::{ScopeModifier, StringContext, VarType, VariableDeclaration},
    datapack::DataPack,
    error::CompileError,
    mcfunction::{Command, CommandTarget, MCFunction, ScoreboardCommand},
    tree::{NodeId, Tree},
};

/// The intermediate representation of an MCFL program. Contains the contents of .mcfunction files as well as associated information like variables
pub struct Program {
    pub program_name: String,
    mcfunctions: HashMap<FunctionID, MCFunction>,
    pub compiled_functions: HashMap<String, FunctionVars>,
    pub scopes: Tree<Scope>,
    pub ints_objective: String,
    cleanup_function_id: String,
    pub private_namespace_name: String,
}

impl Program {
    pub fn new(name: &str) -> Program {
        let mut scopes = Tree::new();
        let nodeid = scopes.next_id();
        scopes.new_node(Scope::new(name, nodeid));

        Program {
            program_name: name.to_owned(),
            mcfunctions: HashMap::new(),
            compiled_functions: HashMap::new(),
            scopes,
            ints_objective: "mcfl_ints".to_owned(),
            cleanup_function_id: "cleanup".to_owned(),
            private_namespace_name: format!("{}-private", name),
        }
    }

    // pub fn global_scope(&self) -> Result<&Scope, CompileError> {
    //     Ok(self.scopes.get_node(self.scopes.get_root()?)?)
    // }

    // pub fn global_scope_mut(&mut self) -> Result<&mut Scope, CompileError> {
    //     Ok(self.scopes.get_node_mut(self.scopes.get_root()?)?)
    // }

    pub fn new_function(&mut self, name: &str, private: bool) -> FunctionID {
        let id = FunctionID {
            mcfl_name: Some(name.to_owned()),
            mc_name: name.to_owned(),
            private,
        };
        self.mcfunctions.insert(id.clone(), MCFunction::new());
        id
    }

    pub fn new_command(
        &mut self,
        func_id: &FunctionID,
        command: Command,
    ) -> Result<(), CompileError> {
        self.mcfunctions
            .get_mut(func_id)
            .ok_or(CompileError::UnknownFunctionID {
                id: func_id.to_owned(),
            })?
            .new_command(command);
        Ok(())
    }

    pub fn new_scope(&mut self, parent: NodeId) -> Result<NodeId, CompileError> {
        let nodeid = self.scopes.next_id();
        self.scopes.new_node(Scope::new(&self.program_name, nodeid));
        self.scopes.append_to(parent, nodeid)?;
        Ok(nodeid)
    }

    pub fn get_var(
        &self,
        name: &str,
        context: &StringContext,
        scope: NodeId,
    ) -> Result<&Variable, CompileError> {
        let ascender = self.scopes.iter_ascend(scope)?;
        for scope_id in ascender {
            if let Some(var) = self.scopes.get_node(scope_id)?.get_var(name) {
                return Ok(var);
            }
        }
        Err(CompileError::VariableNotDeclared {
            var_name: name.to_string(),
            context: context.clone(),
        })
    }

    pub fn new_var(
        &mut self,
        var_type: VarType,
        name: &str,
        context: &StringContext,
        scope_id: NodeId,
        scope_modifier: ScopeModifier,
    ) -> Result<&Variable, CompileError> {
        let scope = self.scopes.get_node_mut(match scope_modifier {
            ScopeModifier::Default => scope_id,
            ScopeModifier::Global => self.scopes.get_root()?,
        })?;
        scope.new_var(var_type, name, context)
    }

    fn generate_cleanup(&mut self) -> Result<(), CompileError> {
        let func_id = self.new_function(&self.cleanup_function_id.to_owned(), false);

        self.new_command(
            &func_id,
            ScoreboardCommand::ObjectivesRemove {
                id: self.ints_objective.to_owned(),
            }
            .into(),
        )?;

        let mut reset_commands: Vec<Command> = vec![];
        for scope in self.scopes.into_iter() {
            for variable in self.scopes.get_node(scope)?.variables.values() {
                reset_commands.push(
                    ScoreboardCommand::PlayersReset {
                        target: CommandTarget::Name {
                            name: variable.mc_name.to_owned(),
                        },
                        objective: None,
                    }
                    .into(),
                );
            }
        }
        // for variable in self.variables.values() {
        //     reset_commands.push(
        //         ScoreboardCommand::PlayersReset {
        //             target: CommandTarget::Name {
        //                 name: variable.mc_name.to_owned(),
        //             },
        //             objective: None,
        //         }
        //         .into(),
        //     );
        // }
        for command in reset_commands {
            self.new_command(&func_id, command)?;
        }

        Ok(())
    }
}

impl From<Program> for DataPack {
    fn from(mut program: Program) -> Self {
        program.generate_cleanup().unwrap();

        let mut dp = DataPack::new(&program.program_name, &program.private_namespace_name);
        for (id, func) in program.mcfunctions {
            if id.private {
                dp.private_namespace.functions.insert(id.mc_name, func);
            } else {
                dp.pub_namespace.functions.insert(id.mc_name, func);
            }
        }
        dp
    }
}

// impl Into<DataPack> for Program {
//     fn into(self) -> DataPack {
//         let mut dp = DataPack::new(&self.program_name);
//         for (id, func) in self.mcfunctions {
//             if (id.private) {
//                 dp.private_namespace.functions.insert(id.mc_name, func);
//             } else {
//                 dp.pub_namespace.functions.insert(id.mc_name, func);
//             }
//         }
//         dp
//     }
// }

fn random_name() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

fn random_id() -> usize {
    rand::thread_rng().gen()
}

#[derive(Debug, Clone)]
pub struct Variable {
    var_type: VarType,
    name: String,
    mc_name: String,
}

impl Variable {
    pub fn get_mc_name(&self) -> &str {
        &self.mc_name
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Eq, Clone, Debug)]
pub struct FunctionID {
    mcfl_name: Option<String>,
    pub mc_name: String,
    pub private: bool,
}

impl PartialEq for FunctionID {
    fn eq(&self, other: &Self) -> bool {
        self.mc_name == other.mc_name
    }
}

impl Hash for FunctionID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mc_name.hash(state);
    }
}

#[derive(Clone)]
pub struct FunctionVars {
    pub func_id: FunctionID,
    pub inputs: Vec<Variable>,
    pub output: Option<Variable>,
}

pub struct Scope {
    variables: HashMap<String, Variable>,
    mc_name_prefix: String,
}

impl Scope {
    pub fn new(program_name: &str, id: NodeId) -> Scope {
        Scope {
            variables: HashMap::new(),
            mc_name_prefix: format!("mcfl-{}-{}", program_name, id.get_id()),
        }
    }

    fn new_var(
        &mut self,
        var_type: VarType,
        name: &str,
        context: &StringContext,
    ) -> Result<&Variable, CompileError> {
        let var = Variable {
            var_type,
            name: name.to_owned(),
            mc_name: format!("{}-{}", self.mc_name_prefix, name.to_owned()),
        };

        let insert_result = self.variables.insert(name.to_owned(), var);
        if let Some(_old_var) = insert_result {
            Err(CompileError::VariableAlreadyDeclared {
                var: self.variables.get(name).unwrap().clone(),
                context: context.clone(),
            })
        } else {
            Ok(self.variables.get(name).unwrap())
        }
    }

    /// Create a new variable with a random name
    pub fn new_var_rand(
        &mut self,
        var_type: VarType,
        context: &StringContext,
    ) -> Result<&Variable, CompileError> {
        let mut name = random_name();
        while self.get_var(&name).is_some() {
            name = random_name();
        }

        self.new_var(var_type, &name, context)
    }

    pub fn new_vars(
        &mut self,
        declarations: &Vec<VariableDeclaration>,
        context: &StringContext,
    ) -> Result<Vec<Variable>, CompileError> {
        let mut vars: Vec<Variable> = Vec::new();
        for declaration in declarations {
            vars.push(
                self.new_var(declaration.var_type, &declaration.name, context)?
                    .clone(),
            );
        }

        Ok(vars)
    }

    fn get_var(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }
}
