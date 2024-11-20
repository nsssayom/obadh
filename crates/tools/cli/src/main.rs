//! Obadh Debug Console
//! 
//! Interactive debugging and testing tool

mod commands;
mod debug;
mod repl;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Obadh Bengali IME - Debug Console v{}", env!("CARGO_PKG_VERSION"));
    
    let mut input = String::new();
    loop {
        print!("obadh> ");
        io::stdout().flush()?;
        
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        match input {
            "exit" | "quit" => break,
            "help" => show_help(),
            _ => process_input(input),
        }
    }
    
    Ok(())
}

fn show_help() {
    println!("Commands:");
    println!("  help    Show this help");
    println!("  exit    Exit the console");
    println!("  quit    Exit the console");
}

fn process_input(input: &str) {
    println!("Processing: {}", input);
}
