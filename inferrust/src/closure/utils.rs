use crate::closure::{ClosureGraph, Node};

use std::collections::{HashMap, HashSet};

/// Source: https://pdfs.semanticscholar.org/47cc/a59310abee097af31d678d6cb2f8263dee37.pdf?_ga=2.26709177.588007852.1584345117-1155404888.1573749711
/// Figure 4
pub fn graph_tc(g: &ClosureGraph) -> HashMap<u64, HashSet<u64>> {
    let mut stack = Vec::new();
    let mut root = HashMap::new();
    let mut tc = HashMap::new();
    let mut adj_comp_roots = HashMap::new();
    let mut num = 0;
    fn node_tc(
        v: &Node,
        stack: &mut Vec<u64>,
        root: &mut HashMap<u64, u64>,
        tc: &mut HashMap<u64, HashSet<u64>>,
        adj_comp_roots: &mut HashMap<u64, HashSet<u64>>,
        g: &ClosureGraph,
        num: &mut usize,
    ) {
        v.set_num(*num);
        *num += 1;
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
        dbg!((&v.id, *num, &v_succ));
        for wi in v_succ.iter() {
            let w = g.node(*wi);
            if *w.dfs_num.borrow() == usize::max_value() {
                node_tc(&w, stack, root, tc, adj_comp_roots, g, num);
                let wroot = g.node(root[&w.id]);
                root.insert(v.id, minn(v, &wroot));
                if *w.in_comp.borrow() {
                    adj_comp_roots
                        .get_mut(&v.id)
                        .unwrap()
                        .insert(root[&w.id]);
                }
            } else if *v.dfs_num.borrow() > *w.dfs_num.borrow() {
                if !*w.in_comp.borrow() {
                    let wroot = g.node(root[&w.id]);
                    root.insert(v.id, minn(v, &wroot));
                } else {
                    adj_comp_roots
                        .get_mut(&v.id)
                        .unwrap()
                        .insert(root[&w.id]);
                }
            }
        }
        for r in adj_comp_roots[&v.id].iter() {
            if !&tc[&root[&v.id]].contains(r) {
                tc.get_mut(&root[&v.id]).unwrap().insert(*r);
                let tcr = tc[r].clone();
                tc.get_mut(&root[&v.id]).unwrap().extend(tcr);
            }
        }
        if root[&v.id] == v.id {
            let top = g.node(*stack.last().unwrap());
            if *top.dfs_num.borrow() > *v.dfs_num.borrow() {
                tc.get_mut(&v.id).unwrap().insert(v.id);
            }
            let mut wid = stack.pop().unwrap();
            while wid != v.id {
                let w = g.node(wid);
                *w.in_comp.borrow_mut() = true;
                if !tc[&wid].is_empty() {
                    let wtc = tc[&wid].clone();
                    tc.get_mut(&v.id).unwrap().extend(wtc);
                }
                root.insert(w.id, v.id);
                wid = stack.pop().unwrap();
            }
        } else {
            tc.get_mut(&root[&v.id]).unwrap().insert(v.id);
            let tcv = tc[&v.id].clone();
            tc.get_mut(&root[&v.id]).unwrap().extend(tcv);
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
                &mut num,
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

