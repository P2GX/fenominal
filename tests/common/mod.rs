use rstest::fixture;
use ontolius::{io::OntologyLoaderBuilder, ontology::csr::FullCsrOntology};
use std::{collections::HashMap, fs::File, io::BufReader, sync::Arc};
use flate2::bufread::GzDecoder;
use std::{str::FromStr, vec};
use ontolius::{term::Term, TermId};




#[fixture]
pub fn hpo() -> Arc<FullCsrOntology> {
    let path = "resources/hp.v2025-03-03.json.gz";
    let reader = GzDecoder::new(BufReader::new(File::open(path).unwrap()));
    let loader = OntologyLoaderBuilder::new().obographs_parser().build();
    let hpo = loader.load_from_read(reader).unwrap();
    let hpo = Arc::new(hpo);
    hpo
}


// region:    --- Tests

