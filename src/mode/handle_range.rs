use std::{collections::HashMap, ops};

use crate::{
    input_parser::{id_nom_builder::IdNomListBuilder, parse, Series},
    util::retrive_string,
};

use super::required_op;



fn range_impl<'a, It, Num>(iter: It) -> HashMap<String, Option<(Num, Num, Num)>>
where
    It: Iterator<Item = (&'a String, &'a Vec<Num>)>,
    Num : required_op::Operation<'a> + 'a
{
    let mut res = HashMap::with_capacity(100);
    

    let range_finder = |(ml, zl, bl): (Num, Num, Num), (mr, zr, br): (Num, Num, Num)| {
        let min = ml.min(mr);
        
        let zero = if (zl - Num::zero()).abs() > (zr - Num::zero()).abs() {
            zr
        } else {
            zl
        };
        let max = bl.max(br);
        return (min, zero, max);
    };

    for series in iter {
        let range = series.1.iter().map(|&x| (x, x, x)).reduce(range_finder);
        res.insert(series.0.to_owned(), range);
    }
    res
}

pub fn handle_range(file1: Option<std::path::PathBuf>) -> HashMap<String, Option<(f64, f64, f64)>> {
    let input = retrive_string(file1);
    let series = parse(&input, IdNomListBuilder);

    range_impl(series.into_iter())

    // for val in vals.into_iter() {
    //     if let Some((min, zero, max)) = val.1 {
    //         println!("{} -> ({:e}, {:e} ,{:e})", val.0, min, zero, max)
    //     }
    // }
}
