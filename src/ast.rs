use std::fmt::{Debug, Display};

use crate::parse::Rule;
use pest::iterators::Pair;

pub struct ASTNode {
    pub node_type: ASTNodeType,
    pub context: StringContext,
}

impl ASTNode {
    pub fn new(node_type: ASTNodeType, pair: Pair<Rule>) -> ASTNode {
        ASTNode {
            node_type,
            context: StringContext::new(pair),
        }
    }
}

impl Debug for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.node_type))?;
        write!(f, "")
    }
}

#[derive(Debug)]
pub enum ASTNodeType {
    Program,
    Function {
        name: String,
        params: Vec<VariableDeclaration>,
        return_type: Option<VarType>,
    },
    MCFunction {
        name: String,
    },
    Block,
    VariableDeclaration {
        declaration: VariableDeclaration,
    },
    Assignment,
    Identifier {
        id: String,
    },
    NumberLiteral {
        value: i32,
    },
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    ReturnStatement,
    FunctionCall {
        id: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ScopeModifier {
    Default,
    Global,
}

#[derive(Debug, Clone, Copy)]
pub enum VarType {
    Int,
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarType::Int => write!(f, "int"),
        }
    }
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub scope_modifier: ScopeModifier,
    pub name: String,
    pub var_type: VarType,
}

/// The string context of an AST node.
#[derive(Clone)]
pub struct StringContext {
    /// Line number of the start of the node
    line: usize,

    /// Column number of the start of the node
    col: usize,

    /// Line of input string containing the start of this node
    line_str: String,

    /// Input substring making up this node
    node_str: String,
}

impl StringContext {
    pub fn new(pair: Pair<Rule>) -> StringContext {
        let node_str = pair.as_str();
        let pos = match pair.tokens().next().unwrap() {
            pest::Token::Start { rule: _, pos } => pos,
            pest::Token::End { rule: _, pos: _ } => unreachable!(),
        };
        let (line, col) = pos.line_col();

        StringContext {
            line,
            col,
            line_str: pos.line_of().to_owned(),
            node_str: node_str.to_owned(),
        }
    }
}

impl Display for StringContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line {} col {}: \n{}\n{}↑ here",
            self.line,
            self.col,
            self.line_str.trim_end(),
            " ".repeat(self.col - 1)
        )
    }
}
