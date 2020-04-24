use sophia::graph::inmem::*;
use sophia::graph::*;
use sophia::ns::*;
use sophia::term::*;
use sophia::triple::stream::*;

use inferrust::inferray::*;

extern crate time;
use time::precise_time_ns;

use std::io::BufRead;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();
    let mut rep = "".to_string();
    let mut lines = std::io::Cursor::new(
        std::fs::read_to_string("inferrust/res/persondata_en_1M.ttl").unwrap(),
    )
    .lines();
    let dbo_person =
        Term::<&'static str>::new_iri("http://wikidata.dbpedia.org/ontology/Person").unwrap();
    let lengths = [10, 20, 50, 100, 500];
    let prev_len = 0;
    for len in lengths.iter() {
        let diff = *len - prev_len;
        for _ in 0..(diff * 1000) {
            rep.push_str(&lines.next().unwrap().unwrap());
            rep.push_str("\r\n");
        }
        let ts = sophia::parser::turtle::parse_str(&rep);
        let t0 = precise_time_ns();
        let s_graph: FastGraph = ts.collect_triples().unwrap();
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

        let ts = sophia::parser::turtle::parse_str(&rep);
        let i_graph = InfGraph::from(ts);
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
}
