
mod handle_range;
mod handle_avg;
mod required_op;
mod handle_cmp;

use handle_range::handle_range;
use handle_avg::handle_avg;
use handle_cmp::handle_cmp;


use crate::{
    cli::{self, Commands},
    input_parser::{id_nom_builder::IdNomListBuilder, parse, Series},
    util::{retrive_file, retrive_string},
};


pub fn dispatch(mut command: Option<cli::Commands>) {
    if let None = command {
        command = Some(Commands::new());
    }

    match command.unwrap() {
        Commands::Range { file1 } => handle_range(file1),
        Commands::Avg { file1 } => handle_avg(file1),
        Commands::Cmp { file1, file2 } => handle_cmp(file1.as_path(), file2.as_path()),
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn crash_1() {
        let data = ":";
        parse(&data, IdNomListBuilder);
    }

    #[test]
    fn crash_2() {
        let data = "9 0E ";
        parse(&data, IdNomListBuilder);
    }

    #[test]
    fn crash_3() {
        let data = "9@\n:";
        parse(&data, IdNomListBuilder);
    }
}
