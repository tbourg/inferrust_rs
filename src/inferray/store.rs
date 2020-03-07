use super::NodeDictionary;

pub struct TripleStore {
    pub elem: Vec<[Vec<[i64; 2]>; 2]>,
}

impl TripleStore {
    pub fn new() -> Self {
        let elem = Vec::new();
        Self { elem }
    }

    pub fn add_triple(&mut self, triple: [i64; 3]) {
        let [is, ip, io] = triple;
        let ip_to_store = NodeDictionary::prop_idx_to_idx(ip);
        // dbg!(is, ip, io);
        // dbg!(is, ip_to_store, io);
        if ip_to_store >= self.elem.len() {
            self.elem.resize_with(ip_to_store+1, Default::default);
        }
        self.elem[ip_to_store][0].push([is, io]);
        self.elem[ip_to_store][1].push([io, is]);
    }

    pub fn add_all(&mut self, other: Self) {
        if other.elem.len() > self.elem.len() {
            self.elem.resize_with(other.elem.len(), Default::default);
        }
        for i in 0..other.elem.len() {
            let ip = NodeDictionary::idx_to_prop_idx(i);
            for pair in &other.elem[i][0] {
                self.add_triple([pair[0], ip, pair[1]]);
            }
        }
    }

    pub fn sort(&mut self) {
        for chunk in &mut self.elem {
            sort(&mut chunk[0]);
            sort(&mut chunk[1]);
        }
    }

    pub fn res_to_prop(&mut self, res: i64, prop: i32) {
        for chunk in &mut self.elem {
            for i in 0..chunk[0].len() {
                let pair = chunk[0][i];
                if pair[0] == res {
                    chunk[0][i][0] = prop.into();
                }
                if pair[1] == res {
                    chunk[0][i][1] = prop.into();
                }
            }
            for i in 0..chunk[1].len() {
                let pair = chunk[1][i];
                if pair[0] == res {
                    chunk[1][i][0] = prop.into();
                }
                if pair[1] == res {
                    chunk[1][i][1] = prop.into();
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
}

pub fn sort(pairs: &mut Vec<[i64; 2]>) {
    if pairs.is_empty() {
        return;
    }
    let min = pairs.iter().min_by_key(|p| p[0]).unwrap()[0];
    let max = pairs.iter().max_by_key(|p| p[0]).unwrap()[0];
    let width: usize = ((max - min + 1) as usize);
    let mut hist = (hist(&pairs, min, width));
    let hist_copy = hist.clone();
    let start = (cum(&hist));
    let len = pairs.len();
    let mut objects = Vec::new();
    objects.resize(len, 0);
    for i in 0..len {
        let pos = start[(pairs[i][0] - min) as usize];
        let remaining = hist[(pairs[i][0] - min) as usize];
        hist[(pairs[i][0] - min) as usize] -= 1;
        objects[(pos + remaining - 1) as usize] = pairs[i][1];
    }
    for i in 0..(width - 1) {
        sort_with_index(&mut objects, start[i], start[i + 1]);
    }
    sort_with_index(&mut objects, start[width - 1], len as i64);
    let mut j = 0;
    let mut l = 0;
    let mut last = -1;
    for i in 0..width {
        let val = hist_copy[i];
        let s = min + i as i64;
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
    pairs.resize(j as usize, [0, 0]);
}

fn sort_with_index(v: &mut Vec<i64>, from: i64, to: i64) {
    for i in from..to {
        let mut j = i;
        let tmp = v[i as usize];
        while j > from && v[(j - 1) as usize] > tmp {
            v[j as usize] = v[(j - 1) as usize];
            j -= 1;
        }
        v[j as usize] = tmp;
    }
}

fn hist(pairs: &Vec<[i64; 2]>, min: i64, len: usize) -> Vec<i64> {
    let mut hist = Vec::new();
    hist.resize(len, 0);
    for pair in pairs {
        hist[(pair[0] - min) as usize] += 1;
    }
    hist
}

fn cum(hist: &Vec<i64>) -> Vec<i64> {
    let mut cum = Vec::new();
    cum.resize(hist.len(), 0);
    for (i, _e) in hist.iter().enumerate() {
        if i != 0 {
            cum[i] = cum[i - 1] + hist[i - 1];
        }
    }
    cum
}
