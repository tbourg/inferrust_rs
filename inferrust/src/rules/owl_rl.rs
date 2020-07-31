use crate::inferray::*;
use crate::rules::*;
use crate::utils::*;

pub fn CLS_SVF2(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let svf = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owlsomeValuesFrom as u64,
    ));
    if svf == None {
        return output;
    }
    let svf = svf.unwrap().os();
    let onp = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::onProperty as u64,
    ));
    if onp == None {
        return output;
    }
    let onp = onp.unwrap().so();
    for [o_s, s_s] in svf {
        if *o_s > NodeDictionary::owlthing as u64 {
            break;
        } else if *o_s == NodeDictionary::owlthing as u64 {
            for [s_o, o_o] in onp {
                if *s_o == *s_s {
                    let pairs = ts.elem().get(NodeDictionary::prop_idx_to_idx(*o_o));
                    if pairs.is_some() {
                        for [u, _] in pairs.unwrap().so() {
                            output.push([*u, NodeDictionary::rdftype as u64, *s_s]);
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn CLS_HV2(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    // let hv = ts.elem().get(NodeDictionary::prop_idx_to_idx(
    //     NodeDictionary::owlhasValue as u64,
    // ));
    // if hv == None {
    //     return output;
    // }
    // let hv = hv.unwrap().os();
    // let onp = ts.elem().get(NodeDictionary::prop_idx_to_idx(
    //     NodeDictionary::onProperty as u64,
    // ));
    // if onp == None {
    //     return output;
    // }
    // let onp = onp.unwrap().so();
    // for [o_h, s_h] in hv {
    //     for [s_o, o_o] in onp {
    //         if *s_o == *s_h {
    //             let pairs = ts.elem().get(NodeDictionary::prop_idx_to_idx(*o_o));
    //             if pairs.is_some() {
    //                 for [y, u] in pairs.unwrap().os() {
    //                     if *y == *o_h {
    //                         output.push([*u, NodeDictionary::rdftype as u64, *s_h]);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    output
}

pub fn CLS_NOTHING2(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let rdftype = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdftype as u64,
    ));
    if rdftype == None {
        return output;
    }
    let rdftype = rdftype.unwrap().os();
    for [o_t, _] in rdftype {
        if *o_t > NodeDictionary::nothing as u64 {
            break;
        } else if *o_t == NodeDictionary::nothing as u64 {
            output.push(INVALID);
            break;
        }
    }
    output
}

pub fn CAX_DW(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let disjoint = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owldisjointWith as u64,
    ));
    if disjoint == None {
        return output;
    }
    let disjoint = disjoint.unwrap().so();
    let rdftype = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdftype as u64,
    ));
    if rdftype == None {
        return output;
    }
    let rdftype = rdftype.unwrap().os();
    for [s_d, o_d] in disjoint {
        for [o_t, s_t] in rdftype {
            if *o_t > *s_d {
                break;
            } else if *o_t == *s_d {
                if binary_search_pair(rdftype, [*o_d, *s_t]) {
                    output.push(INVALID);
                    return output;
                }
            }
        }
    }
    output
}

pub fn EQ_DIFF1(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let same = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owlsameAs as u64,
    ));
    if same == None {
        return output;
    }
    let same = same.unwrap().so();
    let diff = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owldifferentFrom as u64,
    ));
    if diff == None {
        return output;
    }
    let diff = diff.unwrap().so();
    for [s_s, o_s] in same {
        if binary_search_pair(diff, [*s_s, *o_s]) {
            output.push(INVALID);
            break;
        }
    }
    output
}

pub fn PRP_IRP(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let rdftype = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdftype as u64,
    ));
    if rdftype == None {
        return output;
    }
    let rdftype = rdftype.unwrap().os();
    for [o_t, s_t] in rdftype {
        if *o_t > NodeDictionary::irreflexiveProperty as u64 {
            break;
        } else if *o_t == NodeDictionary::irreflexiveProperty as u64 {
            let pairs = ts.elem().get(NodeDictionary::prop_idx_to_idx(*s_t));
            if pairs.is_some() {
                let pairs = pairs.unwrap().so();
                for [s, o] in pairs {
                    if *s == *o {
                        output.push(INVALID);
                        return output;
                    }
                }
            }
            break;
        }
    }
    output
}

