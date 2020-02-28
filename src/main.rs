use sophia::graph::{inmem::LightGraph, *};
use sophia::ns::*;
use sophia::serializer;
use sophia::serializer::TripleStringifier;
use sophia::triple::stream::TripleSource;

mod my_graph;
use my_graph::MyGraph;

mod inferray;
use self::inferray::graph::*;

mod rules;
use self::rules::Rule;
use self::rules::RuleSet;

fn main() {
    let rep = r#"
    @prefix : <http://example.org/> .
    @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
    @prefix owl: <http://www.w3.org/2002/07/owl#> .

    :Bart rdf:type :human .
    :Lisa rdf:type :human .
    :BLOB a :entity .
    :human rdfs:subClassOf :mammal .
    :mammal rdfs:subClassOf :animal .
    :animal owl:equivalentClass :entity .
    "#;
    let mut graph = InfGraph::from(sophia::parser::turtle::parse_str(rep));

    println!("{} triples", graph.size());
    let mut nt_stringifier = serializer::nt::stringifier();
    let example2 = nt_stringifier.stringify_graph(&mut graph).unwrap();
    println!("The resulting graph\n{}", example2);
    let mut rules = <Vec<Box<dyn Rule>> as RuleSet>::new();
    // rules.specialize(std::rc::Rc::new(&graph));
    rules.fire_all(&mut graph);
    println!("{} triples", graph.size());

    let mut nt_stringifier = serializer::nt::stringifier();
    let example2 = nt_stringifier.stringify_graph(&mut graph).unwrap();
    println!("The resulting graph\n{}", example2);
}
