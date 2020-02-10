use sophia::graph::inmem::*;
use sophia::graph::GTripleSource;
use sophia::graph::{Graph, MGResult, MutableGraph};
use sophia::term::factory::ArcTermFactory;
use sophia::term::{RcTerm, Term, TermData};
use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia::triple::Triple;

use std::convert::Infallible;
macro_rules! impl_mutable_graph_for_my_graph {
    ($my_mutable_graph: ty) => {
        impl MutableGraph for $my_mutable_graph {
            impl_mutable_graph_for_my_graph!();
        }
    };
    () => {
        type MutationError = std::convert::Infallible;

        fn insert<T_, U_, V_> (&mut self, s: &Term<T_>, p: &Term<U_>, o: &Term<V_>) -> MGResult< Self, bool> where
            T_: TermData,
            U_: TermData,
            V_: TermData,
        {
            self.triples.push([s.into(), p.into(), o.into()]);
            Ok(true)
        }
        fn remove<T_, U_, V_> (&mut self, s: &Term<T_>, p: &Term<U_>, o: &Term<V_>) -> MGResult< Self, bool> where
            T_: TermData,
            U_: TermData,
            V_: TermData,
        {
            Ok(true)
        }
    };
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

impl MutableGraph for MyGraph {
    impl_mutable_graph_for_my_graph!();
}

impl MyGraph {
    pub fn new() -> Self {
        MyGraph {
            triples: Vec::new(),
        }
    }
}
