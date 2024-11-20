//! Core engine types and processing functionality

use std::collections::HashMap;

/// Main engine for processing keystrokes
pub struct InputEngine {
    buffer: String,
    mappings: HashMap<String, String>,
}

impl InputEngine {
    /// Create a new engine instance with basic mappings
    pub fn new() -> Self {
        let mut mappings = HashMap::new();
        // Add some basic mappings
        mappings.insert("k".to_string(), "ক".to_string());
        mappings.insert("kh".to_string(), "খ".to_string());
        mappings.insert("g".to_string(), "গ".to_string());
        mappings.insert("gh".to_string(), "ঘ".to_string());
        
        Self {
            buffer: String::new(),
            mappings,
        }
    }

    /// Process a single character input
    pub fn process_char(&mut self, c: char) -> Option<String> {
        self.buffer.push(c);
        
        // Try to find the longest matching mapping
        let buffer = self.buffer.as_str();
        for len in (1..=buffer.len()).rev() {
            if let Some(bengali) = self.mappings.get(&buffer[buffer.len()-len..]) {
                // Clear the matched portion from buffer
                self.buffer = buffer[..buffer.len()-len].to_string();
                return Some(bengali.clone());
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_input() {
        let mut engine = InputEngine::new();
        assert_eq!(engine.process_char('k'), Some("ক".to_string()));
        assert_eq!(engine.process_char('h'), Some("খ".to_string()));
    }
}