use std::{path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
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
    Cmp { file1: PathBuf, file2: PathBuf },
}



impl Commands {
    pub fn new() -> Self{
        return Commands::Range { file1: None }
    }
}