use super::args;
use clap::Command;

pub fn new() -> Command {
    Command::new("rm")
        .arg_required_else_help(true)
        .arg(args::name())
        .arg(args::groups())
        .arg(args::phones())
        .arg(args::emails())
        .arg(args::social_nets())
        .arg(args::wallets())
}
