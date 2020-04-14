//! Class beta groups the following rules :
//! <ul>
//! <li>SCM-SCO</li>
//! <li>SCM-EQC2</li>
//! <li>SCM-SPO</li>
//! <li>SCM-EQP2</li>
//! </ul>
//!
//! All these rules have the following properties :
//! <ol>
//! <li>same predicate in both parts</li>
//! </ol>

use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;

fn apply_beta_rule(ts: &mut TripleStore, rule_p: usize, infer_p: usize) -> TripleStore {
    let pairs = ts.elem.get(rule_p);
    if pairs == None {
        return TripleStore::new();
    }
    let rule_p = NodeDictionary::idx_to_prop_idx(rule_p);
    let infer_p = NodeDictionary::idx_to_prop_idx(infer_p);
    let pairs1 = pairs.unwrap();
    let pairs2 = pairs.unwrap();
    let mut output = TripleStore::new();
    for pair1 in pairs1.so() {
        for pair2 in pairs2.so() {
            if pair1[1] == pair2[0] {
                if pair1[0] == pair2[1] {
                    output.add_triple([pair1[0], infer_p, pair1[1]]);
                    output.add_triple([pair2[0], infer_p, pair2[1]]);
                } else {
                    output.add_triple([pair1[0], rule_p, pair2[1]]);
                }
            }
            if pair2[0] > pair1[1] {
                break;
            }
        }
    }
    output
}

fn apply_inverse_beta_rule(ts: &mut TripleStore, rule_p: usize, infer_p: usize) -> TripleStore {
    let pairs = ts.elem.get(rule_p);
    if pairs == None {
        return TripleStore::new();
    }
    let rule_p = NodeDictionary::idx_to_prop_idx(rule_p);
    let infer_p = NodeDictionary::idx_to_prop_idx(infer_p);
    let pairs1 = pairs.unwrap();
    let mut output = TripleStore::new();
    for pair1 in pairs1.so() {
        output.add_triple([pair1[0], infer_p, pair1[1]]);
        output.add_triple([pair1[1], infer_p, pair1[0]]);
        output.add_triple([pair1[0], rule_p, pair1[1]]);
    }
    output
}

/// The SCM-EQC2 rule from the RDFS+ ruleset
///
/// Body:
/// - c1 rdfs:subClassOf c2
/// - c2 rdfs:subClassOf c1
///
/// Head:
/// - c1 owl:equivalentClass c2
/// - c2 owl:equivalentClass c1
pub fn SCM_SCO_EQC2(ts: &mut TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubClassOf as u64);
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::owlequivalentClass as u64);
    apply_beta_rule(ts, id_1, id_2)
}

pub fn SCM_SPO_EQP2(ts: &mut TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubPropertyOf as u64);
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::owlequivalentProperty as u64);
    apply_beta_rule(ts, id_1, id_2)
}

pub fn SCM_EQC1(ts: &mut TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::owlequivalentClass as u64);
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubClassOf as u64);
    apply_inverse_beta_rule(ts, id_1, id_2)
}

pub fn SCM_EQP1(ts: &mut TripleStore) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(NodeDictionary::owlequivalentProperty as u64);
    let id_2 = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubPropertyOf as u64);
    apply_inverse_beta_rule(ts, id_1, id_2)
}
