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
 
    :Toto rdf:type :animal .  
    :ancetre rdfs:domain :human . 
    :ancetre rdfs:range :human .
    :parent rdfs:subPropertyOf :ancetre . 
    :Bart :parent :Lisa .
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
    // graph.init_axiomatic_triples();
    // graph.close();
    // let mut rules = <Vec<Box<Rule>> as RuleSet>::new();
    // rules.specialize(std::rc::Rc::new(&graph));
    // rules.fire_all(&mut graph);
    // println!("{} triples", graph.size());
    // let mut nt_stringifier = NtSerializer::new_stringifier();
    // let example2 = nt_stringifier.serialize_graph(&mut graph).unwrap().as_str();
    // println!("The resulting graph\n{}", example2);
    let mut nt_stringifier = NtSerializer::new_stringifier();
    println!(
        "\nTest triples_with_s(bart):\n{}",
        nt_stringifier
            .serialize_triples(
                &mut graph.triples_with_s(
                    &Namespace::new("http://example.org/")
                        .unwrap()
                        .get("Bart")
                        .unwrap(),
                )
            )
            .unwrap()
            .as_str()
    );
    let mut nt_stringifier = NtSerializer::new_stringifier();
    println!(
        "\nTest triples_with_p(type):\n{}",
        nt_stringifier
            .serialize_triples(&mut graph.triples_with_p(&rdf::type_,))
            .unwrap()
            .as_str()
    );
    let mut nt_stringifier = NtSerializer::new_stringifier();
    println!(
        "\nTest triples_with_o(human):\n{}",
        nt_stringifier
            .serialize_triples(
                &mut graph.triples_with_o(
                    &Namespace::new("http://example.org/")
                        .unwrap()
                        .get("human")
                        .unwrap(),
                )
            )
            .unwrap()
            .as_str()
    );
    let mut nt_stringifier = NtSerializer::new_stringifier();
    println!(
        "\nTest triples_with_sp(lisa,type):\n{}",
        nt_stringifier
            .serialize_triples(
                &mut graph.triples_with_sp(
                    &Namespace::new("http://example.org/")
                        .unwrap()
                        .get("Lisa")
                        .unwrap(),
                    &rdf::type_,
                )
            )
            .unwrap()
            .as_str()
    );
    let mut nt_stringifier = NtSerializer::new_stringifier();
    println!(
        "\nTest triples_with_po(type,human):\n{}",
        nt_stringifier
            .serialize_triples(
                &mut graph.triples_with_po(
                    &rdf::type_,
                    &Namespace::new("http://example.org/")
                        .unwrap()
                        .get("human")
                        .unwrap(),
                )
            )
            .unwrap()
            .as_str()
    );
    let mut nt_stringifier = NtSerializer::new_stringifier();
    println!(
        "\nTest triples_with_so(parent,ancetre):\n{}",
        nt_stringifier
            .serialize_triples(
                &mut graph.triples_with_so(
                    &Namespace::new("http://example.org/")
                        .unwrap()
                        .get("parent")
                        .unwrap(),
                    &Namespace::new("http://example.org/")
                        .unwrap()
                        .get("ancetre")
                        .unwrap(),
                )
            )
            .unwrap()
            .as_str()
    );
    let mut nt_stringifier = NtSerializer::new_stringifier();
    println!(
        "\nTest triples_with_spo(parent,spo,ancetre):\n{}",
        nt_stringifier
            .serialize_triples(
                &mut graph.triples_with_spo(
                    &Namespace::new("http://example.org/")
                        .unwrap()
                        .get("parent")
                        .unwrap(),
                    &rdfs::subPropertyOf,
                    &Namespace::new("http://example.org/")
                        .unwrap()
                        .get("ancetre")
                        .unwrap(),
                )
            )
            .unwrap()
            .as_str()
    );
}
