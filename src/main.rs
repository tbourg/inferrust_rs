use sophia::graph::{*, inmem::LightGraph};
use sophia::ns::Namespace;
use sophia::parser;
use sophia::serializer;
use sophia::serializer::TripleStringifier;
use sophia::triple::stream::{TripleSink, TripleSource};

fn main () {
    let example = r#"
        @prefix : <http://example.org/>.
        @prefix foaf: <http://xmlns.com/foaf/0.1/>.

        :alice foaf:name "Alice";
            foaf:mbox <mailto:alice@work.example> .

        :bob foaf:name "Bob".
    "#;
    let mut graph = LightGraph::new();
    parser::turtle::parse_str(example).in_graph(&mut graph);

    let ex = Namespace::new("http://example.org/").unwrap();
    let foaf = Namespace::new("http://xmlns.com/foaf/0.1/").unwrap();
    let bob = ex.get("bob").unwrap();
    let knows = foaf.get("known").unwrap();
    let alice = ex.get("alice").unwrap();
    graph.insert(&bob, &knows, &alice).unwrap();

    let mut nt_stringifier = serializer::nt::stringifier();
    let example2 = nt_stringifier.stringify_graph(&mut graph).unwrap();
    println!("The resulting graph\n{}", example2);
}