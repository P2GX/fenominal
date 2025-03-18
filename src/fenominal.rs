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
//! use flate2::bufread::GzDecoder;
//! use rfenominal::fenominal::Fenominal;
//! use ontolius::io::OntologyLoaderBuilder;
//! use ontolius::ontology::csr::FullCsrOntology;
//!
//! // Load HPO from the repo, use `flate2` to decompress on the fly
//! let hp_path = "data/hp.v2024-08-13.json.gz";
//! let loader = OntologyLoaderBuilder::new().obographs_parser().build();
//! let hpo = loader.load_from_read(
//!             GzDecoder::new(BufReader::new(File::open(hp_path).expect("HPO should be readable")))
//!           ).expect("HPO should be well formatted");
//!
//! // Configure Fenominal
//! let fenominal = Fenominal::from(&hpo);
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
//! # use std::fs::File;
//! # use std::io::BufReader;
//! # use flate2::bufread::GzDecoder;
//! # use rfenominal::fenominal::Fenominal;
//! # use ontolius::io::OntologyLoaderBuilder;
//! # use ontolius::ontology::csr::FullCsrOntology;
//! # 
//! # let hp_path = "data/hp.v2024-08-13.json.gz";
//! # let loader = OntologyLoaderBuilder::new().obographs_parser().build();
//! # let hpo = loader.load_from_read(
//! #             GzDecoder::new(BufReader::new(File::open(hp_path).expect("HPO should be readable")))
//! #           ).expect("HPO should be well formatted");
//! # 
//! # let fenominal = Fenominal::from(&hpo);
//! use rfenominal::{TextMiner, fenominal::FenominalHit};
//!
//! // Perform text mining
//! let text = "Intellectual disability, macrocephaly, scoliosis";
//! let hits: Vec<FenominalHit> = fenominal.process(&text);
//!
//! let labels: Vec<_> = hits.iter().map(|hit| &hit.label).collect();
//! assert_eq!(labels, &["Intellectual disability", "Macrocephaly", "Scoliosis"]);
//! ```
//!

use std::ops::Range;

use crate::mined_term::MinedTerm;
use crate::TextMiner;
use ontolius::ontology::OntologyTerms;
use ontolius::term::MinimalTerm;
use ontolius::{ontology::csr::FullCsrOntology, TermId};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::hpo::clinical_mapper::ClinicalMapper;

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
}

/// Fenominal text mining.
pub struct Fenominal<'a, O> {
    hpo: &'a O,
    clinical_mapper: ClinicalMapper,
}

impl<'a> From<&'a FullCsrOntology> for Fenominal<'a, FullCsrOntology> {
    fn from(value: &'a FullCsrOntology) -> Self {
        Self {
            hpo: value,
            clinical_mapper: ClinicalMapper::new(value),
        }
    }
}

/// Map an input text to HPO terms.
///
/// This implementation is appropriate for use cases where we want a set of unique terms
/// but do not care about their location in the original text.
impl<'a, O> TextMiner<TermId> for Fenominal<'a, O> {
    fn process(&self, text: &str) -> Vec<TermId> {
        self.clinical_mapper
            .map_text(text)
            .into_iter()
            .map(|mt| mt.term_id)
            .collect()
    }
}

/// Map an input text to [`FenominalHit`]s.
///
/// This implementation retains information about the hit coordinates
/// with respect to the source `text`.
impl TextMiner<FenominalHit> for Fenominal<'_, FullCsrOntology> {
    fn process(&self, text: &str) -> Vec<FenominalHit> {
        let mut hits = vec![];
        for mt in self.clinical_mapper.map_text(text) {
            match self.mined_term_to_hit(&mt) {
                Ok(fhit) => hits.push(fhit),
                Err(e) => println!("Could not map mined term {:?}", e),
            }
        }
        hits
    }
}

impl<'a, O> Fenominal<'a, O> {
    fn mined_term_to_hit<T>(&self, mined_term: &MinedTerm) -> Result<FenominalHit, String>
    where
        O: OntologyTerms<T>,
        T: MinimalTerm,
    {
        match self.hpo.term_by_id(mined_term.get_term_id()) {
            Some(term) => {
                return Ok(FenominalHit::new(
                    term.identifier().to_string(),
                    term.name(),
                    mined_term.get_span(),
                    mined_term.is_observed(),
                ));
            }
            None => Err(format!(
                "Could not retrieve term for {:?}.",
                mined_term.get_term_id()
            )),
        }
    }
}
