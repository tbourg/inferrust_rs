use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;

#[cfg_attr(debug_assertions, flamer::flame)]
fn apply_delta_rule(ts: &TripleStore, prop_idx: usize, invert: bool) -> TripleStore {
    let mut output = TripleStore::new();
    if let Some(pairs) = ts.elem.get(prop_idx) {
        for pair in pairs.so() {
            if pair[0] != pair[1] {
                let prop_idx = NodeDictionary::prop_idx_to_idx(pair[0]);
                if let Some(usable_pairs) = ts.elem.get(prop_idx) {
                    let usable_pairs = if invert {
                        usable_pairs.os()
                    } else {
                        usable_pairs.so()
                    };
                    for usable_pair in usable_pairs {
                        output.add_triple([usable_pair[0], pair[1], usable_pair[1]]);
                    }
                }
                let prop_idx = NodeDictionary::prop_idx_to_idx(pair[1]);
                if let Some(usable_pairs) = ts.elem.get(prop_idx) {
                    let usable_pairs = if invert {
                        usable_pairs.os()
                    } else {
                        usable_pairs.so()
                    };
                    for usable_pair in usable_pairs {
                        output.add_triple([usable_pair[0], pair[0], usable_pair[1]]);
                    }
                }
            }
        }
    }
    output
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn PRP_INV_1_2(ts: &TripleStore) -> TripleStore {
    apply_delta_rule(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::owlinverseOf as u64),
        true,
    )
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn PRP_EQP_1_2(ts: &TripleStore) -> TripleStore {
    apply_delta_rule(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::owlequivalentProperty as u64),
        false,
    )
}
