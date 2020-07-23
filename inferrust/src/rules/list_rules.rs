use crate::inferray::*;
use crate::rules::*;
use crate::utils::*;

#[derive(PartialEq)]
enum Match {
    Any,
    All,
    None,
    Global,
}

fn process_cls(ts: &TripleStore, property_idx: usize, opt: Match) -> RuleResult {
    let mut output = vec![];
    let cls_prop = ts.elem().get(property_idx);
    if cls_prop == None {
        return output;
    }
    let cls_prop = cls_prop.unwrap().so();
    let rdftype = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdftype as u64,
    ));
    if rdftype == None {
        return output;
    }
    let rdftype = rdftype.unwrap().os();
    for [c, l] in cls_prop {
        let list = ts.list(*l).unwrap();
        if opt == Match::None {
            for elem in list.elems.iter() {
                output.push([*elem, NodeDictionary::rdftype as u64, *c]);
            }
        } else {
            for [candidate, y] in rdftype {
                if opt == Match::Global {
                    if *candidate > *c {
                        break;
                    }
                    if *candidate == *c {
                        for elem in list.elems.iter() {
                            output.push([*y, NodeDictionary::rdftype as u64, *elem]);
                        }
                    }
                } else {
                    if list.elems.contains(candidate) {
                        let mut ok = true;
                        if opt == Match::All {
                            for elem in list.elems.iter() {
                                if !binary_search_pair(rdftype, [*y, *elem]) {
                                    ok = false;
                                }
                            }
                        }
                        if ok {
                            output.push([*y, NodeDictionary::rdftype as u64, *c]);
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn CLS_INT1(ts: &TripleStore) -> RuleResult {
    process_cls(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::intersectionOf as u64),
        Match::All,
    )
}

pub fn CLS_INT2(ts: &TripleStore) -> RuleResult {
    process_cls(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::intersectionOf as u64),
        Match::Global,
    )
}

pub fn CLS_UNI(ts: &TripleStore) -> RuleResult {
    process_cls(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::unionOf as u64),
        Match::Any,
    )
}

pub fn CLS_OO(ts: &TripleStore) -> RuleResult {
    process_cls(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::oneOf as u64),
        Match::None,
    )
}

fn process_cls_false(ts: &TripleStore, member_idx: usize) -> RuleResult {
    let mut output = vec![];
    let rdf_type = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdftype as u64,
    ));
    if rdf_type == None {
        return output;
    }
    let rdf_type_rev = rdf_type.unwrap().os();
    let same_as = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owlsameAs as u64,
    ));
    if same_as == None {
        return output;
    }
    let same_as = same_as.unwrap().so();
    let diff = NodeDictionary::owlallDifferent as u64;
    if rdf_type_rev[0][0] > diff || rdf_type_rev[rdf_type_rev.len() - 1][0] < diff {
        return output;
    }
    for [o_t, s_t] in rdf_type_rev {
        if *o_t > diff {
            break;
        } else if *o_t == diff {
            if let Some(pairs_members) = ts.elem().get(member_idx) {
                for [s_m, o_m] in pairs_members.so() {
                    if *s_m > *s_t {
                        break;
                    } else if *s_m == *s_t {
                        let members = &ts.list(*o_m).unwrap().elems;
                        let len = members.len();
                        for i in 0..len {
                            for j in i + 1..len {
                                if binary_search_pair(same_as, [members[i], members[j]]) {
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
