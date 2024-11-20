// crates/core/engine/src/processor.rs

use std::collections::HashMap;
use crate::types::{BengaliChar, ProcessingContext};

pub struct Processor {
    mappings: HashMap<String, Vec<BengaliChar>>,
    context: ProcessingContext,
}

impl Processor {
    pub fn new() -> Self {
        let mut mappings: HashMap<String, Vec<BengaliChar>> = HashMap::new();

        // Vowels and Vowel Signs according to Avro mappings
        mappings.insert("o".to_string(), vec![BengaliChar::Vowel('অ')]);
        mappings.insert("a".to_string(), vec![BengaliChar::Vowel('আ'), BengaliChar::VowelSign('া')]);
        mappings.insert("i".to_string(), vec![BengaliChar::Vowel('ই'), BengaliChar::VowelSign('ি')]);
        mappings.insert("I".to_string(), vec![BengaliChar::Vowel('ঈ'), BengaliChar::VowelSign('ী')]);
        mappings.insert("u".to_string(), vec![BengaliChar::Vowel('উ'), BengaliChar::VowelSign('ু')]);
        mappings.insert("U".to_string(), vec![BengaliChar::Vowel('ঊ'), BengaliChar::VowelSign('ূ')]);
        mappings.insert("rri".to_string(), vec![BengaliChar::Vowel('ঋ'), BengaliChar::VowelSign('ৃ')]);
        mappings.insert("e".to_string(), vec![BengaliChar::Vowel('এ'), BengaliChar::VowelSign('ে')]);
        mappings.insert("OI".to_string(), vec![BengaliChar::Vowel('ঐ'), BengaliChar::VowelSign('ৈ')]);
        mappings.insert("O".to_string(), vec![BengaliChar::Vowel('ও'), BengaliChar::VowelSign('ো')]);
        mappings.insert("OU".to_string(), vec![BengaliChar::Vowel('ঔ'), BengaliChar::VowelSign('ৌ')]);

        // Consonants
        mappings.insert("k".to_string(), vec![BengaliChar::Consonant('ক')]);
        mappings.insert("kk".to_string(), vec![BengaliChar::Compound(vec!['ক', '্', 'ক'])]); // Conjunct ক্ক
        // Add other consonants and conjuncts as needed...

        // Special Characters with Modifier '\'
        mappings.insert("\\^".to_string(), vec![BengaliChar::Special('ঁ')]); // Chandrabindu
        mappings.insert("\\`".to_string(), vec![BengaliChar::Special('্')]); // Hasanta
        mappings.insert("\\\\".to_string(), vec![BengaliChar::Symbol('\\')]); // Backslash
        mappings.insert("\\$".to_string(), vec![BengaliChar::Special('৳')]); // Taka symbol

        Self {
            mappings,
            context: ProcessingContext {
                previous: None,
                previous_output: None,
                prevent_conjunct: false,
            },
        }
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let mut output = String::new();
        let mut pending_consonant: Option<char> = None;
        let mut index = 0;
        let chars: Vec<char> = input.chars().collect();

        while index < chars.len() {
            let mut matched = false;

            // Handle spaces and punctuation
            if is_punctuation(chars[index]) {
                if let Some(consonant_char) = pending_consonant.take() {
                    output.push(consonant_char);
                }
                output.push(chars[index]);
                index += 1;
                self.context.previous = None;
                continue;
            }

            // Try to find the longest possible match
            for i in (1..=5).rev() {
                if index + i > chars.len() {
                    continue;
                }

                let test_str: String = chars[index..index + i].iter().collect();

                if let Some(bengali_chars) = self.mappings.get(&test_str) {
                    matched = true;
                    index += i;

                    // Handle symbol transliteration with backslash
                    if test_str.starts_with('\\') {
                        if let Some(consonant_char) = pending_consonant.take() {
                            output.push(consonant_char);
                        }
                        for bengali_char in bengali_chars {
                            output.push(bengali_char.to_char());
                        }
                        self.context.previous = None;
                        break;
                    }

                    // Special handling for 'o' as combining stopper
                    if test_str == "o" && pending_consonant.is_some() && index < chars.len() && is_consonant_char(chars[index]) {
                        // 'o' as combining stopper between consonants
                        if let Some(consonant_char) = pending_consonant.take() {
                            output.push(consonant_char);
                        }
                        output.push('অ'); // 'o' maps to 'অ'
                        self.context.previous = None;
                        break;
                    }

                    let bengali_char = self.select_bengali_char(bengali_chars);

                    match bengali_char {
                        BengaliChar::Consonant(ch) => {
                            if let Some(prev_consonant) = pending_consonant.take() {
                                // Attempt to form conjunct
                                let conjunct_key = format!("{}{}", prev_consonant, test_str);
                                if let Some(conjunct_chars) = self.mappings.get(&conjunct_key) {
                                    // Output the conjunct
                                    let conjunct_char = &conjunct_chars[0];
                                    if let BengaliChar::Compound(compound_chars) = conjunct_char {
                                        output.push_str(&compound_chars.iter().collect::<String>());
                                        self.context.previous = None;
                                        break;
                                    }
                                } else {
                                    // No conjunct found, output previous consonant
                                    output.push(prev_consonant);
                                }
                            }
                            pending_consonant = Some(ch);
                            self.context.previous = Some(BengaliChar::Consonant(ch));
                        }
                        BengaliChar::VowelSign(ch) => {
                            if let Some(prev_ch) = pending_consonant.take() {
                                let combined = format!("{}{}", prev_ch, ch);
                                output.push_str(&combined);
                            } else {
                                // Vowel sign without consonant, treat as independent vowel
                                if let Some(ind_vowel) = self.vowel_sign_to_independent_vowel(ch) {
                                    output.push(ind_vowel);
                                } else {
                                    output.push(ch);
                                }
                            }
                            self.context.previous = None;
                        }
                        BengaliChar::Vowel(ch) => {
                            if let Some(consonant_char) = pending_consonant.take() {
                                output.push(consonant_char);
                            }
                            output.push(ch);
                            self.context.previous = None;
                        }
                        BengaliChar::Special(ch) => {
                            if let Some(consonant_char) = pending_consonant.take() {
                                output.push(consonant_char);
                            }
                            output.push(ch);
                            self.context.previous = None;
                        }
                        BengaliChar::Symbol(ch) => {
                            if let Some(consonant_char) = pending_consonant.take() {
                                output.push(consonant_char);
                            }
                            output.push(ch);
                            self.context.previous = None;
                        }
                        BengaliChar::Compound(chars) => {
                            if let Some(consonant_char) = pending_consonant.take() {
                                output.push(consonant_char);
                            }
                            let compound_str: String = chars.iter().collect();
                            output.push_str(&compound_str);
                            self.context.previous = None;
                        }
                    }

                    break;
                }
            }

            if !matched {
                // Output any pending consonant before handling the unmatched character
                if let Some(consonant_char) = pending_consonant.take() {
                    output.push(consonant_char);
                }
                // Output the unmatched character as-is
                output.push(chars[index]);
                index += 1;
                self.context.previous = None;
            }
        }

        // Output any pending consonant at the end
        if let Some(consonant_char) = pending_consonant.take() {
            output.push(consonant_char);
        }

        output
    }

