//! # MinedTerm Module
//!
//! This module defines the `MinedTerm` struct, which contains all information about one text-mined HPO term 
//!
//! The `ClinicalMapper`object returns a list of `MinedTerm` structs, see
//! [`simple_hpo_parser`]: ../hpo/clinical_mapper/index.html
//! 
//! ## Example
//!
//! ```no_run
//! let mined_term_list = clinical_mappper.map_text(&input_string); 
//! for mt in mined_term_list {
//!     println!("{}", mt)}
//! }
//! ```
use std::str::FromStr;

use ontolius::base::TermId;
use serde::{Serialize, Deserialize};
use serde_json::Serializer;
use crate::simple_token::SimpleToken;




#[derive(Clone, Serialize, Deserialize)]
pub struct MinedTerm {
    tokens: Vec<SimpleToken>,
    term_id: String,
    start_pos: usize,
    end_pos: usize,
    matching_string: String,
    is_observed: bool
}

impl MinedTerm {
    /// Create a MinedTerm object, representing a "hit"
    /// 
    /// We transform the TermId object into a string for serialization via serde
    pub fn new(tokens: Vec<SimpleToken>,
                tid: TermId,
                start: usize,
                end: usize,
                matching: &str,
                observed: bool) -> Self {
            MinedTerm {
                tokens: tokens,
                term_id: tid.to_string(),
                start_pos: start,
                end_pos: end,
                matching_string: matching.into(),
                is_observed: observed
            }
    }


    pub fn get_start_pos(&self) -> usize {
        self.start_pos
    }

    pub fn get_end_pos(&self) -> usize {
        self.end_pos
    }
}


impl std::fmt::Display for MinedTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinedTerm(id: {} [{}-{}]-observed {})", self.term_id, self.start_pos, self.end_pos, self.is_observed)
    }
}


#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_ctor() {
        let term_id: Result<TermId, _> = TermId::from_str("HP:0001250");
        let term_id = term_id.unwrap();
        let s = term_id.to_string();
        println!("{}", s)
    }


}

