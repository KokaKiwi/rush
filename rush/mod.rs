
fn usage(args: ~[~str])
{
    println(format!("Usage: {exe_name}", exe_name = args[0]));
    fail!();
}

struct Options;

pub fn start(args: ~[~str])
{
    if args.len() > 1
    {
        usage(args);
    }

    let opts: Options = Options;

    main(&opts);
}

fn main(_: &Options)
{

}
