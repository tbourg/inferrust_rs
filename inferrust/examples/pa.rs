use inferrust::closure::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let arcs = vec![
        [10, 0],
        [10, 20],
        [10, 30],
        [20, 30],
        [30, 10],
        [40, 20],
        [40, 50],
    ];
    let mut graph = ClosureGraph::from(arcs);
    let tc = graph.close();
    dbg!(tc);
}
