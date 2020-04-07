#![allow(non_snake_case)]

use sophia::ns::*;
use sophia::term::factory::{RcTermFactory, TermFactory};
use sophia::term::{RcTerm, Term, TermData};

use std::convert::TryInto;

use bimap::hash::BiHashMap;

use super::TripleStore;

pub struct NodeDictionary {
    res_ctr: u64,
    prop_ctr: u32,
    removed_val: Vec<u64>,
    resources: BiHashMap<RcTerm, u64>,
    properties: BiHashMap<RcTerm, u32>,
    pub ts: TripleStore,
    factory: RcTermFactory,
    pub rdfsResource: u64,
    pub rdfsClass: u64,
    pub rdfsDatatype: u64,
    pub rdfsLiteral: u64,
    pub rdfsContainer: u64,
    pub rdfsdomain: u32,
    pub rdfsrange: u32,
    pub rdfssubClassOf: u32,
    pub rdfssubPropertyOf: u32,
    pub rdfsSeeAlso: u32,
    pub rdfsisDefinedBy: u32,
    pub rdfsComment: u32,
    pub rdfsMember: u32,
    pub rdfsContainerMembershipProperty: u32,
    pub rdfsLabel: u32,
    pub rdfList: u64,
    pub rdfAlt: u64,
    pub rdfBag: u64,
    pub rdfSeq: u64,
    pub rdfXMLLiteral: u64,
    pub rdfStatement: u64,
    pub rdfnil: u64,
    pub rdfProperty: u32,
    pub rdftype: u32,
    pub rdfsubject: u32,
    pub rdfobject: u32,
    pub rdfpredicate: u32,
    pub rdffirst: u32,
    pub rdfrest: u32,
    pub rdfValue: u32,
    pub rdf_1: u32,
    pub xsdnonNegativeInteger: u64,
    pub xsdstring: u64,
    pub owlthing: u32,
    pub owltransitiveProperty: u32,
    pub owlequivalentClass: u32,
    pub owlequivalentProperty: u32,
    pub owlobjectProperty: u32,
    pub owldataTypeProperty: u32,
    pub owlsameAs: u32,
    pub owlinverseOf: u32,
    pub owlpropertyDisjointWith: u32,
    pub owldifferentFrom: u32,
    pub owlallDifferent: u32,
    pub owlallDisjointClasses: u32,
    pub owlallValuesFrom: u32,
    pub owlannotationProperty: u32,
    pub owlassertionProperty: u32,
    pub owlclass: u64,
    pub owlcomplementOf: u32,
    pub owldisjoinWith: u32,
    pub owldistinctmembers: u32,
    pub owlfunctionalProperty: u32,
    pub intersectionOf: u32,
    pub unionOf: u32,
    pub owlinverseFunctionalProperty: u32,
    pub irreflexiveProperty: u32,
    pub maxCardinality: u32,
    pub members: u32,
    pub nothing: u32,
    pub onClass: u32,
    pub onProperty: u32,
    pub oneOf: u32,
    pub propertyChainAxiom: u32,
    pub owlsomeValuesFrom: u32,
    pub sourceIndividual: u32,
    pub owlsymmetricProperty: u32,
    pub owltargetIndividual: u32,
    pub targetValue: u32,
    pub maxQualifiedCardinality: u32,
}

impl NodeDictionary {
    pub const START_INDEX: u32 = u32::max_value();

