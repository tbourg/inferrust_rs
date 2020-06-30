use criterion::{criterion_group, criterion_main, Criterion};

use inferrust::inferray::*;
use inferrust::rules::*;

pub fn simpsons_total(c: &mut Criterion) {
    c.bench_function("simpsons_total", |b| {
        b.iter(|| {
            let mut graph = InfGraph::from(sophia::parser::turtle::parse_bufread(
                std::io::BufReader::new(std::fs::File::open("benches/res/simpsons.nt").unwrap()),
            ));
            assert_eq!(graph.size(), 11);
            graph.process(&mut RuleProfile::RDFSPlus());
            assert_eq!(graph.size(), 57);
        })
    });
}

criterion_group!(benches, simpsons_total);
criterion_main!(benches);
