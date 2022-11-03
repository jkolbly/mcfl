use std::fmt::Debug;

pub struct ASTNode {
    pub node_type: ASTNodeType,
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
        func_type: FunctionType,
        name: String,
        params: Vec<VariableDeclaration>,
        return_type: Option<VarType>,
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
}

#[derive(Debug)]
pub enum FunctionType {
    MCFunction,
    Function,
}

#[derive(Debug)]
pub enum VarType {
    Int,
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: VarType,
}
