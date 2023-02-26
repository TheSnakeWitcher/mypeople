use super::args;
use clap::Command;

pub fn new() -> Command {
    Command::new("init").arg(args::path())
}
