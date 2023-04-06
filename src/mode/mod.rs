use std::{path::Path, ops::Sub};

use crate::{cli::{self, Commands}, util::{ retrive_string, retrive_file}, input_parser::{parse, IdListBuilder, Series}};




pub fn dispatch(mut command: Option<cli::Commands>){
    if let None = command {
        command = Some(Commands::new());
    }

    match command.unwrap() {
        Commands::Range { file1 } => handle_range(file1),
        Commands::Avg { file1 } => handle_avg(file1),
        Commands::Cmp { file1, file2 } =>  handle_cmp(file1.as_path(), file2.as_path()),
    }
    todo!()
}

fn handle_cmp(file1: &Path, file2: &Path) {

    let f_list = parse(&retrive_file(file1), IdListBuilder);
    let s_list = parse(&retrive_file(file2), IdListBuilder);


    for (name,base_elems) in f_list.iter() {
        if let Some(cmp_elems) = s_list.get_serie(name){
            let mut tot_err = 0f64;
            let mut tot_rel= 0f64;
            let mut size_e = 0;
            let mut size_r = 0;

           for (&base, &cmp) in base_elems.iter().zip(cmp_elems.iter()){
            tot_err += base.sub(cmp).abs();
                
                if base != 0f64 {
                    tot_rel += ((cmp-base)/base).abs();
                    size_r += 1;
                }
                size_e += 1;
                
           }
           if size_r != 0 && size_e != 0{

           println!("{}, {}, {}",name, tot_rel/size_r as f64 ,tot_err/size_e as f64);
        }

        }
    }

    
}

fn handle_avg(file1: Option<std::path::PathBuf>) {
    let input = retrive_string(file1);
    let series = parse(&input, IdListBuilder);

    for series in series.iter() {
        let avg = series.1.iter().sum::<f64>() / series.1.len() as f64;
        println!("{} -> {}", series.0, avg)

    }
}

fn handle_range(file1: Option<std::path::PathBuf>) {
    let input = retrive_string(file1);
    let series = parse(&input, IdListBuilder);

    let range_finder = |(ml,zl,bl) : (f64, f64, f64), (mr,zr, br) : (f64, f64, f64) | {
        let min = ml.min(mr);
        let zero = if (zl - 0.0).abs() > (zr - 0.0).abs() {zr} else {zl};
        let max = bl.min(br);
        return (min, zero, max)
    };

    for series in series.iter() {
        let range = series.1.iter().map(|&x| (x,x,x)).reduce(range_finder);
        if let Some((min, zero, max)) = range {
            println!("{} -> ({}, {} ,{})", series.0, min, zero, max)
        }        
    }
}
