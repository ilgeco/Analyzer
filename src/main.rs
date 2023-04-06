use clap::Parser;

use crate::{cli::Cli, mode::dispatch};

#[macro_use]
extern crate lazy_static;

mod cli;
mod input_parser;
mod mode;
mod util;

fn main() {
    let cli = Cli::parse();
    dispatch(cli.command);
    println!("Hello, world!");
}
