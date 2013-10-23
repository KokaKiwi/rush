
pub type BuiltinFn = extern "Rust" fn (args: &[~str]) -> Result<bool, CommandErr>;

pub enum CommandErr
{
    CommandNotFound(~str),
}
