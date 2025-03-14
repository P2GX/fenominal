mod core_document;
pub mod fenominal;
mod fenominal_traits;
mod hpo;
mod mined_term;
mod simple_sentence;
mod simple_token;
mod stopwords;

/// Text miner performs named entity recognition in the provided text.
///
/// `O` text mining output type.
/// For instance, [`ontolius::TermId`] for a text miner that finds a "bag of terms".
/// However, the output can be more compex type if e.g. coordinates
/// or observation state (present/excluded) is available.
pub trait TextMiner<O> {
    /// Find terms in the provided `text`.
    fn process(&self, text: &str) -> Vec<O>;
}
