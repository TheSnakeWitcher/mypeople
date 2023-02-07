use super::args;
use clap::Command;

pub fn new() -> Command {
    Command::new("ls").arg(args::name())
}
