| | If | Then | |
| -- | -- | -- | -- |
eq-ref | T(?s,?p,?o) | T(?s,owl:sameAs,?s) <br> T(?p,owl:sameAs,?p) <br> T(?o,owl:sameAs,?o) 
cls-svf2|T(?x,owl:someValuesFrom,owl:Thing) <br> T(?x,owl:onProperty,?p) <br> T(?u,?p,?v)|T(?u,rdf:type,?x)
cls-hv2|T(?x,owl:hasValue,?y) <br> T(?x,owl:onProperty,?p) <br> T(?u,?p,?y)|T(?u,rdf:type,?x)
cls-hv1|T(?x,owl:hasValue,?y) <br> T(?x,owl:onProperty,?p) <br> T(?u,rdf:type,?x)|T(?u,?p,?y)
cls-svf1|T(?x,owl:someValuesFrom,?y) <br> T(?x,owl:onProperty,?p) <br> T(?u,?p,?v) <br> T(?v,rdf:type,?y)|T(?u,rdf:type,?x)
cls-avf|T(?x,owl:allValuesFrom,?y) <br> T(?x,owl:onProperty,?p) <br> T(?u,rdf:type,?x) <br> T(?u,?p,?v)|T(?v,rdf:type,?y)
scm-svf1|T(?c1,owl:someValuesFrom,?y1) <br> T(?c1,owl:onProperty,?p) <br> T(?c2,owl:someValuesFrom,?y2) <br> T(?c2,owl:onProperty,?p) <br> T(?y1,rdfs:subClassOf,?y2)|T(?c1,rdfs:subClassOf,?c2)
scm-avf1|T(?c1,owl:allValuesFrom,?y1) <br> T(?c1,owl:onProperty,?p) <br> T(?c2,owl:allValuesFrom,?y2) <br> T(?c2,owl:onProperty,?p) <br> T(?y1,rdfs:subClassOf,?y2)|T(?c1,rdfs:subClassOf,?c2)
scm-hv|T(?c1,owl:hasValue,?i) <br> T(?c1,owl:onProperty,?p1) <br> T(?c2,owl:hasValue,?i) <br> T(?c2,owl:onProperty,?p2) <br> T(?p1,rdfs:subPropertyOf,?p2)|T(?c1,rdfs:subClassOf,?c2)
scm-svf2|T(?c1,owl:someValuesFrom,?y) <br> T(?c1,owl:onProperty,?p1) <br> T(?c2,owl:someValuesFrom,?y) <br> T(?c2,owl:onProperty,?p2) <br> T(?p1,rdfs:subPropertyOf,?p2)|T(?c1,rdfs:subClassOf,?c2)
scm-avf2|T(?c1,owl:allValuesFrom,?y) <br> T(?c1,owl:onProperty,?p1) <br> T(?c2,owl:allValuesFrom,?y) <br> T(?c2,owl:onProperty,?p2) <br> T(?p1,rdfs:subPropertyOf,?p2)|T(?c2,rdfs:subClassOf,?c1)
dt-type1| |T(dt,rdf:type,rdfs:Datatype)|for each datatype dt supported in OWL2 RL
dt-type2| |T(lt,rdf:type,dt)|for each literal lt and each datatype dt supported in OWL2 RL <br> such that the data value of lt is contained in the valuespace of dt
dt-eq| |T(lt1,owl:sameAs,lt2)|for all literals lt1 and lt2 with the same datavalue
dt-diff| |T(lt1,owl:differentFrom,lt2)|for all literals lt1 and lt2 with different datavalues

## Datatype
| | If | Then | | 
 |--|--|--|--|
cls-maxc1|T(?x,owl:maxCardinality,"0"^^xsd:nonNegativeInteger) <br> T(?x,owl:onProperty,?p) <br> T(?u,rdf:type,?x) <br> T(?u,?p,?y)|false
cls-maxc2|T(?x,owl:maxCardinality,"1"^^xsd:nonNegativeInteger) <br> T(?x,owl:onProperty,?p) <br> T(?u,rdf:type,?x) <br> T(?u,?p,?y1) <br> T(?u,?p,?y2)|T(?y1,owl:sameAs,?y2)
cls-maxqc1|T(?x,owl:maxQualifiedCardinality,"0"^^xsd:nonNegativeInteger) <br> T(?x,owl:onProperty,?p) <br> T(?x,owl:onClass,?c) <br> T(?u,rdf:type,?x) <br> T(?u,?p,?y) <br> T(?y,rdf:type,?c)|false
cls-maxqc2|T(?x,owl:maxQualifiedCardinality,"0"^^xsd:nonNegativeInteger) <br> T(?x,owl:onProperty,?p) <br> T(?x,owl:onClass,owl:Thing) <br> T(?u,rdf:type,?x) <br> T(?u,?p,?y)|false
cls-maxqc3|T(?x,owl:maxQualifiedCardinality,"1"^^xsd:nonNegativeInteger) <br> T(?x,owl:onProperty,?p) <br> T(?x,owl:onClass,?c) <br> T(?u,rdf:type,?x) <br> T(?u,?p,?y1) <br> T(?y1,rdf:type,?c) <br> T(?u,?p,?y2) <br> T(?y2,rdf:type,?c)|T(?y1,owl:sameAs,?y2)
cls-maxqc4|T(?x,owl:maxQualifiedCardinality,"1"^^xsd:nonNegativeInteger) <br> T(?x,owl:onProperty,?p) <br> T(?x,owl:onClass,owl:Thing) <br> T(?u,rdf:type,?x) <br> T(?u,?p,?y1) <br> T(?u,?p,?y2)|T(?y1,owl:sameAs,?y2)

## Axiomatic (I think)
| | If | Then | | 
 |--|--|--|--|
prp-ap| |T(ap,rdf:type,owl:AnnotationProperty)|foreach built-in annotation property of OWL2 RL
cls-thing| |T(owl:Thing,rdf:type,owl:Class)
cls-nothing1| |T(owl:Nothing,rdf:type,owl:Class)

## Invalid
| | If | Then | | 
 |--|--|--|--|
 cls-nothing2|T(?x,rdf:type,owl:Nothing)|false
cax-dw|T(?c1,owl:disjointWith,?c2) <br> T(?x,rdf:type,?c1) <br> T(?x,rdf:type,?c2)|false
eq-diff1|T(?x,owl:sameAs,?y) <br> T(?x,owl:differentFrom,?y)|false
prp-irp|T(?p,rdf:type,owl:IrreflexiveProperty) <br> T(?x,?p,?x)|false
prp-asyp|T(?p,rdf:type,owl:AsymmetricProperty) <br> T(?x,?p,?y) <br> T(?y,?p,?x)|false
prp-pdw|T(?p1,owl:propertyDisjointWith,?p2) <br> T(?x,?p1,?y) <br> T(?x,?p2,?y)|false
prp-npa1|T(?x,owl:sourceIndividual,?i1) <br> T(?x,owl:assertionProperty,?p) <br> T(?x,owl:targetIndividual,?i2) <br> T(?i1,?p,?i2)|false
prp-npa2|T(?x,owl:sourceIndividual,?i) <br> T(?x,owl:assertionProperty,?p) <br> T(?x,owl:targetValue,?lt) <br> T(?i,?p,?lt)|false
cls-com|T(?c1,owl:complementOf,?c2) <br> T(?x,rdf:type,?c1) <br> T(?x,rdf:type,?c2)|false
dt-not-type| T(lt,rdf:type,dt)|false|for each literal lt and each datatype dt supported in OWL2 RL <br> such that the datavalue of lt is not contained in the valuespace of dt