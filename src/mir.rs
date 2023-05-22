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

    /// A function or mcfunction that is not recursive
    Function {
        name: String,
        params: Vec<Variable>,
        return_var: Option<Variable>,
    },
}

/// A variable with an identifier and type
#[derive(Debug)]
pub struct Variable {
    /// The variable name, as will appear in the final datapack
    pub name: String,

    /// The variable's type
    pub var_type: VarType,
}
