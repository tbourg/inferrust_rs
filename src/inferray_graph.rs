// use sophia::graph::inmem::*;
// use sophia::graph::GTripleSource;
// use sophia::graph::{Graph, MGResult, MutableGraph};
// use sophia::ns;
// use sophia::term::factory::{TermFactory, RcTermFactory};
// use sophia::term::{RcTerm, Term, TermData};
// use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};
// use sophia::triple::{
//     stream::{TripleSink, TripleSource},
//     Triple,
// };

// use std::convert::Infallible;

// use bimap::hash::BiHashMap;

// struct CacheStore {}

// impl CacheStore {}

// struct NodeDictionary {
//     res_ctr: i64,
//     prop_ctr: i32,
//     removed_val: Vec<i64>,
//     resources: BiHashMap<RcTerm, i64>,
//     properties: BiHashMap<RcTerm, i32>,
//     store: CacheStore,
//     remap: bool,
//     factory: RcTermFactory
// }

// impl NodeDictionary {
//     const START_INDEX: i32 = i32::max_value();

//     fn new(cs: CacheStore) -> Self {
//         Self {
//             res_ctr: Self::START_INDEX as i64,
//             prop_ctr: Self::START_INDEX,
//             removed_val: vec![],
//             resources: BiHashMap::<RcTerm, i64>::new(),
//             properties: BiHashMap::<RcTerm, i32>::new(),
//             store: cs,
//             remap: false,
//             factory : RcTermFactory::new()
//         }
//     }

//     fn add(&mut self, str: &str) -> i64 {
//         let term = self.factory.iri(str).unwrap();
//         self.add_term(term)
//     }

//     fn add_property(&mut self, str: &str) -> i32 {
//         let term = self.factory.iri(str).unwrap();
//         self.add_property_term(term)
//     }

//     fn add_term(&mut self, t: RcTerm) -> i64 {
//         if self.resources.contains_left(&t) {
//             *self.resources.get_by_left(&t).unwrap()
//         } else {
//             self.res_ctr += 1;
//             self.resources.insert(t, self.res_ctr);
//             self.res_ctr
//         }
//     }

//     fn add_property_term(&mut self, t: RcTerm) -> i32 {
//         if self.resources.contains_left(&t) {
//             self.remap_res_to_prop(&t);
//         }
//         if self.properties.contains_left(&t) {
//             *self.properties.get_by_left(&t).unwrap()
//         } else {
//             self.prop_ctr -= 1;
//             self.properties.insert(t, self.prop_ctr);
//             self.prop_ctr
//         }
//     }

//     fn get(&self, index: i64) -> &str {
//         self.get_term(index).value().as_ref()
//     }

//     fn get_term(&self, index: i64) -> RcTerm {
//         if index < Self::START_INDEX as i64 {
//             self.properties
//                 .get_by_right(&(index as i32))
//                 .unwrap()
//                 .into()
//         } else {
//             self.resources.get_by_right(&index).unwrap().into()
//         }
//     }

//     fn get_str(&self, str: &str) -> Option<i64> {
//         let term = RcTerm::new_iri(str).unwrap();
//         if self.properties.contains_left(&term) {
//             Some(*self.properties.get_by_left(&term).unwrap() as i64)
//         } else if self.resources.contains_left(&term) {
//             Some(*self.resources.get_by_left(&term).unwrap() as i64)
//         } else {
//             Option::None
//         }
//     }

//     fn size(&self) -> i64 {
//         (Self::START_INDEX as i64 - self.prop_ctr as i64)
//             + (self.res_ctr - Self::START_INDEX as i64)
//     }

//     fn remap_res_to_prop(&mut self, t: &RcTerm) {
//         self.remap = true;
//         let old: i32 = *self.resources.get_by_left(t).unwrap() as i32;
//         self.prop_ctr -= 1;
//         let p = self.prop_ctr;
//         self.properties.insert(Term::from(t), p);
//         self.removed_val
//             .push(self.resources.remove_by_left(&t).unwrap().1);
//         self.store.replace_res_by_prop(old, p);
//     }

//     fn get_res_ctr(&self) -> i64 {
//         self.res_ctr
//     }

