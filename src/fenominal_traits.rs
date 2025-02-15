use std::{collections::HashMap, iter::Map};

use ontolius::base::TermId;

use crate::{mined_term::MinedTerm, simple_sentence::SimpleSentence};
use crate::{hpo::hpo_concept_hit::HpoConceptHit};
use crate::{hpo::hpo_concept::HpoConcept};


pub trait SentenceMapper {
    fn map_sentence(&self, ss: &SimpleSentence) -> Vec<MinedTerm>;
}


pub trait TermMetaData {
    fn get_matching_string() -> String;
    fn get_term_id() -> TermId;
    fn get_token_count() -> usize;
}

pub trait TermIdToTextMapper {
    fn get_text_to_term_map(&self) -> HashMap<String, TermId>;
}

pub trait HpoMatcher  {
  fn get_match(&self, words: &[&str]) -> Option<HpoConceptHit>;
  fn add_concept(&mut self, concept: &HpoConcept);
}

