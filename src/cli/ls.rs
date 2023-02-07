use super::args;
use clap::Command;

pub fn new() -> Command {
    Command::new("ls")
        .arg(args::name())
        .arg(args::emails().default_missing_value("true"))
        .arg(args::groups().default_missing_value("true"))
        .arg(args::phones().default_missing_value("true"))
}
