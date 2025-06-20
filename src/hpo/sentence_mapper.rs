use std::{cmp::min, collections::HashMap};

use crate::{hpo::partition::Partition, mined_term::MinedTerm, simple_sentence::SimpleSentence, simple_token::SimpleToken};

use super::default_hpo_mapper::DefaultHpoMapper;

use once_cell::sync::Lazy;
use std::collections::HashSet;

/// This is a set of words that we use to indentify exclusion (negation) of phenotypic abnormality
///
/// e.g. "Proband 1 did not have arachnodactyly" would be flagged as negated because of the word "not".
static NEGATION_CLUES: Lazy<HashSet<String>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("no".to_string());
    set.insert("nil".to_string());
    set.insert("denies".to_string());
    set.insert("not".to_string());
    set.insert("exclude".to_string());
    set.insert("excluded".to_string());
    set.insert("screen".to_string());
    set.insert("screening".to_string());
    set.insert("normal".to_string());
    set
});

pub struct SentenceMapper {
    hpo_mapper: DefaultHpoMapper,
}

impl SentenceMapper {
    pub fn new(mapper: DefaultHpoMapper) -> Self {
        SentenceMapper { hpo_mapper: mapper }
    }

    pub fn map_sentence(&self, simple_sentence: &SimpleSentence) -> Result<Vec<MinedTerm>, String> {
        let tokens: &[SimpleToken] = simple_sentence.get_tokens();
        let start_pos = simple_sentence.get_start_pos();
        let mut candidates: HashMap<usize, Vec<MinedTerm>> = HashMap::new();
        let max_partition_heuristic = min(10, tokens.len());
        let is_excluded = self.has_negation(tokens);
        for i in 1..=max_partition_heuristic {
            let partition = Partition::new(&tokens, i);
            for j in 0..partition.count() {
                let chunk = partition
                    .get(j)
                    .ok_or_else(|| format!("Error: Could not retrieve chunk at index {}", j))?;
                // the comparisons are all done in lower case, so we retrieve the lc version of the tokens
                let string_chunks: Vec<String> = chunk
                    .iter()
                    .map(|stoken| stoken.get_lc_original_token())
                    .map(|str| str.to_string())
                    .collect();
                match self.hpo_mapper.get_match(&string_chunks) {
                    Some(hpo_match) => {
                        let hpo_id = hpo_match.get_term_id();
                        let start_chunk = chunk.get(0);
                        let end_chunk = chunk.get(chunk.len() - 1);
                        if start_chunk.is_none() || end_chunk.is_none() {
                            continue; // should never happen
                        }
                        let startpos = start_chunk.unwrap().get_start_pos() + start_pos;
                        let endpos = end_chunk.unwrap().get_end_pos() + start_pos;
                        let matched = string_chunks.join(" ").clone();
                        let mapped_sentence_part = MinedTerm::new(
                            chunk.to_vec(),
                            hpo_id,
                            startpos..endpos,
                            matched,
                            !is_excluded,
                        );
                        //// insert a default value (empty vector) if the key is not present, then add the concept to the list
                        candidates
                            .entry(startpos)
                            .or_insert(Vec::new())
                            .push(mapped_sentence_part);
                    }
                    None => {} // do nothing if no match
                }
            }
        }
        // When we get here, we have zero, one, or more MappedSentenceParts.
        // Our heuristic is to take the longest match first
        // First get and sort the start positions
        let mut start_pos_list: Vec<usize> = candidates.keys().cloned().collect();
        start_pos_list.sort();
        let mut current_span = 0;
        let mut mapped_sentence_part_list = Vec::new();
        for i in start_pos_list {
            if i < current_span {
                continue;
            }
            let candidates_at_pos_i = candidates.get(&i);
            if candidates_at_pos_i.is_some() {
                let candidates_at_pos_i = candidates_at_pos_i.unwrap();
                let longest_match = candidates_at_pos_i
                    .iter()
                    .max_by(|&a, &b| a.get_span().end.cmp(&b.get_span().end));
                if longest_match.is_some() {
                    let longest_match = longest_match.unwrap();
                    current_span = longest_match.get_span().end;
                    mapped_sentence_part_list.push(longest_match.clone());
                    // advance to the last position of the current match
                    // note that this is String position convention, and so the next hist could start at
                    // currentSpan, but cannot be less than currentSpan without overlapping.
                }
            }
        }
        Ok(mapped_sentence_part_list)
    }

    fn has_negation(&self, tokens: &[SimpleToken]) -> bool {
        tokens
            .iter()
            .any(|token| NEGATION_CLUES.contains(&token.get_lc_original_token()))
    }
}


// region:    --- Tests

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use ontolius::TermId;
    use rstest::{fixture, rstest};

    use crate::hpo::hpo_concept::HpoConcept;

    use super::*;

    
#[fixture]
pub fn paramedian_cleft_palate() -> HpoConcept {
    let hpo_id = TermId::from_str("HP:0009099").unwrap();
    let label = "paramedian cleft lip";
    HpoConcept::new(label, hpo_id)
} 

#[fixture]
fn decreased_hc() -> HpoConcept {
    // Microcephaly HP:0000252
    let hpo_id = TermId::from_str("HP:0040195").unwrap();
    let label = "Decreased head circumference";
    HpoConcept::new(label, hpo_id)
}

#[fixture]
fn component_token_to_concept_map(
    decreased_hc: HpoConcept,
    paramedian_cleft_palate: HpoConcept
) -> HashMap<String, Vec<HpoConcept>> {
    let mut map: HashMap<String, Vec<HpoConcept>> = HashMap::new();
    let dch = vec![decreased_hc];
    for token in vec!["Decreased", "head", "circumference"] {
        map.insert(token.to_string(), dch.clone());
    };
    let pcp = vec![paramedian_cleft_palate];
    for token in vec!["paramedian", "cleft", "lip"] {
        map.insert(token.to_string(), pcp.clone());
    };
    map
}



#[rstest]
fn paramedian_cp(
    component_token_to_concept_map:HashMap<String, Vec<HpoConcept>>,
    paramedian_cleft_palate: HpoConcept
)  {
    let result = component_token_to_concept_map.get("cleft");
    assert!(result.is_some());
    let hpo_concept_list = result.unwrap();
    assert_eq!(1, hpo_concept_list.len());
    let hpo_concept = hpo_concept_list[0].clone();
    let expected_term_id: TermId = paramedian_cleft_palate.get_hpo_id();
    let observed_term_id: TermId = hpo_concept.get_hpo_id();
    assert_eq!(&expected_term_id, &observed_term_id);
}


    
}

// endregion: --- Tests