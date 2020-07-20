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
    let rests = rests.unwrap().so();
    let mut visited = vec![];
    let mut idx = 0;
    while idx < firsts.len() {
        let [id, _] = firsts[idx];
        if !visited.contains(&id) {
            let (ids, elems) = get_elems_by_id(firsts, rests, id);
            visited.extend(ids);
            to_ret.push(List { id, elems });
        }
        idx += 1;
    }
    to_ret.sort_by_key(|e| e.id);
    to_ret
}

fn get_elems_by_id(
    firsts: &Vec<[u64; 2]>,
    rests: &Vec<[u64; 2]>,
    mut id: u64,
) -> (Vec<u64>, Vec<u64>) {
    let mut ids = vec![];
    let mut elems = vec![];
    let end = NodeDictionary::rdfnil;
    let mut has_next = true;
    while has_next {
        if id == end {
            has_next = false;
        } else {
            ids.push(id);
            let elem = get_elem_by_id(firsts, id);
            elems.push(elem);
            id = get_next_id(rests, id);
        }
    }
    (ids, elems)
}

fn get_elem_by_id(firsts: &Vec<[u64; 2]>, id: u64) -> u64 {
    dbg!(get_second_elem(firsts, id)).unwrap_or_else(|| panic!("inconsistent list is declared"))
}

fn get_next_id(rests: &Vec<[u64; 2]>, id: u64) -> u64 {
    get_second_elem(rests, id).unwrap_or_else(|| panic!("inconsistent list is declared"))
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
