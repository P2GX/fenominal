use std::{collections::HashMap, sync::Arc};

use ontolius::{
    ontology::{HierarchyWalks, OntologyTerms},
    term::{MinimalTerm, Synonymous},
    TermId,
};


use super::{
    hpo_concept::HpoConcept, hpo_concept_mapper::HpoConceptMapper,
    hpo_loader::get_text_to_hpo_term_map,
};

pub struct DefaultHpoMapper {
    wordcount_to_matcher: HashMap<usize, HpoConceptMapper>,
}

impl DefaultHpoMapper {
    ///  The HPO term with the longest label has 14 words. This will need to be updated if we introduce a term
    /// with a longer label in the future.
    pub const MAX_HPO_TERM_TOKEN_COUNT: usize = 14;

    pub fn new<O, T>(hpo: Arc<O>) -> Self
    where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous,
    {
        let text_to_term_map = get_text_to_hpo_term_map(hpo);
        DefaultHpoMapper::from_map(text_to_term_map.iter().map(|(k, v)| (k.as_ref(), v)))
    }

    /// Create an HpoMapper from text_to_tid_map
    ///
    /// # Arguments
    ///
    /// * `text_to_term_id` - An iterator with mapping from text to corresponding term ID.
    ///
    /// # Returns
    ///
    /// An HpoMapper object that is ready to use for text mining.
    pub fn from_map<'a, I>(text_to_term_id: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, &'a TermId)>,
    {
        let mut wc_map: HashMap<usize, HpoConceptMapper> = HashMap::new();
        for i in 1..=DefaultHpoMapper::MAX_HPO_TERM_TOKEN_COUNT {
            wc_map.insert(i, HpoConceptMapper::new(i));
        }
        for (key, value) in text_to_term_id {
            let concept = HpoConcept::new(key, value.clone());
            let n_tokens = concept.word_count();
            if n_tokens > 14 {
                panic!("Should never happen, term with more than 14 tokens - need to revise")
            }
            if let Some(cpt_mapper) = wc_map.get_mut(&n_tokens) {
                cpt_mapper.add_concept(&concept);
            }
        }
        DefaultHpoMapper {
            wordcount_to_matcher: wc_map,
        }
    }

    /// Search for an HPO term that matches an input string
    ///
    /// # Arguments
    ///
    /// * `tokens` - A listt of tokens (words) representing the input string
    ///
    /// # Returns
    ///
    /// An HpoConceptHit or None
    pub fn get_match(&self, tokens: &Vec<String>) -> Option<HpoConcept> {
        if tokens.len() > DefaultHpoMapper::MAX_HPO_TERM_TOKEN_COUNT {
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
