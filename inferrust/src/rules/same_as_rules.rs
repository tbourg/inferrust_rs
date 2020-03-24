use crate::inferray::{InfGraph, NodeDictionary, TripleStore};

// /**
//  * Same-as special Rule
//  *
//  * Encompasses :
//  * <ul>
//  * <li>eq-rep-o</li>
//  * <li>eq-rep-p</li>
//  * <li>eq-rep-s</li>
//  * <li>eq-sym</li>
//  * </ul>
//  *
//  * Since same-as will be added for s-o symetrically, eq-rep-o is implide by
//  * eq-rep-s
//  *
//  * @author Julien Subercaze
//  *
//  *         Dec. 13
//  */
fn apply_same_as_rule(graph: &InfGraph) -> TripleStore {
    let mut output = TripleStore::new();
    let pairs1 = graph
        .dictionary
        .ts
        .elem
        .get(NodeDictionary::prop_idx_to_idx(
            graph.dictionary.owlsameAs as u64,
        ));
    if pairs1 == None {
        return output;
    }
    let pairs1 = &pairs1.unwrap()[0];
    if pairs1.is_empty() {
        output
    } else {
        for pair1 in pairs1 {
            if pair1[0] < NodeDictionary::START_INDEX as u64 {
                if let Some(pairs2) = graph
                    .dictionary
                    .ts
                    .elem
                    .get(NodeDictionary::prop_idx_to_idx(pair1[0]))
                {
                    for pair2 in &pairs2[0] {
                        output.add_triple([pair2[0], pair1[1], pair2[1]]);
                    }
                }
            } else {
                output.add_triple([pair1[1], graph.dictionary.owlsameAs as u64, pair1[0]]);
                graph
                    .dictionary
                    .ts
                    .elem
                    .iter()
                    .enumerate()
                    .map(|(idx, chunk)| {
                        if !chunk[0].is_empty() {
                            if chunk[0][0][0] <= pair1[0]
                                && chunk[0][chunk[0].len() - 1][0] >= pair1[0]
                            {
                                let pi = NodeDictionary::idx_to_prop_idx(idx);
                                chunk[0]
                                    .iter()
                                    .take_while(|pair| pair[0] <= pair1[0])
                                    .filter(|pair| pair[0] == pair1[0])
                                    .map(|pair| {
                                        output.add_triple([pair1[1], pi, pair[1]]);
                                    })
                                    .for_each(|_| ());
                            }
                        }
                        if !chunk[1].is_empty() {
                            if chunk[1][0][0] <= pair1[0]
                                && chunk[1][chunk[0].len() - 1][0] >= pair1[0]
                            {
                                let pi = NodeDictionary::idx_to_prop_idx(idx);
                                chunk[1]
                                    .iter()
                                    .take_while(|pair| pair[0] <= pair1[0])
                                    .filter(|pair| pair[0] == pair1[0])
                                    .map(|pair| {
                                        output.add_triple([pair[1], pi, pair1[1]]);
                                    })
                                    .for_each(|_| ());
                            }
                        }
                    })
                    .for_each(|_| ());
            }
        }
        output
    }
}

pub fn SAME_AS(graph: &mut InfGraph) -> TripleStore {
    apply_same_as_rule(graph)
}
