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

use crate::inferray::InfGraph;
use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;
use crate::rules::Rule;

use sophia::ns::*;
use sophia::term::StaticTerm;

// :human rdfs:subclassof :mammal ||| :bart :type :human
//  0           1            2           3    4      5
//                        -->
//          3             4              2
//        :bart         :type         :mammal

pub fn apply_alpha_rule(
    graph: &InfGraph,
    id_1: u64,
    id_2: u64,
    id_s: u64,
    id_p: u64,
    id_o: u64,
    eq_1: u64,
    eq_2: u64,
) -> TripleStore {
    let property_1_pairs = graph.dictionary.ts.elem.get(id_1 as usize);
    let property_2_pairs = graph.dictionary.ts.elem.get(id_2 as usize);
    if property_1_pairs == None || property_2_pairs == None {
        return TripleStore::new();
    }
    let property_1_pairs = property_1_pairs.unwrap();
    let property_2_pairs = property_2_pairs.unwrap();
    let mut output = TripleStore::new();
    for property_1_pair in &property_1_pairs[0] {
        for property_2_pair in &property_2_pairs[0] {
            let index = |i| match i {
                0 => property_1_pair[0],
                1 => id_1,
                2 => property_1_pair[1],
                3 => property_2_pair[0],
                4 => id_2,
                5 => property_2_pair[1],
                _ => 0,
            };
            if index(eq_1) == index(eq_2) {
                output.add_triple([
                    index(id_s),
                    NodeDictionary::idx_to_prop_idx(index(id_p) as usize),
                    index(id_o),
                ]);
            }
            // dbg!(
            //     property_1_pair,
            //     property_2_pair,
            //     &output.elem[index(id_p) as usize]
            // );
        }
    }
    output
}

pub fn CAX_SCO(graph: &mut InfGraph) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfssubClassOf as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdftype as u64) as u64;
    apply_alpha_rule(graph, id_1, id_2, 3, 4, 2, 0, 5)
}

pub fn CAX_EQC1(graph: &mut InfGraph) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(graph.dictionary.owlequivalentClass as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdftype as u64) as u64;
    apply_alpha_rule(graph, id_1, id_2, 3, 4, 2, 0, 5)
}

pub fn CAX_EQC2(graph: &mut InfGraph) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(graph.dictionary.owlequivalentClass as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdftype as u64) as u64;
    apply_alpha_rule(graph, id_1, id_2, 3, 4, 0, 2, 5)
}

pub fn SCM_DOM1(graph: &mut InfGraph) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfsdomain as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfssubClassOf as u64) as u64;
    apply_alpha_rule(graph, id_1, id_2, 0, 1, 5, 2, 3)
}

pub fn SCM_DOM2(graph: &mut InfGraph) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfsdomain as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfssubPropertyOf as u64) as u64;
    apply_alpha_rule(graph, id_1, id_2, 3, 1, 2, 0, 5)
}

pub fn SCM_RNG1(graph: &mut InfGraph) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfsrange as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfssubClassOf as u64) as u64;
    apply_alpha_rule(graph, id_1, id_2, 0, 1, 5, 2, 3)
}

pub fn SCM_RNG2(graph: &mut InfGraph) -> TripleStore {
    let id_1 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfsrange as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfssubPropertyOf as u64) as u64;
    apply_alpha_rule(graph, id_1, id_2, 3, 1, 2, 0, 5)
}
