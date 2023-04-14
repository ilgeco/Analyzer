#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;


use clap::Parser;

use crate::{cli::Cli, mode::dispatch};


mod cli;
mod input_parser;
mod mode;
mod util;

fn main() {
    let cli = Cli::parse();
    dispatch(cli.command);
}
