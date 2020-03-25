use crate::inferray::{InfGraph, NodeDictionary, TripleStore};

fn apply_zeta_rule(
    graph: &InfGraph,
    input_o: u64,
    output_p: u64,
    output_o: u64,
    object_is_subject: bool,
) -> TripleStore {
    let mut output = TripleStore::new();
    let pairs1 = graph
        .dictionary
        .ts
        .elem
        .get(NodeDictionary::prop_idx_to_idx(
            graph.dictionary.rdftype as u64,
        ));
    if pairs1 == None {
        return output;
    }
    let pairs1 = &pairs1.unwrap()[1];
    if pairs1.is_empty() {
        return output;
    }
    for pair1 in pairs1 {
        if pair1[0] > input_o {
            break;
        }
        if pair1[0] == input_o {
            if !object_is_subject {
                output.add_triple([pair1[1], output_p, output_o]);
            } else {
                output.add_triple([pair1[1], output_p, pair1[1]]);
            }
        }
    }
    output
}

pub fn RDFS6(graph: &mut InfGraph) -> TripleStore {
    let input_o = graph.dictionary.rdfProperty as u64;
    let output_p = graph.dictionary.rdfssubPropertyOf as u64;
    apply_zeta_rule(graph, input_o, output_p, 0, true)
}

pub fn RDFS8(graph: &mut InfGraph) -> TripleStore {
    let input_o = graph.dictionary.rdfsClass;
    let output_p = graph.dictionary.rdftype as u64;
    let output_o = graph.dictionary.rdfsResource;
    apply_zeta_rule(graph, input_o, output_p, output_o, false)
}

pub fn RDFS10(graph: &mut InfGraph) -> TripleStore {
    let input_o = graph.dictionary.rdfsClass;
    let output_p = graph.dictionary.rdfssubClassOf as u64;
    apply_zeta_rule(graph, input_o, output_p, 0, true)
}

pub fn RDFS12(graph: &mut InfGraph) -> TripleStore {
    let input_o = graph.dictionary.rdfsContainerMembershipProperty as u64;
    let output_p = graph.dictionary.rdfssubPropertyOf as u64;
    let output_o = graph.dictionary.rdfsMember as u64;
    apply_zeta_rule(graph, input_o, output_p, output_o, false)
}

pub fn RDFS13(graph: &mut InfGraph) -> TripleStore {
    let input_o = graph.dictionary.rdfsDatatype;
    let output_p = graph.dictionary.rdfssubClassOf as u64;
    let output_o = graph.dictionary.rdfsLiteral;
    apply_zeta_rule(graph, input_o, output_p, output_o, false)
}

pub fn SCM_DP_OP(graph: &mut InfGraph) -> TripleStore {
    let mut output = TripleStore::new();
    for object in [
        graph.dictionary.owldataTypeProperty as u64,
        graph.dictionary.owlobjectProperty as u64,
    ]
    .iter()
    {
        let pairs1 = graph
            .dictionary
            .ts
            .elem
            .get(NodeDictionary::prop_idx_to_idx(
                graph.dictionary.rdftype as u64,
            ));
        if pairs1 == None {
            break;
        }
        let pairs1 = &pairs1.unwrap()[1];
        if pairs1.is_empty() {
            break;
        }
        for pair1 in pairs1 {
            if pair1[0] > *object {
                break;
            }
            if pair1[0] == *object {
                output.add_triple([
                    pair1[1],
                    graph.dictionary.rdfssubPropertyOf as u64,
                    pair1[1],
                ]);
                output.add_triple([
                    pair1[1],
                    graph.dictionary.owlequivalentProperty as u64,
                    pair1[1],
                ]);
            }
        }
    }
    output
}

pub fn SCM_CLS(graph: &mut InfGraph) -> TripleStore {
    let mut output = TripleStore::new();
    let pairs1 = graph
        .dictionary
        .ts
        .elem
        .get(NodeDictionary::prop_idx_to_idx(
            graph.dictionary.rdftype as u64,
        ));
    if pairs1 == None {
        return output;
    }
    let pairs1 = &pairs1.unwrap()[1];
    if pairs1.is_empty() {
        return output;
    }
    let object = graph.dictionary.owlclass;
    for pair1 in pairs1 {
        if pair1[0] > object {
            break;
        }
        if pair1[0] == object {
            output.add_triple([pair1[1], graph.dictionary.rdfssubClassOf as u64, pair1[1]]);
            output.add_triple([
                pair1[1],
                graph.dictionary.owlequivalentClass as u64,
                pair1[1],
            ]);
            output.add_triple([
                pair1[1],
                graph.dictionary.rdfssubClassOf as u64,
                graph.dictionary.owlthing as u64,
            ]);
            output.add_triple([
                graph.dictionary.nothing as u64,
                graph.dictionary.rdfssubClassOf as u64,
                pair1[1],
            ]);
        }
    }
    output
}

pub fn RDFS4(graph: &mut InfGraph) -> TripleStore {
    let mut output = TripleStore::new();
    let mut resources_idx = Vec::new();
    let pairs1 = graph
        .dictionary
        .ts
        .elem
        .get(NodeDictionary::prop_idx_to_idx(
            graph.dictionary.rdftype as u64,
        ));
    if pairs1 == None {
        return output;
    }
    let pairs1 = &pairs1.unwrap()[1];
    if pairs1.is_empty() {
        return output;
    }
    let object = graph.dictionary.rdfsResource;
    for pair1 in pairs1 {
        if pair1[0] > object {
            break;
        }
        if pair1[0] == object {
            resources_idx.push(pair1[1])
        }
    }
    for pairs2 in &graph.dictionary.ts.elem {
        for pair2 in &pairs2[0] {
            if resources_idx.contains(&pair2[1]) {
                output.add_triple([pair2[0], graph.dictionary.rdftype as u64, object]);
            }
        }
    }
    output
}
