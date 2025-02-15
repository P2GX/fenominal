use std::collections::HashMap;

use ontolius::ontology::csr::{CsrOntology, MinimalCsrOntology};

use crate::fenominal_traits::{HpoMatcher, TermIdToTextMapper};

use super::{hpo_concept::HpoConcept, hpo_concept_hit::HpoConceptHit, hpo_concept_mapper::HpoConceptMapper, hpo_loader::HpoLoader};





pub struct DefaultHpoMapper {
    wordcount_to_matcher: HashMap<usize, HpoConceptMapper>,
}



impl DefaultHpoMapper {

    pub fn new(hpo: MinimalCsrOntology) -> Self {
        let loader = HpoLoader::from_ontology(hpo);
        let text_to_term_map = loader.get_text_to_term_map();
        let mut wc_map: HashMap<usize, HpoConceptMapper> = HashMap::new();
        for i in 1..=14 {
            wc_map.insert(i, HpoConceptMapper::new(i));
        }
        for (key, value) in &text_to_term_map {
            let concept = HpoConcept::new(key, value.clone());
            let n_tokens = concept.word_count();
            if n_tokens > 14 {
                panic!("Should never happen, term with more than 14 tokens - need to revise")
            }
            if let Some(cpt_mapper) = wc_map.get_mut(&n_tokens) {
                cpt_mapper.add_concept(&concept);
            }
        } 
        DefaultHpoMapper{
            wordcount_to_matcher: wc_map
        }
    }


    pub fn get_match(&self, tokens: Vec<String>) -> Option<HpoConceptHit> {
        if tokens.len() > 14 {
            println!("Malformed input vector: {:?}", tokens);
            return None;
        } else if tokens.is_empty() {
            println!("Empty input vector: {:?}", tokens);
            return None;
        } else {
            let matcher = self.wordcount_to_matcher.get(&tokens.len())?; 
            // TODO -- Figure out API -- should it be a reference to Vec<String> or a slice?
            let vec_of_str_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
            // Convert Vec<&str> to slice &[&str]
            let slice: &[&str] = &vec_of_str_refs;
            return matcher.get_match(slice);
        }
    }
    
}