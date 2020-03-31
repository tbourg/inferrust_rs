use sophia::graph::inmem::*;
use sophia::graph::*;
use sophia::ns::*;
use sophia::term::*;
use sophia::triple::stream::*;

use inferrust::inferray::*;
use inferrust::rules::*;

extern crate time;
use time::precise_time_ns;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(22)
        .build_global()
        .unwrap();
    let rep = &std::fs::read_to_string("inferrust/res/persondata_en_10k.ttl").unwrap();
    let dbo_person = Term::<&'static str>::new_iri("http://dbpedia.org/ontology/Person").unwrap();
    let mut ts = sophia::parser::turtle::parse_str(rep);
    let t0 = precise_time_ns();
    let mut s_graph = FastGraph::new();
    ts.in_graph(&mut s_graph).unwrap();
    let t1 = precise_time_ns();
    let time_creation = (t1 - t0) as f64 / 1e9;

    let mut c = 0;
    let mut time_first: f64 = 0.0;
    let mut t0 = precise_time_ns();
    let results = s_graph.triples_with_po(&rdf::type_, &dbo_person);
    for _ in results {
        if c == 0 {
            let t1 = precise_time_ns();
            time_first = (t1 - t0) as f64 / 1e9;
            t0 = precise_time_ns();
        }
        c += 1;
    }
    let t1 = precise_time_ns();
    let time_rest = (t1 - t0) as f64 / 1e9;
    eprintln!("matching triple: {}\n", c);
    println!("sophia: {},{},{}", time_creation, time_first, time_rest);

    let ts = sophia::parser::turtle::parse_str(rep);
    let mut i_graph = InfGraph::from(ts);
    let t1 = precise_time_ns();
    let time_creation = (t1 - t0) as f64 / 1e9;

    let mut c = 0;
    let mut time_first: f64 = 0.0;
    let mut t0 = precise_time_ns();
    let results = i_graph.triples_with_po(&rdf::type_, &dbo_person);
    for _ in results {
        if c == 0 {
            let t1 = precise_time_ns();
            time_first = (t1 - t0) as f64 / 1e9;
            t0 = precise_time_ns();
        }
        c += 1;
    }
    let t1 = precise_time_ns();
    let time_rest = (t1 - t0) as f64 / 1e9;
    eprintln!("matching triple: {}\n", c);
    println!("inferray: {},{},{}", time_creation, time_first, time_rest);
}
