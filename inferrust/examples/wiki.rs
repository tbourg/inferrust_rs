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
    let sizes = [10, 50, 100, 500, 1000, 10000, 1000000];
    let ents = [0.1, 0.2, 0.5, 1.];
    for size in sizes.iter() {
        for ent in ents.iter() {
            let rng = *size as f64 * *ent;
            let rng = rng as u64;
            let mut pairs: Vec<[u64; 2]> = Vec::new();
            let mut max = 0;
            let mut min = 1000000000;
            for _ in 0..*size {
                let a = rand::thread_rng().gen_range(0, rng);
                let b = rand::thread_rng().gen_range(0, rng);
                max = max.max(a);
                min = min.min(a);
                pairs.push([a, b]);
            }
            // dbg!(max, min);
            let t0 = precise_time_ns();
            let w = max - min + 1;
            let w = w as usize;
            let mut hist = vec![0; w];
            let mut histt = vec![0; w];
            let mut cumul = vec![0; w];
            bucket_sort_pairs(&mut pairs, &mut hist, &mut histt, &mut cumul, min, max, w);
            let t1 = precise_time_ns();
            let time = (t1 - t0) as f64;
            println!("inferrust,{},{},{}", *size, *ent, time);
        }
    }
    std::process::exit(127);
    let mut profiles = [
        RuleProfile::RDFS(),
        RuleProfile::RhoDF(),
        RuleProfile::RDFSPlus(),
    ];
    fs::read_dir("inferrust/res/wiki")
        .unwrap()
        .for_each(|file| {
            println!("file: {:#?}", &file);
            let rep = fs::read_to_string(file.unwrap().path()).unwrap();
            profiles.iter_mut().for_each(|profile| {
                let t0 = precise_time_ns();
                let ts = sophia::parser::turtle::parse_str(&rep);
                let t1 = precise_time_ns();
                let time = (t1 - t0) as f64 / 1e9;
                println!("parsing: {}", time);
                let t0 = precise_time_ns();
                let mut i_graph = InfGraph::from(ts);
                let t1 = precise_time_ns();
                let time = (t1 - t0) as f64 / 1e9;
                println!("creation: {}", time);
                println!("graph size: {}", i_graph.size());
                let t0 = precise_time_ns();
                i_graph.process(profile);
                let t1 = precise_time_ns();
                let time = (t1 - t0) as f64 / 1e9;
                println!(
                    "profile: {}, graph size: {}, time: {}",
                    profile.name(),
                    i_graph.size(),
                    time
                );
            })
        });
}
