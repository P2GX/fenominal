use clap::Parser;
use ontolius::base::TermId;
use ontolius::{io::OntologyLoaderBuilder, ontology::csr::MinimalCsrOntology};
use std::collections::HashMap;
use std::path::PathBuf;
use ferriphene::hpo::clinical_mapper::ClinicalMapper;
use ferriphene::hpo::simple_hpo_parser::SimpleHpoParser;
use ferriphene::fenominal_traits::TermIdToTextMapper;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "A simple CLI test program")]
struct Args {
   /// Path to the file
   #[arg(long, value_name = "FILE")]
   hp: PathBuf,

   /// Input string
   #[arg(short, long, value_name = "STRING")]
   input: String,
}

fn main() {
    println!("Running test program...");
    let args = Args::parse();
    let hp_json_path = args.hp;
    let hp_json_path_str: &str = hp_json_path.to_str().expect("Invalid UTF-8 in path");
    let input_string = args.input;
    println!("Input string: {}", input_string);
    let use_ontolius = false;
    let clin_mapper = match use_ontolius {
        true => get_clinical_matcher_ontolius(hp_json_path_str),
        false => get_clinical_matcher_simple(hp_json_path_str)
    };
    match clin_mapper {
        Ok(mut clmap) => {
            let matching = clmap.map_text(&input_string);
            for m in matching {
                println!("{}", m);
            }
            println!("The equivalent with JSON ");
            let json_string = clmap.map_text_to_json(&input_string);
            print!("{}", json_string);
        },
        _ => println!("Could not initialize clinical mapper")
    }
   
    
    
   
}


fn get_clinical_matcher_simple(hp_json_path: &str) -> Result<ClinicalMapper, String> {
    let simple_mapper = SimpleHpoParser::new(hp_json_path)?;
    let t2tmap: HashMap<String, TermId> = simple_mapper.get_text_to_term_map();
    let mut clinical_mapper = ClinicalMapper::from_map(&t2tmap);
    Ok(clinical_mapper)
}


fn get_clinical_matcher_ontolius(hp_json_path: &str) -> Result<ClinicalMapper, String> {
    let loader = OntologyLoaderBuilder::new()
    .obographs_parser()
    .build();
    println!("Processing file: {:?}", hp_json_path);
    let hpo: MinimalCsrOntology = loader.load_from_path(hp_json_path)
                        .expect("HPO could not be loaded");
    
    let mut clinical_mapper = ClinicalMapper::new(hpo);
    Ok(clinical_mapper)
}