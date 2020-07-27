use crate::inferray::{NodeDictionary, TripleStore};
use crate::utils::*;

pub const INVALID: [u64; 3] = [
    NodeDictionary::owlthing as u64,
    NodeDictionary::rdfssubClassOf as u64,
    NodeDictionary::nothing as u64,
];

#[derive(Default, PartialEq, Debug, Clone)]
pub struct List {
    pub id: u64,
    pub elems: Vec<u64>,
}

pub type Lists = Vec<List>;

pub fn compute_lists(ts: &TripleStore) -> Lists {
    let mut to_ret = vec![];
    let firsts = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdffirst as u64,
    ));
    if firsts == None {
        return to_ret;
    }
    let firsts = firsts.unwrap().so();
    let rests = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::rdfrest as u64,
    ));
    if rests == None {
        return to_ret;
    }
    let rests_rev = rests.unwrap().os();
    let end = NodeDictionary::rdfnil;
    for [rest, id] in rests_rev {
        if *rest > end {
            break;
        }
        if *rest == end {
            if let Some((head, elems)) = get_elems_by_last_id(firsts, rests_rev, *id) {
                to_ret.push(List { id: head, elems });
            }
        }
    }
    to_ret.sort_by_key(|e| e.id);
    to_ret
}

fn get_elems_by_last_id(
    firsts: &Vec<[u64; 2]>,
    rests_rev: &Vec<[u64; 2]>,
    mut id: u64,
) -> Option<(u64, Vec<u64>)> {
    let mut elems = vec![];
    let mut ids = vec![];
    ids.push(id);
    loop {
        let elem = get_elem_by_id(firsts, id);
        elems.push(elem);
        if let Some(prev) = get_prev_id(rests_rev, id) {
            if ids.contains(&prev) {
                return None;
            }
            id = prev;
            ids.push(id);
        } else {
            break;
        }
    }
    elems.reverse();
    Some((id, elems))
}

fn get_elem_by_id(firsts: &Vec<[u64; 2]>, id: u64) -> u64 {
    get_second_elem(firsts, id).unwrap_or_else(|| panic!("inconsistent list is declared"))
}

fn get_prev_id(rests_rev: &Vec<[u64; 2]>, id: u64) -> Option<u64> {
    get_second_elem(rests_rev, id)
}
