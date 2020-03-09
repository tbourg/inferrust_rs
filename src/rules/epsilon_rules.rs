/**
 * The Epsilon class covers :
 * <ul>
 * <li>PRP-INV1/2</li>
 * <li>PRP-EQP1/2</li>
 * <li></li>
 * </ul>
 *
 * These rules have the following template
 * <pre>
 * p1 property p2
 *  x p1 y
 * -----------
 * ? p1/p2 ?
 *
 *  Field with ?
 *  has indices :
 *  x p1 y
 *  0    1
 * </pre>
 * @author Julien Subercaze
 *
 *         Feb. 14
 */
use crate::inferray::InfGraph;
use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;
use crate::rules::Rule;

fn apply_epsilon_rule(graph: &InfGraph, prop_idx: usize, invert: bool) -> TripleStore {
    let mut output = TripleStore::new();
    if let Some(pairs) = graph.dictionary.ts.elem.get(prop_idx) {
        for pair in &pairs[0] {
            if pair[0] != pair[1] {
                // dbg!(pair);
                // dbg!(graph.dictionary.get_term(pair[0]));
                // dbg!(graph.dictionary.get_term(pair[1]));
                let prop_idx = NodeDictionary::prop_idx_to_idx(pair[0]);
                if let Some(usable_pairs) = graph.dictionary.ts.elem.get(prop_idx) {
                    for usable_pair in &usable_pairs[if invert { 1 } else { 0 }] {
                        // dbg!(usable_pair);
                        // dbg!(graph.dictionary.get_term(usable_pair[0]));
                        // dbg!(graph.dictionary.get_term(usable_pair[1]));
                        output.add_triple([usable_pair[0], pair[1], usable_pair[1]]);
                    }
                }
                let prop_idx = NodeDictionary::prop_idx_to_idx(pair[1]);
                if let Some(usable_pairs) = graph.dictionary.ts.elem.get(prop_idx) {
                    for usable_pair in &usable_pairs[if invert { 1 } else { 0 }] {
                        // dbg!(usable_pair);
                        // dbg!(graph.dictionary.get_term(usable_pair[0]));
                        // dbg!(graph.dictionary.get_term(usable_pair[1]));
                        output.add_triple([usable_pair[0], pair[0], usable_pair[1]]);
                    }
                }
            }
        }
    }
    output
}

pub fn PRP_INV_1_2(graph: &mut InfGraph) -> TripleStore {
    apply_epsilon_rule(
        graph,
        NodeDictionary::prop_idx_to_idx(graph.dictionary.owlinverseOf as i64),
        true,
    )
}

pub fn PRP_EQP_1_2(graph: &mut InfGraph) -> TripleStore {
    apply_epsilon_rule(
        graph,
        NodeDictionary::prop_idx_to_idx(graph.dictionary.owlequivalentProperty as i64),
        false,
    )
}