    pub fn new(ts: TripleStore) -> Self {
        let mut me = Self {
            res_ctr: Self::START_INDEX as u64,
            prop_ctr: Self::START_INDEX,
            removed_val: vec![],
            resources: BiHashMap::<RcTerm, u64>::new(),
            properties: BiHashMap::<RcTerm, u32>::new(),
            ts,
            factory: RcTermFactory::new(),
            rdfsResource: 0,
            rdfsClass: 0,
            rdfsDatatype: 0,
            rdfsLiteral: 0,
            rdfsContainer: 0,
            rdfsdomain: 0,
            rdfsrange: 0,
            rdfssubClassOf: 0,
            rdfssubPropertyOf: 0,
            rdfsSeeAlso: 0,
            rdfsisDefinedBy: 0,
            rdfsComment: 0,
            rdfsMember: 0,
            rdfsContainerMembershipProperty: 0,
            rdfsLabel: 0,
            rdfList: 0,
            rdfAlt: 0,
            rdfBag: 0,
            rdfSeq: 0,
            rdfXMLLiteral: 0,
            rdfStatement: 0,
            rdfnil: 0,
            rdfProperty: 0,
            rdftype: 0,
            rdfsubject: 0,
            rdfobject: 0,
            rdfpredicate: 0,
            rdffirst: 0,
            rdfrest: 0,
            rdfValue: 0,
            rdf_1: 0,
            xsdnonNegativeInteger: 0,
            xsdstring: 0,
            owlthing: 0,
            owltransitiveProperty: 0,
            owlequivalentClass: 0,
            owlequivalentProperty: 0,
            owlobjectProperty: 0,
            owldataTypeProperty: 0,
            owlsameAs: 0,
            owlinverseOf: 0,
            owlpropertyDisjointWith: 0,
            owldifferentFrom: 0,
            owlallDifferent: 0,
            owlallDisjointClasses: 0,
            owlallValuesFrom: 0,
            owlannotationProperty: 0,
            owlassertionProperty: 0,
            owlclass: 0,
            owlcomplementOf: 0,
            owldisjoinWith: 0,
            owldistinctmembers: 0,
            owlfunctionalProperty: 0,
            intersectionOf: 0,
            unionOf: 0,
            owlinverseFunctionalProperty: 0,
            irreflexiveProperty: 0,
            maxCardinality: 0,
            members: 0,
            nothing: 0,
            onClass: 0,
            onProperty: 0,
            oneOf: 0,
            propertyChainAxiom: 0,
            owlsomeValuesFrom: 0,
            sourceIndividual: 0,
            owlsymmetricProperty: 0,
            owltargetIndividual: 0,
            targetValue: 0,
            maxQualifiedCardinality: 0,
        };
        me.init_const();
        me
    }

    pub fn add<TD: TermData>(&mut self, term: &Term<TD>) -> u64 {
        let t = self.factory.clone_term(term);
        if self.properties.contains_left(&t) {
            return *self.properties.get_by_left(&t).expect("Err") as u64;
        }
        if self.resources.contains_left(&t) {
            *self.resources.get_by_left(&t).expect("Err")
        } else {
            self.res_ctr += 1;
            self.resources.insert(t, self.res_ctr);
            self.res_ctr
        }
    }

    pub fn add_property<TD: TermData>(&mut self, term: &Term<TD>) -> u32 {
        let t = self.factory.clone_term(term);
        if self.resources.contains_left(&t) {
            self.remap_res_to_prop(t)
        } else if self.properties.contains_left(&t) {
            *self.properties.get_by_left(&t).expect("Err")
        } else {
            self.prop_ctr -= 1;
            self.properties.insert(t, self.prop_ctr);
            self.prop_ctr
        }
    }

    fn remap_res_to_prop(&mut self, t: RcTerm) -> u32 {
        let old = self.resources.remove_by_left(&t).expect("Err").1;
        self.prop_ctr -= 1;
        let p = self.prop_ctr;
        self.properties.insert(t, p);
        self.removed_val.push(old);
        self.ts.res_to_prop(old, p);
        p
    }

    pub fn get_term(&self, index: u64) -> &RcTerm {
        if index < Self::START_INDEX as u64 {
            self.properties
                .get_by_right(&(index as u32))
                .expect(&format!("No such properties {}", index))
        } else {
            self.resources
                .get_by_right(&index)
                .expect(&format!("No such ressources {}", index))
        }
    }

    pub fn get_index<T>(&self, t: &Term<T>) -> Option<u64>
    where
        T: TermData,
    {
        let inner_term = RcTerm::from(t);
        if self.properties.contains_left(&inner_term) {
            Some(*self.properties.get_by_left(&inner_term).unwrap() as u64)
        } else if self.resources.contains_left(&inner_term) {
            Some(*self.resources.get_by_left(&inner_term).unwrap())
        } else {
            None
        }
    }

    pub fn was_removed(&self, res: &u64) -> bool {
        self.removed_val.contains(res)
    }

    pub fn get_res_ctr(&self) -> u64 {
        self.res_ctr
    }

    pub fn prop_idx_to_idx(prop_idx: u64) -> usize {
        (/*dbg!(*/Self::START_INDEX as u64 - prop_idx - 1/*)*/)
            .try_into()
            .expect("Err converting index")
    }

    pub fn idx_to_prop_idx(idx: usize) -> u64 {
        Self::START_INDEX as u64 - idx as u64 - 1
    }

