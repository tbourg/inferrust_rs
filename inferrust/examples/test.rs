use inferrust::inferray::*;
use inferrust::rules::*;

fn main() {
    let mut graph = InfGraph::from(sophia::parser::turtle::parse_str(
        &std::fs::read_to_string("../bsbmtools-0.2/dataset.ttl").unwrap(),
    ));

    // dbg!(&graph.dictionary.ts.elem);

    println!(
        "{} triples and {} p",
        graph.size(),
        graph.dictionary.ts.elem.len()
    );
    // let mut nt_stringifier = NtSerializer::new_stringifier();
    // let example2 = nt_stringifier.serialize_graph(&mut graph).unwrap().as_str();
    // println!("The resulting graph\n{}", example2);
    graph.process(&mut RuleProfile::RDFSPlus());
    println!("{} triples", graph.size());
}
