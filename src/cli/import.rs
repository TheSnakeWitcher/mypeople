use super::args;
use clap::Command;

pub fn new() -> Command {
    Command::new("import").arg(args::path())
}
