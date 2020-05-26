use crate::inferray::*;
use crate::rules::*;

use rayon::prelude::*;

/// A type alias  to unify all the rules of the reasoner
pub type Rule = fn(&TripleStore) -> Box<dyn Iterator<Item = [u64; 3]> + Sync + Send>;

/// A set of Rule, which can be aplly on a InfGraph
pub trait RuleSet {
    /// Process this ruleset, possibly using multiple threads
    fn process(&mut self, graph: &mut InfGraph);
    fn is_empty(&self) -> bool;
}

impl RuleSet for Vec<Box<Rule>> {
    #[cfg_attr(debug_assertions, flamer::flame)]
    fn process(&mut self, graph: &mut InfGraph) {
        if self.is_empty() {
            return;
        }
        let mut outputs = TripleStore::default();
        let ts = graph.dict().ts();
        outputs.add_all(self.par_iter().map(|rule| rule(ts)).collect());
        outputs.sort();
        let ts = TripleStore::join(graph.dict().ts(), &outputs);
        graph.dict_mut().set_ts(ts);
    }

    #[cfg_attr(debug_assertions, flamer::flame)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

pub struct StaticRuleSet {
    rules: Box<dyn RuleSet>,
}

impl RuleSet for StaticRuleSet {
    #[cfg_attr(debug_assertions, flamer::flame)]
    fn process(&mut self, graph: &mut InfGraph) {
        self.rules.process(graph)
    }
    #[cfg_attr(debug_assertions, flamer::flame)]
    fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

pub struct FixPointRuleSet {
    rules: StaticRuleSet,
}

impl FixPointRuleSet {
    #[cfg_attr(debug_assertions, flamer::flame)]
    fn fixpoint<F: FnMut(&mut StaticRuleSet, &mut InfGraph)>(
        &mut self,
        graph: &mut InfGraph,
        mut process: F,
    ) {
        if self.rules.is_empty() {
            return;
        }
        let mut size = graph.size();
        let mut prev_size = size + 1;
        while prev_size != size {
            prev_size = size;
            process(&mut self.rules, graph);
            size = graph.size();
        }
    }
}

impl RuleSet for FixPointRuleSet {
    #[cfg_attr(debug_assertions, flamer::flame)]
    fn process(&mut self, graph: &mut InfGraph) {
        self.fixpoint(graph, <StaticRuleSet as RuleSet>::process)
    }
    #[cfg_attr(debug_assertions, flamer::flame)]
    fn is_empty(&self) -> bool {
        self.rules.is_empty()
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
    pub after_rules: Option<Box<dyn Fn(&mut InfGraph)>>,
    name: String,
}

impl RuleProfile {
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn RDFS() -> Self {
        let rules: Vec<Box<Rule>> = vec![
            // Alpha class
            Box::new(CAX_SCO),
            Box::new(SCM_DOM1),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG1),
            Box::new(SCM_RNG2),
            // Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
        ];
        let before_rules: Vec<Box<Rule>> = vec![
            // Zeta class (trivial rules)
            Box::new(RDFS4),
            Box::new(RDFS6),
            Box::new(RDFS8),
            Box::new(RDFS10),
            Box::new(RDFS12),
            Box::new(RDFS13),
        ];
        Self {
            cl_profile: ClosureProfile {
                on_sa: false,
                on_sco: true,
                on_spo: true,
                on_trp: false,
            },
            axiomatic_triples: true,
            before_rules: StaticRuleSet {
                rules: Box::new(before_rules),
            },
            rules: FixPointRuleSet {
                rules: StaticRuleSet {
                    rules: Box::new(rules),
                },
            },
            after_rules: Some(Box::new(finalize)),
            name: "RDFS".to_string(),
        }
    }
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn Test() -> Self {
        let rules: Vec<Box<Rule>> = vec![
            // Alpha class
            Box::new(CAX_SCO),
            Box::new(CAX_EQC1),
            Box::new(SCM_DOM1),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG1),
            Box::new(SCM_RNG2),
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
                    rules: Box::new(rules),
                },
            },
            after_rules: None,
            name: "Test".to_string(),
        }
    }
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn RDFSDefault() -> Self {
        Self {
            axiomatic_triples: false,
            // after_rules: None,
            ..Self::RDFS()
        }
    }
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn RhoDF() -> Self {
        let before_rules: Vec<Box<Rule>> = vec![
            // Zeta class (trivial rules)
            Box::new(RDFS4),
        ];
        let rules: Vec<Box<Rule>> = vec![
            // Alpha class
            Box::new(CAX_SCO),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG2),
            // Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
        ];
        Self {
            cl_profile: ClosureProfile {
                on_sa: false,
                on_sco: true,
                on_spo: true,
                on_trp: false,
            },
            axiomatic_triples: false,
            before_rules: StaticRuleSet {
                rules: Box::new(before_rules),
            },
            rules: FixPointRuleSet {
                rules: StaticRuleSet {
                    rules: Box::new(rules),
                },
            },
            after_rules: None,
            name: "RHODF".to_string(),
        }
    }
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn Closure() -> Self {
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
                    rules: Box::new(vec![]),
                },
            },
            after_rules: None,
            name: "Closure".to_string(),
        }
    }
    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn RDFSPlus() -> Self {
        let before_rules: Vec<Box<Rule>> = vec![
            // Zeta class (trivial rules)
            Box::new(RDFS4),
            Box::new(SCM_DP_OP),
            Box::new(SCM_CLS),
        ];
        let rules: Vec<Box<Rule>> = vec![
            // Alpha class
            Box::new(CAX_SCO),
            Box::new(CAX_EQC1),
            Box::new(SCM_DOM1),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG1),
            Box::new(SCM_RNG2),
            // Beta class
            Box::new(SCM_SCO_EQC2),
            Box::new(SCM_SPO_EQP2),
            Box::new(SCM_EQC1),
            Box::new(SCM_EQP1),
            // Delta class
            Box::new(PRP_INV_1_2),
            Box::new(PRP_EQP_1_2),
            // Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
            Box::new(PRP_SYMP),
            Box::new(EQ_TRANS),
            // Same as class
            Box::new(SAME_AS),
            // Other rules
            Box::new(PRP_FP),
            Box::new(PRP_IFP),
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
                rules: Box::new(before_rules),
            },
            rules: FixPointRuleSet {
                rules: StaticRuleSet {
                    rules: Box::new(rules),
                },
            },
            after_rules: Some(Box::new(finalize)),
            name: "RDFSPLUS".to_string(),
        }
    }

    #[cfg_attr(debug_assertions, flamer::flame)]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn PRP_FP(ts: &TripleStore) -> Box<dyn Iterator<Item = [u64; 3]> + Sync + Send> {
    let mut output = vec![];
    let pairs_mut = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdftype as u64,
    ));
    if pairs_mut == None {
        return Box::new(output.into_iter());
    }
    let pairs: &Vec<[u64; 2]> = pairs_mut.unwrap().os(); // os copy
    let expected_o = NodeDictionary::owlfunctionalProperty as u64;
    for pair in &*pairs {
        if pair[0] > expected_o {
            break;
        }
        if pair[0] == expected_o {
            let prop = pair[1];
            let raw_prop = NodeDictionary::prop_idx_to_idx(prop);
            let pairs1 = ts.elem().get(raw_prop);
            if pairs1 == None {
                break;
            }
            let pairs2 = pairs1.unwrap().so();
            if pairs2.is_empty() {
                break;
            }
            let pairs1 = pairs1.unwrap().so();
            for pair1 in pairs1 {
                for pair2 in pairs2 {
                    if pair1[0] > pair2[0] {
                        break;
                    }
                    if pair1[0] == pair2[0] {
                        if pair1[1] != pair2[1] {
                            output.push([pair1[1], NodeDictionary::owlsameAs as u64, pair2[1]])
                        }
                    }
                }
            }
        }
    }
    Box::new(output.into_iter())
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn PRP_IFP(ts: &TripleStore) -> Box<dyn Iterator<Item = [u64; 3]> + Sync + Send> {
    let mut output = vec![];
    let pairs = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdftype as u64,
    ));
    if pairs == None {
        return Box::new(output.into_iter());
    }
    let pairs = pairs.unwrap().os(); // os copy
    let expected_o = NodeDictionary::owlinverseFunctionalProperty as u64;
    for pair in &*pairs {
        if pair[0] > expected_o {
            break;
        }
        if pair[0] == expected_o {
            let prop = pair[1];
            let raw_prop = NodeDictionary::prop_idx_to_idx(prop);
            let pairs1 = ts.elem().get(raw_prop);
            if pairs1 == None {
                break;
            }
            let pairs2 = pairs1.unwrap().os();
            if pairs2.is_empty() {
                break;
            }
            let pairs1 = pairs1.unwrap().os();
            for pair1 in &*pairs1 {
                for pair2 in &*pairs2 {
                    if pair1[0] > pair2[0] {
                        break;
                    }
                    if pair1[0] == pair2[0] {
                        if pair1[1] != pair2[1] {
                            output.push([pair1[1], NodeDictionary::owlsameAs as u64, pair2[1]])
                        }
                    }
                }
            }
        }
    }
    Box::new(output.into_iter())
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn finalize(graph: &mut InfGraph) {
    let type_index = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdftype as u64);
    let res = NodeDictionary::rdfsResource;
    ((NodeDictionary::START_INDEX as u64 + 1)..=graph.dict().get_res_ctr()).for_each(|e| {
        if !graph.dict().was_removed(e) {
            graph.dict_mut().ts_mut().add_triple_raw(e, type_index, res);
        }
    });
    graph.dict_mut().ts_mut().sort();
}
