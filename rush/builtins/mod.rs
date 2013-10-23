use std::hashmap::HashMap;
use rush::shell::BuiltinFn;

pub mod exit;
mod eval;

pub fn create_builtins() -> ~HashMap<~str, BuiltinFn>
{
    let mut builtins: HashMap<~str, BuiltinFn> = HashMap::new();

    builtins.insert(exit::NAME.to_owned(), exit::builtin);
    builtins.insert(eval::NAME.to_owned(), eval::builtin);

    ~builtins
}
