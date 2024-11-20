//! ime - Part of the Obadh Bengali Input Method
//!
//! This module provides core functionality for the input method engine.

pub mod error;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert!(true);
    }
}
