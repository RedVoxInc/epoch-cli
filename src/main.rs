mod cli;

use epoch_cli::errors::Result;

fn main() -> Result<()> {
    cli::run()
}
