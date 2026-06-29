use std::fmt;
use std::ops::Range;
use std::sync::Arc;


use crate::core_document::CoreDocument;
use crate::hpo::sentence_mapper::SentenceMapper;
use crate::simple_sentence::SimpleSentence;
use crate::{sanitize, sentence_split};
use ontolius::ontology::{HierarchyWalks, OntologyTerms};
use ontolius::term::{MinimalTerm, Synonymous};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// A sentence of the original text
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FenominalSentence { 
    /// Start offset of this sentence within the original source text.
    /// Units match `FenominalHit::span` (so hit spans and sentence
    /// start are directly comparable).
    pub start: usize,
    /// 0-based position of this sentence within the document
    /// (i.e. the 1st sentence has order 0, the 2nd has order 1, ...).
    pub order: usize,
    pub original_text: String,
    pub hits: Vec<FenominalHit>
}

impl FenominalSentence {
    pub fn new(
        order: usize, 
        start: usize, 
        original: impl Into<String>, 
        hits: Vec<FenominalHit>) -> Self {
        Self { 
            order, 
            start, 
            original_text: original.into(),
            hits 
        }
    }

    pub fn hits(&self) -> &[FenominalHit] {
        &self.hits
    }

    pub fn text_length(&self) -> usize {
        self.original_text.len()
    }
}

impl fmt::Display for FenominalSentence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Sentence #{} (start @ {}):", self.order, self.start)?;
        for hit in &self.hits {
            writeln!(f, "  {}", hit)?;
        }
        Ok(())
    }
}

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

    pub fn process(
        &self, 
        text: &str) -> Vec<FenominalHit> {
        self.map_text(text)
    }

    fn mine_sentence(&self, sentence: &str, order: usize, start: usize) -> FenominalSentence {
        let sentence_end = start + sentence.len() - 1;
        let ss = SimpleSentence::new(sentence, start, sentence_end);
         match self.sentence_mapper.map_sentence(&ss) {
                Ok(hits) => {
                    return FenominalSentence::new(order, start, sentence,hits);
                },
                Err(e) => {
                    // should rarely, if ever happen!
                    println!("Could not map: {}", e.to_ascii_lowercase());
                    return FenominalSentence::new(order, start, sentence,Vec::default());
                }
        }
    }

    pub fn mine_sentences(&self, text: &str) -> Vec<FenominalSentence> {
        let sanitized_text = sanitize(text);
        let sentences = sentence_split(&sanitized_text);
        let mut start = 0 as usize;
        let mut fenom_sent_list = Vec::with_capacity(sentences.len());
        for (i, s) in sentences.into_iter().enumerate() {
            let fsent = self.mine_sentence(&s, i, start);
            start += fsent.text_length() +1;
            fenom_sent_list.push(fsent);
        }
        fenom_sent_list
    }

}    




