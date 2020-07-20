use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;
use crate::rules::*;
use crate::utils::*;

use std::cmp::Ordering;

pub fn CLS_INT1(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let inter = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::intersectionOf as u64,
    ));
    if inter == None {
        return output;
    }
    let inter = inter.unwrap().so();
    let rdftype = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdftype as u64,
    ));
    if rdftype == None {
        return output;
    }
    let rdftype = rdftype.unwrap().so();
    dbg!(inter, rdftype);
    for [c, l] in inter {
        let list = ts.list(*l).unwrap();
        for [y, candidate] in rdftype {
            if list.elems.contains(candidate) {
                dbg!(c, y, candidate);
                let mut ok = true;
                for elem in list.elems.iter() {
                    dbg!(y, elem);
                    if !(dbg!(binary_search_pair(rdftype, [*y, *elem]))) {
                        ok = false;
                    }
                }
                if ok {
                    output.push([*y, NodeDictionary::rdftype as u64, *c]);
                }
            }
        }
    }
    output
}
