use std::{env, fs, process};

use sophia::graph::{inmem::LightGraph, *};
use sophia::parser;
use sophia::serializer;
use sophia::serializer::TripleStringifier;
use sophia::term::Term;
use sophia::triple::stream::{TripleSink, TripleSource};

fn help() {
    println!("Usage");
    process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => (),
        _ => help(),
    }
    let operator = &args[1];
    let operand = &args[2];
    match &operator[..] {
        "o" | "s" | "p" => (),
        _ => help(),
    }
    let filename = "res/sample.ttl";
    let content = fs::read_to_string(filename).expect("Error reading file.");
    let mut graph = LightGraph::new();
    parser::turtle::parse_str(&content)
        .in_graph(&mut graph)
        .expect("Error loading graph.");

    operand.foo();

    /*    let operand_term = Term::new_iri(operand).unwrap();
        let results = match &operator[..] {
            "s" => graph.triples_with_s(&operand_term)
        };

        let mut nt_stringifier = serializer::nt::stringifier();
        let example = nt_stringifier.stringify_graph(&mut graph).unwrap();
        println!("The resulting graph\n{}", example);
    */
}
