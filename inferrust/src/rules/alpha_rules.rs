//! Class alpha groups the following rules :
//! <ul>
//! <li>CAX-SCO</li>
//! <li>SCM-DOM1</li>
//! <li>SCM-DOM2</li>
//! <li>SCM-RNG1</li>
//! <li>SCM-RNG2</li>
//! </ul>
//!
//! All these rules have the following properties :
//! <ol>
//! <li>2 fixed predicates in the head triples</li>
//! <li>Equality between first subject second object or first object second
//! subject</li>
//! <li>Inferred triple contains only s,p,o from the head</li>
//! </ol>

use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;

use std::cmp::Ordering;

// :human rdfs:subclassof :mammal ||| :bart :type :human
//  0           1            2           3    4      5
//                        -->
//          3             4              2
//        :bart         :type         :mammal

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn apply_alpha_rule(
    ts: &TripleStore,
    id_1: u64,
    id_2: u64,
    id_s: usize,
    id_p: usize,
    id_o: usize,
) -> TripleStore {
    let property_1_pairs = ts.elem().get(id_1 as usize);
    let property_2_pairs = ts.elem().get(id_2 as usize);
    if property_1_pairs == None || property_2_pairs == None {
        return TripleStore::default();
    }
    let property_1_pairs = property_1_pairs.unwrap();
    let property_2_pairs = property_2_pairs.unwrap();
    let mut output = TripleStore::default();
    let mut values: [u64; 6] = [0; 6];
    values[1] = id_1;
    values[4] = id_2;
    for property_1_pair in property_1_pairs.so() {
        values[0] = property_1_pair[0];
        values[2] = property_1_pair[1];
        for property_2_pair in property_2_pairs.os() {
            values[3] = property_2_pair[1];
            values[5] = property_2_pair[0];
            match values[5].cmp(&values[0]) {
                Ordering::Equal => output.add_triple([
                    values[id_s],
                    NodeDictionary::idx_to_prop_idx(values[id_p] as usize),
                    values[id_o],
                ]),
                Ordering::Greater => break,
                Ordering::Less => (),
            }
        }
    }
    output
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn CAX_SCO(ts: &TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubClassOf as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdftype as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 4, 2)
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn CAX_EQC1(ts: &TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::owlequivalentClass as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdftype as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 4, 2)
}

/// CAX-EQC2 is implied cause a = b -> b = a

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn SCM_DOM1(ts: &TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubClassOf as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfsdomain as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 4, 2)
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn SCM_DOM2(ts: &TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfsdomain as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubPropertyOf as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 1, 2)
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn SCM_RNG1(ts: &TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubClassOf as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfsrange as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 4, 2)
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn SCM_RNG2(ts: &TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfsrange as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubPropertyOf as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 1, 2)
}
