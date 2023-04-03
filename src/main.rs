pub mod cli;
use clap::Parser;

fn main() {
    let _cli = cli::jim_cli::Jim::parse();
}
