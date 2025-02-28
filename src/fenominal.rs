//! This module is the main entry point.
//! 
//! There are two public functions. One gets a vector of FenominalHit object, and then other gets the corresponding JSON
//! 
//! # Examples
//! ```ignore
//! let hp_json_path = "/some/path/hp.json";
//! let input_string = 'Intellectual disability, macrocephaly, scoliosis'`;
//! let fenominal = Fenominal::new(hp_json_path_str);
//! let json = fenominal.map_text_to_json(&input_string);
//! ```
//! 

use ontolius::{io::OntologyLoaderBuilder, ontology::csr::MinimalCsrOntology, prelude::{MinimalTerm, TermAware}};
use serde::Serialize;
use crate::mined_term::MinedTerm;

use crate::hpo::clinical_mapper::ClinicalMapper;


#[derive(Serialize)]
pub struct FenominalHit {
    term_id: String,
    term_label: String,
    start_pos: usize, 
    end_pos: usize,
    is_observed: bool
}


impl FenominalHit {
    pub fn new(tid: String, label: &str, start: usize, end: usize, observed: bool) -> Self {
        Self { 
            term_id: tid, 
            term_label: label.to_string(), 
            start_pos: start, 
            end_pos: end, 
            is_observed: observed
        }
    }
}



pub struct Fenominal {

    hpo: MinimalCsrOntology,
    clinical_mapper: ClinicalMapper,


}




impl Fenominal {
    pub fn new(hp_json_path: &str) -> Self {
        let loader = OntologyLoaderBuilder::new()
            .obographs_parser()
            .build();
        println!("Processing file: {:?}", hp_json_path);
        let hpo: MinimalCsrOntology = loader.load_from_path(hp_json_path)
                            .expect("HPO could not be loaded");
        let clinical_mapper = ClinicalMapper::new(&hpo);
        Self {
            hpo:hpo,
            clinical_mapper: clinical_mapper
        }
    }


    fn mined_term_to_hit(&self, mined_term: &MinedTerm) -> Result<FenominalHit, String> {
        let tid = mined_term.get_term_id();
        let term_opt = self.hpo.id_to_term(&tid);
        match term_opt {
            Some(term) => {
                let label = term.name();
                let hit = FenominalHit::new(tid.to_string(), label, mined_term.get_start_pos(), mined_term.get_end_pos(), mined_term.is_observed());
                return Ok(hit);
            },
            None => Err(format!("Could not retrieve term for {}.", tid.to_string()))
        }

    }

    pub fn map_text_to_json(&self, input_text: &str) -> String {
        let fenominal_hits = self.map_text(input_text);
        let json_string = serde_json::to_string(&fenominal_hits).unwrap();
        json_string
    }


    pub fn map_text(&self, input_text: &str) -> Vec<FenominalHit> {
        let mut hits = vec![];
        let mined_terms = self.clinical_mapper.map_text(input_text);
        for mt in mined_terms {
            let hit = self.mined_term_to_hit(&mt);
            match hit {
                Ok(fhit) => hits.push(fhit),
                Err(e) => println!("Could not map mined term {:?}", e)
            }
        }
        hits
    }

}