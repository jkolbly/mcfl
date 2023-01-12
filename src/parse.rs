use std::cell::RefCell;
use std::rc::Rc;

use pest::iterators::Pair;
use pest::pratt_parser::Assoc;
use pest::pratt_parser::Op;
use pest::pratt_parser::PrattParser;
use pest::Parser;

use crate::ast::{ASTNode, ASTNodeType, VarType, VariableDeclaration};
use crate::error::CompileError;
use crate::tree::{NodeId, Tree};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MCFLParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        PrattParser::new()
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::subtract, Assoc::Left))
            .op(Op::infix(Rule::multiply, Assoc::Left) | Op::infix(Rule::divide, Assoc::Left) | Op::infix(Rule::modulo, Assoc::Left))
    };
}

/// Parse a string to an AST
pub fn parse(toparse: &str) -> Result<Tree<ASTNode>, CompileError> {
    let parsed = MCFLParser::parse(Rule::program, toparse)?.next().unwrap();
    let mut tree = Tree::<ASTNode>::new();

    parse_pair(&mut tree, parsed);
    return Ok(tree);

    fn parse_pair(tree: &mut Tree<ASTNode>, pair: Pair<Rule>) -> Option<NodeId> {
        let mut inner_pairs = pair.clone().into_inner();
        let rule = pair.as_rule();

        let node_type: Option<ASTNodeType> = match rule {
            Rule::program => Some(ASTNodeType::Program),
            Rule::function => Some(ASTNodeType::Function {
                name: inner_pairs.next().unwrap().as_str().to_owned(),
                params: parse_declaration_list(inner_pairs.next().unwrap()),
                return_type: {
                    if let Rule::var_type = inner_pairs.peek().unwrap().as_rule() {
                        Some(parse_var_type(inner_pairs.next().unwrap()))
                    } else {
                        None
                    }
                },
            }),
            Rule::mcfunction => Some(ASTNodeType::MCFunction {
                name: inner_pairs.next().unwrap().as_str().to_owned(),
            }),
            Rule::block => Some(ASTNodeType::Block),
            Rule::EOI => None,
            Rule::variable_declaration => {
                inner_pairs.next();
                inner_pairs.next();
                Some(ASTNodeType::VariableDeclaration {
                    declaration: parse_variable_declaration(pair.clone()),
                })
            }
            Rule::assignment => Some(ASTNodeType::Assignment),
            Rule::name => Some(ASTNodeType::Identifier {
                id: pair.as_str().to_owned(),
            }),
            Rule::number_literal => Some(ASTNodeType::NumberLiteral {
                value: parse_number_literal(pair.clone()),
            }),
            Rule::binary_operation => None,
            Rule::return_statement => Some(ASTNodeType::ReturnStatement),
            Rule::function_call => Some(ASTNodeType::FunctionCall {
                id: inner_pairs.next().unwrap().as_str().to_owned(),
            }),
            _ => unreachable!("{:?}", pair.as_rule()),
        };

        if let Some(n_type) = node_type {
            let node = tree.new_node(ASTNode::new(n_type, pair));
            for child in inner_pairs {
                if let Some(c_node) = parse_pair(tree, child) {
                    tree.append_to(node, c_node).unwrap();
                }
            }
            Some(node)
        } else if let Rule::binary_operation = rule {
            let treerc = Rc::new(RefCell::new(tree));

            // This will break if `use std::borrow::BorrowMut` is present ;(
            let ret_node = PRATT_PARSER
                .map_primary(|primary| parse_pair(*treerc.borrow_mut(), primary))
                .map_infix(|lhs, op, rhs| {
                    let node_type = match op.as_rule() {
                        Rule::add => ASTNodeType::Add,
                        Rule::subtract => ASTNodeType::Subtract,
                        Rule::multiply => ASTNodeType::Multiply,
                        Rule::divide => ASTNodeType::Divide,
                        Rule::modulo => ASTNodeType::Modulo,
                        _ => unreachable!(),
                    };

                    let node = treerc.borrow_mut().new_node(ASTNode::new(node_type, op));
                    treerc.borrow_mut().append_to(node, lhs.unwrap()).unwrap();
                    treerc.borrow_mut().append_to(node, rhs.unwrap()).unwrap();
                    Some(node)
                })
                .parse(inner_pairs);
            ret_node
        } else {
            None
        }
    }

    fn parse_var_type(pair: Pair<Rule>) -> VarType {
        match pair.into_inner().next().unwrap().as_rule() {
            Rule::int_type => VarType::Int,
            _ => unreachable!(),
        }
    }

    fn parse_declaration_list(pair: Pair<Rule>) -> Vec<VariableDeclaration> {
        pair.into_inner().map(parse_variable_declaration).collect()
    }

    fn parse_variable_declaration(pair: Pair<Rule>) -> VariableDeclaration {
        let mut inner_pairs = pair.into_inner();
        VariableDeclaration {
            var_type: parse_var_type(inner_pairs.next().unwrap()),
            name: inner_pairs.next().unwrap().as_str().to_owned(),
        }
    }

    fn parse_number_literal(pair: Pair<Rule>) -> i32 {
        pair.as_str().parse().unwrap()
    }
}
