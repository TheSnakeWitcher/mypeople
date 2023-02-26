use super::args;
use clap::Command;

pub fn new() -> Command {
    Command::new("export").arg(args::path())
}
