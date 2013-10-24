use std::hashmap::HashMap;
use std::run;
use rush::builtins;
use rush::builtins::exit;
use rush::parser::*;

pub type BuiltinFn = extern fn (args: &[~str]) -> Result<bool, CommandErr>;

struct Shell
{
    prompt: ~str,
    builtins: ~HashMap<~str, BuiltinFn>,

    parser: ~Parser,
}

pub enum CommandErr
{
    CommandNotFound(~str),
}

impl Shell
{
    pub fn new() -> Shell
    {
        Shell {
            prompt: ~"$ ",
            builtins: builtins::create_builtins(),

            parser: ~Parser::new(),
        }
    }

    pub fn run(&self, instream: @Reader, show_prompt: bool)
    {
        if show_prompt
        {
            println("Rush started! Press Ctrl+D or type 'quit' to quit.");
        }

        while !instream.eof()
        {
            if show_prompt
            {
                print(self.prompt);
            }

            let line = instream.read_line();
            if !instream.eof()
            {
                match self.exec_line(line)
                {
                    Ok(stop) if stop => { break; }
                    Ok(_) => {}
                    Err(e) => {
                        match e
                        {
                            CommandNotFound(command) => {
                                println!("Command not found: {:s}", command);
                            }
                        }
                    }
                }
            }
            else
            {
                exit::builtin([]);
            }
        }
    }

    pub fn exec_line(&self, line: ~str) -> Result<bool, CommandErr>
    {
        match self.parser.parse(line)
        {
            Ok(command) => self.exec(command),
            Err(e) => fail!("Error: {:?}", e),
        }
    }

}

impl Shell
{
    fn exec(&self, command: Command) -> Result<bool, CommandErr>
    {
        match command
        {
            Simple(cmd) => self.exec_simple(cmd),
            _ => Ok(false),
        }
    }

    fn exec_simple(&self, command: ~[~str]) -> Result<bool, CommandErr>
    {
        let program = command[0].to_owned();
        let args = command.slice_from(1);

        match self.builtins.find(&program)
        {
            Some(handler) => (*handler)(args),
            None => self.exec_process(program, args),
        }
    }

    fn exec_process(&self, program: ~str, args: &[~str]) -> Result<bool, CommandErr>
    {
        let mut options = run::ProcessOptions::new();
        options.in_fd = Some(0);
        options.out_fd = Some(1);
        options.err_fd = Some(2);

        let _process = run::Process::new(program, args, options);

        Ok(false)
    }
}
