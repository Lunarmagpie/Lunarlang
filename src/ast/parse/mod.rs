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

    match tokens[0].ttype {
        TType::FuncDef => {


        },

        _ => panic!("Syntax error"),
    };

    panic!();
}