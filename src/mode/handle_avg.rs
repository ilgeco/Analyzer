use std::collections::HashMap;

use crate::{
    input_parser::{id_nom_builder::IdNomListBuilder, parse, Series},
    util::retrive_string,
};

use super::required_op;

fn avg_impl<'a, It, Num>(iter: It) -> HashMap<String, Option<Num>>
where
    It: Iterator<Item = (&'a String, &'a Vec<Num>)>,
    Num : required_op::Operation<'a> + 'a
{
    let mut res = HashMap::with_capacity(100);

    for series in iter {
        let avg = series.1.iter().sum::<Num>().divu(series.1.len());
        res.insert(series.0.to_owned(), Some(avg));
    }

    
    res
}
pub fn handle_avg(file1: Option<std::path::PathBuf>) -> HashMap<String, Option<f64>>  {
    let input = retrive_string(file1);
    avg_impl(parse(&input, IdNomListBuilder).into_iter())


    // for (name, avg) in avg_impl(series.into_iter()) {
        
    //     println!("{} -> {:e}", name, avg.unwrap())
    // }
}
