use std::{env, fs};

use sophia::graph::{inmem::LightGraph, *};
use sophia::parser;
use sophia::serializer;
use sophia::serializer::TripleStringifier;
use sophia::triple::stream::{TripleSink, TripleSource};

fn main() {
    let filename = "res/sample.ttl";
    let content = fs::read_to_string(filename).expect("Error reading file.");
    let mut graph = LightGraph::new();
    parser::turtle::parse_str(&content)
        .in_graph(&mut graph)
        .expect("Error loading graph.");

    let mut nt_stringifier = serializer::nt::stringifier();
    let example = nt_stringifier.stringify_graph(&mut graph).unwrap();
    println!("The resulting graph\n{}", example);
}
