/// Pre-condition: vec is an array of pairs sorted on the first elem of each pair
/// then on the second
#[cfg_attr(debug_assertions, flamer::flame)]
pub fn binary_search_pair(vec: &Vec<[u64; 2]>, pair: [u64; 2]) -> bool {
    let mut start = 0;
    let mut end = vec.len() - 1;
    while start <= end {
        let mid = start + (end - start) / 2;
        if (start == mid || end == mid) && vec[mid] != pair {
            return false;
        }
        if vec[mid] == pair {
            return true;
        }
        if vec[mid][0] > pair[0] {
            end = mid;
        } else if vec[mid][0] == pair[0] {
            if vec[mid][1] > pair[1] {
                end = mid;
            } else {
                start = mid;
            }
        } else {
            start = mid;
        }
    }
    false
}

/// Pre-condition: vec is sorted on the first elem of each pair
#[cfg_attr(debug_assertions, flamer::flame)]
pub fn first(
    vec: &Vec<[u64; 2]>,
    x: u64,
    low: usize,
    high: usize,
    n: usize,
    key_pos: usize,
) -> usize {
    if high >= low {
        let mid = low + (high - low) / 2;

        if mid == low && mid == high {
            if vec[mid][key_pos] != x {
                return n;
            }
        }
        if (mid == 0 || x > vec[mid - 1][key_pos]) && vec[mid][key_pos] == x {
            return mid;
        } else if x > vec[mid][key_pos] {
            return first(vec, x, mid + 1, high, n, key_pos);
        } else {
            return first(vec, x, low, mid, n, key_pos);
        }
    }
    return n;
}
