use inferrust::closure::ClosureGraph;

fn main() {
    let pairs = vec![[1, 2], [2, 3], [3, 1]];
    let mut g = ClosureGraph::from(pairs);
    dbg!(g.close());
}
