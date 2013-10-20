#[link(
    name = "rush",
    vers = "0.1.0",
    uuid = "636e5f0c-2815-11e3-989c-6bd39c248c79"
)];

#[desc = "A shell written in Rust"];
#[license = "GPLv3"];

extern mod extra;
use std::os;
mod rush;

fn main()
{
    let args: ~[~str] = os::args();

    ::rush::start(args);
}
