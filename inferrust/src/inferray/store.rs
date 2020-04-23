use rayon::prelude::*;
use std::cmp::Ordering;
use std::mem;

use super::NodeDictionary;

#[derive(PartialEq, Debug, Clone)]
pub struct TripleStore {
    pub elem: Vec<Chunk>,
    size: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Chunk {
    so: Vec<[u64; 2]>,
    os: Option<Vec<[u64; 2]>>,
}

unsafe impl Send for Chunk {}

impl Chunk {
    pub fn so_sort(
        &mut self,
        hist: &mut Vec<usize>,
        hist2: &mut Vec<usize>,
        cumul: &mut Vec<usize>,
        min: u64,
        max: u64,
        width: usize,
    ) -> usize {
        self.os = None;
        bucket_sort_pairs(&mut self.so, hist, hist2, cumul, min, max, width)
    }

    unsafe fn os_sort(&self) {
        // TODO
        let content = &self.os as *const Option<Vec<[u64; 2]>> as *mut Option<Vec<[u64; 2]>>;
        let content = content.as_mut().unwrap();
        std::mem::replace(
            content,
            Some(
                self.so
                    .clone()
                    .iter_mut()
                    .map(|pair| {
                        pair.reverse();
                        *pair
                    })
                    .collect(),
            ),
        );
        // dbg!(&content);
        if !self.so.is_empty() {
            let mut content = content.as_mut().unwrap();
            let (min, max) = content
                .iter()
                .map(|pair| pair[0])
                .fold((u64::max_value(), 0), |acc, x| (acc.0.min(x), acc.1.max(x)));
            let width = (max - min + 1) as usize;
            let mut hist: Vec<usize> = vec![0; width];
            let mut hist2: Vec<usize> = Vec::with_capacity(width);
            let mut cumul: Vec<usize> = vec![0; width];
            bucket_sort_pairs(
                &mut content,
                &mut hist,
                &mut hist2,
                &mut cumul,
                min,
                max,
                width,
            );
        }
    }
    pub fn so(&self) -> &Vec<[u64; 2]> {
        &self.so
    }
    pub fn os(&self) -> &Vec<[u64; 2]> {
        if self.os.is_none() {
            unsafe {
                self.os_sort();
            }
            assert!(!self.os.is_none());
        }
        self.os.as_ref().unwrap()
    }

    fn res_to_prop(&mut self, res: u64, prop: u64) {
        for pair in self.so.iter_mut() {
            for i in 0..=1 {
                if pair[i] == res {
                    pair[i] = prop;
                }
            }
        }
    }

    pub fn add_so(&mut self, so: [u64; 2]) {
        self.so.push(so);
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            so: Vec::new(),
            os: None,
        }
    }
}

