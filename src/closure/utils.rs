use crate::closure::{ClosureGraph, Node};

use std::collections::{HashMap, HashSet};

fn graph_tc(g: ClosureGraph) {
    let mut stack = Vec::new();
    let mut root = HashMap::new();
    let mut tc = HashMap::new();
    let mut num = 1;
    fn node_tc(
        v: &mut Node,
        stack: &mut Vec<i64>,
        root: &mut HashMap<i64, i64>,
        tc: &mut HashMap<i64, HashSet<i64>>,
        g: &ClosureGraph,
        num: i32,
    ) {
        v.set_num(num);
        num += 1;
        root.insert(v.id, v.id);
        stack.push(v.id);
        let mut succ = HashSet::new();
        for edge in &g.edges {
            if edge[0] == v.id {
                succ.insert(edge[1]);
            }
        }
        tc.insert(v.id, succ);
        for wi in &succ {
            let w = g.node(*wi);
            if w.dfs_num != -1 {
                node_tc(w, stack, root, tc, g, num);
                let wroot = g.node(*root.get(&w.id).unwrap());
                root.insert(v.id, minn(v, wroot));
            } else if v.dfs_num > w.dfs_num {
                if !w.in_comp {
                    let wroot = g.node(*root.get(&w.id).unwrap());
                    root.insert(v.id, minn(v, wroot));
                }
            }
            tc.get_mut(&v.id).unwrap().extend(tc.get(&w.id).unwrap());
            //v.tc = new Set([...v.tc,...w.tc]);
        }
        if *root.get(&v.id).unwrap() == v.id {
            let Some(wid) = stack.pop();
            while wid != v.id {
                let w = g.node(wid);
                w.in_comp = true;
                tc.get_mut(&wid).unwrap().extend(tc.get(&v.id).unwrap());
                root.insert(wid, v.id);
                wid = stack.pop().unwrap();
            }
            /*do {
                w = stack.pop();
                w.inC = true;
                w.tc = new Set([...w.tc,...v.tc]);
                w.root = v;
            } while(w != v);*/
        }
    }

    for (_, v) in &g.nodes {
        if v.dfs_num == -1 {
            node_tc(&mut v, &mut stack, &mut root, &mut tc, &g, num);
        }
    }
}

fn minn(a: &Node, b: &Node) -> i64 {
    if a.dfs_num <= b.dfs_num {
        a.id
    } else {
        b.id
    }
}
