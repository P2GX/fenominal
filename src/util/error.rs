use ontolius::TermId;
use thiserror::Error;
use std::ops::Range;

#[derive(Debug, Error)]
pub enum FenominalError {
    #[error("parsing error: {reason}")]
    Parsing { reason: String },

    #[error("ontology error: {reason}")]
    Ontology { reason: String },

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}


impl FenominalError {

    pub fn io_error(reason: impl Into<String>) -> Self {
        FenominalError::Io(std::io::Error::new(std::io::ErrorKind::Other, reason.into()))
    }


    pub fn term_retrieval_error(hpo_id: &TermId) -> Self {
        FenominalError::Ontology {
            reason: format!("could not retrieve term for {}", hpo_id),
        }
    }

    pub fn invalid_span(span: Range<usize>, text_len: usize, original_text: &str) -> Self {
        FenominalError::Parsing {
            reason: format!(
                "invalid span {:?} for text length {} in sentence: {}",
                span, text_len, original_text
            ),
        }
    }

}