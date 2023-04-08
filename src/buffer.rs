// src/buffer.rs

use std::io;
use std::mem;

// Definicja struktury bufora o wielkości 8GB
pub struct Buffer {
    data: [u8; 8 * 1024 * 1024 * 1024],
    cursor: usize,
}

impl Buffer {
    // Inicjalizacja bufora
    pub fn new() -> Self {
        Buffer {
            data: [0; 8 * 1024 * 1024 * 1024],
            cursor: 0,
        }
    }

    // Wczytywanie kodu do bufora
    pub fn load_code(&mut self, code: &[u8]) -> Result<(), String> {
        if self.cursor + code.len() > self.data.len() {
            return Err("Insufficient buffer space.".to_string());
        }

        self.data[self.cursor..self.cursor + code.len()].copy_from_slice(code);
        self.cursor += code.len();

        Ok(())
    }
}
    // !!!
