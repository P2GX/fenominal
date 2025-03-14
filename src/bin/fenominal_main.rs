use clap::Parser;
use rfenominal::fenominal::Fenominal;
use serde_json::Value;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version = "0.1.6", about = "Fenominal implementation in Rust")]
struct Args {
    /// Path to the file
    #[arg(long, value_name = "FILE")]
    hp: PathBuf,

    /// Input string
    #[arg(short, long, value_name = "STRING")]
    input: String,
}

fn main() {
    let args = Args::parse();
    let hp_json_path = args.hp;
    let hp_json_path_str: &str = hp_json_path.to_str().expect("Invalid UTF-8 in path");
    let input_string = args.input;
    let hpo_path = Path::new(hp_json_path_str);
    if hpo_path.exists() {
        println!("[INFO] Processing HPO JSON file: {:?}.", hp_json_path);
    } else {
        eprintln!(
            "[ERROR] Could not find HPO JSON file at {}.",
            hp_json_path_str
        );
        return;
    }
    println!("[INFO] Input string: {}", input_string);
    let fenominal = Fenominal::new(hp_json_path_str);
    let fenominal_hits = fenominal.map_text_to_json(&input_string);
    // pretty-print the JSON response
    let parsed: Value = serde_json::from_str(&fenominal_hits).unwrap();
    let pretty_fenominal_hits = serde_json::to_string_pretty(&parsed).unwrap();
    println!("[INFO] Hits:\n{}", &pretty_fenominal_hits);
}
