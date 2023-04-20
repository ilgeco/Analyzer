use std::collections::HashMap;

use crate::{util::retrive_string, input_parser::{parse, id_nom_builder::IdNomListBuilder, Series}};


fn range_impl<'a, It>(iter : It)
-> HashMap<&'a String, Option<(f64, f64, f64)>> where 
    It : Iterator<Item = (&'a String, &'a Vec<f64>)>,
{
    
    let mut res = HashMap::new();

    let range_finder = |(ml, zl, bl): (f64, f64, f64), (mr, zr, br): (f64, f64, f64)| {
        let min = ml.min(mr);
        let zero = if (zl - 0.0).abs() > (zr - 0.0).abs() {
            zr
        } else {
            zl
        };
        let max = bl.max(br);
        return (min, zero, max);
    };

    for series in iter {
        let range = series.1.iter().map(|&x| (x, x, x)).reduce(range_finder);
        res.insert(series.0, range);
    }
    res

}


pub fn handle_range(file1: Option<std::path::PathBuf>) {
    let input = retrive_string(file1);
    let series = parse(&input, IdNomListBuilder);


    let vals = range_impl(series.iter());

    for val in vals.into_iter() {
        if let Some((min, zero, max)) = val.1 {
            println!("{} -> ({:e}, {:e} ,{:e})", val.0, min, zero, max)
        }
    }
}
