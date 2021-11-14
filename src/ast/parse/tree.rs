use super::super::interpreter::objects::MoonObject;
use super::super::interpreter::actions::Actions;

#[derive(Debug)]
pub struct AST {
    pub left: Option<Box<AST>>,
    pub right: Option<Box<AST>>,
    pub payload: Option<ASTPayload>,
}

#[derive(Debug)]
pub enum Constant {
    String(String),
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub enum ASTPayload {
    Action(Actions),
    Const(Constant),
    Var(String),
    Vars(Vec<String>),
}

impl AST {
    pub fn new(obj: ASTPayload) -> AST {
        AST {
            left: None,
            right: None,
            payload: Some(obj),
        }
    }
}
