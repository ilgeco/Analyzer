use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Sub,
    path::{Path, PathBuf},
};

use crate::{
    input_parser::{
        id_nom_builder::IdNomListBuilder, parse, parse_config, Recipe, RecipeResult, Recipes,
        Series,
    },
    util::retrive_file,
};

macro_rules! map_insert_or {
    ( $hash : ident, $key : expr, $value : expr ) => {
        match $hash.get_mut($key) {
            Some(x) => {
                x.push($value);
            }
            None => {
                $hash.insert($key.to_owned(), vec![$value]);
            }
        }
    };
}

use super::required_op;

fn cmp_impl<'a, It, Num>(
    iter: It,
    series: impl Series<String, Num>,
    recipes: Recipes,
) -> HashMap<String, Vec<RecipeResult<Num>>>
where
    It: Iterator<Item = (&'a String, &'a Vec<Num>)>,
    Num: required_op::Operation<'a> + 'a + Display + Debug,
{
    let mut res: HashMap<String, Vec<RecipeResult<Num>>> = HashMap::with_capacity(100);

    for (name, value) in iter {
        if let Some(cmp_elems) = series.get_serie(name) {
            let mut tot_err: Num = Num::zero();
            let mut tot_rel = Num::zero();
            let mut size_e = 0;
            let mut size_r = 0;
            let mut time_f = Num::zero();
            let mut time_s = Num::zero();
            let mut tot = 0;

            for (&base, &cmp) in value.iter().zip(cmp_elems.iter()) {
                tot_err = tot_err + cmp.sub(base).abs();

                if base.abs() > Num::small_eps() {
                    tot_rel = tot_rel + cmp.sub(base).div(base.abs()).abs();
                    size_r += 1;
                }
                time_f = time_f.add(base);
                time_s = time_s.add(cmp);
                size_e += 1;
                tot += 1;
            }

            for recipe in recipes
                .get(name)
                .map(|x| x.as_slice())
                .unwrap_or(&[Recipe::RError, Recipe::AError])
                .iter()
            {
                match recipe {
                    Recipe::RError => {
                        if size_r != 0 {
                            map_insert_or!(
                                res,
                                name,
                                RecipeResult::RError(tot_rel.divu(size_r).mulu(100))
                            );
                        }
                    }
                    Recipe::AError => {
                        if size_e != 0 {
                            map_insert_or!(res, name, RecipeResult::AError(tot_err.divu(size_e)));
                        }
                    }
                    Recipe::Speed => {
                        if tot != 0 {
                            let avg_time_f = time_f.divu(tot);
                            let avg_time_s = time_s.divu(tot);
                            map_insert_or!(
                                res,
                                name,
                                RecipeResult::Speed(avg_time_f.div(avg_time_s))
                            );
                        }
                    }
                }
            }
        }
    }
    res
}

pub fn handle_cmp(
    file1: &Path,
    file2: &Path,
    config: Option<PathBuf>,
) -> HashMap<String, Vec<RecipeResult<f64>>> {
    let f_list = parse(&retrive_file(file1), IdNomListBuilder);
    let s_list = parse(&retrive_file(file2), IdNomListBuilder);
    let config = match config {
        Some(path) => parse_config(&retrive_file(path)),
        None => Recipes::new(),
    };
    cmp_impl(f_list.into_iter(), s_list, config)
}
