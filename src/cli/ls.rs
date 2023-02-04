use super::args;
use clap::{Arg, ArgAction, Command};

pub fn new() -> Command {
    Command::new("ls")
        .arg(args::name())
        .arg(args::groups())
        .arg(args::email())
        .arg(args::phones())
}