pub fn PRP_ASYP(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    // let rdftype = ts.elem().get(NodeDictionary::prop_idx_to_idx(
    //     NodeDictionary::rdftype as u64,
    // ));
    // if rdftype == None {
    //     return output;
    // }
    // let rdftype = rdftype.unwrap().os();
    // for [o_t, s_t] in rdftype {
    //     if *o_t > NodeDictionary::AsymmetricProperty as u64 {
    //         break;
    //     } else if *o_t == NodeDictionary::AsymmetricProperty as u64 {
    //         let pairs = ts.elem().get(NodeDictionary::prop_idx_to_idx(*s_t));
    //         if pairs.is_some() {
    //             let pairs = pairs.unwrap().so();
    //             for [s, o] in pairs {
    //                 if binary_search_pair(pairs, [*o, *s]) {
    //                     output.push(INVALID);
    //                     return output;
    //                 }
    //             }
    //         }
    //         break;
    //     }
    // }
    output
}

pub fn PRP_PDX(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let dis = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owlpropertyDisjointWith as u64,
    ));
    if dis.is_some() {
        let dis = dis.unwrap().so();
        for [p1, p2] in dis {
            let pairs1 = ts.elem().get(NodeDictionary::prop_idx_to_idx(*p1));
            let pairs2 = ts.elem().get(NodeDictionary::prop_idx_to_idx(*p2));
            if pairs1.is_some() && pairs2.is_some() {
                let pairs1 = pairs1.unwrap().so();
                let pairs2 = pairs2.unwrap().so();
                if !pairs1.is_empty() && !pairs2.is_empty() {
                    for [x, y] in pairs1 {
                        if binary_search_pair(pairs2, [*y, *x]) {
                            output.push(INVALID);
                            return output;
                        }
                    }
                }
            }
        }
    }
    output
}

fn prp_npa(ts: &TripleStore, target_idx: usize) -> RuleResult {
    let mut output = vec![];
    let src = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::sourceIndividual as u64,
    ));
    let assert = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owlassertionProperty as u64,
    ));
    let target = ts.elem().get(target_idx);
    if src.is_some() && assert.is_some() && target.is_some() {
        let src = src.unwrap().so();
        let assert = assert.unwrap().so();
        let target = target.unwrap().so();
        for [x, i] in src {
            for [y, p] in assert {
                if *y > *x {
                    break;
                } else if *y == *x {
                    let pairs = ts.elem().get(NodeDictionary::prop_idx_to_idx(*p));
                    if pairs.is_some() {
                        let pairs = pairs.unwrap().so();
                        for [z, l] in target {
                            if *z > *y {
                                break;
                            } else if *z == *y {
                                if binary_search_pair(pairs, [*i, *l]) {
                                    output.push(INVALID);
                                    return output;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn PRP_NPA1(ts: &TripleStore) -> RuleResult {
    prp_npa(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::owltargetIndividual as u64),
    )
}

pub fn PRP_NPA2(ts: &TripleStore) -> RuleResult {
    prp_npa(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::targetValue as u64),
    )
}

pub fn CLS_COM(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let comp = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owldisjointWith as u64,
    ));
    if comp == None {
        return output;
    }
    let comp = comp.unwrap().so();
    let rdftype = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdftype as u64,
    ));
    if rdftype == None {
        return output;
    }
    let rdftype = rdftype.unwrap().os();
    for [s_c, o_c] in comp {
        for [o_t, s_t] in rdftype {
            if *o_t > *s_c {
                break;
            } else if *o_t == *s_c {
                if binary_search_pair(rdftype, [*o_c, *s_t]) {
                    output.push(INVALID);
                    return output;
                }
            }
        }
    }
    output
}
