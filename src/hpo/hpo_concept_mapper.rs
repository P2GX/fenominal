use std::collections::{HashMap, HashSet};

use crate::fenominal_traits::HpoMatcher;

use super::{hpo_concept::HpoConcept, hpo_concept_hit::HpoConceptHit};

pub struct HpoConceptMapper {
    n_words: usize,
    component_token_to_concept_map: HashMap<String, Vec<HpoConcept>>,
}

impl HpoConceptMapper {
    pub fn new(n: usize) -> Self {
        let map: HashMap<String, Vec<HpoConcept>> = HashMap::new();
        HpoConceptMapper {
            n_words: n,
            component_token_to_concept_map: map,
        }
    }
}

impl HpoMatcher for HpoConceptMapper {
    /// Attempt to find a match to an HpoConcept
    ///
    /// Arguments   
    ///  list of lexical clusters mapped to the original text that have been preprocessed to remove stopwords

    fn get_match(&self, words: &[&str]) -> Option<HpoConceptHit> {
        let token_set: HashSet<String> = words.iter().map(|&s| s.to_string()).collect();

        for token in &token_set {
            let concept_list = self.component_token_to_concept_map.get(token);
            match concept_list {
                Some(clist) => {
                    for cpt in clist {
                        if cpt.non_stop_set_equal(&token_set) {
                            // We have a match!
                            return Some(HpoConceptHit::new(cpt.clone(), token_set.len()));
                        }
                    }
                }
                _ => {}
            }
        }
        None // if we get here, we have not matched anything
    }

    fn add_concept(&mut self, concept: &HpoConcept) {
        for token in concept.get_non_stop_words() {
            // insert a default value (empty vector) if the key is not present, then add the concept to the list
            self.component_token_to_concept_map
                .entry(token.clone())
                .or_insert(Vec::new())
                .push(concept.clone());
        }
    }
}
