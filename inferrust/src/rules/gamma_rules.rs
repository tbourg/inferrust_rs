use crate::inferray::{InfGraph, NodeDictionary, TripleStore};

/**
 * Gamma rules are of the following form :
 *
 * <pre>
 * p property c
 * x p y
 * ------------
 * ? ? c
 * </pre>
 *
 * Rules :
 * <ul>
 * <li>PRP-DOM</li>
 * <li>PRP-RNG</li>
 * <li>PRP-SPO1</li>
 * </ul>
 *
 * @author Julien Subercaze
 *
 *         Dec. 13
 */

fn apply_gamma_rule(
    graph: &InfGraph,
    head_prop: usize,
    output_prop: u64,
    subject: bool,
    raw_idx: bool,
) -> TripleStore {
    let mut output = TripleStore::new();
    let pairs1 = &graph.dictionary.ts.elem[head_prop];
    for pair1 in &pairs1[0] {
        let pairs2 = &graph.dictionary.ts.elem[NodeDictionary::prop_idx_to_idx(pair1[0])];
        for pair2 in &pairs2[0] {
            if raw_idx {
                output.add_triple([pair2[if subject { 0 } else { 1 }], output_prop, pair1[1]]);
            } else {
                output.add_triple([pair2[0], pair1[1], pair2[1]]);
            }
        }
    }
    output
}

pub fn PRP_DOM(graph: &mut InfGraph) -> TripleStore {
    apply_gamma_rule(
        graph,
        NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfsdomain as u64),
        graph.dictionary.rdftype as u64,
        true,
        true,
    )
}

pub fn PRP_RNG(graph: &mut InfGraph) -> TripleStore {
    apply_gamma_rule(
        graph,
        NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfsrange as u64),
        graph.dictionary.rdftype as u64,
        false,
        true,
    )
}

pub fn PRP_SPO1(graph: &mut InfGraph) -> TripleStore {
    apply_gamma_rule(
        graph,
        NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfssubPropertyOf as u64),
        0,
        false,
        false,
    )
}
