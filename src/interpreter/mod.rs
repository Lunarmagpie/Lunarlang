use std::collections::HashMap;

pub mod actions;
pub mod objects;

use super::ast::parse::{Payload, Constant, Node};
use actions::Actions;
pub use objects::{MoonArgs, MoonObject, MoonResult};

mod builtins;
use builtins::*;

pub fn main(ast: Vec<Node>) {
    let mut mem: Vec<HashMap<String, MoonObject>> = vec![HashMap::from([(
        "echo".to_string(),
        MoonObject::Function(&echo),
    )])];

    for action in ast {
        parse_node(&mut mem, action);
    }
}

fn get_mem<'a>(mem: &'a Vec<HashMap<String, MoonObject>>, var: &String) -> &'a MoonObject<'a> {
    for level in mem.into_iter().rev() {
        let vec: Vec<&String> = level.keys().collect();
        if vec.contains(&var) {
            return &level[var];
        }
    }

    panic!("variable `{}` not found!", var);
}

fn parse_node(mem: &mut Vec<HashMap<String, MoonObject>>, action: Node) {
    let a = action.payload.unwrap();
    match a {
        Payload::Action(Actions::ASSIGN) => {
            let var_name = action.left.unwrap().payload.unwrap();
            let assign_value = action.right.unwrap().payload.unwrap();

            mem.last_mut().unwrap().insert(
                match var_name {
                    Payload::Var(value) => value,
                    _ => panic!("something went wrong"),
                },
                MoonObject::String(match assign_value {
                    Payload::Const(value) => match value {
                        Constant::String(value2) => value2,
                        _ => panic!("something went wrong"),
                    },
                    _ => panic!("something went wrong"),
                }),
            );
        }
        Payload::Action(Actions::CALL) => {
            let func_var = action.left.unwrap().payload.unwrap();
            let args_vec = action.right.unwrap().payload.unwrap();

            let args = if let Payload::Vars(a) = args_vec {
                a
            } else {
                vec![]
            };

            if let Payload::Var(name) = func_var {
                let vars: MoonArgs = args.iter().map(|a| get_mem(mem, a)).collect();
                let func_holder = get_mem(&mem, &name);
                if let MoonObject::Function(func) = func_holder {
                    func(vars);
                }
            }
        }

        _ => {}
    }
}
