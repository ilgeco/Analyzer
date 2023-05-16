use crate::mode;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod printer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = false)]
    pub ascii: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Find max min in values
    Range { file1: Option<PathBuf> },
    /// Find avg in values
    Avg { file1: Option<PathBuf> },
    /// Cmp two file
    Cmp { 
        file1: PathBuf, 
        file2: PathBuf, 
        #[arg(short, long)] 
        config: Option<PathBuf>
    },
}

impl Commands {
    pub fn new() -> Self {
        return Commands::Range { file1: None };
    }
}

pub fn pretty_print(res: mode::COMRESULT, ascii: bool) {
    match ascii {
        true => printer::ascii_print(res),
        false => printer::ansi_print(res),
    }
}
