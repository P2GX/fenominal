mod common;

use std::sync::Arc;

use ontolius::ontology::csr::FullCsrOntology;
use rfenominal::{fenominal::{Fenominal, FenominalHit}, TextMiner};
use rstest::rstest;
use common::hpo;


const para1: &str = r#"This patient was a 2-year-old boy, the second child of nonconsanguineous parents of German and Dutch/Polish origin, respectively. The further family history was unremarkable. Pregnancy was complicated by fetal hydronephrosis and bilateral dilated ureter. The boy was delivered spontaneously at 40 weeks of gestation with a length of 53 cm (mean), weight of 4,510 g (+2.0 SD), and occipitofrontal head circumference (OFC) of 38.5 cm (+2.2 SD). The Apgar scores were 8, 10, and 10 at 1, 5, and 10 min, respectively, and the umbilical arterial cord pH of 7.19 was normal. Physical examination showed a median cleft palate which had been surgically corrected in the first months of life. Abdominal ultrasound confirmed bilateral dilated ureter and showed in addition ureteral ectopia requiring surgical therapy."#;

/// We expect hydronephrosis	HP:0000126
/// median cleft palate	HP:0009099

#[rstest]
fn test_parse_para_1(
    hpo: FullCsrOntology
) {
    let fenominal = Fenominal::from(&hpo);
    let fenominal_hits: Vec<FenominalHit> = fenominal.process(para1);
    let hydronephrosis_start = para1.find("hydronephrosis").unwrap();
    let dilated_ureter_start = para1.find("dilated ureter").unwrap();
    for h in &fenominal_hits {
        println!("{:?}",h);
    }
    
    // pretty-print the JSON response
    let pretty_fenominal_hits = serde_json::to_string_pretty(&fenominal_hits).unwrap();
    println!("[INFO] Hits:\n{}", &pretty_fenominal_hits);
    let hit1 = fenominal_hits.get(0).unwrap();
    assert_eq!(hydronephrosis_start, hit1.span.start);
}