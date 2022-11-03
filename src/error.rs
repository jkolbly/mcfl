use crate::parse::Rule;
use crate::tree::NodeId;
use pest::error::Error;

#[derive(Debug)]
pub enum TreeError {
    NodeNotFound { node_id: NodeId },
    RootNotFound,
}

pub enum CompileError {
    ParseError { err: Error<Rule> },
    TreeError { err: TreeError },
    NoEntryPoint {},
    TickParams {},
    TickNotMCFunction {},
    MCFunctionReturnType {},
    CompilingNonFunction {},
    CompilingNonMCFunction {},
}

impl std::fmt::Debug for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError { err } => write!(f, "ParseError: {:?}", err),
            Self::TreeError { err } => write!(f, "TreeError: {:?}", err),
            Self::NoEntryPoint {} => {
                write!(f, "No entrypoint ('tick' or 'startup' function) found")
            }
            Self::TickParams {} => write!(f, "'tick' function cannot have any parameters"),
            Self::TickNotMCFunction {} => write!(f, "'tick' function must be an mcfunction"),
            Self::MCFunctionReturnType {} => write!(f, "mcfunction cannot have a return type"),
            Self::CompilingNonFunction {} => {
                write!(f, "'compile_function' called on non-function node")
            }
            Self::CompilingNonMCFunction {} => {
                write!(
                    f,
                    "'compile_function' called on function that isn't an mcfunction"
                )
            }
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
        CompileError::ParseError { err }
    }
}
