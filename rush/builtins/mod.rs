use std::hashmap::HashMap;
use rush::shared::BuiltinFn;

pub mod exit;

pub fn create_builtins() -> ~HashMap<~str, BuiltinFn>
{
    let mut builtins: HashMap<~str, BuiltinFn> = HashMap::new();

    builtins.insert(exit::NAME.to_owned(), exit::builtin_exit);

    ~builtins
}

#[test]
fn test_builtins()
{
    let builtins = create_builtins();
    assert_eq!(builtins.len(), 1);
}
