use crate::ast::{StringContext, VarType};

/// A node in the MIR (MCFL intermediate representation)
///
/// The MIR is a tree lying between the MCFL code and IR.
pub struct MIRNode {
    /// The type of node and associated data
    pub node_type: MIRNodeType,

    /// The string context of this node, for writing useful error messages
    pub context: StringContext,
}

impl MIRNode {
    pub fn new(node_type: MIRNodeType, context: StringContext) -> MIRNode {
        MIRNode { node_type, context }
    }
}

/// Stores the types of MIR nodes and their associated data
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
pub struct Variable {
    /// The variable name, as will appear in the final datapack
    name: String,

    /// The variable's type
    var_type: VarType,
}
