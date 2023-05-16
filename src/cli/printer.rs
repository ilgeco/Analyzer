use std::vec;

use crate::{input_parser::RecipeResult, mode};
use ansi_term::{Colour::Red, Style};

pub(crate) fn ansi_print(res: mode::COMRESULT) {
    match res {
        mode::COMRESULT::RRANGE(res) => ansi_range_print(res),
        mode::COMRESULT::RAVG(res) => ansi_avg_print(res),
        mode::COMRESULT::RCMP(res) => ansi_cmp_print(res),
    }
}

fn ansi_cmp_print(mut res: std::collections::HashMap<String, Vec<RecipeResult<f64>>>) {
    let max_name_length = res
        .iter()
        .filter(|x| !x.1.is_empty())
        .map(|x| x.0.len())
        .max()
        .unwrap();
    
    

    let mut iter = res.iter_mut().filter(|x| !x.1.is_empty());
    println!("{}", Red.bold().paint("Comparison"));

    while let Some((name, recipies_res)) = iter.next() {
        print!(
            "{0},{2:1$}",
            Style::new()
                .italic()
                .fg(ansi_term::Color::Green)
                .paint(name),
            max_name_length - name.len() + 3,
            ""
        );
        // 39 87 63
        let mut iter = recipies_res.iter().peekable();
        while let Some(recipie) = iter.next() {
            match recipie {
                RecipeResult::RError(x) => {
                    print!("{}", ansi_term::Color::Fixed(39).paint(format!("{0:.8e}", x)))
                }
                RecipeResult::AError(x) => {
                    print!("{}", ansi_term::Color::Fixed(87).paint(format!("{0:.8e}", x)))
                }
                RecipeResult::Speed(x) => {
                    print!("{}", ansi_term::Color::Fixed(54).paint(format!("{0:.8e}", x)))
                }
            }
            if iter.peek().is_some() {
                print!(", ");
            }
        }

        println!();
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

pub(crate) fn ascii_print(res: mode::COMRESULT) {
    match res {
        mode::COMRESULT::RRANGE(res) => ascii_range_print(res),
        mode::COMRESULT::RAVG(res) => ascii_avg_print(res),
        mode::COMRESULT::RCMP(res) => ascii_cmp_print(res),
    }
}

fn ascii_cmp_print(res: std::collections::HashMap<String, Vec<RecipeResult<f64>>>) {
    let mut iter = res.iter().filter(|x| !x.1.is_empty());

    while let Some((name, recipies_res)) = iter.next() {
        print!("{name}");

        // 39 87 63
        for recipie in recipies_res {
            print!(", ");
            match recipie {
                RecipeResult::RError(x) => {
                    print!("{x}")
                }
                RecipeResult::AError(x) => {
                    print!("{x}")
                }
                RecipeResult::Speed(x) => {
                    print!("{x}")
                }
            }
        }

        println!();
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
