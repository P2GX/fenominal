
use std::collections::{HashMap, HashSet};

use ontolius::{base::{Identified, TermId}, io::OntologyLoaderBuilder, ontology::csr::MinimalCsrOntology, prelude::{DescendantNodes, HierarchyAware, MinimalTerm, TermAware}};

use crate::fenominal_traits::TermIdToTextMapper;

/// Load the HPO 
/// The purpose of this struct is to create a Map with term labels and synonyms (Keys) and TermIds (value)
/// We keep the lower-case values of the String for ease of comparisons label on
pub struct HpoLoader {
    hpo: MinimalCsrOntology
}


impl HpoLoader {

    pub fn new(hpo_json_path: &str) -> Self {
        let loader = OntologyLoaderBuilder::new()
                .obographs_parser()
                .build();
        
        let hpo: MinimalCsrOntology = loader.load_from_path(hpo_json_path)
                                    .expect("HPO could not be loaded");
        HpoLoader {
            hpo: hpo
        }
    }

    pub fn from_ontology(hp: MinimalCsrOntology) -> Self {
        HpoLoader {
            hpo: hp
        }
    }
}



pub fn get_text_to_hpo_term_map(hpo: &MinimalCsrOntology)-> HashMap<String, TermId> {
    let mut test_to_tid_map = HashMap::new();
    // These are commmon false-positive results related to HPO synonyms that occur in other contexts
    let omittable_labels: HashSet<String> = ["negative", "weakness"]
                        .iter()
                        .map(|s| s.to_ascii_lowercase())
                        .collect();
    let min_synonym_length = 4;

    let PHENOTYPIC_ABNORMALITY: TermId = ("HP", "0000118").into();
    let idx = hpo.id_to_idx(&PHENOTYPIC_ABNORMALITY)
        .expect("Phenotypic abnormality not found in HPO");
    let hierarchy = hpo.hierarchy();
    let pheno_abn_terms: Vec<_> = hierarchy.iter_descendants_of(idx)
                    .flat_map(|tidx| hpo.idx_to_term(tidx))
                    .collect();
    for term in pheno_abn_terms {
        let term_id = term.identifier();
        let term_label_lc = term.name().to_ascii_lowercase();
        if omittable_labels.contains(&term_label_lc) || term_label_lc.len() < min_synonym_length {
            continue;
        }
        test_to_tid_map.insert(term_label_lc, term_id.clone());
        // TODO: Add parsing of synonyms -- need Ontolius upstream!
        // Remember not to add if length is less than min_synonym_length
        // Remember to skip the omittable_labels

    }
        
    

    test_to_tid_map
}



impl TermIdToTextMapper for HpoLoader {
    fn get_text_to_term_map(&self) -> HashMap<String, TermId> {
        let mut test_to_tid_map = HashMap::new();
        // These are commmon false-positive results related to HPO synonyms that occur in other contexts
        let omittable_labels: HashSet<&str> = ["negative", "weakness"].iter().cloned().collect();
        let min_synonym_length = 4;
   
        let PHENOTYPIC_ABNORMALITY: TermId = ("HP", "0000118").into();
        let idx = self.hpo.id_to_idx(&PHENOTYPIC_ABNORMALITY)
            .expect("Phenotypic abnormality not found in HPO");
        let hierarchy = self.hpo.hierarchy();
        let pheno_abn_terms: Vec<_> = hierarchy.iter_descendants_of(idx)
                        .flat_map(|tidx| self.hpo.idx_to_term(tidx))
                        .collect();
        for term in pheno_abn_terms {
            let term_id = term.identifier();
            test_to_tid_map.insert(term.name().to_ascii_lowercase(), term_id.clone());
            // TODO: Add parsing of synonyms -- need Ontolius upstream!
            // Remember not to add if length is less than min_synonym_length
            // Remember to skip the omittable_labels

        }
            
        

        test_to_tid_map
    
    }
    
    
}