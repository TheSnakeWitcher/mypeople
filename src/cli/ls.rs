use super::args;
use clap::{Command,ArgAction};

pub fn new() -> Command {
    Command::new("ls")
        .arg(args::name())
        .arg(args::pic(ArgAction::SetTrue))
        .arg(args::groups(ArgAction::SetTrue))
        .arg(args::phones(ArgAction::SetTrue))
        .arg(args::emails(ArgAction::SetTrue))
        .arg(args::social_nets(ArgAction::SetTrue))
        .arg(args::wallets(ArgAction::SetTrue))
        .arg(args::notes(ArgAction::SetTrue))
}
