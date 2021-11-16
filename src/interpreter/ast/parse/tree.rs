use super::super::super::actions::Actions;
use super::super::super::objects::MoonObject;

#[derive(Debug)]
pub struct Node<'a> {
    pub left: Option<Box<Node<'a>>>,
    pub right: Option<Box<Node<'a>>>,
    pub payload: Option<Payload<'a>>,
}

#[derive(Debug)]
pub enum Payload<'a> {
    Action(Actions),
    Var(Var<'a>),
    Vars(Vec<Var<'a>>),
}

#[derive(Debug)]
pub enum Var<'a> {
    Ref(String),
    Object(MoonObject<'a>)
}

impl Node<'_> {
    pub fn new(obj: Payload) -> Node {
        Node {
            left: None,
            right: None,
            payload: Some(obj),
        }
    }

    pub fn get_exception(content: String) -> Node<'static> {
        Node {
            left: None,
            right: None,
            payload: Some(Payload::Action(Actions::Panic(content))),
        }
    }

    pub fn action_branches(act: Actions, left: Payload<'static>, right: Payload<'static>) -> Node<'static> {
        Node {
            left: Some(Box::new(Node::new(left))),
            right: Some(Box::new(Node::new(right))),
            payload: Some(Payload::Action(act)),
        }
    }
}
