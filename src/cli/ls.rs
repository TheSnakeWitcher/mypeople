use super::args;
use clap::Command;

pub fn new() -> Command {
    Command::new("ls")
        .arg(args::name())
        .arg(args::groups())
        .arg(args::emails())
        .arg(args::phones())
}
