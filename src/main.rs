mod compiler;
mod aigrow;

mod buffer;
use buffer::Buffer;

use std::env;
use std::fs;
use std::io::{self, Write};
use aigrow::functions::create_aigrow_keywords;
use aig_compiler::compiler::lexer::tokenize;
use aig_compiler::compiler::parser::{Parser, Stmt};

fn main() {
    // Przetwarzanie argumentów linii poleceń
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Użycie: {} <plik_źródłowy>", args[0]);
        return;
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

    match result {
        Ok(statements) => println,
        Err(_) => todo!(),
    }

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
