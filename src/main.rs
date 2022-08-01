pub mod amounts;
pub mod check;
pub mod defs;
pub mod disk;
pub mod cli;
pub mod net_client;
pub mod core;
pub mod net_verify;

use clap::Parser;
fn main() {
    let mut cli = cli::Cli::parse();
    cli::elaborate(&mut cli);
}
