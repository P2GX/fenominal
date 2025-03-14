use ontolius::TermId;

use super::hpo_concept::HpoConcept;



///  * This is the object that represents an actual match of an HPO concept with a part of the text and includes
/// the length of the longest stretch of words matched in the original order.
/// TODO -- not sure we need extra struct for this?
pub struct HpoConceptHit {
    hpo_concept: HpoConcept, 
    n_matching_words: usize,
}


impl HpoConceptHit {
    pub fn new(concept: HpoConcept, n_matches: usize) -> Self {
        HpoConceptHit {
            hpo_concept: concept,
            n_matching_words: n_matches
        }
    }

    pub fn get_term_id(&self) -> TermId {
        self.hpo_concept.get_hpo_id()
    } 
}