    fn init_const(&mut self) {
        // ---------------RDFS
        self.rdfsResource = self.add(&rdfs::Resource);
        self.rdfsClass = self.add(&rdfs::Class);
        self.rdfsDatatype = self.add(&rdfs::Datatype);
        self.rdfsLiteral = self.add(&rdfs::Literal);
        self.rdfsContainer = self.add(&rdfs::Container);

        self.rdfsdomain = self.add_property(&rdfs::domain);
        self.rdfsrange = self.add_property(&rdfs::range);
        self.rdfssubClassOf = self.add_property(&rdfs::subClassOf);
        self.rdfssubPropertyOf = self.add_property(&rdfs::subPropertyOf);
        self.rdfsSeeAlso = self.add_property(&rdfs::seeAlso);
        self.rdfsisDefinedBy = self.add_property(&rdfs::isDefinedBy);
        self.rdfsComment = self.add_property(&rdfs::comment);
        self.rdfsMember = self.add_property(&rdfs::member);
        self.rdfsContainerMembershipProperty =
            self.add_property(&rdfs::ContainerMembershipProperty);
        self.rdfsLabel = self.add_property(&rdfs::label);

        // -----------------RDF

        self.rdfList = self.add(&rdf::List);
        self.rdfAlt = self.add(&rdf::Alt);
        self.rdfBag = self.add(&rdf::Bag);
        self.rdfSeq = self.add(&rdf::Seq);
        self.rdfXMLLiteral = self.add(&rdf::XMLLiteral);
        self.rdfStatement = self.add(&rdf::Statement);
        self.rdfnil = self.add(&rdf::nil);

        self.rdfProperty = self.add_property(&rdf::Property);
        self.rdftype = self.add_property(&rdf::type_);
        self.rdfsubject = self.add_property(&rdf::subject);
        self.rdfobject = self.add_property(&rdf::object);
        self.rdfpredicate = self.add_property(&rdf::predicate);
        self.rdffirst = self.add_property(&rdf::first);
        self.rdfrest = self.add_property(&rdf::rest);
        self.rdfValue = self.add_property(&rdf::value);
        // TODO: add rdf1 to sophia
        self.rdf_1 = self.add_property(
            &sophia::ns::Namespace::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#")
                .unwrap()
                .get("_1")
                .unwrap(),
        );

        // ------------------XSD

        self.xsdnonNegativeInteger = self.add(&xsd::nonNegativeInteger);
        self.xsdstring = self.add(&xsd::string);

        // ------------------OWL

        self.owlthing = self.add_property(&owl::Thing);
        self.owltransitiveProperty = self.add_property(&owl::TransitiveProperty);
        self.owlequivalentClass = self.add_property(&owl::equivalentClass);
        self.owlequivalentProperty = self.add_property(&owl::equivalentProperty);
        self.owlobjectProperty = self.add_property(&owl::ObjectProperty);
        self.owldataTypeProperty = self.add_property(&owl::DatatypeProperty);
        self.owlsameAs = self.add_property(&owl::sameAs);

        self.owlinverseOf = self.add_property(&owl::inverseOf);
        self.owlpropertyDisjointWith = self.add_property(&owl::propertyDisjointWith);
        self.owldifferentFrom = self.add_property(&owl::differentFrom);
        self.owlallDifferent = self.add_property(&owl::AllDifferent);
        self.owlallDisjointClasses = self.add_property(&owl::AllDisjointClasses);
        self.owlallValuesFrom = self.add_property(&owl::allValuesFrom);
        self.owlannotationProperty = self.add_property(&owl::AnnotationProperty);
        self.owlassertionProperty = self.add_property(&owl::assertionProperty);
        self.owlclass = self.add(&owl::Class);
        self.owlcomplementOf = self.add_property(&owl::complementOf);
        self.owldisjoinWith = self.add_property(&owl::disjointWith);
        self.owldistinctmembers = self.add_property(&owl::distinctMembers);
        self.owlfunctionalProperty = self.add_property(&owl::FunctionalProperty);
        self.intersectionOf = self.add_property(&owl::intersectionOf);
        self.unionOf = self.add_property(&owl::unionOf);
        self.owlinverseFunctionalProperty = self.add_property(&owl::InverseFunctionalProperty);
        self.irreflexiveProperty = self.add_property(&owl::IrreflexiveProperty);
        self.maxCardinality = self.add_property(&owl::maxCardinality);
        self.members = self.add_property(&owl::members);
        self.nothing = self.add_property(&owl::Nothing);
        self.onClass = self.add_property(&owl::onClass);
        self.onProperty = self.add_property(&owl::onProperty);
        self.oneOf = self.add_property(&owl::oneOf);
        self.propertyChainAxiom = self.add_property(&owl::propertyChainAxiom);
        self.owlsomeValuesFrom = self.add_property(&owl::someValuesFrom);
        self.sourceIndividual = self.add_property(&owl::sourceIndividual);
        self.owlsymmetricProperty = self.add_property(&owl::SymmetricProperty);
        self.owltargetIndividual = self.add_property(&owl::targetIndividual);
        self.targetValue = self.add_property(&owl::targetValue);
        self.maxQualifiedCardinality = self.add_property(&owl::maxQualifiedCardinality);
    }
}
