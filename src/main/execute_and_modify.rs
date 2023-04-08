// src/main/execute_and_modify.rs

use buffer::Buffer;

fn main() {
    let mut buffer = Buffer::new();

    // Wczytywanie kodu sieci neuronowej do bufora
    // Zakładamy, że 'neural_network_code' jest tablicą bajtów reprezentujących kod
    match buffer.load_code(&neural_network_code) {
        Ok(()) => println!("Neural network code loaded successfully."),
        Err(e) => println!("Error loading neural network code: {}", e),
    }

    // Wykonanie kodu sieci neuronowej z bufora
    match buffer.execute() {
        Ok(()) => println!("Neural network code executed successfully."),
        Err(e) => println!("Error executing neural network code: {}", e),
    }

    // Modyfikacja kodu sieci neuronowej w buforze
    // Zakładamy, że 'start', 'end' i 'new_code' są właściwie ustawione
    match buffer.modify(start, end, &new_code) {
        Ok(()) => println!("Neural network code modified successfully."),
        Err(e) => println!("Error modifying neural network code: {}", e),
    }

    // Wykonanie zmodyfikowanego kodu sieci neuronowej z bufora
    match buffer.execute() {
        Ok(()) => println!("Modified neural network code executed successfully."),
        Err(e) => println!("Error executing modified neural network code: {}", e),
    }
}
