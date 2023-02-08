use super::args;
use clap::Command;

pub fn new() -> Command {
    Command::new("add")
        .arg_required_else_help(true)
        .arg(args::name())
        .arg(args::pic())
        .arg(args::groups())
        .arg(args::phones())
        .arg(args::emails())
}
