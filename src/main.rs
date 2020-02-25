use sophia::graph::{inmem::LightGraph, *};
use sophia::ns::*;
use sophia::serializer;
use sophia::serializer::TripleStringifier;
use sophia::triple::stream::TripleSource;

mod my_graph;
use my_graph::MyGraph;

mod inferray;
use self::inferray::graph::*;

fn main() {
    let rep = r#"
    @prefix : <http://example.org/> .
    @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

    :Bart rdf:type :human .
    :Lisa rdf:type :human .
    :human rdfs:subClassOf :mammal .
    :mammal rdfs:subClassOf :animal .
    "#;
    let mut graph = InfGraph::from(sophia::parser::turtle::parse_str(rep));

    let mut cols = 0;
    graph.triples().for_each_triple(|_| cols += 1);
    println!("{} triples", cols);

    let mut nt_stringifier = serializer::nt::stringifier();
    let example2 = nt_stringifier.stringify_graph(&mut graph).unwrap();
    println!("The resulting graph\n{}", example2);
}
