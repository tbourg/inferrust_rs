use crate::inferray::{NodeDictionary, TripleStore};

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
            let (head, elems) = get_elems_by_last_id(firsts, rests_rev, *id);
            to_ret.push(List { id: head, elems });
        }
    }
    to_ret.sort_by_key(|e| e.id);
    to_ret
}

fn get_elems_by_last_id(
    firsts: &Vec<[u64; 2]>,
    rests_rev: &Vec<[u64; 2]>,
    mut id: u64,
) -> (u64, Vec<u64>) {
    let mut elems = vec![];
    loop {
        let elem = get_elem_by_id(firsts, id);
        elems.push(elem);
        if let Some(prev) = get_prev_id(rests_rev, id) {
            id = prev;
        } else {
            break;
        }
    }
    elems.reverse();
    (id, elems)
}

fn get_elem_by_id(firsts: &Vec<[u64; 2]>, id: u64) -> u64 {
    dbg!(get_second_elem(firsts, id)).unwrap_or_else(|| panic!("inconsistent list is declared"))
}

fn get_prev_id(rests_rev: &Vec<[u64; 2]>, id: u64) -> Option<u64> {
    get_second_elem(rests_rev, id)
}

use std::cmp::Ordering;

/// Pre-condition: vec is an array of pairs sorted on the first elem of each pair
/// then on the second
pub fn get_second_elem(vec: &[[u64; 2]], first: u64) -> Option<u64> {
    dbg!(vec, first);
    let start = 0;
    let end = vec.len() - 1;
    let mid = start + (end - start) / 2;
    dbg!(start, end, mid);
    if start == end && vec[mid][0] != first {
        return None;
    }
    match vec[mid][0].cmp(&first) {
        Ordering::Greater => get_second_elem(&vec[..mid], first),
        Ordering::Equal => Some(vec[mid][1]),
        Ordering::Less => get_second_elem(&vec[(mid + 1)..], first),
    }
}
