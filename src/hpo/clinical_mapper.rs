use ontolius::ontology::csr::{CsrOntology, MinimalCsrOntology};

use crate::{core_document::CoreDocument, mined_term::MinedTerm};

use super::{default_hpo_mapper::{self, DefaultHpoMapper}, hpo_concept_mapper::HpoConceptMapper, sentence_mapper::SentenceMapper};




pub struct ClinicalMapper {
    sentence_matcher: SentenceMapper
}



impl ClinicalMapper {

    pub fn new(hpo: MinimalCsrOntology) -> Self {
        let default_hpo_mapper = DefaultHpoMapper::new(hpo);
        let smatcher = SentenceMapper::new(default_hpo_mapper);
        ClinicalMapper{
            sentence_matcher: smatcher
        }
    }

    pub fn map_text(&mut self, text: &str) -> Vec<MinedTerm> {
        let core_document = CoreDocument::new(text);
        let sentences = core_document.get_sentences();
        let mut mapped_parts: Vec<MinedTerm> = Vec::new();
        for ss in sentences {
            let sentence_parts = self.sentence_matcher.map_sentence(ss.get_tokens());
            mapped_parts.extend(sentence_parts);
        }
        mapped_parts
    }
    
}