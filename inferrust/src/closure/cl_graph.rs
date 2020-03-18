use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use crate::closure::utils;

pub struct Node {
    pub id: u64,
    pub dfs_num: RefCell<i32>,
    pub in_comp: RefCell<bool>,
}

impl Node {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            dfs_num: RefCell::new(-1),
            in_comp: RefCell::new(false),
        }
    }
    pub fn set_num(&self, num: i32) {
        *self.dfs_num.borrow_mut() = num;
    }
}

pub struct ClosureGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<[u64; 2]>,
    pub map: HashMap<u64, usize>,
    _len: usize,
}

impl ClosureGraph {
    pub fn from(pairs: Vec<[u64; 2]>) -> Self {
        let (nodes, map, _len) = Self::create_nodes(&pairs);
        Self {
            nodes,
            edges: pairs,
            map,
            _len,
        }
    }

    fn create_nodes(pairs: &Vec<[u64; 2]>) -> (Vec<Node>, HashMap<u64, usize>, usize) {
        let values: HashSet<u64> = pairs.iter().flat_map(|p| p.iter().cloned()).collect();
        let len = values.len();
        let mut nodes = Vec::with_capacity(len);
        let mut map = HashMap::new();
        let mut c = 0;
        for value in values {
            map.insert(value, c);
            nodes.push(Node::new(value));
            c += 1;
        }
        (nodes, map, len)
    }

    pub fn node(&self, id: u64) -> &Node {
        &self.nodes[self.index(id)]
    }

    pub fn index(&self, id: u64) -> usize {
        *self.map.get(&id).unwrap()
    }

    pub fn close(&mut self) -> HashMap<u64, HashSet<u64>> {
        utils::graph_tc(self)
    }
}
