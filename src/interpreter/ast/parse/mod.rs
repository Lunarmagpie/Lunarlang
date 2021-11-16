use super::lex::TType;
use super::lex::Token;

pub mod tree;
pub use tree::{Payload, Node, Constant};

pub fn main(tokens: Vec<Vec<Token>>) -> Vec<Node> {
    let mut tree: Vec<Node> = Vec::new();

    for action in tokens {
        tree.push(parse_line(action))
    }

    tree
}

fn parse_line(tokens: Vec<Token>) -> Node {

    match tokens[0].ttype {
        TType::FuncDef => {


        },

        _ => panic!("Syntax error"),
    };

    panic!();
}