impl TripleStore {
    pub fn new() -> Self {
        let elem = Vec::new();
        Self { elem, size: 0 }
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
            for [is, io] in other.elem[ip].so() {
                self.add_triple_raw(*is, ip, *io);
            }
        }
    }

    /// # Pre-condition
    /// `self.elem` must have an element at index `ip`
    #[inline]
    pub fn add_triple_raw(&mut self, is: u64, ip: usize, io: u64) {
        self.elem[ip].add_so([is, io]);
        self.size += 1;
    }

    pub fn sort(&mut self) {
        if self.elem.is_empty() {
            return;
        }
        let (min, max, width) = self.width();
        self.size = self
            .elem
            .par_iter_mut()
            .map_init(
                || {
                    let hist: Vec<usize> = vec![0; width];
                    let hist2: Vec<usize> = Vec::with_capacity(width);
                    let cumul: Vec<usize> = vec![0; width];
                    (hist, hist2, cumul)
                },
                |(hist, hist2, cumul), chunk| chunk.so_sort(hist, hist2, cumul, min, max, width),
            )
            .sum();
    }

    pub fn res_to_prop(&mut self, res: u64, prop: u32) {
        for chunk in &mut self.elem {
            chunk.res_to_prop(res, prop.into());
        }
        /////////
    }

    pub fn size(&mut self) -> usize {
        self.size
    }

    pub fn width(&mut self) -> (u64, u64, usize) {
        let (min, max) = self
            .elem
            .iter()
            .map(|chunk| chunk.so())
            .flat_map(|pairs| pairs.iter())
            .map(|pair| pair[0])
            .fold((u64::max_value(), 0), |acc, x| (acc.0.min(x), acc.1.max(x)));
        (min, max, (max - min + 1) as usize)
    }

    pub fn join(a: &Self, b: &Self) -> Self {
        let len = std::cmp::max(a.elem.len(), b.elem.len());
        let mut chunks: Vec<Chunk> = Vec::new();
        let mut size = 0;
        if b.elem.is_empty() {
            chunks = a.elem.clone();
            size = a.size;
        } else {
            for i in 0..len {
                let mut new_so: Vec<[u64; 2]> = Vec::new();
                let mut original = &vec![];
                let mut infered = &vec![];
                if let Some(opt) = a.elem.get(i) {
                    original = opt.so();
                }
                if let Some(opt) = b.elem.get(i) {
                    infered = opt.so();
                }
                match (original.is_empty(), infered.is_empty()) {
                    (false, false) => {
                        let mut index_o = 0;
                        let mut index_i = 0;
                        let mut new_o = true;
                        let mut new_i = true;
                        let mut s_o = 0;
                        let mut o_o = 0;
                        let mut s_i = 0;
                        let mut o_i = 0;
                        while index_o < original.len() || index_i < infered.len() {
                            if new_o {
                                if index_o < original.len() {
                                    let pair_o = original[index_o];
                                    s_o = pair_o[0];
                                    o_o = pair_o[1];
                                } else {
                                    s_o = u64::max_value();
                                    o_o = u64::max_value();
                                }
                                index_o += 1;
                            }
                            if new_i {
                                if index_i < infered.len() {
                                    let pair_i = infered[index_i];
                                    s_i = pair_i[0];
                                    o_i = pair_i[1];
                                } else {
                                    s_i = u64::max_value();
                                    o_i = u64::max_value();
                                }
                                index_i += 1;
                            }
                            new_o = false;
                            new_i = false;
                            match s_o.cmp(&s_i) {
                                Ordering::Less => {
                                    new_so.push([s_o, o_o]);
                                    new_o = true;
                                }
                                Ordering::Greater => {
                                    new_so.push([s_i, o_i]);
                                    new_i = true;
                                }
                                Ordering::Equal => match o_o.cmp(&o_i) {
                                    Ordering::Less => {
                                        new_so.push([s_o, o_o]);
                                        new_o = true;
                                    }
                                    Ordering::Greater => {
                                        new_so.push([s_i, o_i]);
                                        new_i = true;
                                    }
                                    Ordering::Equal => {
                                        new_so.push([s_o, o_o]);
                                        new_o = true;
                                        new_i = true;
                                    }
                                },
                            }
                        }
                    }
                    (false, true) => {
                        new_so = original.clone();
                    }
                    (true, false) => {
                        new_so = infered.clone();
                    }
                    (true, true) => (),
                }
                size += new_so.len();
                chunks.push(Chunk {
                    so: new_so,
                    os: None,
                });
            }
        }
        Self { elem: chunks, size }
    }
}

