use super::args;
use clap::{Arg, ArgAction, Command};

pub fn new() -> Command {
    Command::new("add")
        .arg_required_else_help(true)
        .arg(args::name())
}
