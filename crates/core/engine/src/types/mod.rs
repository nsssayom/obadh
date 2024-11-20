// crates/core/engine/src/types/mod.rs

//! Types used in the Bengali input engine

#[derive(Clone, Debug)]
pub enum BengaliChar {
    Vowel(char),
    Consonant(char),
    VowelSign(char),
    Special(char),
    Symbol(char),
    Compound(Vec<char>),
}

#[derive(Clone, Debug)]
pub struct ProcessingContext {
    pub previous: Option<BengaliChar>,
    pub previous_output: Option<String>,
    pub prevent_conjunct: bool,
}
