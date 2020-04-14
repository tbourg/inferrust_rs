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
use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;

fn apply_delta_rule(ts: &mut TripleStore, prop_idx: usize, invert: bool) -> TripleStore {
    let mut output = TripleStore::new();
    if let Some(pairs) = ts.elem.get(prop_idx) {
        for pair in &pairs[0] {
            if pair[0] != pair[1] {
                // dbg!(pair);
                // dbg!(.get_term(pair[0]));
                // dbg!(.get_term(pair[1]));
                let prop_idx = NodeDictionary::prop_idx_to_idx(pair[0]);
                if let Some(usable_pairs) = ts.elem.get(prop_idx) {
                    for usable_pair in &usable_pairs[if invert { 1 } else { 0 }] {
                        // dbg!(usable_pair);
                        // dbg!(.get_term(usable_pair[0]));
                        // dbg!(.get_term(usable_pair[1]));
                        output.add_triple([usable_pair[0], pair[1], usable_pair[1]]);
                    }
                }
                let prop_idx = NodeDictionary::prop_idx_to_idx(pair[1]);
                if let Some(usable_pairs) = ts.elem.get(prop_idx) {
                    for usable_pair in &usable_pairs[if invert { 1 } else { 0 }] {
                        // dbg!(usable_pair);
                        // dbg!(.get_term(usable_pair[0]));
                        // dbg!(.get_term(usable_pair[1]));
                        output.add_triple([usable_pair[0], pair[0], usable_pair[1]]);
                    }
                }
            }
        }
    }
    output
}

pub fn PRP_INV_1_2(ts: &mut TripleStore) -> TripleStore {
    apply_delta_rule(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::owlinverseOf as u64),
        true,
    )
}

pub fn PRP_EQP_1_2(ts: &mut TripleStore) -> TripleStore {
    apply_delta_rule(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::owlequivalentProperty as u64),
        false,
    )
}
