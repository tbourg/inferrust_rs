use crate::inferray::{InfGraph, NodeDictionary, TripleStore};

fn apply_zeta_rule(
    graph: &InfGraph,
    input_o: u64,
    output_p: u64,
    output_o: u64,
    object_is_subject: bool,
) -> TripleStore {
    let mut output = TripleStore::new();
    let pairs1 =
        &graph.dictionary.ts.elem[NodeDictionary::prop_idx_to_idx(graph.dictionary.rdftype as u64)];
    if pairs1[1].is_empty() {
        return output;
    }
    for pair1 in &pairs1[1] {
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
