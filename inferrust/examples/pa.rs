use inferrust::closure::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};

struct MyStruct {
    elem: Vec<u64>,
}

impl MyStruct {
    fn new() -> Self {
        Self { elem: Vec::new() }
    }

    fn add(&mut self, e: u64) {
        dbg!(&e);
        if !self.contains(&e) {
            if self.elem.is_empty() || e > self.elem[self.elem.len() - 1] {
                self.elem.push(e);
            } else {
                self.elem.insert(0, e);
            }
        }
        dbg!(&self.elem);
    }

    fn contains(&mut self, e: &u64) -> bool {
        if self.elem.is_empty() {
            false
        } else {
            *e >= self.elem[0] && *e <= self.elem[self.elem.len() - 1]
        }
    }

    fn size(&mut self) -> usize {
        self.elem.len()
    }
}

fn main() {
    let arcs = vec![
        [10, 0],
        [10, 20],
        [10, 30],
        [20, 30],
        [30, 10],
        [40, 20],
        [40, 50],
    ];
    let mut graph = ClosureGraph::from(arcs);
    let tc = graph.close();
    dbg!(tc);
    let numbers: Vec<u64> = (0..10)
        .map(|_| rand::thread_rng().gen_range(1, 11))
        .collect();
    let mut myset = MyStruct::new();
    let t = time::precise_time_ns();
    for n in numbers.iter() {
        myset.add(*n);
    }
    dbg!((time::precise_time_ns() - t) as f64 / 1e9, myset.size());
    let mut set = HashSet::new();
    let t = time::precise_time_ns();
    for n in numbers.iter() {
        set.insert(*n);
    }
    dbg!((time::precise_time_ns() - t) as f64 / 1e9, set.len());
}
