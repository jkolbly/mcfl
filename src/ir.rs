use std::{collections::HashMap, fmt::Display};

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

    /// Maps the variables in the MIR to the variables in the IR.
    mir_vars: HashMap<Variable, String>,
}

impl IR {
    /// Create a new empty IR
    pub fn new() -> IR {
        IR {
            functions: HashMap::new(),
            mir_vars: HashMap::new(),
        }
    }

    /// Add an IRFunc to this IR under a given name
    pub fn add_func(&mut self, func: IRFunc, name: String) {
        self.functions.insert(name, func);
    }
}

impl Display for IR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, func) in &self.functions {
            writeln!(f, "{}{}{}", "-".repeat(20), name, "-".repeat(20))?;
            write!(f, "{}", func)?;
        }
        Ok(())
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

impl Display for IRFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }
        Ok(())
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

impl Display for IRNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRNode::SetIntNamed { name, value } => write!(f, "SetIntNamed {} = {}", name, value),
            IRNode::BinaryOperationIntNamed {
                lhs,
                rhs,
                operation,
            } => write!(f, "BinaryOperationIntNamed {} {} {}", lhs, operation, rhs),
        }
    }
}
