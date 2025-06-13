//! # MinedTerm Module
//!
//! This module defines the `MinedTerm` struct, which contains all information about one text-mined HPO term
//!
//! The `ClinicalMapper`object returns a list of `MinedTerm` structs, see
//! [`simple_hpo_parser`]: ../hpo/clinical_mapper/index.html
//!
//! ## Example
//!
//! ```ignore
//! let mined_term_list = clinical_mappper.map_text(&input_string);
//! for mt in mined_term_list {
//!     println!("{}", mt);
//! }
//! ```

use std::{fmt::Display, ops::Range};

use ontolius::TermId;

use crate::simple_token::SimpleToken;

#[derive(Debug, Clone)]
pub struct MinedTerm {
    pub(crate) tokens: Vec<SimpleToken>,
    pub(crate) term_id: TermId,
    pub(crate) span: Range<usize>,
    pub(crate) matching_string: String,
    pub(crate) is_observed: bool,
}

impl MinedTerm {
    /// Create a MinedTerm object, representing a "hit"
    ///
    /// We transform the TermId object into a string for serialization via serde
    pub fn new(
        tokens: Vec<SimpleToken>,
        term_id: TermId,
        span: Range<usize>,
        matching: impl Into<String>,
        is_observed: bool,
    ) -> Self {
        MinedTerm {
            tokens,
            term_id,
            span: span,
            matching_string: matching.into(),
            is_observed,
        }
    }

    pub fn get_span(&self) -> Range<usize> {
        Clone::clone(&self.span)
    }

    pub fn get_term_id(&self) -> &TermId {
        &self.term_id
    }

    pub fn is_observed(&self) -> bool {
        self.is_observed
    }
}

impl Display for MinedTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MinedTerm(id: {} [{}-{}]-observed {})",
            self.term_id.to_string(),
            &self.span.start,
            &self.span.end,
            self.is_observed
        )
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_ctor() {
        let term_id: TermId = "HP:0001250".parse().unwrap();
        assert_eq!("HP:0001250", term_id.to_string());
    }
}
