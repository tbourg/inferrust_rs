use crate::inferray::InfGraph;
use crate::inferray::TripleStore;
use crate::rules::*;

pub trait Rule {
    // fn specialize(&mut self, graph: std::rc::Rc<&'static InfGraph>);
    fn fire(&mut self, graph: &mut InfGraph) -> TripleStore;
}

pub trait RuleSet {
    fn new() -> Vec<Box<dyn Rule>>;
    // fn specialize(&mut self, graph: std::rc::Rc<&'static InfGraph>);
    fn fire_all(&mut self, graph: &mut InfGraph);
}

impl RuleSet for Vec<Box<dyn Rule>> {
    fn new() -> Vec<Box<dyn Rule>> {
        vec![
            Box::new(CAX_SCO),
            Box::new(CAX_EQC1),
            Box::new(CAX_EQC2),
            Box::new(SCM_EQC2),
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
                outputs.add_all(rule.fire(graph));
            }
            graph.dictionary.ts.add_all(outputs);
            graph.dictionary.ts.sort();
            size = graph.size();
        }
    }
}
