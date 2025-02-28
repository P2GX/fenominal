use clap::Parser;
use ontolius::base::TermId;
use ontolius::{io::OntologyLoaderBuilder, ontology::csr::MinimalCsrOntology};
use std::collections::HashMap;
use std::path::PathBuf;
use ferriphene::hpo::fenominal::{self, Fenominal};



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
    println!("Processing HPO JSON file: {:?}", hp_json_path);
    println!("Input string: {}", input_string);
    let fenominal = Fenominal::new(hp_json_path_str);
    let json = fenominal.map_text_to_json(&input_string);
    println!("{}", &json);
}

