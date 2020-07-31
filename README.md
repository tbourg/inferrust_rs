# InferRust
This crate aims to provide an implementation of `Inferray` in Rust, based on `sophia`.

---------------
## Citation
```
Julien Subercaze, Christophe Gravier, Jules Chevalier, Frédérique Laforest:
Inferray: fast in-memory RDF inference. PVLDB 9(6): 468-479 (2016)
```

-----------------------
## Authors
- Thomas Bourg (core)
- Pierre-Antoine Champin (ideas and improvements)

-----------------------------
## TO DO 
- [ ] Support full OWL-RL
    - [x] Support list
    - [ ] Implement new rules
        - [x] On list
        - [ ] Others ([list](foo/owl-rl.md))
            - [ ] eq-ref
            - [x] cls-svf2
            - [ ] cls-hv2
            - [ ] cls-hv1
            - [ ] cls-svf1
            - [ ] cls-avf
            - [ ] scm-svf1
            - [ ] scm-avf1
            - [ ] scm-hv
            - [ ] scm-svf2
            - [ ] scm-avf2
            - [ ] dt-type1
            - [ ] dt-type2
            - [ ] dt-eq
            - [ ] dt-diff
            - [ ] cls-maxc1
            - [ ] cls-maxc2
            - [ ] cls-maxqc1
            - [ ] cls-maxqc2
            - [ ] cls-maxqc3
            - [ ] cls-maxqc4
            - [ ] prp-ap
            - [ ] cls-thing
            - [ ] cls-nothing1
            - [x] cls-nothing2
            - [x] cax-dw
            - [x] eq-diff1
            - [x] prp-irp
            - [ ] prp-asyp
            - [x] prp-pdw
            - [x] prp-npa1
            - [x] prp-npa2
            - [x] cls-com
            - [ ] dt-not-type