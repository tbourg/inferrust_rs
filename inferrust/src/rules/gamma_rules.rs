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
    let pairs1 = graph.dictionary.ts.elem.get(head_prop);
    if pairs1 == None {
        return output;
    }
    let pairs1 = &pairs1.unwrap()[0];
    for pair1 in pairs1 {
        let pairs2 = graph
            .dictionary
            .ts
            .elem
            .get(NodeDictionary::prop_idx_to_idx(pair1[0]));
        if pairs2 == None {
            break;
        }
        let pairs2 = &pairs2.unwrap()[0];
        for pair2 in pairs2 {
            if raw_idx {
                output.add_triple([pair2[if subject { 0 } else { 1 }], output_prop, pair1[1]]);
            } else {
                output.add_triple([pair2[0], pair1[1], pair2[1]]);
            }
        }
    }
    output
}

pub fn PRP_DOM(graph: &InfGraph) -> TripleStore {
    apply_gamma_rule(
        graph,
        NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfsdomain as u64),
        graph.dictionary.rdftype as u64,
        true,
        true,
    )
}

pub fn PRP_RNG(graph: &InfGraph) -> TripleStore {
    apply_gamma_rule(
        graph,
        NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfsrange as u64),
        graph.dictionary.rdftype as u64,
        false,
        true,
    )
}

pub fn PRP_SPO1(graph: &InfGraph) -> TripleStore {
    apply_gamma_rule(
        graph,
        NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfssubPropertyOf as u64),
        0,
        false,
        false,
    )
}

pub fn PRP_SYMP(graph: &InfGraph) -> TripleStore {
    let mut output = TripleStore::new();
    let expected_ip = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdftype as u64);
    let expected_io = graph.dictionary.owlsymmetricProperty as u64;
    let pairs1 = graph.dictionary.ts.elem.get(expected_ip);
    if pairs1 == None {
        return output;
    }
    let pairs1 = &pairs1.unwrap()[1]; // os sorted copy
    for pair1 in pairs1 {
        if pair1[0] == expected_io {
            let pairs2 = graph
                .dictionary
                .ts
                .elem
                .get(NodeDictionary::prop_idx_to_idx(pair1[1]));
            if pairs2 == None {
                break;
            }
            let pairs2 = &pairs2.unwrap()[0];
            for pair2 in pairs2 {
                output.add_triple([pair2[1], pair1[1], pair2[0]]);
            }
        }
        if pair1[0] > expected_io {
            break;
        }
    }
    output
}

pub fn EQ_TRANS(graph: &InfGraph) -> TripleStore {
    let pairs = graph
        .dictionary
        .ts
        .elem
        .get(NodeDictionary::prop_idx_to_idx(
            graph.dictionary.owlsameAs as u64,
        ));
    if pairs == None {
        return TripleStore::new();
    }
    let pairs1 = pairs.unwrap();
    let pairs2 = pairs.unwrap();
    let mut output = TripleStore::new();
    for pair1 in &pairs1[0] {
        for pair2 in &pairs2[0] {
            if pair1[1] == pair2[0] {
                if pair1[0] != pair2[1] {
                    output.add_triple([pair1[0], graph.dictionary.owlsameAs as u64, pair2[1]]);
                    output.add_triple([pair2[1], graph.dictionary.owlsameAs as u64, pair1[0]]);
                }
            }
            if pair2[0] > pair1[1] {
                break;
            }
        }
    }
    output
}
