mod commands;
mod config;

use crate::config::cli::Cli;

fn main() {
    let cli = Cli::run();
    cli.execute();
}
