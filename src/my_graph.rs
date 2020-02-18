use sophia::graph::inmem::{LightGraph, TermIndexMapU};
use sophia::graph::GTripleSource;
use sophia::graph::{Graph, MGResult, MutableGraph};
use sophia::term::factory::RcTermFactory;
use sophia::term::{RcTerm, Term, TermData};
use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia::triple::{
    stream::{TripleSink, TripleSource},
    Triple,
};

use bit_matrix::BitMatrix;

use std::clone::Clone;
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
    matrix: BitMatrix,
    index_map: TermIndexMapU<u16, RcTermFactory>,
}
impl Graph for MyGraph {
    type Triple = ByTermRefs<std::rc::Rc<str>>;
    type Error = Infallible;

    fn triples(&self) -> GTripleSource<Self> {
        self.triples()
    }
}

impl MyGraph {
    pub fn from(g: &LightGraph) -> () {
        let rows = g.subjects().unwrap().len() + g.objects().unwrap().len();
        let mut cols = 0;
        g.triples().for_each_triple(|_| cols += 1);
        let matrix = BitMatrix::new(rows, cols as usize);
    }
}
