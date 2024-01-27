use crate::ast::{StringContext, VarType};
use crate::parse::Rule;
use crate::tree::NodeId;
use pest::error::Error;

#[derive(Debug)]
pub enum TreeError {
    NodeNotFound { node_id: NodeId },
    RootNotFound,
    ExpectedOnlyChild { parent_id: NodeId, child_num: usize },
    ChildNotFound { parent_id: NodeId },
    ParentNotFound { child_id: NodeId },
    MismatchedTreeAndNodeID { node_id: NodeId, tree_id: usize },
}

pub enum CompileError {
    ParseError {
        err: Box<Error<Rule>>,
    },
    TreeError {
        err: TreeError,
    },
    IOError {
        err: std::io::Error,
    },
    NoEntryPoint {},
    CompilingNonMCFunction {},
    VariableAlreadyDeclared {
        var: String,
        context: StringContext,
    },
    VariableNotDeclared {
        var_name: String,
        context: StringContext,
    },
    UnknownFunction {
        name: String,
        context: StringContext,
    },
    UnknownFunctionID {
        id: String,
    },
    AttemptedIllegalReturn {
        context: StringContext,
    },
    MismatchedParamCount {
        func_name: String,
        expected: usize,
        received: usize,
        context: StringContext,
    },
    MismatchedReturnType {
        func_name: String,
        expected: VarType,
        received: VarType,
        context: StringContext,
    },
    ReturnFromVoid {
        func_name: String,
        context: StringContext,
    },
    NoReturnStatement {
        func_name: String,
        context: StringContext,
    },
    EmptyReturnStatement {
        func_name: String,
        context: StringContext,
    },
    UsingVoidReturn {
        func_name: String,
        context: StringContext,
    },
}

impl std::fmt::Debug for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut include_pos = |ctx: &StringContext, msg: &str| write!(f, "{} on {}", msg, ctx);
        match self {
            Self::ParseError { err } => write!(f, "ParseError: {:?}", err),
            Self::TreeError { err } => write!(f, "TreeError: {:?}", err),
            Self::IOError { err } => write!(f, "I/O error: {:?}", err),
            Self::NoEntryPoint {} => {
                write!(f, "No entrypoint ('tick' or 'startup' function) found")
            }
            Self::CompilingNonMCFunction {} => {
                write!(f, "'compile_mcfunction' called on non-mcfunction node")
            }
            Self::VariableAlreadyDeclared { var, context } => include_pos(
                context,
                &format!("Variable {:?} has already been declared", var),
            ),
            Self::VariableNotDeclared { var_name, context } => include_pos(
                context,
                &format!("Variable {:?} has not been declared", var_name),
            ),
            Self::UnknownFunction { name, context } => include_pos(
                context,
                &format!(
                    "Tried to access a non-existent function within a program with name {}",
                    name
                ),
            ),
            Self::UnknownFunctionID { id } => write!(
                f,
                "Tried to access a non-existent function with ID {:?}",
                id
            ),
            Self::AttemptedIllegalReturn { context } => {
                include_pos(context, "Attempted to return outside of a function")
            }
            Self::MismatchedParamCount {
                func_name,
                expected,
                received,
                context,
            } => include_pos(
                context,
                &format!(
                    "Function {} takes {} arguments but {} were given",
                    func_name, expected, received
                ),
            ),
            Self::MismatchedReturnType {
                func_name,
                expected,
                received,
                context,
            } => include_pos(
                context,
                &format!(
                    "Function {} should return {} but returned {}",
                    func_name, expected, received
                ),
            ),
            Self::ReturnFromVoid { func_name, context } => include_pos(
                context,
                &format!(
                    "Function {} has no return type but a value was returned",
                    func_name
                ),
            ),
            Self::NoReturnStatement { func_name, context } => include_pos(
                context,
                &format!(
                    "Function {} has a return type but a return statement may not always be reached",
                    func_name
                ),
            ),
            Self::EmptyReturnStatement { func_name, context } => include_pos(
                context,
                &format!(
                    "Function {} has a return type but an empty return statement was found",
                    func_name
                ),
            ),
            Self::UsingVoidReturn { func_name, context } => include_pos(
                context,
                &format!(
                    "Function {} has no return value but it is used in an expression",
                    func_name
                ),
            ),
        }
    }
}

impl From<TreeError> for CompileError {
    fn from(err: TreeError) -> Self {
        CompileError::TreeError { err }
    }
}

impl From<Error<Rule>> for CompileError {
    fn from(err: Error<Rule>) -> Self {
        CompileError::ParseError { err: Box::new(err) }
    }
}

impl From<std::io::Error> for CompileError {
    fn from(err: std::io::Error) -> Self {
        CompileError::IOError { err }
    }
}
