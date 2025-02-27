//! # ClinicalMapper Module
//!
//! This module defines the `ClinicalMapper` struct, which is the struct that performs the text-mining of input texts.
//! See [`simple_hpo_parser`] for an example of how to construct a `ClinicalMapper` struct.
//!
//! [`simple_hpo_parser`]: ../simple_hpo_parser/index.html
//! 
//! // file:///Users/robin/GIT/ferriphene/target/doc/ferriphene/hpo/clinical_mapper/ferriphene.hpo.simple_hpo_parser.html
//! file:///Users/robin/GIT/ferriphene/target/doc/ferriphene/hpo/simple_hpo_parser/index.html
//! ## Example
//!
//! ```ignore
//! let mined_term_list = clinical_mappper.map_text(&input_string); 
//! for mt in mined_term_list {
//!     println!("{}", mt)}
//! }
//! ```

use std::collections::HashMap;
use ontolius::{base::TermId, ontology::csr::MinimalCsrOntology};
use crate::{core_document::CoreDocument, mined_term::MinedTerm};
use super::{default_hpo_mapper::DefaultHpoMapper, sentence_mapper::SentenceMapper};



pub struct ClinicalMapper {
    sentence_matcher: SentenceMapper
}



impl ClinicalMapper {

    pub fn new(hpo: &MinimalCsrOntology) -> Self {
        let default_hpo_mapper = DefaultHpoMapper::new(hpo);
        let smatcher = SentenceMapper::new(default_hpo_mapper);
        ClinicalMapper{
            sentence_matcher: smatcher
        }
    }




    pub fn from_map(text_to_tid_map: &HashMap<String, TermId>) -> Self {
        let default_hpo_mapper = DefaultHpoMapper::from_map(text_to_tid_map);
        let smatcher = SentenceMapper::new(default_hpo_mapper);
        ClinicalMapper{
            sentence_matcher: smatcher
        }

    }

    pub fn map_text(&self, text: &str) -> Vec<MinedTerm> {
        let core_document = CoreDocument::new(text);
        let sentences = core_document.get_sentences();
        let mut mapped_parts: Vec<MinedTerm> = Vec::new();
        for ss in sentences {
            match self.sentence_matcher.map_sentence(ss.get_tokens()) {
                Ok(sentence_parts) => mapped_parts.extend(sentence_parts),
                Err(e) => println!("Could not map: {}", e.to_ascii_lowercase())
            }
            
        }
        mapped_parts
    }

    pub fn map_text_to_json(&self, text: &str) -> String {
        let mined_terms = self.map_text(text);
        serde_json::to_string(&mined_terms).expect("Failed to serialize mined terms")
    }
    
}