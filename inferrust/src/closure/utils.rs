use crate::closure::{ClosureGraph, Node};
use crate::utils::*;

use std::collections::{HashMap, HashSet};

/// Source: https://pdfs.semanticscholar.org/47cc/a59310abee097af31d678d6cb2f8263dee37.pdf?_ga=2.26709177.588007852.1584345117-1155404888.1573749711
/// Figure 4
pub fn graph_tc(g: &ClosureGraph) -> HashMap<u64, HashSet<u64>> {
    let mut stack = Vec::new();
    let mut root = HashMap::new();
    let mut tc = HashMap::new();
    let mut num = 0;
    dbg!(&g.edges);
    fn node_tc(
        v: &Node,
        stack: &mut Vec<u64>,
        root: &mut HashMap<u64, u64>,
        tc: &mut HashMap<u64, HashSet<u64>>,
        g: &ClosureGraph,
        num: &mut usize,
    ) {
        v.set_num(*num);
        *num += 1;
        stack.push(v.id);
        root.insert(v.id, v.id);
        tc.insert(v.id, HashSet::new());
        let mut adj_comp_roots = HashSet::new();
        let len = g.edges.len();
        let start_index = first(&g.edges, v.id, 0, len - 1, len, 0);
        let v_succ: Vec<u64> = g.edges[start_index..]
            .iter()
            .take_while(|e| e[0] == v.id)
            .map(|e| e[1])
            .collect();
        dbg!((&v.id, *num, start_index, &g.edges[start_index..], &v_succ));
        for wi in v_succ.iter() {
            let w = g.node(*wi);
            if *w.dfs_num.borrow() == usize::max_value() {
                node_tc(&w, stack, root, tc, g, num);
                let vroot = g.node(root[&v.id]);
                let wroot = g.node(root[&w.id]);
                root.insert(v.id, minn(&vroot, &wroot));
                if *w.in_comp.borrow() {
                    adj_comp_roots.insert(root[&w.id]);
                }
            } else if *v.dfs_num.borrow() > *w.dfs_num.borrow() {
                if !*w.in_comp.borrow() {
                    let vroot = g.node(root[&v.id]);
                    let wroot = g.node(root[&w.id]);
                    root.insert(v.id, minn(&vroot, &wroot));
                } else {
                    adj_comp_roots.insert(root[&w.id]);
                }
            }
        }
        for r in adj_comp_roots.iter() {
            if !&tc[&root[&v.id]].contains(r) {
                let tc_r = tc[r].iter().cloned().collect::<Vec<_>>();
                let tc_root_v = tc.get_mut(&root[&v.id]).unwrap();
                tc_root_v.insert(*r);
                tc_root_v.extend(tc_r);
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
            *v.in_comp.borrow_mut() = true;
        } else {
            let tc_v = tc[&v.id].iter().cloned().collect::<Vec<_>>();
            let tc_root_v = tc.get_mut(&root[&v.id]).unwrap();
            tc_root_v.insert(v.id);
            tc_root_v.extend(tc_v);
            tc.get_mut(&v.id).unwrap().clear();
        }
    }
    for (_, v) in g.map.iter() {
        if *v.dfs_num.borrow() == usize::max_value() {
            node_tc(v, &mut stack, &mut root, &mut tc, &g, &mut num);
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
