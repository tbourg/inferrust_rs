use std::{any, boxed, env, fs, process};

use sophia::graph::{inmem::LightGraph, *};
use sophia::parser;
use sophia::serializer;
use sophia::serializer::TripleStringifier;
use sophia::term::Term;
use sophia::triple::stream::{TripleSink, TripleSource};

fn print_type<T>(_: T) {
    println!("{}", any::type_name::<T>())
}

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
    let operand = std::string::String::from(&args[2]);
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

    let operand_term = Term::<Box<str>>::new_iri(operand.into_boxed_str()).unwrap();
    let results = match &operator[..] {
        "s" => graph.triples_with_s(&operand_term),
        _ => graph.triples_with_s(&operand_term),
    };
    let result_it = unsafe { boxed::Box::into_raw(results).as_mut().unwrap() };
    let mut nt_stringifier = serializer::nt::stringifier();
    for result in result_it {
        let triple = result.unwrap();
        println!("{}", nt_stringifier.stringify_triple(&triple).unwrap());
    }

    /*let example = nt_stringifier.stringify_graph(&mut graph).unwrap();
    println!("The resulting graph\n{}", example);*/
}
