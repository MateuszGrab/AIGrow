mod compiler;
mod aigrow;

use std::env;
use std::fs;
use std::io::{self, Write};
use compiler::{Parser, Optimizer, CodeGenerator};
use aigrow::{AIGrowTypes, AIGrowFunctions};

fn main() {
    // Read AIGrow source code file from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        return;
    }
    
    // Read AIGrow source code from file
    let source_code = match fs::read_to_string(&args[1]) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    // Parse AIGrow source code
    let mut parser = Parser::new(&source_code);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Error parsing source code: {}", e);
            return;
        }
    };

    // Optimize the AST for efficient TensorFlow execution
    let mut optimizer = Optimizer::new(ast);
    let optimized_ast = optimizer.optimize();

    // Generate TensorFlow-compatible code from the optimized AST
    let code_generator = CodeGenerator::new(optimized_ast);
    let tensorflow_code = code_generator.generate_code();

    // Output the generated TensorFlow code
    let mut stdout = io::stdout();
    if let Err(e) => {
        eprintln!("Error writing output: {}", e);
        return;
    }
    stdout.write_all(tensorflow_code.as_bytes()).unwrap();
}
