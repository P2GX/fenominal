use clap::Parser;
use std::path::PathBuf;
use ferriphene::fenominal::Fenominal;



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
    println!("Processing HPO JSON file: {:?}", hp_json_path);
    println!("Input string: {}", input_string);
    let fenominal = Fenominal::new(hp_json_path_str);
    let json = fenominal.map_text_to_json(&input_string);
    println!("{}", &json);
}

