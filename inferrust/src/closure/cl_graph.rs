use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use crate::closure::utils;

pub struct Node {
    pub id: u64,
    pub dfs_num: RefCell<usize>,
    pub in_comp: RefCell<bool>,
}

impl Node {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            dfs_num: RefCell::new(usize::max_value()),
            in_comp: RefCell::new(false),
        }
    }
    pub fn set_num(&self, num: usize) {
        *self.dfs_num.borrow_mut() = num;
    }
}

pub struct ClosureGraph {
    pub edges: Vec<[u64; 2]>,
    pub map: HashMap<u64, Node>,
}

impl ClosureGraph {
    pub fn from(pairs: Vec<[u64; 2]>) -> Self {
        let map = Self::create_nodes(&pairs);
        Self { edges: pairs, map }
    }

    fn create_nodes(pairs: &Vec<[u64; 2]>) -> HashMap<u64, Node> {
        let values: HashSet<u64> = pairs.iter().flat_map(|p| p.iter().cloned()).collect();
        let mut map = HashMap::new();
        for value in values {
            map.insert(value, Node::new(value));
        }
        map
    }

    pub fn node(&self, id: u64) -> &Node {
        self.map.get(&id).unwrap()
    }

    pub fn close(&mut self) -> HashMap<u64, HashSet<u64>> {
        utils::graph_tc(self)
    }
}
