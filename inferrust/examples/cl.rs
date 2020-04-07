use inferrust::closure::ClosureGraph;
use inferrust::inferray::*;
use inferrust::rules::*;

use time::precise_time_ns;

use std::io::BufRead;

use sophia::serializer::nt::*;
use sophia::serializer::*;

fn main() {
    let pairs = vec![
        [1, 2],
        [2, 3],
        [3, 1],
        [4, 5],
        [5, 6],
        [6, 7],
        [11, 12],
        [12, 13],
    ];
    let mut g = ClosureGraph::from(pairs);
    dbg!(g.close());
    let mut rep = "".to_string();
    let mut lines =
        std::io::Cursor::new(std::fs::read_to_string("inferrust/res/subClassOf1000.nt").unwrap())
            .lines();
    let lengths = [10, 20, 50, 100, 500, 1000];
    let mut prev_len = 1;
    rep.push_str(&lines.next().unwrap().unwrap());
    rep.push_str(&lines.next().unwrap().unwrap());
    for len in lengths.iter() {
        let diff = *len - prev_len;
        for _ in 0..diff {
            prev_len += 1;
            rep.push_str(&lines.next().unwrap().unwrap());
            rep.push_str("\r\n");
            rep.push_str(&lines.next().unwrap().unwrap());
            rep.push_str("\r\n");
        }
        let ts = sophia::parser::turtle::parse_str(&rep);
        let mut i_graph = InfGraph::from(ts);
        println!("chain length: {}\ngraph size: {}", *len, i_graph.size());
        // let mut nt_stringifier = NtSerializer::new_stringifier();
        // let example2 = nt_stringifier
        //     .serialize_graph(&mut i_graph)
        //     .unwrap()
        //     .as_str();
        // println!("The resulting graph\n{}", example2);

        let mut c = 0;
        let mut time_first: f64 = 0.0;
        let mut t0 = precise_time_ns();
        i_graph.process(&mut RuleProfile::Closure());
        let t1 = precise_time_ns();
        let time = (t1 - t0) as f64 / 1e9;
        // let mut nt_stringifier = NtSerializer::new_stringifier();
        // let example2 = nt_stringifier
        //     .serialize_graph(&mut i_graph)
        //     .unwrap()
        //     .as_str();
        // println!("The resulting graph\n{}", example2);
        println!("graph size: {}, time: {}", i_graph.size(), time);
        // panic!();
    }
}
