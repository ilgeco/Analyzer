use std::{collections::HashMap, path::Path, ops::Sub};

use crate::{
    input_parser::{id_nom_builder::IdNomListBuilder, parse, Series},
    util::{retrive_file},
};

use super::required_op;





pub fn handle_cmp(file1: &Path, file2: &Path) {
    let f_list = parse(&retrive_file(file1), IdNomListBuilder);
    let s_list = parse(&retrive_file(file2), IdNomListBuilder);

    for (name, base_elems) in f_list.into_iter() {
        if let Some(cmp_elems) = s_list.get_serie(name) {
            let mut tot_err = 0f64;
            let mut tot_rel = 0f64;
            let mut size_e = 0;
            let mut size_r = 0;

            for (&base, &cmp) in base_elems.iter().zip(cmp_elems.iter()) {
                tot_err += base.sub(cmp).abs();

                if base != 0f64 {
                    tot_rel += ((cmp - base) / base).abs();
                    size_r += 1;
                }
                size_e += 1;
            }
            if size_r != 0 && size_e != 0 {
                println!(
                    "{}, {:e}, {:e}",
                    name,
                    tot_rel / size_r as f64,
                    tot_err / size_e as f64
                );
            }
        }
    }
}

