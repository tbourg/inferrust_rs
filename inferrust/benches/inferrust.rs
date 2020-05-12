use criterion::{criterion_group, criterion_main, Criterion};

use inferrust::inferray::*;
use inferrust::rules::*;

pub fn simpsons_total(c: &mut Criterion) {
    c.bench_function("simpsons_total", |b| {
        b.iter(|| {
            let mut graph = InfGraph::from(sophia::parser::turtle::parse_str(SIMPSONS));
            assert_eq!(graph.size(), 11);
            graph.process(&mut RuleProfile::RDFSPlus());
            assert_eq!(graph.size(), 57);
        })
    });
}

criterion_group!(benches, simpsons_total);
criterion_main!(benches);

static SIMPSONS: &str = r#"
@prefix : <http://example.org/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

:Bart rdf:type :human .
:Lisa rdf:type :human .
:BLOB a :entity .
:human rdfs:subClassOf :mammal .
:mammal rdfs:subClassOf :animal .
:animal rdfs:subClassOf :entity .
:entity rdfs:subClassOf :animal .
:Bart :enfant :Lisa .
:enfant owl:inverseOf :parent .
:parent owl:equivalentProperty :geniteur . 
:progeniture owl:equivalentProperty :enfant .
"#;
