#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use crate::{cli::Cli, mode::dispatch};


pub mod cli;
pub mod input_parser;
pub mod mode;
pub mod util;




