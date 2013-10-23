use std::os;
use rush::shell::CommandErr;

pub static NAME: &'static str = "exit";

pub fn builtin(args: &[~str]) -> Result<bool, CommandErr>
{
    println("Goodbye. :)");

    let mut status = 0;
    if args.len() == 1
    {
        status = match from_str(args[0]) {
            Some(code) => code,
            None => 1,
        };
    }

    os::set_exit_status(status);

    Ok(true)
}
