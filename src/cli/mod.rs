use std::path::PathBuf;

use ansi_term::{Colour::Red, Style};
use clap::{Parser, Subcommand};

use crate::mode;

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
    Cmp { file1: PathBuf, file2: PathBuf },
}

impl Commands {
    pub fn new() -> Self {
        return Commands::Range { file1: None };
    }
}

pub fn pretty_print(res: mode::COMRESULT, ascii: bool) {
    match ascii {
        true => ascii_print(res),
        false => ansi_print(res),
    }
}

fn ansi_print(res: mode::COMRESULT) {
    match res {
        mode::COMRESULT::RRANGE(res) => ansi_range_print(res),
        mode::COMRESULT::RAVG(res) => ansi_avg_print(res),
        mode::COMRESULT::RCMP(res) => ansi_cmp_print(res),
    }
}

fn ansi_cmp_print(res: std::collections::HashMap<String, Option<(f64, f64)>>) {
    let max_name_length = res
        .iter()
        .filter(|x| x.1.is_some())
        .map(|x| x.0.len())
        .max()
        .unwrap();

    let mut iter = res.iter().filter(|x| x.1.is_some());
    println!("{}", Red.bold().paint("Comparison"));
    while let Some((name, Some((rel, abs)))) = iter.next() {
        println!(
            "{0},{2:1$}{rel:.8e}, {abs:.8e}",
            Style::new()
                .italic()
                .fg(ansi_term::Color::Green)
                .paint(name),
                max_name_length - name.len() + 3,
                ""
        );
    }
}

fn ansi_avg_print(res: std::collections::HashMap<String, Option<f64>>) {
    let max_name_length = res
        .iter()
        .filter(|x| x.1.is_some())
        .map(|x| x.0.len())
        .max()
        .unwrap();

    let mut iter = res.iter().filter(|x| x.1.is_some());
    println!("{}", Red.bold().paint("Average"));
    while let Some((name, Some(avg))) = iter.next() {
        println!(
            "{0},{2:1$}{avg:.8e}",
            Style::new()
                .italic()
                .fg(ansi_term::Color::Green)
                .paint(name),
                max_name_length - name.len() + 3,
                ""
        );
    }
}

fn ansi_range_print(res: std::collections::HashMap<String, Option<(f64, f64, f64)>>) {
    let max_name_length = res
        .iter()
        .filter(|x| x.1.is_some())
        .map(|x| x.0.len())
        .max()
        .unwrap();

    let mut iter = res.iter().filter(|x| x.1.is_some());
    println!("{}", Red.bold().paint("Comparison"));
    while let Some((name, Some((min, near_zero, max)))) = iter.next() {
        println!(
            "{0},{2:1$}{min:.8e}, {near_zero:.8e}, {max:.8e}",
            Style::new()
                .italic()
                .fg(ansi_term::Color::Green)
                .paint(name),
                max_name_length - name.len() + 3,
                ""
        );
    }
}

fn ascii_print(res: mode::COMRESULT) {
    match res {
        mode::COMRESULT::RRANGE(res) => ascii_range_print(res),
        mode::COMRESULT::RAVG(res) => ascii_avg_print(res),
        mode::COMRESULT::RCMP(res) => ascii_cmp_print(res),
    }
}

fn ascii_cmp_print(res: std::collections::HashMap<String, Option<(f64, f64)>>) {
    let mut iter = res.iter().filter(|x| x.1.is_some());
    while let Some((name, Some((rel, abs)))) = iter.next() {
        println!("{name}, {rel}, {abs}");
    }
}

fn ascii_avg_print(res: std::collections::HashMap<String, Option<f64>>) {
    let mut iter = res.iter().filter(|x| x.1.is_some());
    while let Some((name, Some(avg))) = iter.next() {
        println!("{name}, {avg}");
    }
}

fn ascii_range_print(res: std::collections::HashMap<String, Option<(f64, f64, f64)>>) {
    let mut iter = res.iter().filter(|x| x.1.is_some());
    while let Some((name, Some((min, near_zero, max)))) = iter.next() {
        println!("{name}, {min}, {near_zero}, {max}");
    }
}
