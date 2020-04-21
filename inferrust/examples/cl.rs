use inferrust::closure::ClosureGraph;
use inferrust::inferray::*;
use inferrust::rules::*;

use rand::Rng;
use time::precise_time_ns;

use std::fs;
use std::io::BufRead;

use sophia::serializer::nt::*;
use sophia::serializer::*;

fn main() {
    fs::read_dir("inferrust/res/closure")
        .unwrap()
        .for_each(|file| {
            let path = &file.unwrap().path();
            let len = path
                .to_str()
                .unwrap()
                .matches(char::is_numeric)
                .collect::<Vec<&str>>()
                .concat();
            let rep = fs::read_to_string(path).unwrap();
            let ts = sophia::parser::turtle::parse_str(&rep);
            let mut i_graph = InfGraph::from(ts);
            println!("{}", i_graph.size());
            let t0 = precise_time_ns();
            i_graph.process(&mut RuleProfile::Closure());
            let t1 = precise_time_ns();
            let time = (t1 - t0) as f64 / 1e9;
            println!("{}", i_graph.size());
            println!("inferrust,{},{}", len, time);
        });
}
