#![allow(warnings)]

use sophia::graph::inmem::{LightGraph, TermIndexMapU};
use sophia::graph::GTripleSource;
use sophia::graph::{Graph, MGResult, MutableGraph};
use sophia::term::factory::RcTermFactory;
use sophia::term::{index_map::TermIndexMap, RcTerm, RefTerm, Term, TermData};
use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia::triple::{
    stream::{TripleSink, TripleSource},
    Triple,
};

use bit_matrix::BitMatrix;

use std::clone::Clone;
use std::collections::HashMap;
use std::convert::Infallible;

// pub struct CountSink {
//     size: i32,
// }

// impl TripleSink for CountSink {
//     type Outcome = i32;
//     type Error = Infallible;

//     fn feed<T: Triple>(&mut self, _: &T) -> Result<(), Self::Error> {
//         Ok(self.size += 1)
//     }

//     fn finish(&mut self) -> Result<Self::Outcome, Self::Error> {
//         Ok(self.size)
//     }
// }

// impl CountSink {
//     pub fn new() -> Self {
//         CountSink { size: 0 }
//     }
// }

pub struct MyGraph {
    // matrix: BitMatrix,
    store: HashMap<u16, HashMap<u16, [u16; 2]>>,
    index_map: TermIndexMapU<u16, RcTermFactory>,
}
impl Graph for MyGraph {
    type Triple = ByTermRefs<std::rc::Rc<str>>;
    type Error = Infallible;

    fn triples(&self) -> GTripleSource<Self> {
        let mut v: Vec<Result<StreamedTriple<ByTermRefs<std::rc::Rc<str>>>, Self::Error>> =
            Vec::new();
        eprintln!(":{:?}", self.store);
        for (p, tso) in &self.store {
            eprintln!("p: {}", p);
            let p = self.index_map.get_term(*p).unwrap();
            for (t, so) in tso {
                eprintln!("t: {}", t);
                let s = self.index_map.get_term(so[0]).unwrap();
                let o = self.index_map.get_term(so[1]).unwrap();
                v.push(Ok(StreamedTriple::by_term_refs(s, p, o)));
            }
        }
        Box::from(v.into_iter())
    }
}

impl MyGraph {
    pub fn from(g: &LightGraph) -> Self {
        let mut store: HashMap<u16, HashMap<u16, [u16; 2]>> = HashMap::new();
        let mut index_map: TermIndexMapU<u16, RcTermFactory> = TermIndexMapU::default();
        let mut c = 1;
        g.triples().for_each_triple(|t| {
            let p = t.p();
            let o = t.o();
            let s = t.s();
            let ip = index_map.make_index(&RefTerm::from_with(p, |td| &*td));
            let is = index_map.make_index(&RefTerm::from_with(s, |td| &*td));
            let io = index_map.make_index(&RefTerm::from_with(o, |td| &*td));
            let mut chunk = if store.contains_key(&ip) {
                store.get(&ip).unwrap().clone()
            } else {
                HashMap::new()
            };
            chunk.insert(c, [is, io]);
            store.insert(ip, chunk);
            c += 1;
            eprintln!("{:?}", store);
            // store.insert([is, io], ip);
        });
        MyGraph { store, index_map }
    }
}