    fn select_bengali_char(&self, bengali_chars: &Vec<BengaliChar>) -> BengaliChar {
        // Implement selection logic based on context
        // If previous is a consonant, prefer VowelSign
        if let Some(BengaliChar::Consonant(_)) = &self.context.previous {
            for bengali_char in bengali_chars {
                if matches!(bengali_char, BengaliChar::VowelSign(_)) {
                    return bengali_char.clone();
                }
            }
        }
        // Otherwise, prefer Vowel
        for bengali_char in bengali_chars {
            if matches!(bengali_char, BengaliChar::Vowel(_)) {
                return bengali_char.clone();
            }
        }
        // Return the first available BengaliChar
        bengali_chars[0].clone()
    }

    fn vowel_sign_to_independent_vowel(&self, vowel_sign: char) -> Option<char> {
        match vowel_sign {
            'া' => Some('আ'),
            'ি' => Some('ই'),
            'ী' => Some('ঈ'),
            'ু' => Some('উ'),
            'ূ' => Some('ঊ'),
            'ৃ' => Some('ঋ'),
            'ে' => Some('এ'),
            'ৈ' => Some('ঐ'),
            'ো' => Some('ও'),
            'ৌ' => Some('ঔ'),
            _ => None,
        }
    }
}

fn is_punctuation(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_whitespace()
}

fn is_consonant_char(c: char) -> bool {
    // Add logic to determine if a character is a consonant in your mappings
    // For simplicity, we'll check if it's in the mappings as a consonant
    let consonants = vec!['k', 'g', 'c', 'j', 't', 'd', 'n', 'p', 'b', 'm', 'y', 'r', 'l', 's', 'h'];
    consonants.contains(&c)
}

impl BengaliChar {
    fn to_char(&self) -> char {
        match self {
            BengaliChar::Vowel(ch)
            | BengaliChar::Consonant(ch)
            | BengaliChar::VowelSign(ch)
            | BengaliChar::Special(ch)
            | BengaliChar::Symbol(ch) => *ch,
            BengaliChar::Compound(_) => '\0', // Not applicable for single char conversion
        }
    }
}
