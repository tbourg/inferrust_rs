//! This crate is an adaptation of [Inferray] in Rust, based on the crate [Sophia].
//!
//!
//! # Getting started
//!
//! Here a quick example on how to build a graph (using [Sophia parser]), and launch the reasoner.
//!
//! ```
//! use sophia::serializer;
//! use sophia::serializer::TripleStringifier;
//!
//! use inferrust::inferray::*;
//! use inferrust::rules::{Rule, RuleSet};
//!
//! fn main() {
//!     let rep = r#"
//!     @prefix : <http://example.org/> .
//!     @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
//!     @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
//!     @prefix owl: <http://www.w3.org/2002/07/owl#> .
//!
//!     :Bart rdf:type :human .
//!     :Lisa rdf:type :human .
//!     :BLOB a :entity .
//!     :human rdfs:subClassOf :mammal .
//!     :mammal rdfs:subClassOf :animal .
//!     :animal owl:equivalentClass :entity .
//!     "#;
//!     let mut graph = InfGraph::from(sophia::parser::turtle::parse_str(rep));
//!
//!     let mut rules = <Vec<Box<dyn Rule>> as RuleSet>::new();
//!     rules.fire_all(&mut graph);
//!
//!     let mut nt_stringifier = serializer::nt::stringifier();
//!     let example2 = nt_stringifier.stringify_graph(&mut graph).unwrap();
//!     println!("The resulting graph\n{}", example2);
//! }
//! ```
//!
//! ## Citation
//! ```
//! Julien Subercaze, Christophe Gravier, Jules Chevalier, Frédérique Laforest:
//! Inferray: fast in-memory RDF inference. PVLDB 9(6): 468-479 (2016)
//! ```

pub mod inferray;
pub mod rules;
