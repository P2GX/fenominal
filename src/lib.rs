

mod core_document;
mod fenominal_traits;
mod hpo;
mod mined_term;
mod simple_sentence;
mod simple_token;
mod stopwords;
pub mod fenominal;


use std::collections::HashMap;

use ontolius::{base::TermId, io::OntologyLoaderBuilder, ontology::csr::MinimalCsrOntology};


pub fn load_hpo(hp_json_path: &str) -> Result<MinimalCsrOntology, String> {
    let loader = OntologyLoaderBuilder::new()
        .obographs_parser()
        .build();

    let hpo: MinimalCsrOntology = loader.load_from_path(hp_json_path)
        .expect("HPO could not be loaded");
    Ok(hpo)
}

pub fn get_text_to_term_map(hpo: MinimalCsrOntology) -> HashMap<String, TermId> {
    return hpo::hpo_loader::get_text_to_hpo_term_map(&hpo);
}
