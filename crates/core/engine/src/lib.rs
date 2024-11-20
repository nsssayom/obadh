// crates/core/engine/src/lib.rs
pub mod error;
pub mod types;
pub mod utils;
pub mod processor;

// Re-export main types
pub use processor::Processor;