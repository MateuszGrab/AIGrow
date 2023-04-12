mod compiler;
mod aigrow;

mod buffer;
use buffer::Buffer;

use std::io::Read;
use std::path::Path;
use std::env;
use std::fs;
use std::io::{self, Write};
use aigrow::functions::create_aigrow_keywords;
use compiler::lexer::tokenize;
use compiler::parser::{Parser, Stmt};

fn tokens_to_yaml_string(tokens: &[compiler::lexer::Token]) -> String {
    // Implementacja funkcji konwertującej tokeny na YAML (wcześniej opisane)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <folder_path> [--generate]", args[0]);
        std::process::exit(1);
    }

    let folder_path = &args[1];
    let generate_flag = &args[2];

    if generate_flag != "--generate" {
        eprintln!("Invalid flag. Usage: {} <folder_path> [--generate]", args[0]);
        std::process::exit(1);
    }

    let entries = fs::read_dir(folder_path).expect("Failed to read directory");

    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let file_path = entry.path();
                    let mut file = fs::File::open(&file_path).expect("Failed to open file");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).expect("Failed to read file contents");

                    let tokens = tokenize(&contents);
                    let yaml_string = tokens_to_yaml_string(&tokens);

                    println!("YAML for file {:?}:\n{}", file_path, yaml_string);
                }
            }
        }
    }
    // Sprawdzenie, czy plik ma rozszerzenie .aig
    let input_file = &args[1];
    if !input_file.ends_with(".aig") {
        eprintln!("Błąd: plik źródłowy musi mieć rozszerzenie .aig");
        return;
    }

    // Wczytywanie pliku źródłowego AIGrow
    let source_code = match fs::read_to_string(input_file) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Błąd podczas wczytywania pliku: {}", e);
            return;
        }
    };

    let aigrow_keywords = create_aigrow_keywords();

    for keyword in aigrow_keywords {
        println!(
            "Keyword: {:?}, Abbreviation: {}, Description: {}",
            keyword.keyword, keyword.abbreviation, keyword.description
        );
    }

    let code = "pf func_name(a: Number, b: Number) => Number;";
    let tokens = tokenize(code);
    let mut parser = Parser::new(&tokens);
    let result = parser.parse();

    let mut buffer = Buffer::new();

    // Wczytywanie kodu sieci neuronowej do bufora
    // Zakładamy, że 'neural_network_code' jest tablicą bajtów reprezentujących kod
    match buffer.load_code(&neural_network_code) {
        Ok(()) => println!("Neural network code loaded successfully."),
        Err(e) => println!("Error loading neural network code: {}", e),
    }

    match result {
        Ok(statements) => println!("{:?}", statements),
        Err(e) => eprintln!("Error parsing the code: {}", e),
    }
}
