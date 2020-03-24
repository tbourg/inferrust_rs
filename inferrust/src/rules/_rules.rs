use crate::inferray::InfGraph;
use crate::inferray::TripleStore;
use crate::rules::*;

/// A type alias  to unify all the rules of the reasoner
pub type Rule = fn(&mut InfGraph) -> TripleStore;
// pub trait Rule {
//     // fn specialize(&mut self, graph: std::rc::Rc<&'static InfGraph>);
//     fn fire(&mut self, graph: &mut InfGraph) -> TripleStore;
// }

/// A set of Rule, which can be aplly on a InfGraph
pub trait RuleSet {
    fn new() -> Vec<Box<Rule>>;
    // fn specialize(&mut self, graph: std::rc::Rc<&'static InfGraph>);
    fn fire_all(&mut self, graph: &mut InfGraph);
}

impl RuleSet for Vec<Box<Rule>> {
    fn new() -> Vec<Box<Rule>> {
        vec![
            /// Alpha class
            Box::new(CAX_SCO),
            Box::new(CAX_EQC1),
            Box::new(CAX_EQC2),
            Box::new(SCM_DOM1),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG1),
            Box::new(SCM_RNG2),
            /// Beta class
            Box::new(SCM_EQC2),
            Box::new(SCM_EQP2),
            Box::new(SCM_EQC1),
            Box::new(SCM_EQP1),
            /// Delta class
            Box::new(PRP_INV_1_2),
            Box::new(PRP_EQP_1_2),
            /// Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
            /// Same as class
            Box::new(SAME_AS),
            /// Zeta class (trivial rules)
            Box::new(RDFS6),
            Box::new(RDFS8),
            Box::new(RDFS10),
            Box::new(RDFS12),
            Box::new(RDFS13),
            Box::new(SCM_DP_OP),
            Box::new(SCM_CLS),
        ]
    }
    // fn specialize(&mut self, graph: std::rc::Rc<&'static InfGraph>) {
    // for rule in self.iter() {
    // rule.specialize(std::rc::Rc::clone(&graph));
    // }
    // }
    fn fire_all(&mut self, graph: &mut InfGraph) {
        let mut prev_size = 0;
        let mut size = graph.size();
        while prev_size != size {
            prev_size = size;
            let mut outputs = TripleStore::new();
            for rule in self.iter_mut() {
                outputs.add_all(rule(graph));
            }
            graph.dictionary.ts.add_all(outputs);
            graph.dictionary.ts.sort();
            size = graph.size();
        }
    }
}
