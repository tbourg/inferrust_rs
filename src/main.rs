use std::{any, boxed, convert, env, fs, process, rc};

use sophia::graph::{inmem::LightGraph, *};
use sophia::parser;
use sophia::query::Query;
use sophia::serializer;
use sophia::serializer::TripleStringifier;
use sophia::term::{BoxTerm, RcTerm, Term};
use sophia::triple::stream::{TripleSink, TripleSource};
use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia::triple::Triple;

use clap::{App, Arg, SubCommand};

fn print_type<T>(_: T) {
    println!("{}", any::type_name::<T>())
}

fn create_term(op: &str, iri: bool) -> BoxTerm {
    let term = if iri {
        BoxTerm::new_iri(op)
    } else {
        BoxTerm::new_literal_dt(
            op,
            BoxTerm::new_iri("http://www.w3.org/2001/XMLSchema#string").unwrap(),
        )
    }
    .expect("Error creating term");
    return term;
}

fn load_graph(filename: &str) -> LightGraph {
    let content = fs::read_to_string(filename).expect("Error reading graph");
    let mut graph = LightGraph::new();
    parser::turtle::parse_str(&content)
        .in_graph(&mut graph)
        .expect("Error loading graph");
    graph
}

fn display_triples_spo(g: LightGraph, term: &str, s: bool, p: bool, o: bool) {
    let term: BoxTerm = create_term(term, s || p);
    let it = if s {
        g.triples_with_s(&term)
    } else if p {
        g.triples_with_p(&term)
    } else if o {
        g.triples_with_o(&term)
    } else {
        g.triples()
    };
    let mut vec = Vec::new();
    unsafe {
        for t in boxed::Box::into_raw(it).as_mut().unwrap() {
            vec.push(t.unwrap());
        }
    }
    display_triples(vec);
}

fn display_triple(triple: StreamedTriple<ByTermRefs<rc::Rc<str>>>) {
    let mut nt_stringifier = serializer::nt::stringifier();
    println!("{}", nt_stringifier.stringify_triple(&triple).unwrap());
}

fn display_triples(col: Vec<StreamedTriple<ByTermRefs<rc::Rc<str>>>>) {
    for triple in col {
        display_triple(triple);
    }
}

/*fn main() {
    let filename = "res/sample.ttl";
    let content = fs::read_to_string(filename).expect("Error reading file.");
    let mut graph = LightGraph::new();
    parser::turtle::parse_str(&content)
        .in_graph(&mut graph)
        .expect("Error loading graph.");

    let s
    let p
    let o

    let mut vec = Vec::new();
    vec.push(s);
    vec.push(p);
    vec.push(o);
    let query = Query(vec);

    let result_it = unsafe { boxed::Box::into_raw(results).as_mut().unwrap() };

    let mut nt_stringifier = serializer::nt::stringifier();
    for result in result_it {
        let triple = result.unwrap();
        println!("{}", nt_stringifier.stringify_triple(&triple).unwrap());
    }
    /*let example = nt_stringifier.stringify_graph(&mut graph).unwrap();
    println!("The resulting graph\n{}", example);*/
}*/
// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
// extern crate clap;
// use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("InterPhia (A Sophia Interface)")
        .version("1.0")
        .author("Thomas Bourg")
        .about("Does awesome things")
        .arg(
            Arg::with_name("graph")
                .short("g")
                .long("graph")
                .value_name("FILE")
                .help("The graph in a Turtle file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("subject")
                .short("s")
                .long("subject")
                .value_name("SUBJECT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("predicate")
                .short("p")
                .long("predicate")
                .value_name("PREDICATE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("object")
                .short("o")
                .long("object")
                .value_name("OBJECT")
                .takes_value(true),
        )
        .get_matches();

    let filename = matches.value_of("graph").unwrap_or("res/sample.ttl");
    let graph = load_graph(filename);

    let by_s = matches.is_present("subject");
    let by_p = matches.is_present("predicate");
    let by_o = matches.is_present("object");
    let val = format!(
        "{}{}{}",
        matches.value_of("subject").unwrap_or(""),
        matches.value_of("predicate").unwrap_or(""),
        matches.value_of("object").unwrap_or("")
    );

    display_triples_spo(graph, &val, by_s, by_p, by_o);
}
