mod args;
mod add;
mod ls;
mod rm;

pub use clap;
use clap::builder::{Arg, ArgAction, ArgGroup, Command};

const NAME: &'static str = "mypeople";
const ABOUT: &'static str = "a fictional versioning cli";

pub fn new() -> Command {
    Command::new(NAME)
        .about(ABOUT)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(ls::new())
        .subcommand(add::new())
}


