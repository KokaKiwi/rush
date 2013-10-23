use rush::shared::*;

pub static NAME: &'static str = "exit";

pub fn builtin_exit(_args: &[~str]) -> Result<bool, CommandErr>
{
    println("Goodbye. :)");
    Ok(true)
}
