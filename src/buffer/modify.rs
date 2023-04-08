// src/buffer/modify.rs

use super::Buffer;

impl Buffer {
    // Modyfikowanie kodu w buforze
    pub fn modify(&mut self, start: usize, end: usize, new_code: &[u8]) -> Result<(), String> {
        if end < start || start > self.cursor {
            return Err("Invalid modification range.".to_string());
        }

        if end - start != new_code.len() {
            return Err("New code length does not match modification range.".to_string());
        }

        self.data[start..end].copy_from_slice(new_code);
        Ok(())
    }
}
