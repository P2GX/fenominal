//! Implementation of the Fenominal text mining algorithm.
//!
//! ## Configure Fenominal
//!
//! [`Fenominal`] is created from [`ontolius::ontology::csr::FullCsrOntology`],
//! which can, in turn, be loaded from a HPO JSON file:
//!
//! ```
//! use std::fs::File;
//! use std::io::BufReader;
//! use std::sync::Arc;
//! use flate2::bufread::GzDecoder;
//! use fenominal::fenominal::Fenominal;
//! use ontolius::io::OntologyLoaderBuilder;
//! use ontolius::ontology::csr::FullCsrOntology;
//!
//! // Load HPO from the repo, use `flate2` to decompress on the fly
//! let hp_path = "resources/hp.v2025-03-03.json.gz";
//! let loader = OntologyLoaderBuilder::new().obographs_parser().build();
//! let hpo: FullCsrOntology = loader.load_from_read(
//!             GzDecoder::new(BufReader::new(File::open(hp_path).expect("HPO should be readable")))
//!           ).expect("HPO should be well formatted");
//! let hpo = Arc::new(hpo);
//! // Configure Fenominal
//! let fenominal = Fenominal::new(hpo);
//! ```
//!
//! ## Use Fenominal
//! 
//! There are two implementations of [`TextMiner`] trait,
//! one for mining [`FenominalHit`]s and the other for getting [`TermId`]s.
//! 
//!
//! ### Example
//! 
//! Get [`FenominalHit`]s for an example text:
//! 
//! ```
//! use std::fs::File;
//! use std::io::BufReader;
//! use std::sync::Arc;
//! use flate2::bufread::GzDecoder;
//! use fenominal::fenominal::Fenominal;
//! use ontolius::io::OntologyLoaderBuilder;
//! use ontolius::ontology::csr::FullCsrOntology;
//! let hp_path = "resources/hp.v2025-03-03.json.gz";
//! let loader = OntologyLoaderBuilder::new().obographs_parser().build();
//! let hpo: FullCsrOntology = loader.load_from_read(
//!              GzDecoder::new(BufReader::new(File::open(hp_path).expect("HPO should be readable")))
//!            ).expect("HPO should be well formatted");
//! let hpo = Arc::new(hpo);
//! let fenominal = Fenominal::new(hpo);
//! use fenominal::fenominal::FenominalHit;
//!
//! // Perform text mining
//! let text = "Intellectual disability, macrocephaly, scoliosis";
//! let hits: Vec<FenominalHit> = fenominal.process(&text);
//!
//! let labels: Vec<_> = hits.iter().map(|hit| &hit.label).collect();
//! assert_eq!(labels, &["Intellectual disability", "Macrocephaly", "Scoliosis"]);
//! ```
//!

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


