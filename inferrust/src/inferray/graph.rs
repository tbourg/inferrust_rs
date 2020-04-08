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
                .filter(|(_, chunk)| !chunk[0].is_empty())
                .map(move |(pi, chunk)| {
                    let p = self
                        .dictionary
                        .get_term(NodeDictionary::idx_to_prop_idx(pi));
                    chunk[0].iter().map(move |[si, oi]| {
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
                    .filter(|(_, chunk)| !chunk[0].is_empty())
                    .map(move |(pi, chunk)| {
                        let p = self
                            .dictionary
                            .get_term(NodeDictionary::idx_to_prop_idx(pi));
                        let len = chunk[0].len();
                        let start_index = first(&chunk[0], si, 0, len - 1, len, 0);
                        chunk[0][start_index..]
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
            if !chunk[0].is_empty() {
                let p = self.dictionary.get_term(ip);
                Box::from(chunk[0].iter().map(move |[si, oi]| {
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
                    .filter(|(_, chunk)| !chunk[1].is_empty())
                    .map(move |(pi, chunk)| {
                        let p = self
                            .dictionary
                            .get_term(NodeDictionary::idx_to_prop_idx(pi));
                        let len = chunk[1].len();
                        let start_index = first(&chunk[1], oi, 0, len - 1, len, 0);
                        chunk[1][start_index..]
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
            if !chunk[0].is_empty() {
                let s = self.dictionary.get_term(si);
                let p = self.dictionary.get_term(pi);
                let len = chunk[0].len();
                let start_index = first(&chunk[0], si, 0, len - 1, len, 0);
                Box::from(
                    chunk[0][start_index..]
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
                        if chunk[0].is_empty() {
                            None
                        } else {
                            if binary_search_pair(&chunk[0], [si, oi]) {
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
            if !chunk[1].is_empty() {
                let p = self.dictionary.get_term(pi);
                let o = self.dictionary.get_term(oi);
                let len = chunk[1].len();
                let start_index = first(&chunk[1], oi, 0, len - 1, len, 0);
                Box::from(
                    chunk[1][start_index..]
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
            if chunk[0].is_empty() {
                Box::from(std::iter::empty())
            } else {
                if binary_search_pair(&chunk[0], [si, oi]) {
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
        let prop_in_s_or_o = contains_prop_in_s_or_o(p, to, &self.dictionary);
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
        match profile.after_rules {
            Some(rule) => {
                let out = rule(self);
                self.dictionary.ts.add_all(out);
                self.dictionary.ts.sort();
            }
            None => (),
        }
    }

    pub fn close(&mut self, profile: &mut ClosureProfile) {
        // eprintln!("SubClassOf");
        if profile.on_sco {
            self.close_on(self.dictionary.rdfssubClassOf);
        }
        if profile.on_spo {
            // eprintln!("SubPropertyOf");
            self.close_on(self.dictionary.rdfssubPropertyOf);
        }
        if profile.on_sa {
            // eprintln!("SameAs");
            self.close_on(self.dictionary.owlsameAs);
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
        let pairs = pairs.unwrap()[0].clone();
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
        let t = time::precise_time_ns();
        self.dictionary.ts.sort();
        dbg!((time::precise_time_ns() - t) as f64 / 1e9);
    }

    fn get_tr_idx(&mut self) -> Vec<u32> {
        if let Some(pairs) = self.dictionary.ts.elem.get(NodeDictionary::prop_idx_to_idx(
            self.dictionary.rdftype as u64,
        )) {
            pairs[0]
                .iter()
                .filter(|pair| pair[1] == self.dictionary.owltransitiveProperty as u64)
                .map(|pair| pair[0] as u32)
                .collect()
        } else {
            vec![]
        }
    }

    pub fn init_axiomatic_triples(&mut self) {
        self.dictionary.ts.add_triple([
            self.dictionary.rdftype as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsubject as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfpredicate as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfobject as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdffirst as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfrest as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfValue as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdf_1 as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfnil as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfList,
        ]);
        // Domain
        self.dictionary.ts.add_triple([
            self.dictionary.rdftype as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfssubClassOf as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfssubPropertyOf as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsubject as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfStatement,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfpredicate as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfStatement,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfobject as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfStatement,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsMember as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdffirst as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfList,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfrest as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfList,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsSeeAlso as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsisDefinedBy as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsComment as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsLabel as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfValue as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsResource,
        ]);
        // Range
        self.dictionary.ts.add_triple([
            self.dictionary.rdftype as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfssubClassOf as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfssubPropertyOf as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsubject as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfpredicate as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfobject as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsMember as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdffirst as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfrest as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfList,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsSeeAlso as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsisDefinedBy as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsComment as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsLiteral,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsLabel as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsLiteral,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfValue as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsResource,
        ]);
        // MISC
        self.dictionary.ts.add_triple([
            self.dictionary.rdfAlt,
            self.dictionary.rdfssubClassOf as u64,
            self.dictionary.rdfsContainer,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfBag,
            self.dictionary.rdfssubClassOf as u64,
            self.dictionary.rdfsContainer,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfSeq,
            self.dictionary.rdfssubClassOf as u64,
            self.dictionary.rdfsContainer,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsContainerMembershipProperty as u64,
            self.dictionary.rdfssubClassOf as u64,
            self.dictionary.rdfProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdf_1 as u64,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfsContainerMembershipProperty as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdf_1 as u64,
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdf_1 as u64,
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfsResource,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsisDefinedBy as u64,
            self.dictionary.rdfssubPropertyOf as u64,
            self.dictionary.rdfsSeeAlso as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfXMLLiteral,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfsDatatype,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfXMLLiteral,
            self.dictionary.rdfssubClassOf as u64,
            self.dictionary.rdfsLiteral,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsDatatype,
            self.dictionary.rdfssubClassOf as u64,
            self.dictionary.rdfsClass,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.xsdnonNegativeInteger,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfsDatatype,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.xsdstring,
            self.dictionary.rdftype as u64,
            self.dictionary.rdfsDatatype,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdftype as u64,
            self.dictionary.rdfssubPropertyOf as u64,
            self.dictionary.rdftype as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsdomain as u64,
            self.dictionary.rdfssubPropertyOf as u64,
            self.dictionary.rdfsdomain as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfsrange as u64,
            self.dictionary.rdfssubPropertyOf as u64,
            self.dictionary.rdfsrange as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfssubPropertyOf as u64,
            self.dictionary.rdfssubPropertyOf as u64,
            self.dictionary.rdfssubPropertyOf as u64,
        ]);
        self.dictionary.ts.add_triple([
            self.dictionary.rdfssubClassOf as u64,
            self.dictionary.rdfssubPropertyOf as u64,
            self.dictionary.rdfssubClassOf as u64,
        ]);
    }
}

impl<TS> From<TS> for InfGraph
where
    TS: TripleSource,
{
    fn from(mut ts: TS) -> Self {
        let store = TripleStore::new();
        let dictionary = NodeDictionary::new(store);
        let mut me = Self { dictionary };
        ts.for_each_triple(|t| {
            let rep = me.encode_triple(&t);
            //eprintln!("{:?}", rep);
            me.dictionary.ts.add_triple(rep);
        })
        .expect("Streaming error");
        me.dictionary.ts.sort();
        me
    }
}

// Should return -1 if both s and o are res,
// 1 if s is prop and o is res,
// and 3 if both s and o are prop
fn contains_prop_in_s_or_o<TD>(
    property_index: u32,
    object: &Term<TD>,
    dictionary: &NodeDictionary,
) -> i8
where
    TD: std::convert::AsRef<str> + std::clone::Clone + std::cmp::Eq + std::hash::Hash,
{
    // Special case: if p a ...Property -> return 3
    if property_index == dictionary.rdftype {
        let o_str = object.value();
        if o_str.to_lowercase().ends_with("property") {
            return 3;
        }
    }
    let prop_in_s = vec![dictionary.rdfsdomain, dictionary.rdfsrange];
    let prop_in_s_and_o = vec![
        dictionary.owlequivalentProperty,
        dictionary.owlinverseOf,
        dictionary.rdfssubPropertyOf,
    ];
    if prop_in_s_and_o.contains(&property_index) {
        3
    } else if prop_in_s.contains(&property_index) {
        1
    } else {
        -1
    }
}
