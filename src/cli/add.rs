use clap::{Arg, ArgAction, Command};

pub fn new() -> Command {
    Command::new("add").arg(
        Arg::new("name").action(ArgAction::Set)
    )
}
