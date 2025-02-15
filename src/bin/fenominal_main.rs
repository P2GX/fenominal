use clap::Parser;
use ontolius::{io::OntologyLoaderBuilder, ontology::csr::MinimalCsrOntology};
use std::path::PathBuf;
use ferriphene::hpo::default_hpo_mapper::DefaultHpoMapper;


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
    let input_string = args.input;
    let loader = OntologyLoaderBuilder::new()
    .obographs_parser()
    .build();
    println!("Processing file: {:?}", hp_json_path);
    println!("Input string: {}", input_string);
    let hpo: MinimalCsrOntology = loader.load_from_path(hp_json_path)
                        .expect("HPO could not be loaded");
    let hp_matcher = DefaultHpoMapper::new(hpo);
    //let match = hp_matcher.g
    
    
   
}