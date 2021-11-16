use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod actions;
pub mod objects;
pub mod ast;

use ast::parse::{Var, Node, Payload};
use actions::Actions;
pub use objects::{MoonArgs, MoonObject, MoonResult};

mod builtins;

pub struct Interpreter<'a> {
    mem: Vec<HashMap<String, MoonObject<'a>>>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Interpreter<'a> {
        Interpreter {
            mem: vec![HashMap::from([])],
        }
    }

    fn get_var(&'a self, var: &String) -> &'a MoonObject<'a> {
        let mem = &self.mem;
        for level in mem.into_iter().rev() {
            let vec: Vec<&String> = level.keys().collect();
            if vec.contains(&var) {
                return &level[var];
            }
        }
        panic!("variable `{}` not found!", var);
    }

    pub fn parse_file(&self, filename: &String) {
        // Create a path to the desired file
        let path = Path::new(filename);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        let tokens = match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => ast::main(s),
        };
        
        println!("{:?}", tokens);




    }
}

// fn parse_node(mem: &mut Vec<HashMap<String, MoonObject>>, action: Node) {
//     let a = action.payload.unwrap();
//     match a {
//         Payload::Action(Actions::ASSIGN) => {
//             let var_name = action.left.unwrap().payload.unwrap();
//             let assign_value = action.right.unwrap().payload.unwrap();

//             mem.last_mut().unwrap().insert(
//                 match var_name {
//                     Payload::Var(value) => value,
//                     _ => panic!("something went wrong"),
//                 },
//                 MoonObject::String(match assign_value {
//                     Payload::Const(value) => match value {
//                         Constant::String(value2) => value2,
//                         _ => panic!("something went wrong"),
//                     },
//                     _ => panic!("something went wrong"),
//                 }),
//             );
//         }
//         Payload::Action(Actions::CALL) => {
//             let func_var = action.left.unwrap().payload.unwrap();
//             let args_vec = action.right.unwrap().payload.unwrap();

//             let args = if let Payload::Vars(a) = args_vec {
//                 a
//             } else {
//                 vec![]
//             };

//             if let Payload::Var(name) = func_var {
//                 let vars: MoonArgs = args.iter().map(|a| get_mem(mem, a)).collect();
//                 let func_holder = get_mem(&mem, &name);
//                 if let MoonObject::Function(func) = func_holder {
//                     func(vars);
//                 }
//             }
//         }

//         _ => {}
//     }
// }
