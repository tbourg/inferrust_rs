#![allow(non_snake_case)]

use sophia::ns::*;
use sophia::term::factory::{RcTermFactory, TermFactory};
use sophia::term::{RcTerm, Term, TermData};

use std::convert::TryInto;

use bimap::hash::BiHashMap;

use super::TripleStore;

pub struct NodeDictionary {
    res_ctr: i64,
    prop_ctr: i32,
    removed_val: Vec<i64>,
    resources: BiHashMap<RcTerm, i64>,
    properties: BiHashMap<RcTerm, i32>,
    pub ts: TripleStore,
    factory: RcTermFactory,
    pub(crate) rdfsResource: i64,
    pub(crate) rdfsClass: i64,
    pub(crate) rdfsDatatype: i64,
    pub(crate) rdfsLiteral: i64,
    pub(crate) rdfsContainer: i64,
    pub(crate) rdfsdomain: i32,
    pub(crate) rdfsrange: i32,
    pub(crate) rdfssubClassOf: i32,
    pub(crate) rdfssubPropertyOf: i32,
    pub(crate) rdfsSeeAlso: i32,
    pub(crate) rdfsisDefinedBy: i32,
    pub(crate) rdfsComment: i32,
    pub(crate) rdfsMember: i32,
    pub(crate) rdfsContainerMembershipProperty: i32,
    pub(crate) rdfsLabel: i32,
    pub(crate) rdfList: i64,
    pub(crate) rdfAlt: i64,
    pub(crate) rdfBag: i64,
    pub(crate) rdfSeq: i64,
    pub(crate) rdfXMLLiteral: i64,
    pub(crate) rdfStatement: i64,
    pub(crate) rdfnil: i64,
    pub(crate) rdfProperty: i32,
    pub(crate) rdftype: i32,
    pub(crate) rdfsubject: i32,
    pub(crate) rdfobject: i32,
    pub(crate) rdfpredicate: i32,
    pub(crate) rdffirst: i32,
    pub(crate) rdfrest: i32,
    pub(crate) rdfValue: i32,
    pub(crate) xsdnonNegativeInteger: i64,
    pub(crate) xsdstring: i64,
    pub(crate) owlthing: i32,
    pub(crate) owltransitiveProperty: i32,
    pub(crate) owlequivalentClass: i32,
    pub(crate) owlequivalentProperty: i32,
    pub(crate) owlobjectProperty: i32,
    pub(crate) owldataTypeProperty: i32,
    pub(crate) owlsameAs: i32,
    pub(crate) owlinverseOf: i32,
    pub(crate) owlpropertyDisjointWith: i32,
    pub(crate) owldifferentFrom: i32,
    pub(crate) owlallDifferent: i32,
    pub(crate) owlallDisjointClasses: i32,
    pub(crate) owlallValuesFrom: i32,
    pub(crate) owlannotationProperty: i32,
    pub(crate) owlassertionProperty: i32,
    pub(crate) owlclass: i64,
    pub(crate) owlcomplementOf: i32,
    pub(crate) owldisjoinWith: i32,
    pub(crate) owldistinctmembers: i32,
    pub(crate) owlfunctionalProperty: i32,
    pub(crate) intersectionOf: i32,
    pub(crate) unionOf: i32,
    pub(crate) owlinverseFunctionalProperty: i32,
    pub(crate) irreflexiveProperty: i32,
    pub(crate) maxCardinality: i32,
    pub(crate) members: i32,
    pub(crate) nothing: i32,
    pub(crate) onClass: i32,
    pub(crate) onProperty: i32,
    pub(crate) oneOf: i32,
    pub(crate) propertyChainAxiom: i32,
    pub(crate) owlsomeValuesFrom: i32,
    pub(crate) sourceIndividual: i32,
    pub(crate) owlsymetricProperty: i32,
    pub(crate) owltargetIndividual: i32,
    pub(crate) targetValue: i32,
    pub(crate) maxQualifiedCardinality: i32,
}

impl NodeDictionary {
    const START_INDEX: i32 = i32::max_value();

