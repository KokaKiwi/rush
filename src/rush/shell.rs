use std::hashmap::HashMap;
use std::vec;
use std::run;
use rush::builtins;
use rush::builtins::exit;

pub type BuiltinFn = extern fn (args: &[~str]) -> Result<bool, CommandErr>;

struct Shell
{
    prompt: ~str,

    builtins: ~HashMap<~str, BuiltinFn>,
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
        self.exec(line)
    }

    // command will be replaced by a Command type (pipe, redirs, etc...)
    fn exec(&self, command: ~str) -> Result<bool, CommandErr>
    {
        let mut words: ~[~str] = ~[];

        for word in command.word_iter()
        {
            words = vec::append_one(words, word.to_owned());
        }

        let program = words[0].to_owned();
        let args = words.slice_from(1);

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
