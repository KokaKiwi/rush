use extra::getopts::groups;
use extra::fileinput::FileInput;
use std::io::stdin;

mod builtins;
mod shell;

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

    println("Rush started! Press Ctrl+D or type 'quit' to quit.");

    let mut instream = stdin();
    if !matches.free.is_empty()
    {
        instream = @FileInput::from_args() as @Reader;
    }

    let shell = shell::Shell::new(instream);
    shell.run();
}
