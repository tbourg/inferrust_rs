use sophia::graph::GTripleSource;
use sophia::graph::Graph;
use sophia::term::{Term, TermData};
use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia::triple::{stream::TripleSource, Triple};

use std::convert::Infallible;

use crate::closure::*;
use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;
use crate::rules::*;
use crate::utils::*;

pub struct InfGraph {
    pub dictionary: NodeDictionary,
}

impl Graph for InfGraph {
    type Triple = ByTermRefs<std::rc::Rc<str>>;
    type Error = Infallible;

    fn triples(&self) -> GTripleSource<Self> {
        Box::from(
            self.dictionary
                .ts
                .elem
                .iter()
                .enumerate()
                .filter(|(_, chunk)| !chunk.so().is_empty())
                .map(move |(pi, chunk)| {
                    let p = self
                        .dictionary
                        .get_term(NodeDictionary::idx_to_prop_idx(pi));
                    chunk.so().iter().map(move |[si, oi]| {
                        Ok(StreamedTriple::by_term_refs(
                            self.dictionary.get_term(*si),
                            p,
                            self.dictionary.get_term(*oi),
                        ))
                    })
                })
                .flatten(),
        )
    }

    fn triples_with_s<'s, T>(&'s self, s: &'s Term<T>) -> GTripleSource<'s, Self>
    where
        T: TermData,
    {
        if let Some(si) = self.dictionary.get_index(s) {
            let s = self.dictionary.get_term(si);
            Box::from(
                self.dictionary
                    .ts
                    .elem
                    .iter()
                    .enumerate()
                    .filter(|(_, chunk)| !chunk.so().is_empty())
                    .map(move |(pi, chunk)| {
                        let p = self
                            .dictionary
                            .get_term(NodeDictionary::idx_to_prop_idx(pi));
                        let len = chunk.so().len();
                        let start_index = first(&chunk.so(), si, 0, len - 1, len, 0);
                        chunk.so()[start_index..]
                            .iter()
                            .take_while(move |[is, _]| si == *is)
                            .map(move |[_, oi]| {
                                Ok(StreamedTriple::by_term_refs(
                                    s,
                                    p,
                                    self.dictionary.get_term(*oi),
                                ))
                            })
                    })
                    .flatten(),
            )
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_p<'s, T>(&'s self, p: &'s Term<T>) -> GTripleSource<'s, Self>
    where
        T: TermData,
    {
        println!("My impl");
        if let Some(ip) = self.dictionary.get_index(p) {
            let idx = NodeDictionary::prop_idx_to_idx(ip);
            let chunk = &self.dictionary.ts.elem[idx];
            if !chunk.so().is_empty() {
                let p = self.dictionary.get_term(ip);
                Box::from(chunk.so().iter().map(move |[si, oi]| {
                    Ok(StreamedTriple::by_term_refs(
                        self.dictionary.get_term(*si),
                        p,
                        self.dictionary.get_term(*oi),
                    ))
                }))
            } else {
                Box::from(std::iter::empty())
            }
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_o<'s, T>(&'s self, o: &'s Term<T>) -> GTripleSource<'s, Self>
    where
        T: TermData,
    {
        if let Some(oi) = self.dictionary.get_index(o) {
            let o = self.dictionary.get_term(oi);
            Box::from(
                self.dictionary
                    .ts
                    .elem
                    .iter()
                    .enumerate()
                    .filter(|(_, chunk)| !chunk.os().is_empty())
                    .map(move |(pi, chunk)| {
                        let p = self
                            .dictionary
                            .get_term(NodeDictionary::idx_to_prop_idx(pi));
                        let len = chunk.os().len();
                        let start_index = first(&chunk.os(), oi, 0, len - 1, len, 0);
                        chunk.os()[start_index..]
                            .iter()
                            .take_while(move |[io, _]| oi == *io)
                            .map(move |[_, si]| {
                                Ok(StreamedTriple::by_term_refs(
                                    self.dictionary.get_term(*si),
                                    p,
                                    o,
                                ))
                            })
                    })
                    .flatten(),
            )
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_sp<'s, T, U>(
        &'s self,
        s: &'s Term<T>,
        p: &'s Term<U>,
    ) -> GTripleSource<'s, Self>
    where
        T: TermData,
        U: TermData,
    {
        if let (Some(si), Some(pi)) = (self.dictionary.get_index(s), self.dictionary.get_index(p)) {
            let idx = NodeDictionary::prop_idx_to_idx(pi);
            let chunk = &self.dictionary.ts.elem[idx];
            if !chunk.so().is_empty() {
                let s = self.dictionary.get_term(si);
                let p = self.dictionary.get_term(pi);
                let len = chunk.so().len();
                let start_index = first(&chunk.so(), si, 0, len - 1, len, 0);
                Box::from(
                    chunk.so()[start_index..]
                        .iter()
                        .take_while(move |[is, _]| *is == si)
                        .map(move |[_, oi]| {
                            Ok(StreamedTriple::by_term_refs(
                                s,
                                p,
                                self.dictionary.get_term(*oi),
                            ))
                        }),
                )
            } else {
                Box::from(std::iter::empty())
            }
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_so<'s, T, U>(
        &'s self,
        s: &'s Term<T>,
        o: &'s Term<U>,
    ) -> GTripleSource<'s, Self>
    where
        T: TermData,
        U: TermData,
    {
        if let (Some(si), Some(oi)) = (self.dictionary.get_index(s), self.dictionary.get_index(o)) {
            let s = self.dictionary.get_term(si);
            let o = self.dictionary.get_term(oi);
            Box::from(
                self.dictionary
                    .ts
                    .elem
                    .iter()
                    .enumerate()
                    .filter_map(move |(pi, chunk)| {
                        if chunk.so().is_empty() {
                            None
                        } else {
                            if binary_search_pair(&chunk.so(), [si, oi]) {
                                Some(Ok(StreamedTriple::by_term_refs(
                                    s,
                                    self.dictionary
                                        .get_term(NodeDictionary::idx_to_prop_idx(pi)),
                                    o,
                                )))
                            } else {
                                None
                            }
                        }
                    }),
            )
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_po<'s, T, U>(
        &'s self,
        p: &'s Term<T>,
        o: &'s Term<U>,
    ) -> GTripleSource<'s, Self>
    where
        T: TermData,
        U: TermData,
    {
        println!("My impl",);
        if let (Some(pi), Some(oi)) = (self.dictionary.get_index(p), self.dictionary.get_index(o)) {
            let idx = NodeDictionary::prop_idx_to_idx(pi);
            let chunk = &self.dictionary.ts.elem[idx];
            if !chunk.os().is_empty() {
                let p = self.dictionary.get_term(pi);
                let o = self.dictionary.get_term(oi);
                let len = chunk.os().len();
                let start_index = first(&chunk.os(), oi, 0, len - 1, len, 0);
                Box::from(
                    chunk.os()[start_index..]
                        .iter()
                        .take_while(move |[io, _]| *io == oi)
                        .map(move |[_, si]| {
                            Ok(StreamedTriple::by_term_refs(
                                self.dictionary.get_term(*si),
                                p,
                                o,
                            ))
                        }),
                )
            } else {
                Box::from(std::iter::empty())
            }
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_spo<'s, T, U, V>(
        &'s self,
        s: &'s Term<T>,
        p: &'s Term<U>,
        o: &'s Term<V>,
    ) -> GTripleSource<'s, Self>
    where
        T: TermData,
        U: TermData,
        V: TermData,
    {
        if let (Some(si), Some(pi), Some(oi)) = (
            self.dictionary.get_index(s),
            self.dictionary.get_index(p),
            self.dictionary.get_index(o),
        ) {
            let idx = NodeDictionary::prop_idx_to_idx(pi);
            let chunk = &self.dictionary.ts.elem[idx];
            if chunk.so().is_empty() {
                Box::from(std::iter::empty())
            } else {
                if binary_search_pair(&chunk.so(), [si, oi]) {
                    let s = self.dictionary.get_term(si);
                    let o = self.dictionary.get_term(oi);
                    let p = self.dictionary.get_term(pi);
                    Box::from(vec![Ok(StreamedTriple::by_term_refs(s, p, o))].into_iter())
                } else {
                    Box::from(std::iter::empty())
                }
            }
        } else {
            Box::from(std::iter::empty())
        }
    }
}

impl InfGraph {
    fn encode_triple<TD>(&mut self, t: &dyn Triple<TermData = TD>) -> [u64; 3]
    where
        TD: std::convert::AsRef<str> + std::clone::Clone + std::cmp::Eq + std::hash::Hash,
    {
        let mut s: u64 = 0;
        let mut o: u64 = 0;
        let p: u32;
        let ts = t.s();
        let to = t.o();
        let tp = t.p();
        // Property will always be property
        p = self.dictionary.add_property(tp);
        let prop_in_s_or_o = contains_prop_in_s_or_o(p, to);
        if prop_in_s_or_o != -1 {
            match prop_in_s_or_o {
                1 => {
                    s = self.dictionary.add_property(ts).into();
                    o = self.dictionary.add(to);
                }
                3 => {
                    s = self.dictionary.add_property(ts).into();
                    o = self.dictionary.add_property(to).into();
                }
                _ => (),
            }
        } else {
            // Add a regular triple
            s = self.dictionary.add(ts);
            o = self.dictionary.add(to);
        }
        [s, p as u64, o]
    }

    pub fn size(&mut self) -> usize {
        self.dictionary.ts.size()
    }

    pub fn process(&mut self, profile: &mut RuleProfile) {
        self.close(&mut profile.cl_profile);
        profile.before_rules.process(self);
        if profile.axiomatic_triples {
            self.init_axiomatic_triples();
        }
        profile.rules.process(self);
        match &profile.after_rules {
            Some(rule) => {
                rule(self);
            }
            None => (),
        }
    }

    pub fn close(&mut self, profile: &mut ClosureProfile) {
        // eprintln!("SubClassOf");
        if profile.on_sco {
            self.close_on(NodeDictionary::rdfssubClassOf);
        }
        if profile.on_spo {
            // eprintln!("SubPropertyOf");
            self.close_on(NodeDictionary::rdfssubPropertyOf);
        }
        if profile.on_sa {
            // eprintln!("SameAs");
            self.close_on(NodeDictionary::owlsameAs);
        }
        if profile.on_trp {
            for tr_idx in self.get_tr_idx() {
                self.close_on(tr_idx);
            }
        }
    }

    fn close_on(&mut self, index: u32) {
        let ip_to_store = NodeDictionary::prop_idx_to_idx(index as u64);
        self.close_on_raw(ip_to_store);
    }

    fn close_on_raw(&mut self, raw_index: usize) {
        // dbg!(&self.dictionary.ts.elem);
        let pairs = self.dictionary.ts.elem.get(raw_index);
        if pairs == None {
            return;
        }
        let pairs = pairs.unwrap().so().clone();
        if pairs.is_empty() {
            return;
        }
        let mut tc_g = ClosureGraph::from(pairs);
        let closure = tc_g.close();
        for (s, os) in closure.iter() {
            for o in os.iter() {
                self.dictionary.ts.add_triple_raw(*s, raw_index, *o);
            }
        }
        // let t = time::precise_time_ns();
        self.dictionary.ts.sort();
        // dbg!((time::precise_time_ns() - t) as f64 / 1e9);
    }

    fn get_tr_idx(&mut self) -> Vec<u32> {
        if let Some(pairs) = self.dictionary.ts.elem.get(NodeDictionary::prop_idx_to_idx(
            NodeDictionary::rdftype as u64,
        )) {
            pairs
                .so()
                .iter()
                .filter(|pair| pair[1] == NodeDictionary::owltransitiveProperty as u64)
                .map(|pair| pair[0] as u32)
                .collect()
        } else {
            vec![]
        }
    }

    pub fn init_axiomatic_triples(&mut self) {
        self.dictionary.ts.add_triple([
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsubject as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfpredicate as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfobject as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdffirst as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfrest as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfValue as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfnil as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfList,
        ]);
        // Domain
        self.dictionary.ts.add_triple([
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsubject as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfStatement,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfpredicate as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfStatement,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfobject as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfStatement,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsMember as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdffirst as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfList,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfrest as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfList,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsSeeAlso as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsisDefinedBy as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsComment as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsLabel as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfValue as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        // Range
        self.dictionary.ts.add_triple([
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsubject as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfpredicate as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfobject as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsMember as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdffirst as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfrest as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfList,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsSeeAlso as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsisDefinedBy as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsComment as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsLiteral,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsLabel as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsLiteral,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfValue as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        // MISC
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfAlt,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsContainer,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfBag,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsContainer,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfSeq,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsContainer,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsContainerMembershipProperty as u64,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsContainerMembershipProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsisDefinedBy as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsSeeAlso as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfXMLLiteral,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsDatatype,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfXMLLiteral,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsLiteral,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsDatatype,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::xsdnonNegativeInteger,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsDatatype,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::xsdstring,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsDatatype,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdftype as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsdomain as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsrange as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
        ]);
        self.dictionary.ts.add_triple([
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfssubClassOf as u64,
        ]);
    }
}

impl<TS> From<TS> for InfGraph
where
    TS: TripleSource,
{
    fn from(mut ts: TS) -> Self {
        let t0 = time::precise_time_ns();
        let store = TripleStore::new();
        let dictionary = NodeDictionary::new(store);
        let mut me = Self { dictionary };
        let mut enc_time = 0.0;
        let mut add_time = 0.0;
        ts.for_each_triple(|t| {
            let t0 = time::precise_time_ns();
            let rep = me.encode_triple(&t);
            let t1 = time::precise_time_ns();
            let time = (t1 - t0) as f64 / 1e9;
            enc_time += time;
            //eprintln!("{:?}", rep);
            let t0 = time::precise_time_ns();
            me.dictionary.ts.add_triple(rep);
            let t1 = time::precise_time_ns();
            let time = (t1 - t0) as f64 / 1e9;
            add_time += time;
        })
        .expect("Streaming error");
        let t1 = time::precise_time_ns();
        let time = (t1 - t0) as f64 / 1e9;
        println!("filling: {}({},{})", time, enc_time, add_time);
        let t0 = time::precise_time_ns();
        me.dictionary.ts.sort();
        let t1 = time::precise_time_ns();
        let time = (t1 - t0) as f64 / 1e9;
        println!("first sort: {}", time);
        me
    }
}

// Should return -1 if both s and o are res,
// 1 if s is prop and o is res,
// and 3 if both s and o are prop
fn contains_prop_in_s_or_o<TD>(property_index: u32, object: &Term<TD>) -> i8
where
    TD: std::convert::AsRef<str> + std::clone::Clone + std::cmp::Eq + std::hash::Hash,
{
    // Special case: if p a ...Property -> return 3
    if property_index == NodeDictionary::rdftype {
        let o_str = object.value();
        if o_str.to_lowercase().ends_with("property") {
            return 3;
        }
    }
    let prop_in_s = vec![NodeDictionary::rdfsdomain, NodeDictionary::rdfsrange];
    let prop_in_s_and_o = vec![
        NodeDictionary::owlequivalentProperty,
        NodeDictionary::owlinverseOf,
        NodeDictionary::rdfssubPropertyOf,
    ];
    if prop_in_s_and_o.contains(&property_index) {
        3
    } else if prop_in_s.contains(&property_index) {
        1
    } else {
        -1
    }
}
