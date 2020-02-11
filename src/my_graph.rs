use sophia::graph::inmem::*;
use sophia::graph::GTripleSource;
use sophia::graph::{Graph, MGResult, MutableGraph};
use sophia::term::factory::ArcTermFactory;
use sophia::term::{RcTerm, Term, TermData};
use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia::triple::{
    stream::{TripleSink, TripleSource},
    Triple,
};

use std::convert::Infallible;

struct MySink {
    triples: Vec<[RcTerm; 3]>,
}

impl TripleSink for MySink {
    type Outcome = Vec<[RcTerm; 3]>;
    type Error = Infallible;

    fn feed<T: Triple>(&mut self, t: &T) -> Result<(), Self::Error> {
        Ok(self
            .triples
            .push([t.s().into(), t.p().into(), t.o().into()]))
    }

    fn finish(&mut self) -> Result<Self::Outcome, Self::Error> {
        Ok(self.triples.clone())
    }
}

impl MySink {
    fn new() -> Self {
        MySink {
            triples: Vec::new(),
        }
    }
}

pub struct MyGraph {
    triples: Vec<[RcTerm; 3]>,
}

impl Graph for MyGraph {
    type Triple = ByTermRefs<std::rc::Rc<str>>;
    type Error = Infallible;

    fn triples(&self) -> GTripleSource<Self> {
        Box::from(
            self.triples
                .iter()
                .map(move |[s, p, o]| Ok(StreamedTriple::by_term_refs(s, p, o))),
        )
    }
}

impl MyGraph {
    pub fn from(g: LightGraph) -> Self {
        let mut tsk = MySink::new();
        g.triples().in_sink(&mut tsk);
        MyGraph {
            triples: tsk.finish().unwrap(),
        }
    }
}
