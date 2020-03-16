use std::mem;

use super::NodeDictionary;

pub struct TripleStore {
    pub elem: Vec<[Vec<[u64; 2]>; 2]>,
}

impl TripleStore {
    pub fn new() -> Self {
        let mut elem = Vec::new();
        // expand the store to fit all the default props
        elem.resize_with(69, Default::default);
        Self { elem }
    }

    pub fn add_triple(&mut self, triple: [u64; 3]) {
        let [is, ip, io] = triple;
        let ip_to_store = NodeDictionary::prop_idx_to_idx(ip);
        // dbg!(is, ip, io);
        // dbg!(is, ip_to_store, io);
        if ip_to_store >= self.elem.len() {
            self.elem.resize_with(ip_to_store + 1, Default::default);
        }
        self.add_triple_raw(is, ip_to_store, io);
    }

    pub fn add_all(&mut self, other: Self) {
        if other.elem.len() > self.elem.len() {
            self.elem.resize_with(other.elem.len(), Default::default);
        }
        for ip in 0..other.elem.len() {
            for [is, io] in &other.elem[ip][0] {
                self.add_triple_raw(*is, ip, *io);
            }
        }
    }

    /// # Pre-condition
    /// `self.elem` must have an element at index `ip`
    #[inline]
    pub fn add_triple_raw(&mut self, is: u64, ip: usize, io: u64) {
        self.elem[ip][0].push([is, io]);
        self.elem[ip][1].push([io, is]);
    }

    pub fn sort(&mut self) {
        let (min, max, width) = self.width();
        let mut hist: Vec<usize> = vec![0; width];
        let mut hist2: Vec<usize> = Vec::with_capacity(width);
        let mut cumul: Vec<usize> = vec![0; width];
        for chunk in &mut self.elem {
            bucket_sort_pairs(
                &mut chunk[0],
                &mut hist,
                &mut hist2,
                &mut cumul,
                min,
                max,
                width,
            );
            bucket_sort_pairs(
                &mut chunk[1],
                &mut hist,
                &mut hist2,
                &mut cumul,
                min,
                max,
                width,
            );
        }
    }

    pub fn res_to_prop(&mut self, res: u64, prop: u32) {
        for chunk in &mut self.elem {
            for i in 0..=1 {
                for pair in &mut chunk[i] {
                    if pair[0] == res {
                        pair[0] = prop.into();
                    }
                    if pair[1] == res {
                        pair[1] = prop.into();
                    }
                }
            }
        }
        /////////
    }

    pub fn size(&mut self) -> usize {
        let mut s = 0;
        for chunk in &self.elem {
            s += chunk[0].len();
        }
        s
    }

    pub fn width(&mut self) -> (u64, u64, usize) {
        let mut min: u64 = u64::max_value();
        let mut max: u64 = 0;
        for chunk in &self.elem {
            for pair in &chunk[0] {
                let (local_min, local_max) = if pair[0] <= pair[1] {
                    (pair[0], pair[1])
                } else {
                    (pair[1], pair[0])
                };
                min = if local_min < min { local_min } else { min };
                max = if local_max > max { local_max } else { max };
            }
        }
        (min, max, (max - min + 1) as usize)
    }
}

/// Sort the pairs and remove duplicates
pub fn bucket_sort_pairs(
    pairs: &mut Vec<[u64; 2]>,
    hist: &mut Vec<usize>,
    hist2: &mut Vec<usize>,
    cumul: &mut Vec<usize>,
    min: u64,
    max: u64,
    width: usize,
) {
    if pairs.is_empty() {
        return;
    }
    build_hist(&pairs, min, hist);
    mem::replace(hist2, hist.to_vec());
    build_cumul(&hist, cumul);
    let len = pairs.len();
    let mut objects = vec![0; len];
    for i in 0..len {
        let pos = cumul[(pairs[i][0] - min) as usize];
        let remaining = hist[(pairs[i][0] - min) as usize];
        hist[(pairs[i][0] - min) as usize] -= 1;
        objects[(pos + remaining - 1) as usize] = pairs[i][1];
    }
    for i in 0..(width - 1) {
        insertion_sort_slice(&mut objects, cumul[i], cumul[i + 1]);
    }
    insertion_sort_slice(&mut objects, cumul[width - 1], len);
    let mut j = 0;
    let mut l = 0;
    let mut last = 0;
    for i in 0..width {
        let val = hist2[i];
        let s = min + i as u64;
        for k in 0..val {
            let o = objects[l];
            l += 1;
            if k == 0 || o != last {
                pairs[j] = [s, o];
                j += 1;
            }
            last = o;
        }
    }
    pairs.truncate(j);
}

fn insertion_sort_slice(v: &mut [u64], from: usize, to: usize) {
    for i in from..to {
        let mut j = i;
        let tmp = v[i];
        while j > from && v[j - 1] > tmp {
            v[j] = v[j - 1];
            j -= 1;
        }
        v[j] = tmp;
    }
}

fn build_hist(pairs: &[[u64; 2]], min: u64, hist: &mut Vec<usize>) {
    for pair in pairs {
        hist[(pair[0] - min) as usize] += 1;
    }
}

fn build_cumul(hist: &[usize], cumul: &mut Vec<usize>) {
    for (i, _e) in hist.iter().enumerate() {
        if i != 0 {
            cumul[i] = cumul[i - 1] + hist[i - 1];
        }
    }
}
