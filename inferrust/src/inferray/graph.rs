use sophia::graph::GTripleSource;
use sophia::graph::Graph;
use sophia::term::{Term, TermData};
use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia::triple::{stream::TripleSource, Triple};

use std::convert::Infallible;

use crate::closure::*;
use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;

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
                        let start_index = lower_bound(&chunk[0], si, 0, chunk[0].len());
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
            Box::from(Vec::new().into_iter())
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
                Box::from(Vec::new().into_iter())
            }
        } else {
            Box::from(Vec::new().into_iter())
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
                        let start_index = lower_bound(&chunk[1], oi, 0, chunk[1].len());
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
            Box::from(Vec::new().into_iter())
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
                let start_index = lower_bound(&chunk[0], si, 0, chunk[0].len());
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
                Box::from(Vec::new().into_iter())
            }
        } else {
            Box::from(Vec::new().into_iter())
        }
    }

    // fn triples_with_so<'s, T, U>(
    //     &'s self,
    //     s: &'s Term<T>,
    //     o: &'s Term<U>,
    // ) -> GTripleSource<'s, Self>
    // where
    //     T: TermData,
    //     U: TermData,
    // {
    //     if let (Some(si), Some(oi)) = (self.dictionary.get_index(s), self.dictionary.get_index(o)) {
    //         let idx = NodeDictionary::prop_idx_to_idx(pi);
    //         let chunk = &self.dictionary.ts.elem[idx];
    //         if !chunk[0].is_empty() {
    //             let s = self.dictionary.get_term(si);
    //             let p = self.dictionary.get_term(pi);
    //             let start_index = lower_bound(&chunk[0], si, 0, chunk[0].len());
    //             Box::from(
    //                 chunk[0][start_index..]
    //                     .iter()
    //                     .take_while(|[is, _]| *is == si)
    //                     .map(move |[_, oi]| {
    //                         Ok(StreamedTriple::by_term_refs(
    //                             s,
    //                             p,
    //                             self.dictionary.get_term(*oi),
    //                         ))
    //                     }),
    //             )
    //         } else {
    //             Box::from(Vec::new().into_iter())
    //         }
    //     } else {
    //         Box::from(Vec::new().into_iter())
    //     }
    // }

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
                let start_index = lower_bound(&chunk[1], oi, 0, chunk[1].len());
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
                Box::from(Vec::new().into_iter())
            }
        } else {
            Box::from(Vec::new().into_iter())
        }
    }

    //     fn triples_with_spo<'s, T, U, V>(
    //         &'s self,
    //         s: &'s Term<T>,
    //         p: &'s Term<U>,
    //         o: &'s Term<V>,
    //     ) -> GTripleSource<'s, Self>
    //     where
    //         T: TermData,
    //         U: TermData,
    //         V: TermData,
    //     {
    //         Box::new(self.triples_with_sp(s, p).filter_ok(move |t| t.o() == o))
    //     }
}

/// Pre-condition: vec is an array of pairs sorted on the first elem of each pair
fn _binary_search_seconds_by_first(_vec: Vec<[u64; 2]>, _first: u64) -> Vec<u64> {
    let seconds = Vec::new();
    // vec.binary_search_by_key(&first, mut f: F)
    seconds
}

/// src: https://stackoverflow.com/a/25966181
fn lower_bound(vec: &Vec<[u64; 2]>, key: u64, low: usize, high: usize) -> usize {
    if low > high {
        return low;
    }

    let mid = low + ((high - low) >> 1);
    if mid == 0 {
        return 0;
    }
    if mid == high {
        return high;
    }
    //Attention here, we go left for lower_bound when meeting equal values
    if vec[mid][0] >= key {
        return lower_bound(vec, key, low, mid - 1);
    } else {
        return lower_bound(vec, key, mid + 1, high);
    }
}

fn _upper_bound(vec: &Vec<[u64; 2]>, key: u64, low: usize, high: usize) -> usize {
    if low > high {
        return low;
    }

    let mid = low + ((high - low) >> 1);
    if mid == 0 {
        return 0;
    }
    if mid == high {
        return high;
    }
    //Attention here, we go right for upper_bound when meeting equal values
    if vec[mid][0] > key {
        return _upper_bound(vec, key, low, mid - 1);
    } else {
        return _upper_bound(vec, key, mid + 1, high);
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
        let prop_in_s_or_o = contains_prop_in_s_or_o(p, &self.dictionary);
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

    pub fn close(&mut self) {
        // eprintln!("SubClassOf");
        self.close_on(self.dictionary.rdfssubClassOf);
        // eprintln!("SubPropertyOf");
        self.close_on(self.dictionary.rdfssubPropertyOf);
        // eprintln!("SameAs");
        self.close_on(self.dictionary.owlsameAs);
        for tr_idx in self.get_tr_idx().iter() {
            self.close_on(*tr_idx);
        }
    }

    fn close_on(&mut self, index: u32) {
        let ip_to_store = NodeDictionary::prop_idx_to_idx(index as u64);
        self.close_on_raw(ip_to_store);
    }

    fn close_on_raw(&mut self, raw_index: usize) {
        // dbg!(&self.dictionary.ts.elem);
        let pairs = self.dictionary.ts.elem[raw_index][0].clone();
        let mut tc_g = ClosureGraph::from(pairs);
        // eprintln!("fermeture transitive");
        let closure = tc_g.close();
        for (s, os) in closure.iter() {
            for o in os.iter() {
                self.dictionary.ts.add_triple_raw(*s, raw_index, *o);
            }
        }
        self.dictionary.ts.sort();
        // dbg!(&self.dictionary.ts.elem[raw_index][0]);
    }

    fn get_tr_idx(&mut self) -> Vec<u32> {
        self.dictionary.ts.elem[NodeDictionary::prop_idx_to_idx(self.dictionary.rdftype as u64)][0]
            .iter()
            .filter(|pair| pair[1] == self.dictionary.owltransitiveProperty as u64)
            .map(|pair| pair[0] as u32)
            .collect()
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
fn contains_prop_in_s_or_o(property_index: u32, dictionary: &NodeDictionary) -> i32 {
    let prop_in_s = vec![dictionary.rdfsdomain, dictionary.rdfsrange];
    let prop_in_s_and_o = vec![
        dictionary.owlequivalentProperty,
        dictionary.owlinverseOf,
        dictionary.rdfssubPropertyOf,
        dictionary.owlsymetricProperty,
    ];
    if prop_in_s_and_o.contains(&property_index) {
        3
    } else if prop_in_s.contains(&property_index) {
        1
    } else {
        -1
    }
}
