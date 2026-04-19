mod common;

use std::sync::Arc;

use ontolius::ontology::csr::FullCsrOntology;
use fenominal::{fenominal::{Fenominal, FenominalHit}};
use rstest::rstest;
use common::hpo;


const SENTENCE_1: &str = r#"An 8-week-old female twin born at 36 weeks had a history of frequent  diarrhea  and  emesis , with feeds resulting in failure to thrive (FTT)."#;




#[rstest]
fn test_ftt(
    hpo: Arc<FullCsrOntology>
) {
    let text="Failure to thrive";
    let fenominal = Fenominal::new(hpo);
    let fenominal_hits: Vec<FenominalHit> = fenominal.process(&text);
    assert_eq!(1, fenominal_hits.len());
    let fhit = &fenominal_hits[0];
    assert_eq!("Failure to thrive", fhit.label);
}

#[rstest]
fn test_parse_para_1(
    hpo: Arc<FullCsrOntology>
) {
    let fenominal = Fenominal::new(hpo);
    let sanitized = fenominal::util::text_util::sanitize(SENTENCE_1);
    let fenominal_hits: Vec<FenominalHit> = fenominal.process(&sanitized);
    assert_eq!(3, fenominal_hits.len());
}