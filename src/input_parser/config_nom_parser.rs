use std::collections::HashMap;

use super::Recipe;
use nom::bytes::complete::tag;
use nom::{
    branch::alt, character::complete::*, combinator::opt, number::complete::double, sequence::*,
    IResult, InputTakeAtPosition,
};

fn token(i: &str) -> IResult<&str, Recipe> {
    let (i, _) = multispace0(i)?;
    let (i, tok) = alt((tag("RERR"), tag("AERR"), tag("SPD")))(i)?;

    let tok = match tok {
        "RERR" => Recipe::RError,
        "AERR" => Recipe::AError,
        "SPD" => Recipe::Speed,
        _ => panic!("Unreach"),
    };

    Ok((i, tok))
}

fn find_middle(i: &str) -> IResult<&str, (String, Recipe)> {
    let (rgt, lft) = i.split_at_position(|x| x == ':')?;
    let name = lft.split_ascii_whitespace().rev().next().unwrap_or("");
    let (left, rec) = token(&rgt[1..])?;

    Ok((left, (name.to_owned(), rec)))
}

pub fn get_config(mut i: &str) -> HashMap<String, Vec<Recipe>> {
    let mut result: HashMap<String, Vec<Recipe>> = HashMap::new();
    while let Ok((j, (name, recipe))) = find_middle(i) {
        match result.get_mut(&name) {
            Some(x) => x.push(recipe),
            None => {result.insert(name.to_owned(), vec![recipe]);},
        }
        i = j;
    }
    result
}

#[cfg(test)]
mod test {
    use super::get_config;

    #[test]
    fn test_get_config() {
        let input = r"
Cycle: SPD
            A1  : CMP
A2: CMP        
            ";

        let res = get_config(input);
        println!("{:?}", res);
    }
}
