# ferriphene
Rust (ferric oxide) version of [fenominal](https://pubmed.ncbi.nlm.nih.gov/38001031/)




## Test drive

This project is in an early stage, but a simple run is possible and reveals the term Macrocephaly.
The other term is not captured because synonyms are not yet implemented by Ontoloius

```shell
 cargo run --bin fenominal_main  -- --hp /path/hp.json --input "intellectual disability (IQ 65), macrocephaly and dysmorphisms"
```

