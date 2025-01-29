extern crate prettytable;
use clap::Parser;
use partitionElection::cli::{self, Cli};

fn main() {
    let args = Cli::parse();
    cli::run(args);
}
