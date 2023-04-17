#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate analyzer;

use analyzer::input_parser::id_nom_builder::IdNomListBuilder;
use analyzer::input_parser::parse;



fuzz_target!(|data: &str | {
    parse(&data, IdNomListBuilder);
    // fuzzed code goes here
});
