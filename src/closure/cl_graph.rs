use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::rc::Rc;

pub struct Node {
    pub id: i64,
    pub dfs_num: i32,
    pub in_comp: bool,
}

impl Node {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            dfs_num: -1,
            in_comp: false,
        }
    }
    pub fn set_num(&mut self, num: i32) {
        self.dfs_num = num;
    }
}

pub struct ClosureGraph {
    pub nodes: HashMap<i64, Node>,
    pub edges: Vec<[i64; 2]>,
}

impl ClosureGraph {
    pub fn from(pairs: Vec<[i64; 2]>) -> Self {
        let mut me = Self {
            nodes: HashMap::new(),
            edges: Vec::clone(&pairs),
        };
        for pair in pairs {
            me.add_nodes(pair);
        }
        me
    }

    fn add_nodes(&mut self, ids: [i64; 2]) {
        for id in &ids {
            if !self.nodes.contains_key(id) {
                self.nodes.insert(*id, Node::new(*id));
            }
        }
    }

    pub fn node(&mut self, id: i64) -> &mut Node {
        self.nodes.get_mut(&id).unwrap()
    }
}
