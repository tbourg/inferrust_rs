use sophia::serializer::nt::NtSerializer;
use sophia::serializer::*;

use inferrust::inferray::*;
use inferrust::rules::*;

fn main() {
    let rep = r#"
    @prefix : <http://example.org/> .
    @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
    @prefix owl: <http://www.w3.org/2002/07/owl#> .
 
    :parent :qq :ancetre . 
    :parent rdfs:subPropertyOf :ancetre . 
    :Toto rdf:type :animal .  
    :ancetre rdfs:domain :human . 
    :ancetre rdfs:range :human .
    :Bart :parent :Lisa .
    "#;
    let mut graph = InfGraph::from(sophia::parser::turtle::parse_str(rep));
    // dbg!(&graph.dictionary.ts.elem);
    graph.process(&mut RuleProfile::RDFSDefault());
    println!(
        "{} triples and {} p",
        graph.size(),
        graph.dictionary.ts.elem.len()
    );
    let mut nt_stringifier = NtSerializer::new_stringifier();
    let example2 = nt_stringifier.serialize_graph(&mut graph).unwrap().as_str();
    println!("The resulting graph\n{}", example2);
}
