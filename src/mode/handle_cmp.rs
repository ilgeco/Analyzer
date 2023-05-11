use std::{collections::HashMap, path::Path, ops::Sub};

use crate::{
    input_parser::{id_nom_builder::IdNomListBuilder, parse, Series},
    util::{retrive_file},
};

use super::required_op;



fn cmp_impl<'a, It, Num>(iter: It, series: impl Series<String, Num>) -> HashMap<String, Option<(Num, Num)>>
where
    It: Iterator<Item = (&'a String, &'a Vec<Num>)>,
    Num : required_op::Operation<'a> + 'a
{
    let mut res = HashMap::with_capacity(100);

    for (name, value) in iter {
        if let Some(cmp_elems) = series.get_serie(name) {
            let mut tot_err = Num::zero();
            let mut tot_rel = Num::zero();
            let mut size_e = 0;
            let mut size_r = 0;

            for (&base, &cmp) in value.iter().zip(cmp_elems.iter()) {
                tot_err =  base.sub(cmp).abs();

                if base != Num::zero() {
                    tot_rel = tot_rel + ((cmp - base) / base).abs();
                    size_r += 1;
                }
                size_e += 1;
            }
            if size_r != 0 && size_e != 0 {
                res.insert(name.to_owned(), Some(( tot_rel.divu(size_r), tot_err.divu(size_e))));

            }else{
                res.insert(name.to_owned(), None);
            }
        }
    }
    res
}



pub fn handle_cmp(file1: &Path, file2: &Path) -> HashMap<String, Option<(f64, f64)>> {
    let f_list = parse(&retrive_file(file1), IdNomListBuilder);
    let s_list = parse(&retrive_file(file2), IdNomListBuilder);


    cmp_impl(f_list.into_iter(), s_list)

    
    // for elem in res.into_iter(){
    //     if let (nome, Some((avg, abs))) = elem {
    //         println!("{}, {}, {}", nome, avg, abs);
    //     }
    // }

}

