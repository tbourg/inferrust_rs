use sophia::graph::{inmem::LightGraph, *};
use sophia::ns::Namespace;
use sophia::parser;
use sophia::serializer;
use sophia::serializer::TripleStringifier;
use sophia::triple::stream::{TripleSink, TripleSource};

mod my_graph;
use my_graph::MyGraph;

use std::convert::Infallible;

// mod inferray_graph;
// use inferray_graph::InfGraph;

fn main() {
    let example = r#"
        @prefix : <http://example.org/>.
        @prefix foaf: <http://xmlns.com/foaf/0.1/>.
        :alice foaf:name "Alice";
            foaf:mbox <mailto:alice@work.example> .
        :bob foaf:name "Bob".
    "#;
    let mut graph: LightGraph = LightGraph::new();
    parser::turtle::parse_str(example).in_graph(&mut graph);

    let mut cols = 0;
    graph.triples().for_each_triple(|_| cols += 1);
    println!("{} triples", cols);

    let mut nt_stringifier = serializer::nt::stringifier();
    let example2 = nt_stringifier.stringify_graph(&mut graph).unwrap();
    println!("The resulting graph\n{}", example2);
}
