use std::collections::HashMap;

use crate::mcfunction::ScoreboardOperation;

/// The intermediate representation of an MCFL program.
/// Keeps track of useful things like:
///     - Functions
///     - Variables
pub struct IR {
    /// The functions making up this program.
    /// Keys are the name of the program, as written in the MCFL source.
    ///
    /// Note that not all functions appear in the MCFL source (ex. loops). These will have a randomly generated name.
    functions: HashMap<String, IRFunc>,
}

impl IR {
    /// Create a new empty IR
    pub fn new() -> IR {
        IR {
            functions: HashMap::new(),
        }
    }

    /// Add an IRFunc to this IR under a given name
    pub fn add_func(&mut self, func: IRFunc, name: String) {
        self.functions.insert(name, func);
    }
}

/// An IRFunc is a one-dimensional set of instructions that are higher level than Minecraft but lower level than MCFL
///
/// Note that the memory structure of an MCFL program in Minecraft is as follows:
///     - Named: Variables whose names can be known at compile time. Least overhead, as variable access is simply by name and is O(1)
///     - Stack: Callstacks for functions with variables that may be overwritten otherwise. Medium overhead, as stack traversal requires nonzero overhead but is O(1)
///     - Heap: Arrays and everything else that doesn't fit above. Significant overhead, as using a reference on the heap is O(logn)
pub struct IRFunc {
    /// The nodes composing this function, to be executed linearly.
    nodes: Vec<IRNode>,
}

impl IRFunc {
    /// Create a new empty function.
    pub fn new() -> IRFunc {
        IRFunc { nodes: Vec::new() }
    }

    /// Add an IRNode to the end of this function.
    pub fn add_node(&mut self, node: IRNode) {
        self.nodes.push(node);
    }
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
