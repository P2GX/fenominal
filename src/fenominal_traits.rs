use std::collections::HashMap;

use ontolius::base::TermId;

use crate::simple_token::SimpleToken;
use crate::{mined_term::MinedTerm, simple_sentence::SimpleSentence};
use crate::hpo::hpo_concept_hit::HpoConceptHit;
use crate::hpo::hpo_concept::HpoConcept;


/// A trait that is implemented by code that identify `MinedTerm` instances from an input sentence
pub trait SentenceMapper {
    /// Identify `MinedTerm` instances from an input sentence
    ///
    /// Input sentences (text) are first tranformed into SimpleSentance instances that contain lists of Tokens (words) 
    /// and other information.
    /// # Example
    ///
    /// ```ignore
    /// use crate::core_document;
    /// let text = "Individual A had arachnodactyly and scoliosis."
    /// let core_document = CoreDocument::new(text); 
    /// let sentences = core_document.get_sentences();
    /// let mut mapped_parts: Vec<MinedTerm> = Vec::new();
    /// for ss in sentences {
    ///     let sentence_parts = self.sentence_matcher.map_sentence(ss.get_tokens());
    ///     mapped_parts.extend(sentence_parts);
    /// }
    /// ```
    fn map_sentence(&self, ss: &SimpleSentence) -> Vec<MinedTerm>;
    /// return true if the sentence has a negation word 
    fn has_negation(&self,  tokens: &[SimpleToken]) -> bool;
}

/// A trait that is implemented by objects that are returned from text mining and have information about the hit
pub trait TermMetaData {
    fn get_matching_string() -> String;
    fn get_term_id() -> TermId;
    fn get_token_count() -> usize;
}

/// A trait that is implement by structs that take the hp.json file and create a map with keys (labels, synoynms) and values (TermIds)
pub trait TermIdToTextMapper {
    fn get_text_to_term_map(&self) -> HashMap<String, TermId>;
}

/// A trait that is implemented by code that checks for matching HPO terms starting from slices of tokens (parts of sentences)
pub trait HpoMatcher  {
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tokens: Vec<String> = vec!["Individual", "A", "had", "arachnodactyly", "and", "scoliosis."]; 
    /// let vec_of_str_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    /// let slice: &[&str] = &vec_of_str_refs;
    /// let matcher = self.wordcount_to_matcher.get(&tokens.len())?; 
    /// let hit:  Option<HpoConceptHit> = matcher.get_match(slice);
    /// ```
  fn get_match(&self, words: &[&str]) -> Option<HpoConceptHit>;
  fn add_concept(&mut self, concept: &HpoConcept);
}

