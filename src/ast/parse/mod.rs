use super::lex::TType;
use super::lex::Token;
use super::super::interpreter::actions::Actions;
use super::super::interpreter::objects::MoonObject;

pub mod tree;
pub use tree::{ASTPayload, AST, Constant};

pub fn main(tokens: Vec<Vec<Token>>) -> Vec<AST> {
    let mut tree: Vec<AST> = Vec::new();

    for action in tokens {
        tree.push(parse_line(action))
    }

    tree
}

fn parse_line(tokens: Vec<Token>) -> AST {

    let var = &tokens[0];
    let action = &tokens[1];
    let value = &tokens[2];

    match action.ttype {
        TType::EQUALS => {
            
            
            AST {
                left: Some(Box::new(AST {
                    left: None,
                    right: None,
                    payload: Some(ASTPayload::Var(var.token.to_string()))
                })),
                right: Some(Box::new(AST {
                    left: None,
                    right: None,
                    payload: Some(ASTPayload::Const(Constant::String(value.token.to_string())))
                })),
                payload: Some(ASTPayload::Action(Actions::ASSIGN))
            }
            
        },
        TType::OPEN_PAREN => {
            
            AST {
                left: Some(Box::new(AST {
                    left: None,
                    right: None,
                    payload: Some(ASTPayload::Var(var.token.to_string()))
                })),
                right: Some(Box::new(AST {
                    left: None,
                    right: None,
                    payload: Some(ASTPayload::Vars(vec![value.token.to_string()]))
                })),
                payload: Some(ASTPayload::Action(Actions::CALL))
            }
        },

        _ => panic!("Syntax error"),
    }
}