//     fn prop_cnt(&self) -> i32 {
//         Self::START_INDEX - self.prop_ctr
//     }

//     fn has_remap(&self) -> bool {
//         self.remap
//     }

//     fn was_removed(&self, index: i64) -> bool {
//         self.removed_val.contains(&index)
//     }
// }

// pub struct InfGraph {
//     dictionary: NodeDictionary,
//     store: CacheStore,
// }

// // impl Graph for InfGraph {
// //     type Triple = ByTermRefs<std::rc::Rc<str>>;
// //     type Error = Infallible;

// //     fn triples(&self) -> GTripleSource<Self> {
// //         Box::from(
// //             self.triples
// //                 .iter()
// //                 .map(move |[s, p, o]| Ok(StreamedTriple::by_term_refs(s, p, o))),
// //         )
// //     }
// // }

// impl InfGraph {
//     fn encode_triple(&mut self, t: &dyn Triple<TermData = std::rc::Rc<str>>) -> [i64; 3] {
//         let contains_prop = contains_prop_in_s_or_o(t);
//         let mut s: i64 = -1;
//         let mut o: i64 = -1;
//         let mut p: i32 = -1;
//         let s_str = t.s().value();
//         let o_str = t.o().value();
//         let p_str = t.p().value();
//         // Property will always be property
//         p = self.dictionary.add_property(&p_str);
//         if contains_prop != -1 {
//             match contains_prop {
//                 1 => {
//                     s = self.dictionary.add_property(&s_str).into();
//                     o = self.dictionary.add(&o_str);
//                 }
//                 3 => {
//                     s = self.dictionary.add_property(&s_str).into();
//                     o = self.dictionary.add_property(&o_str).into();
//                 }
//                 _ => (),
//             }
//         } else {
//             // Add a regular triple
//             s = self.dictionary.add(&s_str);
//             o = self.dictionary.add(&o_str);
//         }
//         [s, p as i64, o]
//     }
// }

// fn contains_prop_in_s_or_o(t: &dyn Triple<TermData = std::rc::Rc<str>>) -> i32 {
//     let p_str = t.p().value();
//     if p_str == ns::rdf::type_.value() {
//         3
//     } else {
//         -1
//     }
// }

// //     if (triple.getObject().toString().toLowerCase()
// //     .endsWith("property")) {

// //     if (triple.getObject().toString()
// //     .startsWith("http://www.w3.org/2002/07/owl#")) {

// //     return 3;
// //     }
// //     }

// //     }
// //     if (havePropertiesInObject.contains(ps)) {
// //     return 3;
// //     } else if (havePropertiesInSubject.contains(ps)) {
// //     return 1;
// //     }
// //     return -1;
// //     }

// //     public void setself.store(final Cacheself.store self.store) {
// //     this.self.store = store;
// //     }

// //     /**
// //      * Initialize the different sets of the parser
// //      */
// //     protected void initSets() {
// //     havePropertiesInSubject = new HashSet<>();
// //     havePropertiesInObject = new HashSet<>();
// //     // For subjects
// //     havePropertiesInSubject.add(dictionary
// //     .get(AbstractDictionary.rdfsdomain));
// //     havePropertiesInSubject.add(dictionary
// //     .get(AbstractDictionary.rdfsrange));
// //     havePropertiesInSubject.add(dictionary
// //     .get(AbstractDictionary.owlequivalentProperty));
// //     havePropertiesInSubject.add(dictionary
// //     .get(AbstractDictionary.owlinverseOf));
// //     havePropertiesInSubject.add(dictionary
// //     .get(AbstractDictionary.rdfssubPropertyOf));
// //     havePropertiesInSubject.add(dictionary
// //     .get(AbstractDictionary.owlsymetricProperty));
// //     // Objects
// //     havePropertiesInObject.add(dictionary
// //     .get(AbstractDictionary.rdfssubPropertyOf));
// //     havePropertiesInObject.add(dictionary
// //     .get(AbstractDictionary.owlinverseOf));
// //     havePropertiesInObject.add(dictionary
// //     .get(AbstractDictionary.owlequivalentProperty));
// //     havePropertiesInObject.add(dictionary
// //     .get(AbstractDictionary.owlsymetricProperty));

// //     }
