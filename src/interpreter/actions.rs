#[derive(Debug)]
pub enum Actions {
    ScopeUp,
    ScopeDown,

    Call,
    FuncDef,
    Assign,


    Panic(String),
}