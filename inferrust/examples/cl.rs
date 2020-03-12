use inferrust::closure::ClosureGraph;

fn main() {
    let pairs = vec![[1, 2], [2, 3], [11, 13], [13, 15]];
    let mut g = ClosureGraph::from(pairs);
    dbg!(g.close());
}
