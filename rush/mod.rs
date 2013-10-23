use extra::getopts::groups;
use extra::fileinput::FileInput;
use std::io::stdin;
use std::vec;
use std::run;

pub fn start(args: ~[~str])
{
    let program = args[0].as_slice();
    let opts = ~[
        groups::optflag("h", "help", "display this help and exit"),
        groups::optflag("V", "version", "output version information and exit"),
    ];
    let matches = match groups::getopts(args.tail(), opts)
    {
        Ok(m) => m,
        Err(f) => fail2!(f.to_err_msg()),
    };
    if matches.opt_present("h") || matches.opt_present("help")
    {
        println("rush 1.0.0");
        println("");
        println("Usage:");
        println!("  {:s}", program);
        println("");
        print(groups::usage("Rush Shell", opts));
        return;
    }
    if matches.opt_present("V") || matches.opt_present("version")
    {
        println("rush 1.0.0");
        return;
    }

    println("Rush started! Press Ctrl+C or type 'quit' to quit.");

    let mut instream = stdin();
    if !matches.free.is_empty()
    {
        instream = @FileInput::from_args() as @Reader;
    }

    while !instream.eof()
    {
        print("> ");
        let line = instream.read_line();
        if !instream.eof()
        {
            match exec_command(line)
            {
                CommandStop => { break; }
                _ => {}
            }
        }
        else
        {
            println("");
        }
    }
}

enum CommandResult
{
    CommandOk,
    CommandStop
}

fn exec_command(command: ~str) -> CommandResult
{
    match command
    {
        ~"exit" => { CommandStop }
        _ => {
            let mut words: ~[~str] = ~[];

            for word in command.word_iter()
            {
                words = vec::append_one(words, word.into_owned());
            }

            let program = words[0].as_slice();
            let args = words.slice_from(1);

            let mut options = run::ProcessOptions::new();
            options.in_fd = Some(0);
            options.out_fd = Some(1);
            options.err_fd = Some(2);

            run::Process::new(program, args, options);

            CommandOk
        }
    }
}
