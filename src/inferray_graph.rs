use sophia::graph::inmem::*;
use sophia::graph::GTripleSource;
use sophia::graph::{Graph, MGResult, MutableGraph};
use sophia::ns;
use sophia::term::factory::{RcTermFactory, TermFactory};
use sophia::term::{RcTerm, Term, TermData};
use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia::triple::{
    stream::{TripleSink, TripleSource},
    Triple,
};

use std::convert::Infallible;
use std::convert::TryInto;

use bimap::hash::BiHashMap;

struct TripleStore {
    elem: Vec<Vec<[i64; 2]>>,
}

impl TripleStore {
    fn new(capacity: usize) -> Self {
        let mut elem = Vec::new();
        elem.resize(capacity, Vec::new());
        Self { elem }
    }

    fn add_triple(&mut self, triple: [i64; 3]) {
        let [is, ip, io] = triple;
        let ip = NodeDictionary::prop_idx_to_idx(ip);
        self.elem[ip].push([is, io]);
    }
}

struct NodeDictionary {
    res_ctr: i64,
    prop_ctr: i32,
    removed_val: Vec<i64>,
    resources: BiHashMap<RcTerm, i64>,
    properties: BiHashMap<RcTerm, i32>,
    remap: bool,
    factory: RcTermFactory,
}

impl NodeDictionary {
    const START_INDEX: i32 = i32::max_value();

    fn new() -> Self {
        Self {
            res_ctr: Self::START_INDEX as i64,
            prop_ctr: Self::START_INDEX,
            removed_val: vec![],
            resources: BiHashMap::<RcTerm, i64>::new(),
            properties: BiHashMap::<RcTerm, i32>::new(),
            remap: false,
            factory: RcTermFactory::new(),
        }
    }

    fn add(&mut self, str: &str) -> i64 {
        let term = self.factory.iri(str).expect("Err");
        self.add_term(term)
    }

    fn add_property(&mut self, str: &str) -> i32 {
        let term = self.factory.iri(str).expect("Err");
        self.add_property_term(term)
    }

    fn add_term(&mut self, t: RcTerm) -> i64 {
        if self.resources.contains_left(&t) {
            *self.resources.get_by_left(&t).expect("Err")
        } else {
            self.res_ctr += 1;
            self.resources.insert(t, self.res_ctr);
            self.res_ctr
        }
    }

    fn add_property_term(&mut self, t: RcTerm) -> i32 {
        if self.properties.contains_left(&t) {
            *self.properties.get_by_left(&t).expect("Err")
        } else {
            self.prop_ctr -= 1;
            self.properties.insert(t, self.prop_ctr);
            self.prop_ctr
        }
    }

    fn get_term(&self, index: i64) -> &RcTerm {
        if index < Self::START_INDEX as i64 {
            self.properties.get_by_right(&(index as i32)).expect("Err")
        } else {
            self.resources.get_by_right(&index).expect("Err")
        }
    }

    fn size(&self) -> i64 {
        (Self::START_INDEX as i64 - self.prop_ctr as i64)
            + (self.res_ctr - Self::START_INDEX as i64)
    }

    fn get_res_ctr(&self) -> i64 {
        self.res_ctr
    }

    fn prop_cnt(&self) -> i32 {
        Self::START_INDEX - self.prop_ctr
    }

    fn has_remap(&self) -> bool {
        self.remap
    }

    fn was_removed(&self, index: i64) -> bool {
        self.removed_val.contains(&index)
    }

    fn prop_idx_to_idx(prop_idx: i64) -> usize {
        (Self::START_INDEX as i64 - prop_idx - 1)
            .try_into()
            .expect("Err converting index")
    }

    fn idx_to_prop_idx(idx: usize) -> i64 {
        Self::START_INDEX as i64 - idx as i64 - 1
    }
}

pub struct InfGraph {
    dictionary: NodeDictionary,
    store: TripleStore,
}

impl Graph for InfGraph {
    type Triple = ByTermRefs<std::rc::Rc<str>>;
    type Error = Infallible;

    fn triples(&self) -> GTripleSource<Self> {
        let mut v: Vec<Result<StreamedTriple<ByTermRefs<std::rc::Rc<str>>>, Self::Error>> =
            Vec::new();
        eprintln!(":{:?}", self.store.elem);
        for (ip, chunk) in (&self.store.elem).iter().enumerate() {
            // eprintln!("p: {}", p);
            let ip = NodeDictionary::idx_to_prop_idx(ip);
            let p = self.dictionary.get_term(ip);
            for pair in chunk {
                // eprintln!("t: {}", t);
                let s = self.dictionary.get_term(pair[0]);
                let o = self.dictionary.get_term(pair[1]);
                v.push(Ok(StreamedTriple::by_term_refs(s, p, o)));
            }
        }
        Box::from(v.into_iter())
    }
}

impl InfGraph {
    pub fn from(g: &LightGraph) -> Self {
        let capacity: usize = g.predicates().expect("from").len();
        let store = TripleStore::new(capacity);
        let dictionary = NodeDictionary::new();
        let mut me = Self { dictionary, store };
        g.triples()
            .for_each_triple(|t| {
                let rep = me.encode_triple(&t);
                eprintln!("{:?}", rep);
                me.store.add_triple(rep);
            })
            .expect("Streaming error");
        me
    }

    fn encode_triple(&mut self, t: &dyn Triple<TermData = std::rc::Rc<str>>) -> [i64; 3] {
        let contains_prop = contains_prop_in_s_or_o(t);
        let mut s: i64 = -1;
        let mut o: i64 = -1;
        let mut p: i32 = -1;
        let s_str = t.s().value();
        let o_str = t.o().value();
        let p_str = t.p().value();
        // Property will always be property
        p = self.dictionary.add_property(&p_str);
        if contains_prop != -1 {
            match contains_prop {
                1 => {
                    s = self.dictionary.add_property(&s_str).into();
                    o = self.dictionary.add(&o_str);
                }
                3 => {
                    s = self.dictionary.add_property(&s_str).into();
                    o = self.dictionary.add_property(&o_str).into();
                }
                _ => (),
            }
        } else {
            // Add a regular triple
            s = self.dictionary.add(&s_str);
            o = self.dictionary.add(&o_str);
        }
        [s, p as i64, o]
    }
}

// Should return -1 if both s and o are res,
// 1 if s is prop and o is res,
// and 3 if both s and o are prop
fn contains_prop_in_s_or_o(t: &dyn Triple<TermData = std::rc::Rc<str>>) -> i32 {
    -1
}
