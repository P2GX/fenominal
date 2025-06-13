Fenominal: Phenomenal text mining for disease and phenotype concepts 
====================================================================

Rust implementation of the exact-matching algorithm from [fenominal](https://pubmed.ncbi.nlm.nih.gov/38001031/). Fenomimal implements mining of 
[Human Phenotype Ontology (HPO)](https://pubmed.ncbi.nlm.nih.gov/37953324/) terms from clinical texts.




## Test drive

This project is in an early stage, but a simple run is possible and reveals the term Macrocephaly.
The other term is not captured because synonyms are not yet implemented by Ontoloius

```shell
 cargo run --bin fenominal_main  -- --hp /path/hp.json --input "intellectual disability (IQ 65), macrocephaly and dysmorphisms"
```
This will show the identified HPO terms and also will return a JSON string that could be adapted for use in a front end.
