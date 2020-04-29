use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::closure::utils;

#[derive(Clone)]
pub struct Node {
    pub id: u64,
    dfs_num: RefCell<usize>,
    in_comp: RefCell<bool>,
    root: RefCell<Option<u64>>,
    tc: RefCell<HashSet<u64>>,
}

impl Node {
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn new(id: u64) -> Self {
        Self {
            id,
            dfs_num: RefCell::new(usize::max_value()),
            in_comp: RefCell::new(false),
            root: RefCell::new(None),
            tc: RefCell::new(HashSet::new()),
        }
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn set_num(&self, num: usize) {
        *self.dfs_num.borrow_mut() = num;
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn num(&self) -> usize {
        *self.dfs_num.borrow()
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn in_comp(&self) -> bool {
        *self.in_comp.borrow()
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn set_in_comp(&self, in_c: bool) {
        *self.in_comp.borrow_mut() = in_c;
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn root(&self) -> u64 {
        self.root.borrow().unwrap()
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn set_root(&self, r: u64) {
        *self.root.borrow_mut() = Some(r);
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn tc_contains(&self, val: u64) -> bool {
        self.tc.borrow().contains(&val)
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn tc_insert(&self, val: u64) {
        self.tc.borrow_mut().insert(val);
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn tc_extend<I: IntoIterator<Item = u64>>(&self, vals: I) {
        self.tc.borrow_mut().extend(vals);
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn tc_iter(&self) -> Vec<u64> {
        self.tc.borrow().iter().cloned().collect::<Vec<u64>>()
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn tc_clear(&self) {
        self.tc.borrow_mut().clear();
    }

    #[inline]
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn tc_is_empty(&self) -> bool {
        self.tc.borrow().is_empty()
    }
}

pub struct ClosureGraph {
    pub edges: Vec<[u64; 2]>,
    pub nodes: Vec<Option<Node>>,
    pub offset: u64,
}

impl ClosureGraph {
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn from(pairs: Vec<[u64; 2]>) -> Self {
        let mut offset = pairs[0][0];
        let maxv = pairs[pairs.len() - 1][0];
        let values: Vec<u64> = pairs.iter().flat_map(|p| p.iter().cloned()).collect();
        // NB: this is just a heuristics, some nodes may be objects only,
        // so their identifier may be outside those bounds
        let mut nodes = vec![None; (maxv - offset + 1) as usize];
        for value in values {
            if value < offset {
                let shift = offset - value;
                offset -= shift;
                for _ in 0..shift {
                    nodes.insert(0, None);
                }
            }
            let idx = (value - offset) as usize;
            if nodes.len() <= idx {
                nodes.resize_with(idx + 1, Default::default);
            }
            nodes[idx] = Some(Node::new(value));
        }
        Self {
            edges: pairs,
            nodes,
            offset,
        }
    }

    /*
    #[cfg_attr(debug_assertions, flamer::flame)]
        fn create_nodes(pairs: &Vec<[u64; 2]>) -> HashMap<u64, Node> {
            let values: HashSet<u64> = pairs.iter().flat_map(|p| p.iter().cloned()).collect();
            let mut map = HashMap::new();
            for value in values {
                map.insert(value, Node::new(value));
            }
            map
        }
        */

    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn node(&self, id: u64) -> &Node {
        let idx = (id - self.offset) as usize;
        /*
            if idx >= self.nodes.borrow().len() {
                self.nodes.borrow_mut().resize(idx + 1, None);
            }
            self.nodes.borrow().get(idx).unwrap().as_ref().unwrap()
        */
        self.nodes.get(idx).unwrap().as_ref().unwrap()
    }

    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter().filter_map(|opt| opt.as_ref())
    }

    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn edges(&self, id: u64) -> Vec<u64> {
        let len = self.edges.len();
        let start_index = crate::utils::first(&self.edges, id, 0, len - 1, len, 0);
        self.edges[start_index..]
            .iter()
            .take_while(|e| e[0] == id)
            .map(|e| e[1])
            .collect()
    }

    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn close(&mut self) -> HashMap<u64, Rc<Vec<u64>>> {
        utils::graph_tc(self);

        let mut tc = HashMap::new();
        for v in self.iter_nodes() {
            if v.root() == v.id {
                tc.insert(v.id, Rc::new(v.tc_iter()));
            }
        }
        for v in self.iter_nodes() {
            if v.root() != v.id {
                debug_assert!(v.tc_is_empty());
                tc.insert(v.id, Rc::clone(&tc[&v.root()]));
            }
        }
        tc
    }
}