/// Sort the pairs and remove duplicates
pub fn bucket_sort_pairs(
    pairs: &mut Vec<[u64; 2]>,
    hist: &mut Vec<usize>,
    hist2: &mut Vec<usize>,
    cumul: &mut Vec<usize>,
    min: u64,
    _max: u64,
    width: usize,
) -> usize {
    if pairs.is_empty() {
        return 0;
    }
    // let t0 = precise_time_ns();
    build_hist(pairs, min, hist);
    // let t1 = precise_time_ns();
    // let time = (t1 - t0) as f64 / :1e9;
    // println!("hist: {}", time);
    // let t0 = precise_time_ns();
    mem::replace(hist2, hist.to_vec());
    // let t1 = precise_time_ns();
    // let time = (t1 - t0) as f64 / 1e9;
    // println!("hist copy: {}", time);
    // let t0 = precise_time_ns();
    build_cumul(&hist, cumul);
    // let t1 = precise_time_ns();
    // let time = (t1 - t0) as f64 / 1e9;
    // println!("cumul: {}", time);
    // let mut access = 0.0;
    // let mut calc = 0.0;
    // let mut assign = 0.0;
    // let t0 = precise_time_ns();
    let len = pairs.len();
    let mut objects = vec![0; len];
    for i in 0..len {
        // let t0 = precise_time_ns();
        let val = pairs[i];
        let val_s = val[0];
        let val_o = val[1];
        // let t1 = precise_time_ns();
        // let time = (t1 - t0) as f64 / 1e9;
        // access += time;
        // let t0 = precise_time_ns();
        let idx = (val_s - min) as usize;
        // let t1 = precise_time_ns();
        // let time = (t1 - t0) as f64 / 1e9;
        // calc += time;
        // let t0 = precise_time_ns();
        let pos = cumul[idx];
        let remaining = hist[idx];
        // let t1 = precise_time_ns();
        // let time = (t1 - t0) as f64 / 1e9;
        // access += time;
        // let t0 = precise_time_ns();
        let obj_idx = (pos + remaining - 1) as usize;
        // let t1 = precise_time_ns();
        // let time = (t1 - t0) as f64 / 1e9;
        // calc += time;
        // let t0 = precise_time_ns();
        hist[idx] -= 1;
        objects[obj_idx] = val_o;
        // let t1 = precise_time_ns();
        // let time = (t1 - t0) as f64 / 1e9;
        // assign += time;
    }
    // let t1 = precise_time_ns();
    // let time = (t1 - t0) as f64 / 1e9;
    // println!("obj creation: {}({}, {}, {})", time, access, calc, assign);
    // let t0 = precise_time_ns();
    for i in 0..(width - 1) {
        insertion_sort_slice(&mut objects, cumul[i], cumul[i + 1]);
    }
    insertion_sort_slice(&mut objects, cumul[width - 1], len);
    // let t1 = precise_time_ns();
    // let time = (t1 - t0) as f64 / 1e9;
    // println!("obj sorting: {}", time);
    // let t0 = precise_time_ns();
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
    // let t1 = precise_time_ns();
    // let time = (t1 - t0) as f64 / 1e9;
    // println!("output creation: {}", time);
    // let t0 = precise_time_ns();
    pairs.truncate(j);
    // let t1 = precise_time_ns();
    // let time = (t1 - t0) as f64 / 1e9;
    // println!("truncation: {}", time);
    j
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

fn build_hist(pairs: &[[u64; 2]], min: u64, hist: &mut [usize]) {
    for pair in pairs {
        hist[(pair[0] - min) as usize] += 1;
    }
}

fn build_cumul(hist: &[usize], cumul: &mut [usize]) {
    for i in 1..hist.len() {
        cumul[i] = cumul[i - 1] + hist[i - 1];
    }
}

/// Reverse the pairs and sort them
fn _bucket_sort_pairs_os(
    pairs: &mut [[u64; 2]],
    hist: &mut [usize],
    hist2: &mut [usize],
    cumul: &mut [usize],
    min: u64,
    _max: u64,
    width: usize,
) -> usize {
    if pairs.is_empty() {
        return 0;
    }
    build_hist(pairs, min, hist);
    dbg!(&pairs, &hist);
    // mem::replace(hist2, hist.to_vec());
    build_cumul(&hist, cumul);
    let len = pairs.len();
    let mut objects = vec![0; len];
    for i in 0..len {
        let pos = cumul[(pairs[i][0] - min) as usize];
        let remaining = hist[(pairs[i][0] - min) as usize];
        hist[(pairs[i][0] - min) as usize] -= 1;
        objects[(pos + remaining - 1) as usize] = pairs[i][1];
    }
    dbg!(&objects, &cumul);
    for i in 0..(width - 1) {
        insertion_sort_slice(&mut objects, cumul[i], cumul[i + 1]);
    }
    insertion_sort_slice(&mut objects, cumul[width - 1], len);
    dbg!(&hist2, &objects);
    let mut j = 0;
    let mut l = 0;
    for i in 0..width {
        let val = hist2[i];
        for _ in 0..val {
            let o = objects[l];
            l += 1;
            let s = min + i as u64;
            pairs[j] = [o, s];
            j += 1;
        }
    }
    // pairs.truncate(j);
    j
}

#[test]
fn test_sort() {
    let mut hist = vec![0; 1000];
    let mut hist2 = vec![0; 1000];
    let mut cumul = vec![0; 1000];
    let mut pairs = vec![[2, 1], [1, 3]];
    bucket_sort_pairs(&mut pairs, &mut hist, &mut hist2, &mut cumul, 1, 3, 2);
    let expected = [[1, 3], [2, 1]];
    assert_eq!(pairs, expected);
    let mut pairs = vec![[2, 1], [1, 3], [2, 1]];
    bucket_sort_pairs(&mut pairs, &mut hist, &mut hist2, &mut cumul, 1, 3, 2);
    let expected = [[1, 3], [2, 1]];
    assert_eq!(pairs, expected);
    let mut pairs = vec![[2, 1], [1, 3], [1, 3]];
    bucket_sort_pairs(&mut pairs, &mut hist, &mut hist2, &mut cumul, 1, 3, 2);
    let expected = [[1, 3], [2, 1]];
    assert_eq!(pairs, expected);
    let mut pairs = vec![[2, 3], [2, 1]];
    bucket_sort_pairs(&mut pairs, &mut hist, &mut hist2, &mut cumul, 1, 3, 2);
    let expected = [[2, 1], [2, 3]];
    assert_eq!(pairs, expected);
}

#[test]
fn test_join() {
    let a = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let b = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let expected = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    assert_eq!(TripleStore::join(&a, &b), expected);
    let a = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1]],
            os: None,
        }],
        size: 1,
    };
    let b = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let expected = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1]],
            os: None,
        }],
        size: 1,
    };
    assert_eq!(TripleStore::join(&a, &b), expected);
    let a = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let b = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1]],
            os: None,
        }],
        size: 1,
    };
    let expected = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1]],
            os: None,
        }],
        size: 1,
    };
    assert_eq!(TripleStore::join(&a, &b), expected);
    let a = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1]],
            os: None,
        }],
        size: 1,
    };
    let b = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1]],
            os: None,
        }],
        size: 1,
    };
    let expected = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1]],
            os: None,
        }],
        size: 1,
    };
    assert_eq!(TripleStore::join(&a, &b), expected);
    let a = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1], [1, 2]],
            os: None,
        }],
        size: 2,
    };
    let b = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1]],
            os: None,
        }],
        size: 1,
    };
    let expected = TripleStore {
        elem: vec![Chunk {
            so: vec![[1, 1], [1, 2]],
            os: None,
        }],
        size: 2,
    };
    assert_eq!(TripleStore::join(&a, &b), expected);
    let a = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let b = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let expected = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    assert_eq!(TripleStore::join(&a, &b), expected);
    let a = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let b = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let expected = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    assert_eq!(TripleStore::join(&a, &b), expected);
    let a = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let b = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let expected = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    assert_eq!(TripleStore::join(&a, &b), expected);
    let a = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let b = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    let expected = TripleStore {
        elem: vec![Chunk {
            so: vec![],
            os: None,
        }],
        size: 0,
    };
    assert_eq!(TripleStore::join(&a, &b), expected);
}
