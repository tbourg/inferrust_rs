use crate::closure::{ClosureGraph, Node};

use std::collections::HashMap;

fn graph_tc(g: ClosureGraph) {
    let stack = Vec::new();
    let root = HashMap::new();
    let tc = HashMap::new();
    let num = 1;
      let node_tc = |v: &mut Node| {
          v.set_num(num);
          num += 1;
          root.insert(v.id, v.id);
          stack.push(v);
          let succ = Vec::new();
          for edge in g.edges {
              if edge[0] == v.id {
                  succ.push(edge[1]);
              }
          }
          tc.insert(v.id, succ);
          for wi in succ {
              let w = g.node(wi);
              if w.dfs_num != -1 {
                  node_tc(w);
                  v.root = minn(v.root, w.root);
                } else if(v.n > w.n) {
                    if(!w.inC) {
                        v.root = minn(v.root, w.root);
                    }
                }
                //v.tc = new Set([...v.tc,...w.tc]);
            }
            if(v.root == v) {
                /*do {
                    w = stack.pop();
                    w.inC = true;
                    w.tc = new Set([...w.tc,...v.tc]);
                    w.root = v;
                } while(w != v);*/
            }
        }
        
        for (_,v) in g.nodes {
            if(!v.visited) {
                node_tc(&mut v);
            }
        }
    }
    
    fn minn(a,b) {
        return a.n <= b.n ? a : b;
    }
    