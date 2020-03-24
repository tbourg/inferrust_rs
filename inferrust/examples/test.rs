use sophia::graph::Graph;
use sophia::ns::*;
use sophia::serializer::nt::NtSerializer;
use sophia::serializer::*;

use inferrust::inferray::*;
use inferrust::rules::{Rule, RuleSet};

fn main() {
    let rep = r#"
    @prefix : <http://example.org/> .
    @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
    @prefix owl: <http://www.w3.org/2002/07/owl#> .
 
    :a :p :b .
    "#;
    let mut graph = InfGraph::from(sophia::parser::turtle::parse_str(rep));

    // dbg!(&graph.dictionary.ts.elem);

    println!(
        "{} triples and {} p",
        graph.size(),
        graph.dictionary.ts.elem.len()
    );
    let mut nt_stringifier = NtSerializer::new_stringifier();
    let example2 = nt_stringifier.serialize_graph(&mut graph).unwrap().as_str();
    println!("The resulting graph\n{}", example2);
    graph.close();
    let mut rules = <Vec<Box<Rule>> as RuleSet>::new();
    // rules.specialize(std::rc::Rc::new(&graph));
    rules.fire_all(&mut graph);
    println!("{} triples", graph.size());
    let mut nt_stringifier = NtSerializer::new_stringifier();
    let example2 = nt_stringifier.serialize_graph(&mut graph).unwrap().as_str();
    println!("The resulting graph\n{}", example2);
}
