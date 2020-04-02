use crate::closure::{ClosureGraph, Node};

use std::collections::{HashMap, HashSet};

/// Source: https://pdfs.semanticscholar.org/47cc/a59310abee097af31d678d6cb2f8263dee37.pdf?_ga=2.26709177.588007852.1584345117-1155404888.1573749711
/// Figure 4
pub fn graph_tc(g: &ClosureGraph) -> HashMap<u64, HashSet<u64>> {
    let mut stack = Vec::new();
    let mut root = HashMap::new();
    let mut tc = HashMap::new();
    let mut adj_comp_roots = HashMap::new();
    let num = 0;
    fn node_tc(
        v: &Node,
        stack: &mut Vec<u64>,
        root: &mut HashMap<u64, u64>,
        tc: &mut HashMap<u64, HashSet<u64>>,
        adj_comp_roots: &mut HashMap<u64, HashSet<u64>>,
        g: &ClosureGraph,
        num: usize,
    ) {
        v.set_num(num);
        stack.push(v.id);
        root.insert(v.id, v.id);
        tc.insert(v.id, HashSet::new());
        adj_comp_roots.insert(v.id, HashSet::new());
        let v_succ: Vec<u64> = g
            .edges
            .iter()
            .filter(|e| e[0] == v.id)
            .map(|e| e[1])
            .collect();
        dbg!(&v.id, &v_succ);
        for wi in v_succ.iter() {
            let w = g.node(*wi);
            if *w.dfs_num.borrow() == usize::max_value() {
                node_tc(&w, stack, root, tc, adj_comp_roots, g, num + 1);
                let wroot = g.node(*root.get(&w.id).unwrap());
                root.insert(v.id, minn(v, &wroot));
                if *w.in_comp.borrow() {
                    adj_comp_roots
                        .get_mut(&v.id)
                        .unwrap()
                        .insert(*root.get(&w.id).unwrap());
                }
            } else if *v.dfs_num.borrow() > *w.dfs_num.borrow() {
                if !*w.in_comp.borrow() {
                    let wroot = g.node(*root.get(&w.id).unwrap());
                    root.insert(v.id, minn(v, &wroot));
                } else {
                    adj_comp_roots
                        .get_mut(&v.id)
                        .unwrap()
                        .insert(*root.get(&w.id).unwrap());
                }
            }
        }
        for r in adj_comp_roots.get(&v.id).unwrap().iter() {
            if !tc.get(root.get(&v.id).unwrap()).unwrap().contains(r) {
                tc.get_mut(root.get(&v.id).unwrap()).unwrap().insert(*r);
                let tcr = tc.get(r).unwrap().clone();
                tc.get_mut(root.get(&v.id).unwrap()).unwrap().extend(tcr);
            }
        }
        if *root.get(&v.id).unwrap() == v.id {
            let top = g.node(*stack.last().unwrap());
            if *top.dfs_num.borrow() > *v.dfs_num.borrow() {
                tc.get_mut(&v.id).unwrap().insert(v.id);
            }
            let mut wid = stack.pop().unwrap();
            while wid != v.id {
                let w = g.node(wid);
                *w.in_comp.borrow_mut() = true;
                if !tc.get(&wid).unwrap().is_empty() {
                    let wtc = tc.get(&wid).unwrap().clone();
                    tc.get_mut(&v.id).unwrap().extend(wtc);
                }
                root.insert(w.id, v.id);
                wid = stack.pop().unwrap();
            }
        } else {
            tc.get_mut(root.get(&v.id).unwrap()).unwrap().insert(v.id);
            let tcv = tc.get(&v.id).unwrap().clone();
            tc.get_mut(root.get(&v.id).unwrap()).unwrap().extend(tcv);
            tc.get_mut(&v.id).unwrap().clear();
        }
    }
    for v in g.nodes.iter() {
        if *v.dfs_num.borrow() == usize::max_value() {
            node_tc(
                v,
                &mut stack,
                &mut root,
                &mut tc,
                &mut adj_comp_roots,
                &g,
                num,
            );
        }
    }
    dbg!(root);
    tc
}

fn minn(a: &Node, b: &Node) -> u64 {
    if *a.dfs_num.borrow() <= *b.dfs_num.borrow() {
        a.id
    } else {
        b.id
    }
}
