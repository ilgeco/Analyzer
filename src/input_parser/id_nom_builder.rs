use nom::{
    character::complete::*, number::complete::double, sequence::*, IResult, InputTakeAtPosition,
};

use super::{CreateSeries, IdList};
pub struct IdNomListBuilder;

impl CreateSeries<String, f64, IdList> for IdNomListBuilder {
    
    fn create_series(mut input: &str) -> IdList {
        let mut res = IdList::new();
        while let Ok((name,Some((i, num)))) = get_next_id_number(input) {
            match res.series.get_mut(name) {
                Some(serie) => serie.push(num),
                None => {
                    let mut tmp = Vec::with_capacity(100);
                    tmp.push(num);
                    res.series.insert(name.to_owned(), tmp);
                },
            }
            input = i;
        }
        res
    }
}

#[derive(Debug)]
enum PStart<'a> {
    Number(&'a str),
    Column(&'a str, &'a str),
}

fn get_float(i: &str) -> IResult<&str, f64> {
    preceded(multispace0, double)(i)
}

fn find_possible_start(i: &str) -> IResult<&str, PStart<'_>> {
    let init = |x: char| x.is_ascii_digit() || x == ':' || x == '-' || x == '+';
    let (rgt, lft) = i.split_at_position(init)?;
    let o = if rgt.chars().next().unwrap() == ':' {
        let name = lft.split_ascii_whitespace().rev().next().unwrap_or("");
        PStart::Column(name, &rgt[1..])
    } else {
        PStart::Number(rgt)
    };

    Ok((i, o))
}

fn try_get_id_number(i: &str) -> IResult<&str, (&str, f64)> {
    let (_, o) = find_possible_start(i)?;
    let res = match o {
        PStart::Number(num) => ("", get_float(num)?),
        PStart::Column(name, num) => (name, get_float(num)?),
    };
    Ok(res)
}

fn get_next_id_number(mut i: &str) -> IResult<&str, Option<(&str, f64)>> {
    loop {
        match try_get_id_number(i) {
            Ok((i, o)) => return Ok((i, Some(o))),
            Err(x) => match x {
                nom::Err::Incomplete(_) => return Ok((i, None)),
                nom::Err::Error(e) | nom::Err::Failure(e) => if e.input.len() > 0 { i = &e.input[1..]} else { return Ok((i,None))},
                
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_possible_start_test() {
        let input = "casa: 32";
        println!("{:?}", find_possible_start(input));
        let input = ": 32.77";
        println!("{:?}", find_possible_start(input));
        let input = " 32.77";
        println!("{:?}", find_possible_start(input));
        let input = "dai come : -32.77";
        println!("{:?}", find_possible_start(input));
        let input = "ecco\n: -32.77";
        println!("{:?}", find_possible_start(input));
        let input = "-4";
        println!("{:?}", find_possible_start(input));
        let input = "-";
        println!("{:?}", find_possible_start(input));
        let input = "errore";
        println!("{:?}", find_possible_start(input));
    }

    #[test]
    fn get_id_test() {
        let input = "casa: 32";
        println!("{:?}", try_get_id_number(input));
        let input = ": 32.77";
        println!("{:?}", try_get_id_number(input));
        let input = " 32.77";
        println!("{:?}", try_get_id_number(input));
        let input = "dai come : -32.77";
        println!("{:?}", try_get_id_number(input));
        let input = "ecco\n: -32.77";
        println!("{:?}", try_get_id_number(input));
        let input = "-4";
        println!("{:?}", try_get_id_number(input));
        let input = "-";
        println!("{:?}", try_get_id_number(input));
        let input = "errore";
        println!("{:?}", try_get_id_number(input));
        let input = "come : si capisce, 44.123";
        println!("{:?}", try_get_id_number(input));
    }


    #[test]
    fn get_next_id_test() {
        let input = "casa: 32";
        
        println!("{:?}", get_next_id_number(input));
        let input = ": 32.77coccole";
        println!("{:?}", get_next_id_number(input));
        let input = " 32.77 dubbio";
        println!("{:?}", get_next_id_number(input));
        let input = "dai come : -32.77";
        println!("{:?}", get_next_id_number(input));
        let input = "ecco\n: -32.77";
        println!("{:?}", get_next_id_number(input));
        let input = "-4";
        println!("{:?}", get_next_id_number(input));
        let input = "-";
        println!("{:?}", get_next_id_number(input));
        let input = "errore";
        println!("{:?}", get_next_id_number(input));
        let input = "come : si capisce, 44.123";
        println!("{:?}", get_next_id_number(input));
        let input = "";
        println!("{:?}", get_next_id_number(input));
        let input = ":::::::::::::::::::::::::::::::---------------3-----------------------::::::::::::::::::------";
        println!("{:?}", get_next_id_number(input));
        let input = ":::::::::::::::::::::::::44::::::---------------3-----------------------::::::::::::::::::------";
        println!("{:?}", get_next_id_number(input));
    }
}
