use super::super::actions::Actions;
use super::super::objects::MoonObject;
use super::lex::TType;
use super::lex::Token;

pub mod tree;
pub use tree::{Var, Node, Payload};

pub fn main(tokens: Vec<Vec<Token>>) -> Vec<Node<'static>> {
    let mut action_list: Vec<Node> = Vec::new();

    for phrase in tokens {
        action_list.push(parse_phrase(phrase));
    }

    action_list
}

fn parse_phrase(tokens: Vec<Token>) -> Node<'static> {
    match tokens[0].ttype {
        TType::FuncDef => Node::new(Payload::Action(Actions::FuncDef)),

        TType::Var => {
            // See if it is being assigned to or called

            match tokens[1].ttype {
                // Assignment
                TType::Equals => {
                    panic!();
                }
                //Called
                TType::OpenParen => {

                    let var = &tokens[0].token;

                    let args = tokens[2..tokens.len()]
                        .split(|x| x.ttype == TType::Comma)
                        .map(|x| &x[0])
                        .collect::<Vec<&Token>>();

                    Node::action_branches(
                        Actions::Call,
                        Payload::Var(Var::Ref(var.to_string())),
                        Payload::Var(Var::Object(MoonObject::String("1234".to_string())))
                    )
                }

                _ => {
                    Node::get_exception(format!("Syntax error at some place"))
                }
            }
        }

        _ => Node::get_exception(format!("Syntax error at some place")),
    }
}
