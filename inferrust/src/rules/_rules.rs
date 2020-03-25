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
    // fn new() -> Vec<Box<Rule>>;
    // fn new(&mut self, rules: Vec<Box<Rule>>);
    fn process(&mut self, graph: &mut InfGraph);
}

impl RuleSet for Vec<Box<Rule>> {
    // fn new(&mut self, rules: Vec<Box<Rule>>) {
    //     self.append(&mut rules);
    // }

    // fn new() -> Vec<Box<Rule>> {
    //     vec![
    //         /// Alpha class
    //         Box::new(CAX_SCO),
    //         Box::new(CAX_EQC1),
    //         Box::new(CAX_EQC2),
    //         Box::new(SCM_DOM1),
    //         Box::new(SCM_DOM2),
    //         Box::new(SCM_RNG1),
    //         Box::new(SCM_RNG2),
    //         /// Beta class
    //         Box::new(SCM_SCO_EQC2),
    //         Box::new(SCM_SPO_EQP2),
    //         Box::new(SCM_EQC1),
    //         Box::new(SCM_EQP1),
    //         /// Delta class
    //         Box::new(PRP_INV_1_2),
    //         Box::new(PRP_EQP_1_2),
    //         /// Gamma class
    //         Box::new(PRP_DOM),
    //         Box::new(PRP_RNG),
    //         Box::new(PRP_SPO1),
    //         Box::new(PRP_SYMP),
    //         Box::new(EQ_TRANS),
    //         /// Same as class
    //         Box::new(SAME_AS),
    //         /// Zeta class (trivial rules)
    //         Box::new(RDFS4),
    //         Box::new(RDFS6),
    //         Box::new(RDFS8),
    //         Box::new(RDFS10),
    //         Box::new(RDFS12),
    //         Box::new(RDFS13),
    //         Box::new(SCM_DP_OP),
    //         Box::new(SCM_CLS),
    //     ]
    // }

    fn process(&mut self, graph: &mut InfGraph) {
        let mut outputs = TripleStore::new();
        for rule in self.iter_mut() {
            outputs.add_all(rule(graph));
        }
        graph.dictionary.ts.add_all(outputs);
        graph.dictionary.ts.sort();
    }
}

pub struct StaticRuleSet {
    rules: Box<dyn RuleSet>,
}

impl StaticRuleSet {
    pub fn process(&mut self, graph: &mut InfGraph) {
        self.rules.process(graph);
    }
}

pub struct FixPointRuleSet {
    rules: StaticRuleSet,
}

impl FixPointRuleSet {
    pub fn process(&mut self, graph: &mut InfGraph) {
        let mut size = graph.size();
        let mut prev_size = size + 1;
        while prev_size != size {
            prev_size = size;
            self.rules.process(graph);
            size = graph.size();
        }
    }
}

pub struct ClosureProfile {
    pub on_sa: bool,
    pub on_sco: bool,
    pub on_spo: bool,
    pub on_trp: bool,
}

pub struct RuleProfile {
    pub cl_profile: ClosureProfile,
    pub axiomatic_triples: bool,
    pub before_rules: StaticRuleSet,
    pub rules: FixPointRuleSet,
    pub after_rules: StaticRuleSet,
}

impl RuleProfile {
    // pub fn RDFS() -> Self {
    //     Self {

    //     }
    // }

    // pub fn RhoDF() -> Self {
    //     Self {

    //     }
    // }

    pub fn RDFSPlus() -> Self {
        let all_rules: Vec<Box<Rule>> = vec![
            /// Alpha class
            Box::new(CAX_SCO),
            Box::new(CAX_EQC1),
            Box::new(CAX_EQC2),
            Box::new(SCM_DOM1),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG1),
            Box::new(SCM_RNG2),
            /// Beta class
            Box::new(SCM_SCO_EQC2),
            Box::new(SCM_SPO_EQP2),
            Box::new(SCM_EQC1),
            Box::new(SCM_EQP1),
            /// Delta class
            Box::new(PRP_INV_1_2),
            Box::new(PRP_EQP_1_2),
            /// Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
            Box::new(PRP_SYMP),
            Box::new(EQ_TRANS),
            /// Same as class
            Box::new(SAME_AS),
            /// Zeta class (trivial rules)
            Box::new(RDFS4),
            Box::new(RDFS6),
            Box::new(RDFS8),
            Box::new(RDFS10),
            Box::new(RDFS12),
            Box::new(RDFS13),
            Box::new(SCM_DP_OP),
            Box::new(SCM_CLS),
        ];
        Self {
            cl_profile: ClosureProfile {
                on_sa: true,
                on_sco: true,
                on_spo: true,
                on_trp: true,
            },
            axiomatic_triples: false,
            before_rules: StaticRuleSet {
                rules: Box::new(vec![]),
            },
            rules: FixPointRuleSet {
                rules: StaticRuleSet {
                    rules: Box::new(all_rules),
                },
            },
            after_rules: StaticRuleSet {
                rules: Box::new(vec![]),
            },
        }
    }
}
