use super::super::interpreter::objects::MoonObject;
use super::super::interpreter::actions::Actions;

#[derive(Debug)]
pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub payload: Option<Payload>,
}

#[derive(Debug)]
pub enum Constant {
    String(String),
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub enum Payload {
    Action(Actions),
    Const(Constant),
    Var(String),
    Vars(Vec<String>),
}

impl Node {
    pub fn new(obj: Payload) -> Node {
        Node {
            left: None,
            right: None,
            payload: Some(obj),
        }
    }
}
