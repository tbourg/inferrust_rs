use crate::closure::{ClosureGraph, Node};

use std::collections::{HashMap, HashSet};

/// Source: https://pdfs.semanticscholar.org/47cc/a59310abee097af31d678d6cb2f8263dee37.pdf?_ga=2.26709177.588007852.1584345117-1155404888.1573749711
/// Figure 3, (line 16-22 necessary?)
pub fn graph_tc(g: &ClosureGraph) -> HashMap<u64, HashSet<u64>> {
    let mut stack = Vec::new();
    let mut root = HashMap::new();
    let mut tc = HashMap::new();
    let num = 1;
    fn node_tc(
        v: &Node,
        stack: &mut Vec<u64>,
        root: &mut HashMap<u64, u64>,
        tc: &mut HashMap<u64, HashSet<u64>>,
        g: &ClosureGraph,
        num: i32,
    ) {
        v.set_num(num);
        stack.push(v.id);
        root.insert(v.id, v.id);
        let v_succ: Vec<u64> = g
            .edges
            .iter()
            .filter(|e| e[0] == v.id)
            .map(|e| e[1])
            .collect();
        tc.insert(v.id, HashSet::new());
        tc.get_mut(&v.id).unwrap().extend(v_succ.clone());
        for wi in v_succ.iter() {
            let w = g.node(*wi);
            if *w.dfs_num.borrow() == -1 {
                node_tc(&w, stack, root, tc, g, num + 1);
                let wroot = g.node(*root.get(&w.id).unwrap());
                root.insert(v.id, minn(v, &wroot));
            } else if *v.dfs_num.borrow() > *w.dfs_num.borrow() {
                if !*w.in_comp.borrow() {
                    let wroot = g.node(*root.get(&w.id).unwrap());
                    root.insert(v.id, minn(v, &wroot));
                }
            }
            let wtc = tc.get(&w.id).unwrap().clone();
            tc.get_mut(&v.id).unwrap().extend(wtc);
        }
        // if *root.get(&v.id).unwrap() == v.id {
        //     let mut wid = stack.pop().unwrap();
        //     while wid != v.id {
        //         let w = g.node(wid);
        //         *w.in_comp.borrow_mut() = true;
        //         let vtc = tc.get(&v.id).unwrap().clone();
        //         tc.get_mut(&w.id).unwrap().extend(vtc);
        //         root.insert(w.id, v.id);
        //         wid = stack.pop().unwrap();
        //     }
        // }
    }
    for v in g.nodes.iter() {
        if *v.dfs_num.borrow() == -1 {
            node_tc(v, &mut stack, &mut root, &mut tc, &g, num);
        }
    }
    tc
}

fn minn(a: &Node, b: &Node) -> u64 {
    if *a.dfs_num.borrow() <= *b.dfs_num.borrow() {
        a.id
    } else {
        b.id
    }
}
