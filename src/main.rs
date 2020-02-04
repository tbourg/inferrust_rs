use std::{any, boxed, env, fs, process};

use sophia::graph::{inmem::LightGraph, *};
use sophia::parser;
use sophia::serializer;
use sophia::serializer::TripleStringifier;
use sophia::term::{BoxTerm, RcTerm, Term};
use sophia::triple::stream::{TripleSink, TripleSource};
use sophia::triple::Triple;

fn print_type<T>(_: T) {
    println!("{}", any::type_name::<T>())
}

fn help() {
    println!("Usage");
    process::exit(0);
}

fn create_term(op: String, iri: bool) -> BoxTerm {
    let term = if iri {
        BoxTerm::new_iri(op.into_boxed_str())
    } else {
        BoxTerm::new_literal_lang(op.into_boxed_str(), "fr")
    }
    .expect("Error creating term");
    return term;
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

    //let operand_term = match &operand[..] {
    //    "s" => create_term(operand, true),
    //    "p" => create_term(operand, true),
    //    "o" | _ => create_term(operand, false),
    //};
    // print_type(operand_term);
    let operand_term = BoxTerm::new_iri(operand.into_boxed_str()).unwrap();
    // print_type(op_term);*/
    let results = match &operator[..] {
        "s" => graph.triples_with_s(&operand_term),
        "p" => graph.triples_with_p(&operand_term),
        "o" => graph.triples_with_o(&operand_term),
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
