use super::args;
use clap::Command;

pub fn new() -> Command {
    Command::new("ls")
        .arg(args::name())
        .arg(args::pic())
        .arg(args::groups())
        .arg(args::phones())
        .arg(args::emails())
        .arg(args::social_nets())
        .arg(args::wallets())
}