    pub(crate) fn new(ts: TripleStore) -> Self {
        let mut me = Self {
            res_ctr: Self::START_INDEX as i64,
            prop_ctr: Self::START_INDEX,
            removed_val: vec![],
            resources: BiHashMap::<RcTerm, i64>::new(),
            properties: BiHashMap::<RcTerm, i32>::new(),
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
            owlsymetricProperty: 0,
            owltargetIndividual: 0,
            targetValue: 0,
            maxQualifiedCardinality: 0,
        };
        me.init_const();
        me
    }

    pub(crate) fn add(&mut self, str: &str) -> i64 {
        let term = self.factory.iri(str).expect("Err");
        self.add_term(term)
    }

    pub(crate) fn add_property(&mut self, str: &str) -> i32 {
        let term = self.factory.iri(str).expect("Err");
        self.add_property_term(term)
    }

    fn add_term(&mut self, t: RcTerm) -> i64 {
        if self.properties.contains_left(&t) {
            return *self.properties.get_by_left(&t).expect("Err") as i64;
        }
        if self.resources.contains_left(&t) {
            *self.resources.get_by_left(&t).expect("Err")
        } else {
            self.res_ctr += 1;
            self.resources.insert(t, self.res_ctr);
            self.res_ctr
        }
    }

    fn add_property_term(&mut self, t: RcTerm) -> i32 {
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

    fn remap_res_to_prop(&mut self, t: RcTerm) -> i32 {
        let old = self.resources.remove_by_left(&t).expect("Err").1;
        self.prop_ctr -= 1;
        let p = self.prop_ctr;
        self.properties.insert(t, p);
        self.removed_val.push(old);
        self.ts.res_to_prop(old, p);
        p
    }

    pub(crate) fn get_term(&self, index: i64) -> &RcTerm {
        if index < Self::START_INDEX as i64 {
            self.properties
                .get_by_right(&(index as i32))
                .expect(&format!("No such properties {}", index))
        } else {
            self.resources
                .get_by_right(&index)
                .expect("No such ressources")
        }
    }

    pub(crate) fn get_index<T>(&self, t: &Term<T>) -> Option<i64>
    where
        T: TermData,
    {
        let inner_term = RcTerm::new_iri(t.value()).unwrap();
        if self.properties.contains_left(&inner_term) {
            Some(*self.properties.get_by_left(&inner_term).unwrap() as i64)
        } else if self.resources.contains_left(&inner_term) {
            Some(*self.resources.get_by_left(&inner_term).unwrap())
        } else {
            None
        }
    }

    pub(crate) fn prop_idx_to_idx(prop_idx: i64) -> usize {
        (Self::START_INDEX as i64 - prop_idx - 1)
            .try_into()
            .expect("Err converting index")
    }

    pub(crate) fn idx_to_prop_idx(idx: usize) -> i64 {
        Self::START_INDEX as i64 - idx as i64 - 1
    }

    fn init_const(&mut self) {
        // ---------------RDFS
        self.rdfsResource = self.add(&rdfs::Resource.value());
        self.rdfsClass = self.add(&rdfs::Class.value());
        self.rdfsDatatype = self.add(&rdfs::Datatype.value());
        self.rdfsLiteral = self.add(&rdfs::Literal.value());
        self.rdfsContainer = self.add(&rdfs::Container.value());

        self.rdfsdomain = self.add_property(&rdfs::domain.value());
        self.rdfsrange = self.add_property(&rdfs::range.value());
        self.rdfssubClassOf = self.add_property(&rdfs::subClassOf.value());
        self.rdfssubPropertyOf = self.add_property(&rdfs::subPropertyOf.value());
        self.rdfsSeeAlso = self.add_property(&rdfs::seeAlso.value());
        self.rdfsisDefinedBy = self.add_property(&rdfs::isDefinedBy.value());
        self.rdfsComment = self.add_property(&rdfs::comment.value());
        self.rdfsMember = self.add_property(&rdfs::member.value());
        self.rdfsContainerMembershipProperty =
            self.add_property(&rdfs::ContainerMembershipProperty.value());
        self.rdfsLabel = self.add_property(&rdfs::label.value());

        // -----------------RDF

        self.rdfList = self.add(&rdf::List.value());
        self.rdfAlt = self.add(&rdf::Alt.value());
        self.rdfBag = self.add(&rdf::Bag.value());
        self.rdfSeq = self.add(&rdf::Seq.value());
        self.rdfXMLLiteral = self.add(&rdf::XMLLiteral.value());
        self.rdfStatement = self.add(&rdf::Statement.value());
        self.rdfnil = self.add(&rdf::nil.value());

        self.rdfProperty = self.add_property(&rdf::Property.value());
        self.rdftype = self.add_property(&rdf::type_.value());
        self.rdfsubject = self.add_property(&rdf::subject.value());
        self.rdfobject = self.add_property(&rdf::object.value());
        self.rdfpredicate = self.add_property(&rdf::predicate.value());
        self.rdffirst = self.add_property(&rdf::first.value());
        self.rdfrest = self.add_property(&rdf::rest.value());
        self.rdfValue = self.add_property(&rdf::value.value());

        // ------------------XSD

        self.xsdnonNegativeInteger = self.add(&xsd::nonNegativeInteger.value());
        self.xsdstring = self.add(&xsd::string.value());

        // ------------------OWL

        self.owlthing = self.add_property(&owl::Thing.value());
        self.owltransitiveProperty = self.add_property(&owl::TransitiveProperty.value());
        self.owlequivalentClass = self.add_property(&owl::equivalentClass.value());
        self.owlequivalentProperty = self.add_property(&owl::equivalentProperty.value());
        self.owlobjectProperty = self.add_property(&owl::ObjectProperty.value());
        self.owldataTypeProperty = self.add_property(&owl::DatatypeProperty.value());
        self.owlsameAs = self.add_property(&owl::sameAs.value());

        self.owlinverseOf = self.add_property(&owl::inverseOf.value());
        self.owlpropertyDisjointWith = self.add_property(&owl::propertyDisjointWith.value());
        self.owldifferentFrom = self.add_property(&owl::differentFrom.value());
        self.owlallDifferent = self.add_property(&owl::AllDifferent.value());
        self.owlallDisjointClasses = self.add_property(&owl::AllDisjointClasses.value());
        self.owlallValuesFrom = self.add_property(&owl::allValuesFrom.value());
        self.owlannotationProperty = self.add_property(&owl::AnnotationProperty.value());
        self.owlassertionProperty = self.add_property(&owl::assertionProperty.value());
        self.owlclass = self.add(&owl::Class.value());
        self.owlcomplementOf = self.add_property(&owl::complementOf.value());
        self.owldisjoinWith = self.add_property(&owl::disjointWith.value());
        self.owldistinctmembers = self.add_property(&owl::distinctMembers.value());
        self.owlfunctionalProperty = self.add_property(&owl::FunctionalProperty.value());
        self.intersectionOf = self.add_property(&owl::intersectionOf.value());
        self.unionOf = self.add_property(&owl::unionOf.value());
        self.owlinverseFunctionalProperty =
            self.add_property(&owl::InverseFunctionalProperty.value());
        self.irreflexiveProperty = self.add_property(&owl::IrreflexiveProperty.value());
        self.maxCardinality = self.add_property(&owl::maxCardinality.value());
        self.members = self.add_property(&owl::members.value());
        self.nothing = self.add_property(&owl::Nothing.value());
        self.onClass = self.add_property(&owl::onClass.value());
        self.onProperty = self.add_property(&owl::onProperty.value());
        self.oneOf = self.add_property(&owl::oneOf.value());
        self.propertyChainAxiom = self.add_property(&owl::propertyChainAxiom.value());
        self.owlsomeValuesFrom = self.add_property(&owl::someValuesFrom.value());
        self.sourceIndividual = self.add_property(&owl::sourceIndividual.value());
        self.owlsymetricProperty = self.add_property(&owl::SymmetricProperty.value());
        self.owltargetIndividual = self.add_property(&owl::targetIndividual.value());
        self.targetValue = self.add_property(&owl::targetValue.value());
        self.maxQualifiedCardinality = self.add_property(&owl::maxQualifiedCardinality.value());
    }
}
