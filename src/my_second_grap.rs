// use sophia::graph::inmem::*;
// use sophia::graph::Graph;
// use sophia_src::graph::GTripleSource;
// use sophia_src::graph::{Graph as GraphSrc, MGResult, MutableGraph};
// use sophia_src::term::factory::ArcTermFactory;
// use sophia_src::term::{RcTerm, Term, TermData};
// use sophia_src::triple::streaming_mode::{ByTermRefs, StreamedTriple};
// use sophia_src::triple::{
//     stream::{TripleSink, TripleSource},
//     Triple,
// };

// use std::convert::Infallible;

// pub struct MySecondGraph {
//     triples: Vec<[RcTerm; 3]>,
// }

// impl GraphSrc for MySecondGraph {
//     type Triple = ByTermRefs<std::rc::Rc<str>>;
//     type Error = Infallible;

//     fn triples(&self) -> GTripleSource<Self> {
//         Box::from(
//             self.triples
//                 .iter()
//                 .map(move |[s, p, o]| Ok(StreamedTriple::by_term_refs(s, p, o))),
//         )
//     }
// }

// impl MySecondGraph {
//     pub fn from(g: LightGraph) -> Self {
//         let vec = Vec::new();
//         g.triples()
//             .try_for_each_triple(|t| Ok(vec.push([t.s().into(), t.p().into(), t.o().into()])));
//         MySecondGraph { triples: vec }
//     }
// }
