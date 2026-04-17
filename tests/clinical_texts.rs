mod common;

use std::sync::Arc;

use ontolius::ontology::csr::FullCsrOntology;
use fenominal::{fenominal::{Fenominal, FenominalHit}};
use rstest::rstest;
use common::hpo;


const PARA1: &str = r#"An 8-week-old female twin born at 36 weeks had a history of frequent  diarrhea  and  emesis , with feeds resulting in failure to thrive (FTT)."#;



#[rstest]
fn test_parse_para_1(
    hpo: Arc<FullCsrOntology>
) {
    let fenominal = Fenominal::new(hpo);
    let sanitized = fenominal::text_util::sanitize(PARA1);
    let fenominal_hits: Vec<FenominalHit> = fenominal.process(&sanitized);
    

    for h in &fenominal_hits {
        println!("{:?}",h);
    }
  
    
   
}