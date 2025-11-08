use std::fmt;
use std::ops::Range;
use std::sync::Arc;


use crate::core_document::CoreDocument;
use crate::hpo::sentence_mapper::SentenceMapper;
use ontolius::ontology::{HierarchyWalks, OntologyTerms};
use ontolius::term::{MinimalTerm, Synonymous};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// A named entity identified by text mining.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FenominalHit {
    /// The entity's term ID.
    pub term_id: String,
    /// The entity's label.
    pub label: String,
    /// The coordinates of the entity within the source text.
    pub span: Range<usize>,
    /// The observation status (present/excluded).
    pub is_observed: bool,
}

impl FenominalHit {
    pub fn new(term_id: String, label: &str, span: Range<usize>, is_observed: bool) -> Self {
        Self {
            term_id,
            label: label.to_string(),
            span,
            is_observed,
        }
    }

    /// get the start/end position of a 'Hit'
    pub fn get_span(&self) -> Range<usize> {
        Clone::clone(&self.span)
    }
}


impl fmt::Display for FenominalHit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} [{}] @ {}..{} ({})",
            self.label,
            self.term_id,
            self.span.start,
            self.span.end,
            if self.is_observed { "observed" } else { "excluded" }
        )
    }
}

/// Fenominal text mining.
pub struct Fenominal<O, T> where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous  {
    sentence_mapper: SentenceMapper<O,T>,
}

impl<O, T> Fenominal<O, T> 
    where
    O: OntologyTerms<T> + HierarchyWalks,
    T: MinimalTerm + Synonymous  
    {

    pub fn new(hpo: Arc<O>)-> Self {
        let hpo_arc = Arc::clone(&hpo);
        Self {
            sentence_mapper: SentenceMapper::new(hpo_arc),
        }
    }

    pub fn map_text(&self, text: &str) -> Vec<FenominalHit> {
        let core_document = CoreDocument::new(text);
        let sentences = core_document.get_sentences();
        let mut mapped_parts: Vec<FenominalHit> = Vec::new();
        for ss in sentences {
            match self.sentence_mapper.map_sentence(ss) {
                Ok(sentence_parts) => mapped_parts.extend(sentence_parts),
                Err(e) => println!("Could not map: {}", e.to_ascii_lowercase()),
            }
        }
        mapped_parts
    }

    pub fn process(&self, text: &str) -> Vec<FenominalHit> {
        self.map_text(text)
    }
}    


