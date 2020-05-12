use inferrust::inferray::*;
use inferrust::rules::*;

use time::precise_time_ns;

use std::fs;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();
    let mut profiles = [
        RuleProfile::RDFS(),
        RuleProfile::RhoDF(),
        RuleProfile::RDFSPlus(),
    ];
    fs::read_dir("inferrust/res/bsbm")
        .unwrap()
        .for_each(|file| {
            // println!("file: {:#?}", &file);
            let rep = fs::read_to_string(file.as_ref().unwrap().path()).unwrap();
            profiles.iter_mut().for_each(|profile| {
                for _ in 0..5 {
                    for par in &[false, true] {
                        let t0 = precise_time_ns();
                        let ts = sophia::parser::nt::parse_str(&rep);
                        let mut i_graph = InfGraph::from(ts);
                        let len = i_graph.size();
                        // println!("graph size: {}", i_graph.size());
                        let t1 = precise_time_ns();
                        if *par {
                            i_graph.process_par(profile);
                        } else {
                            i_graph.process(profile);
                        }
                        let t2 = precise_time_ns();
                        let load_time = (t1 - t0) as f64 / 1e9;
                        let process_time = (t2 - t1) as f64 / 1e9;
                        // println!(
                        //     "profile: {}, graph size: {}, ltime: {}, ptime: {}",
                        //     profile.name(),
                        //     i_graph.size(),
                        //     load_time,
                        //     process_time,
                        // );
                        println!(
                            "rust {},{},{},{},{}",
                            if *par { "par" } else { "single" },
                            profile.name(),
                            len,
                            // file.as_ref().unwrap().path().to_str().unwrap(),
                            load_time,
                            process_time
                        );
                    }
                }
            })
        });
}
