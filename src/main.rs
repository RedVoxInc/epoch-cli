mod cli;

use clap::Parser;
use epoch_cli::errors::Result;

fn main() -> Result<()> {
    cli::run(cli::Cli::parse())
}
