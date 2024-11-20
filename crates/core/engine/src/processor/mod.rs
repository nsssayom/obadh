// crates/core/engine/src/processor/mod.rs

use crate::types::{BengaliChar, ProcessingContext};
use std::collections::HashMap;

pub struct Processor {
    mappings: HashMap<String, Vec<BengaliChar>>,
    context: ProcessingContext,
}

impl Processor {
    pub fn new() -> Self {
        let mut mappings: HashMap<String, Vec<BengaliChar>> = HashMap::new();

        // Vowels and Vowel Signs according to Avro mappings
        mappings.insert("o".to_string(), vec![BengaliChar::Vowel('অ')]);
        mappings.insert(
            "a".to_string(),
            vec![BengaliChar::Vowel('আ'), BengaliChar::VowelSign('া')],
        );
        mappings.insert(
            "i".to_string(),
            vec![BengaliChar::Vowel('ই'), BengaliChar::VowelSign('ি')],
        );
        mappings.insert(
            "I".to_string(),
            vec![BengaliChar::Vowel('ঈ'), BengaliChar::VowelSign('ী')],
        );
        mappings.insert(
            "u".to_string(),
            vec![BengaliChar::Vowel('উ'), BengaliChar::VowelSign('ু')],
        );
        mappings.insert(
            "U".to_string(),
            vec![BengaliChar::Vowel('ঊ'), BengaliChar::VowelSign('ূ')],
        );
        mappings.insert(
            "rri".to_string(),
            vec![BengaliChar::Vowel('ঋ'), BengaliChar::VowelSign('ৃ')],
        );
        mappings.insert(
            "e".to_string(),
            vec![BengaliChar::Vowel('এ'), BengaliChar::VowelSign('ে')],
        );
        mappings.insert(
            "OI".to_string(),
            vec![BengaliChar::Vowel('ঐ'), BengaliChar::VowelSign('ৈ')],
        );
        mappings.insert(
            "O".to_string(),
            vec![BengaliChar::Vowel('ও'), BengaliChar::VowelSign('ো')],
        );
        mappings.insert(
            "OU".to_string(),
            vec![BengaliChar::Vowel('ঔ'), BengaliChar::VowelSign('ৌ')],
        );

        // Consonants (both uppercase and lowercase as per Avro mappings)
        mappings.extend(Self::create_consonant_mappings());

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

    /// Creates consonant mappings including uppercase and lowercase letters
    fn create_consonant_mappings() -> HashMap<String, Vec<BengaliChar>> {
        let mut consonant_mappings = HashMap::new();

        // Avro consonant mappings
        let consonants = vec![
            ("k", 'ক'),
            ("kh", 'খ'),
            ("g", 'গ'),
            ("gh", 'ঘ'),
            ("Ng", 'ঙ'),
            ("c", 'চ'),
            ("ch", 'ছ'),
            ("j", 'জ'),
            ("jh", 'ঝ'),
            ("NG", 'ঞ'),
            ("T", 'ট'),
            ("Th", 'ঠ'),
            ("D", 'ড'),
            ("Dh", 'ঢ'),
            ("N", 'ণ'),
            ("t", 'ত'),
            ("th", 'থ'),
            ("d", 'দ'),
            ("dh", 'ধ'),
            ("n", 'ন'),
            ("p", 'প'),
            ("ph", 'ফ'),
            ("f", 'ফ'),
            ("b", 'ব'),
            ("bh", 'ভ'),
            ("v", 'ভ'),
            ("m", 'ম'),
            ("z", 'য'),
            ("r", 'র'),
            ("l", 'ল'),
            ("sh", 'শ'),
            ("S", 'শ'),
            ("Sh", 'ষ'),
            ("s", 'স'),
            ("h", 'হ'),
            ("R", 'ড়'),
            ("Rh", 'ঢ়'),
            ("y", 'য়'),
        ];

        for &(roman, bengali_char) in &consonants {
            consonant_mappings.insert(
                roman.to_string(),
                vec![BengaliChar::Consonant(bengali_char)],
            );
        }

        consonant_mappings
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let mut output = String::new();
        let mut pending_consonant: Option<String> = None;
        let mut index = 0;
        let chars: Vec<char> = input.chars().collect();

        while index < chars.len() {
            let mut matched = false;

            // Handle spaces and punctuation
            if is_punctuation(chars[index]) {
                if let Some(consonant_str) = pending_consonant.take() {
                    output.push_str(&self.process_pending_consonant(&consonant_str, false));
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

                // Handle symbol transliteration with backslash
                if test_str.starts_with('\\') {
                    if let Some(consonant_str) = pending_consonant.take() {
                        output.push_str(&self.process_pending_consonant(&consonant_str, false));
                    }
                    let bengali_chars_option = self.mappings.get(&test_str).cloned();
                    if let Some(bengali_chars) = bengali_chars_option {
                        for bengali_char in bengali_chars {
                            output.push(bengali_char.to_char());
                        }
                        index += i;
                        matched = true;
                        self.context.previous = None;
                        break;
                    } else {
                        // Output the backslash and continue
                        output.push('\\');
                        index += 1;
                        matched = true;
                        break;
                    }
                }

                // Case-insensitive matching for consonants
                let test_str_lower = test_str.to_lowercase();

                let bengali_chars_option = if let Some(bengali_chars) = self.mappings.get(&test_str)
                {
                    Some(bengali_chars.clone())
                } else if let Some(bengali_chars) = self.mappings.get(&test_str_lower) {
                    if self.is_case_sensitive_consonant(&test_str) {
                        None
                    } else {
                        Some(bengali_chars.clone())
                    }
                } else {
                    None
                };

                if let Some(bengali_chars) = bengali_chars_option {
                    matched = true;
                    index += i;

                    // Special handling for 'o' as combining blocker
                    if test_str == "o" && pending_consonant.is_some() {
                        // Set prevent_conjunct flag to true
                        self.context.prevent_conjunct = true;
                        // Do not output 'অ' here; it is implicit
                        break;
                    }

                    self.handle_bengali_chars(
                        &bengali_chars,
                        &mut pending_consonant,
                        &mut output,
                        test_str.as_str(),
                    );
                    break;
                }
            }

            if !matched {
                // Output any pending consonant before handling the unmatched character
                if let Some(consonant_str) = pending_consonant.take() {
                    output.push_str(&self.process_pending_consonant(&consonant_str, false));
                }
                // Output the unmatched character as-is
                output.push(chars[index]);
                index += 1;
                self.context.previous = None;
            }
        }

        // Output any pending consonant at the end
        if let Some(consonant_str) = pending_consonant.take() {
            output.push_str(&self.process_pending_consonant(&consonant_str, false));
        }

        output
    }

    fn handle_bengali_chars(
        &mut self,
        bengali_chars: &Vec<BengaliChar>,
        pending_consonant: &mut Option<String>,
        output: &mut String,
        input_str: &str,
    ) {
        let bengali_char = self.select_bengali_char(bengali_chars);

        match bengali_char {
            BengaliChar::Consonant(ch) => {
                if let Some(prev_consonant) = pending_consonant.take() {
                    if self.context.prevent_conjunct {
                        // Output previous consonant with inherent vowel
                        output.push_str(&self.process_pending_consonant(&prev_consonant, true));
                        self.context.prevent_conjunct = false;
                    } else {
                        // Insert hasanta to form conjunct
                        output.push_str(&format!("{}্", prev_consonant));
                    }
                }
                pending_consonant.replace(ch.to_string());
                self.context.previous = Some(BengaliChar::Consonant(ch));
            }
            BengaliChar::VowelSign(ch) => {
                if let Some(consonant_str) = pending_consonant.take() {
                    let combined = format!("{}{}", consonant_str, ch);
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
                if let Some(consonant_str) = pending_consonant.take() {
                    output.push_str(&self.process_pending_consonant(&consonant_str, false));
                }
                output.push(ch);
                self.context.previous = None;
            }
            BengaliChar::Special(ch) | BengaliChar::Symbol(ch) => {
                if let Some(consonant_str) = pending_consonant.take() {
                    output.push_str(&self.process_pending_consonant(&consonant_str, false));
                }
                output.push(ch);
                self.context.previous = None;
            }
            BengaliChar::Compound(chars) => {
                if let Some(consonant_str) = pending_consonant.take() {
                    output.push_str(&self.process_pending_consonant(&consonant_str, false));
                }
                let compound_str: String = chars.iter().collect();
                output.push_str(&compound_str);
                self.context.previous = None;
            }
        }
    }

    fn process_pending_consonant(&self, consonant_str: &str, with_inherent_vowel: bool) -> String {
        if with_inherent_vowel {
            consonant_str.to_string()
        } else {
            // Output consonant with inherent vowel (default behavior)
            consonant_str.to_string()
        }
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

    fn is_case_sensitive_consonant(&self, s: &str) -> bool {
        // Consonants where case matters (e.g., 't' vs 'T')
        let case_sensitive_consonants = vec!["t", "T", "d", "D", "n", "N", "s", "S", "r", "R"];
        case_sensitive_consonants.contains(&s)
    }
}

fn is_punctuation(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_whitespace()
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