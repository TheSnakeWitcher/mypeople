use super::args;
use clap::{ArgAction, Command};

pub fn new() -> Command {
    Command::new("add")
        .arg_required_else_help(true)
        .arg(args::name())
        .arg(args::pic(ArgAction::Set))
        .arg(args::groups(ArgAction::Set))
        .arg(args::phones(ArgAction::Set))
        .arg(args::emails(ArgAction::Set))
        .arg(args::social_nets(ArgAction::Set))
        .arg(args::wallets(ArgAction::Set))
}
