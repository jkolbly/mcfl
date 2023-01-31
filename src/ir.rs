use std::collections::HashMap;

use crate::{mcfunction::ScoreboardOperation, program::FunctionID};

/// The IR is a set of functions, which are one-dimensional sets of instructions that are higher level than Minecraft but lower level than MCFL
///
/// Note that the memory structure of an MCFL program in Minecraft is as follows
///     - Named: Variables whose names can be known at compile time. Least overhead, as variable access is simply by name and is O(1)
///     - Stack: Callstacks for functions with variables that may be overwritten otherwise. Medium overhead, as stack traversal requires nonzero overhead but is O(1)
///     - Heap: Arrays and everything else that doesn't fit above. Significant overhead, as using a reference on the heap is O(logn)
pub struct IR {
    nodes: HashMap<FunctionID, IRFunc>,
}

pub struct IRFunc {
    nodes: Vec<IRNode>,
}

/// An IR instruction
pub enum IRNode {
    /// Set a named int with a name and value.
    ///
    /// No declaration step is necessary for named ints
    SetIntNamed { name: String, value: i32 },

    /// Perform a binary operation on two named ints.
    ///
    /// Supported operations are any Minecraft scoreboard operation.
    BinaryOperationIntNamed {
        lhs: String,
        rhs: String,
        operation: ScoreboardOperation,
    },
}
