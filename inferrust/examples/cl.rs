use inferrust::closure::ClosureGraph;

fn main() {
    let pairs = vec![
        [1, 2],
        [2, 3],
        [3, 1],
        [4, 5],
        [5, 6],
        [6, 7],
        [11, 12],
        [12, 13],
    ];
    let mut g = ClosureGraph::from(pairs);
    dbg!(g.close());
}
