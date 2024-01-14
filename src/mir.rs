use crate::ast::{StringContext, VarType};
use std::fmt::Debug;

/// A node in the MIR (MCFL intermediate representation)
///
/// The MIR is a tree lying between the MCFL code and IR.
pub struct MIRNode {
    /// The type of node and associated data
    pub node_type: MIRNodeType,

    /// The string context of this node, for writing useful error messages
    pub context: StringContext,
}

impl Debug for MIRNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.node_type))?;
        write!(f, "")
    }
}

impl MIRNode {
    pub fn new(node_type: MIRNodeType, context: StringContext) -> MIRNode {
        MIRNode { node_type, context }
    }
}

/// Stores the types of MIR nodes and their associated data
#[derive(Debug)]
pub enum MIRNodeType {
    /// The root of a program. Should only appear once in the tree
    Program,

    /// A function or mcfunction
    Function {
        name: String,
        params: Vec<Variable>,
        return_var: Option<Variable>,
        is_recursive: bool,
    },

    /// The identifier of a variable
    VarIdentifier { var: Variable },

    /// A return statement from a function
    ReturnStatement,

    /// An assignment statement. Note that the second child is the variable identifier being assigned
    AssignmentStatement,

    /// An addition expression
    Addition,
    /// A subtraction expression
    Subtraction,
    /// A multiplication expression
    Multiplication,
    /// A division expression
    Division,
    /// A modulo expression
    Modulo,

    /// A number literal expression
    NumberLiteral { value: i32 },

    /// A function call expression
    FunctionCall { id: String },
}

/// A variable with an identifier and type
#[derive(Debug, Clone)]
pub struct Variable {
    /// The variable name in MCFL code
    pub name: String,

    /// The variable name in the final datapack
    pub name_internal: String,

    /// The variable's type
    pub var_type: VarType,
}
