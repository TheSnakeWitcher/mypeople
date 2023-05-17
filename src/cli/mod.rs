mod add;
mod args;
mod config;
mod export;
mod import;
mod init;
mod ls;
mod rm;

pub use clap;
use clap::Command;

const NAME: &'static str = "mypeople";
const ABOUT: &'static str = "contact book";

pub fn new() -> Command {
    Command::new(NAME)
        .about(ABOUT)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(init::new())
        .subcommand(export::new())
        .subcommand(import::new())
        .subcommand(ls::new())
        .subcommand(add::new())
        .subcommand(rm::new())
        .subcommand(config::new())
}
