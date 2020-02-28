use crate::inferray::dictionary::NodeDictionary;
use crate::inferray::graph::InfGraph;
use crate::inferray::store::TripleStore;
use crate::rules::Rule;

use sophia::ns::*;
use sophia::term::StaticTerm;

// :human rdfs:subclassof :mammal ||| :bart :type :human
//  0           1            2           3    4      5
//                        -->
//          3             4              2
//        :bart         :type         :mammal

/**
 *
 * Class alpha groups the following rules :
 * <ul>
 * <li>CAX-SCO</li>
 * <li>SCM-DOM1</li>
 * <li>SCM-DOM2</li>
 * <li>SCM-RNG1</li>
 * <li>SCM-RNG2</li>
 * </ul>
 *
 * All these rules have the following properties :
 * <ol>
 * <li>2 fixed predicates in the head triples</li>
 * <li>Equality between first subject second object or first object second
 * subject</li>
 * <li>Inferred triple contains only s,p,o from the head</li>
 * </ol>
 */
pub fn apply_alpha_rule(
    graph: &InfGraph,
    id_1: i64,
    id_2: i64,
    id_s: i64,
    id_p: i64,
    id_o: i64,
) -> TripleStore {
    let property_1_pairs = &graph.dictionary.ts.elem[id_1 as usize];
    let property_2_pairs = &graph.dictionary.ts.elem[id_2 as usize];
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
            if index(5) == index(0) {
                output.add_triple([
                    index(id_s),
                    NodeDictionary::prop_idx_to_idx(index(id_p)) as i64,
                    index(id_o),
                ]);
            }
        }
    }
    output
}

pub struct CAX_SCO;

impl Rule for CAX_SCO {
    fn fire(&mut self, graph: &mut InfGraph) -> TripleStore {
        let id_1 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdfssubClassOf as i64) as i64;
        let id_2 = NodeDictionary::prop_idx_to_idx(graph.dictionary.rdftype as i64) as i64;
        apply_alpha_rule(graph, id_1, id_2, 3, 4, 2)
    }
}
