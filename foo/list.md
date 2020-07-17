# List of rules involving RDF Lists 

## Rules to detect inconsistent graphs

|  | If | Then | |
|---|---|---|---|
|eq-diff2| T(?x, rdf:type, owl:AllDifferent)<br> T(?x, owl:members,&nbsp;?y)<br> LIST[?y,&nbsp;?z<sub>1</sub>, ...,&nbsp;?z<sub>n</sub>]<br> T(?z<sub>i</sub>, owl:sameAs,&nbsp;?z<sub>j</sub>)| false | for each 1 ≤ i &lt; j ≤ n |
| eq-diff3 | T(?x, rdf:type, owl:AllDifferent)<br> T(?x, owl:distinctMembers,&nbsp;?y)<br> LIST[?y,&nbsp;?z<sub>1</sub>, ...,&nbsp;?z<sub>n</sub>]<br> T(?z<sub>i</sub>, owl:sameAs,&nbsp;?z<sub>j</sub>) | false | for each 1 ≤ i &lt; j ≤ n |
| prp-adp | T(?x, rdf:type, owl:AllDisjointProperties)<br> T(?x, owl:members,&nbsp;?y)<br> LIST[?y,&nbsp;?p<sub>1</sub>, ...,&nbsp;?p<sub>n</sub>]<br> T(?u,&nbsp;?p<sub>i</sub>,&nbsp;?v)<br> T(?u,&nbsp;?p<sub>j</sub>,&nbsp;?v) | false | for each 1 ≤ i &lt; j ≤ n |
| cax-adc | T(?x, rdf:type, owl:AllDisjointClasses)<br> T(?x, owl:members,&nbsp;?y)<br> LIST[?y,&nbsp;?c<sub>1</sub>, ...,&nbsp;?c<sub>n</sub>]<br> T(?z, rdf:type,&nbsp;?c<sub>i</sub>)<br> T(?z, rdf:type,&nbsp;?c<sub>j</sub>) | false | for each 1 ≤ i &lt; j ≤ n|

## Rules that create new triples

|  | If | Then | |
|---|---|---|---|
| prp-spo2 | T(?p, owl:propertyChainAxiom,&nbsp;?x)<br> LIST[?x,&nbsp;?p<sub>1</sub>, ...,&nbsp;?p<sub>n</sub>]<br> T(?u<sub>1</sub>,&nbsp;?p<sub>1</sub>,&nbsp;?u<sub>2</sub>)<br> T(?u<sub>2</sub>,&nbsp;?p<sub>2</sub>,&nbsp;?u<sub>3</sub>)<br> ...<br> T(?u<sub>n</sub>,&nbsp;?p<sub>n</sub>,&nbsp;?u<sub>n+1</sub>) | T(?u<sub>1</sub>,&nbsp;?p,&nbsp;?u<sub>n+1</sub>) |
| prp-key | T(?c, owl:hasKey,&nbsp;?u)<br> LIST[?u,&nbsp;?p<sub>1</sub>, ...,&nbsp;?p<sub>n</sub>]<br> T(?x, rdf:type,&nbsp;?c)<br> T(?x,&nbsp;?p<sub>1</sub>,&nbsp;?z<sub>1</sub>)<br> ...<br> T(?x,&nbsp;?p<sub>n</sub>,&nbsp;?z<sub>n</sub>)<br> T(?y, rdf:type,&nbsp;?c)<br> T(?y,&nbsp;?p<sub>1</sub>,&nbsp;?z<sub>1</sub>)<br> ...<br> T(?y,&nbsp;?p<sub>n</sub>,&nbsp;?z<sub>n</sub>) | T(?x, owl:sameAs,&nbsp;?y) |
| cls-int1 | T(?c, owl:intersectionOf,&nbsp;?x)<br> LIST[?x,&nbsp;?c<sub>1</sub>, ...,&nbsp;?c<sub>n</sub>]<br> T(?y, rdf:type,&nbsp;?c<sub>1</sub>)<br> T(?y, rdf:type,&nbsp;?c<sub>2</sub>)<br> ...<br> T(?y, rdf:type,&nbsp;?c<sub>n</sub>) | T(?y, rdf:type,&nbsp;?c) |
| cls-int2 | T(?c, owl:intersectionOf,&nbsp;?x)<br> LIST[?x,&nbsp;?c<sub>1</sub>, ...,&nbsp;?c<sub>n</sub>]<br> T(?y, rdf:type,&nbsp;?c) | T(?y, rdf:type,&nbsp;?c<sub>1</sub>)<br> T(?y, rdf:type,&nbsp;?c<sub>2</sub>)<br> ...<br> T(?y, rdf:type,&nbsp;?c<sub>n</sub>) |
| cls-uni | T(?c, owl:unionOf,&nbsp;?x)<br> LIST[?x,&nbsp;?c<sub>1</sub>, ...,&nbsp;?c<sub>n</sub>]<br> T(?y, rdf:type,&nbsp;?c<sub>i</sub>) | T(?y, rdf:type,&nbsp;?c) | for each 1 ≤ i ≤ n |
| cls-oo | T(?c, owl:oneOf,&nbsp;?x)<br> LIST[?x,&nbsp;?y<sub>1</sub>, ...,&nbsp;?y<sub>n</sub>] | T(?y<sub>1</sub>, rdf:type,&nbsp;?c)<br> ...<br> T(?y<sub>n</sub>, rdf:type,&nbsp;?c) |
